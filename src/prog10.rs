// Write the logic to first filter all odd values then double each value and create a new vector

fn solve(vec1: &Vec<i32>) -> Vec<i32> {
    let iter1 = vec1.iter();
    let new_vec: Vec<i32> = iter1.filter(|x| *x % 2 == 1).map(|x| x * 2).collect();

    new_vec
}

pub fn run() {
    let vec1: Vec<i32> = vec![4, 5, 9, 10, 12];
    println!("{:?}",solve(&vec1));
}