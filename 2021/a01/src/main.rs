mod input;

fn main() {
    let x = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263].to_vec();
    let v = input::INPUT.to_vec();
    println!("increases {x}", x=increases(&v));
    println!("increas3s {x}", x=increases(&sums3(&v)));
}

fn sums3(arr: &Vec<u32>) -> Vec<u32>
{
    return arr.iter().enumerate().filter(|(idx, _)| *idx > 1 ).map(|(idx, _)| 
        if idx > 1 { arr[idx] + arr[idx-1] + arr[idx-2] } else { 0 }
    ).collect::<Vec<_>>();
}

fn increases(v: &Vec<u32>) -> u32
{
    return v.iter().enumerate().fold(0, |acc, (idx, val)| if idx > 0 && val > &v[idx-1] { acc + 1} else {acc} );
}
