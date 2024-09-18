use std::collections::HashMap;

fn iterators() {
    let mut nums: Vec<i32> = vec![1, 2, 3];
   
   // 1. using iter() - provides imutable ref to nums
    let iter = nums.iter(); //imutable borrow of nums
    for value in iter {
        println!("{}", value);
    }

    // 2. using iter_mut() - provides a mutable ref to nums
    let nums_iter = nums.iter_mut();
    for val in nums_iter {
        *val = *val * 2;
    }

    // 3. using iter.next() - it returns Optional<&mut T>, option of a mutable reference to that type 
    let mut iter = nums.iter();
    while let Some(val) = iter.next() {
        print!("{}", val);
    }

    /* 4. using for loop - here we consumed the nums vector means ownership is moved away, 
    after this loop nums is not available, we can mutate the values here but scope ends after the loop */
    for mut value in nums {
        value *= 2;
        println!("{}", value);
    }

    //println!("{:?}", nums); this line will not work - as consumed earlier

    /* 5. into_iter() - this trait is used to convert a collection into an iterator that takes ownership of the collection
        - Useful when - you no longer need the original collection
                      - when you need to squeeze performance benefits by transferring owneship (avoiding references)

        This is same as normal for loop, i.e. consumes the vector
    */
    let nums: Vec<i32> = vec![1, 2, 3];
    let iter = nums.into_iter();

    for value in iter { // transfers ownership of the items
        println!("{}", value);
    }

}

fn adaptors() {
     // Adapters //
    /* 1. Consuming adapters - 
    Methods that call next are called consuming adaptors because calling them uses up the iterator */
    
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();
    println!("Sum is {}", total);

    // for i in v1_iter {} //can't used as the iter already consumed
    println!("{:?}", v1); // v1 is not consumed
    
    /* 2. Iterator adaptors - are methods defined on the iterator trait that don't consuem the iterator. 
        Insted, they produce different iterators by changing some aspect of the original iterator. */

    let iter = v1.iter();
    let iter2 = iter.map(|x| x + 1); //returns new iterator
    for x in iter2 { //iter2 consumed here
        println!("{}", x);
    }

    let iter = v1.iter();
    let iter3 = iter.filter(|x| *x % 2 == 0);
    for x in iter3 { //iter3 consumed here
        println!("{}", x);
    }
}

fn iterators_on_hashmap() {
    let mut scores = HashMap::new();
    scores.insert("Alice", 50);
    scores.insert("Bob", 40);
    scores.insert("Charlie", 30);

    // Example 1: Iterating over references to key-value pairs
    println!("Iterating over key-value pairs: ");
    for (key, value) in scores.iter() {
        println!("{} : {}", key, value);
    }

    // Example 2: Iterating over mutable references to key-value paris
    println!("\nIterating over mutable key-value paris: ");
    for (key, value) in scores.iter_mut() {
        *value += 10;
        println!("{} : {}", key, value);
    }
}

pub fn run() {
    iterators();
    adaptors();
    iterators_on_hashmap();
}