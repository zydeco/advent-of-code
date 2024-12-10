package main

import (
	"bufio"
	"fmt"
	"os"
)

type inode int32

const (
	Free inode = -1
)

func readInput() []uint8 {
	scanner := bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			panic("empty line")
		}
		items := make([]uint8, len(line))
		for i := 0; i < len(line); i++ {
			items[i] = line[i] - uint8('0')
		}
		return items
	}
	panic("no input")
}

func parseDiskMap(diskMap []uint8) []inode {
	// count number of blocks
	blkcnt := 0
	for _, value := range diskMap {
		blkcnt += int(value)
	}
	fmt.Println("Disk size:", blkcnt)

	// fill blocks array
	blocks := make([]inode, blkcnt)
	currentFile := inode(0)
	currentBlock := 0
	for idx, numBlocks := range diskMap {
		var thisFile inode
		if idx%2 == 1 {
			// free space
			thisFile = Free
		} else {
			thisFile = currentFile
			currentFile += 1
		}
		// mark blocks
		for i := 0; i < int(numBlocks); i++ {
			blocks[currentBlock] = thisFile
			currentBlock += 1
		}
	}
	return blocks
}

func checksum(disk []inode) uint64 {
	sum := uint64(0)
	for index, value := range disk {
		if value > 0 {
			sum += uint64(index) * uint64(value)
		}
	}
	return sum
}

func compact(disk []inode) {
	nextFree := nextFreeBlock(disk, 0)
	lastUsed := lastUsedBlock(disk, len(disk)-1)
	for nextFree < lastUsed {
		disk[nextFree] = disk[lastUsed]
		disk[lastUsed] = Free
		nextFree = nextFreeBlock(disk, nextFree)
		lastUsed = lastUsedBlock(disk, lastUsed)
	}
}

func nextFreeBlock(disk []inode, start int) int {
	for i := start; i < len(disk); i++ {
		if disk[i] == Free {
			return i
		}
	}
	panic("no free block found")
}

func lastUsedBlock(disk []inode, start int) int {
	for i := start; i >= 0; i-- {
		if disk[i] != Free {
			return i
		}
	}
	panic("no used block found")
}

func contiguousBlocks(disk []inode, start int) int {
	cnt := 0
	for i := start; i < len(disk) && disk[i] == disk[start]; i++ {
		cnt += 1
	}
	return cnt
}

func maxFile(disk []inode) inode {
	maxFile := Free
	for i := 0; i < len(disk); i++ {
		maxFile = max(maxFile, disk[i])
	}
	return maxFile
}

func findFile(disk []inode, fileNo inode) (int, int) {
	filePos := -1
	fileSize := 0
	for i, file := range disk {
		if file == fileNo {
			if filePos == -1 {
				filePos = i
			}
			fileSize += 1
		}
	}
	return filePos, fileSize
}

func findFreeSpace(disk []inode, size int, before int) int {
	freeBlock := nextFreeBlock(disk, 0)
	for freeBlock < before {
		freeSize := contiguousBlocks(disk, freeBlock)
		if freeSize >= size {
			return freeBlock
		}
		freeBlock = nextFreeBlock(disk, freeBlock+freeSize)
	}
	return -1
}

func moveFile(disk []inode, from int, to int, size int) {
	for i := 0; i < size; i++ {
		disk[to+i] = disk[from+i]
		disk[from+i] = Free
	}
}

func defrag(disk []inode) {
	for fileNo := maxFile(disk); fileNo > 0; fileNo-- {
		filePos, fileSize := findFile(disk, fileNo)
		if filePos == -1 {
			panic("file not found")
		}

		// find somewhere to put file
		target := findFreeSpace(disk, fileSize, filePos)
		if target != -1 {
			moveFile(disk, filePos, target, fileSize)
		}
	}
}

func main() {
	diskMap := readInput()
	blocks := parseDiskMap(diskMap)

	// part 1
	blocks2 := make([]inode, len(blocks))
	copy(blocks2, blocks)
	compact(blocks2)
	sum := checksum(blocks2)
	fmt.Println("Checksum part 1:", sum)

	// part 2
	copy(blocks2, blocks)
	defrag(blocks2)
	sum = checksum(blocks2)
	fmt.Println("Checksum part 2:", sum)
}
