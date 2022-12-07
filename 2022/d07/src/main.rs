use std::{
    fmt::Display,
    io::{self, BufRead},
    mem,
};

enum Entry {
    Directory { name: String, entries: Vec<Entry> },
    File { name: String, size: usize },
}

impl Entry {
    fn fmt_internal(&self, f: &mut std::fmt::Formatter<'_>, indent: usize) -> std::fmt::Result {
        match self {
            Self::Directory { name, entries } => {
                writeln!(f, "{:>width$}{}", "", name, width = indent)?;
                for entry in entries {
                    entry.fmt_internal(f, indent + 2)?;
                }
                Ok(())
            }
            Self::File { name, size } => {
                writeln!(f, "{:>width$}{} ({})", "", name, size, width = indent)
            }
        }
    }

    fn name(&self) -> &String {
        match self {
            Self::Directory { name, entries: _ } => name,
            Self::File { name, size: _ } => name,
        }
    }

    fn total_size(&self) -> usize {
        match self {
            Self::Directory { name: _, entries } => entries.iter().map(Self::total_size).sum(),
            Self::File { name: _, size } => *size,
        }
    }

    fn iter(&self) -> EntryIter<'_> {
        EntryIter {
            entries: std::slice::from_ref(self),
            parent: None,
        }
    }

    fn is_dir(&self) -> bool {
        match self {
            Self::Directory {
                name: _,
                entries: _,
            } => true,
            _ => false,
        }
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_internal(f, 0)
    }
}

struct EntryIter<'a> {
    entries: &'a [Entry],
    parent: Option<Box<EntryIter<'a>>>,
}

impl<'a> Iterator for EntryIter<'a> {
    type Item = &'a Entry;

    fn next(&mut self) -> Option<Self::Item> {
        match self.entries.get(0) {
            None => match self.parent.take() {
                Some(parent) => {
                    *self = *parent;
                    self.next()
                }
                None => None,
            },
            Some(entry) => {
                self.entries = &self.entries[1..];
                if let Entry::Directory { name: _, entries } = entry {
                    *self = EntryIter {
                        entries: entries.as_slice(),
                        parent: Some(Box::new(mem::take(self))),
                    }
                }
                Some(entry)
            }
        }
    }
}

impl Default for EntryIter<'_> {
    fn default() -> Self {
        EntryIter {
            entries: &[],
            parent: None,
        }
    }
}

fn read_input() -> Entry {
    let mut iter = io::stdin().lock().lines().filter_map(Result::ok);
    let first = iter.next();
    assert!(first.unwrap().eq(&"$ cd /"));
    read_dir(&mut iter, "/")
}

fn read_dir(iter: &mut dyn Iterator<Item = String>, name: &str) -> Entry {
    assert!(iter.next().unwrap().eq(&"$ ls"));
    let mut entries: Vec<Entry> = vec![];
    loop {
        let next = iter.next();
        if next.is_none() {
            break;
        }
        let item = next.unwrap();
        let words = item.split(' ').collect::<Vec<_>>();
        if words.len() < 2 {
            break;
        } else if words[0].eq("$") {
            // command
            if words[1].eq("cd") {
                let arg = words[2];
                if arg.eq("..") {
                    break;
                } else {
                    entries.push(read_dir(iter, arg));
                }
            }
        } else if words[0].eq("dir") {
            // directory
            // ignore and wait for cd/ls
        } else {
            // file
            entries.push(Entry::File {
                name: words[1].into(),
                size: words[0].parse::<usize>().unwrap(),
            })
        }
    }
    Entry::Directory {
        name: String::from(name),
        entries: entries,
    }
}

fn main() {
    let root = read_input();
    println!("{}", root);
    println!("total size {}", root.total_size());

    // part 1
    let part1: usize = root
        .iter()
        .filter(|e| e.is_dir())
        .map(|e| e.total_size())
        .filter(|s| *s <= 100000usize)
        .sum();
    println!("part1: {}", part1);

    // part 2
    let fs_size = 70000000usize;
    let need_size = 30000000usize;
    let free_size = fs_size - root.total_size();
    let need_to_free = need_size - free_size;
    println!("need to free: {}", need_to_free);

    let part2 = root
        .iter()
        .filter(|e| e.is_dir())
        .map(|e| e.total_size())
        .filter(|s| *s >= need_to_free)
        .min()
        .unwrap();
    println!("size to free: {}", part2);
}
