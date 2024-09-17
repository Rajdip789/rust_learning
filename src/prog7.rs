//Write a function that takes a vector as an input and returns a vector with even values

fn even_values_1(vec : &Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();

    for val in vec {
        if val % 2 == 0 {
            new_vec.push(*val);
        }
    }

    return  new_vec;
}

fn even_values_2(vec : &mut Vec<i32>) {
    let mut i = 0;

    while i < vec.len() {
        if vec[i] % 2 != 0 {
            vec.remove(i);
        } else {
            i += 1;
        }
    }
}

pub fn run() {
    let mut vec: Vec<i32> = Vec::new();
    vec.push(5);
    vec.push(8);
    vec.push(2);
    vec.push(13);
    vec.push(4);

    println!("New vector with even values: {:?}", even_values_1(&vec));
    even_values_2(&mut vec);
    println!("New vector with even values: {:?}", vec);
    
}