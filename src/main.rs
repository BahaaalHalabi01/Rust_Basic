#![allow(unused)]
mod restaurant;
use core::panic;
use rand::Rng;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::ops::Add;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main(){}

fn threads() {
    // common problems of parallel programming
    // involve :
    // 1. Thread are accessing data in the wrong order
    // 2. Threads are blocked form executing because of confusion
    // over requirements to proced with execution / deadlock
    let thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned thread : {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..20 {
        println!(" Main Thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    //if you do not join, first thread will not finish
    thread1.join().unwrap();

    pub struct Bank {
        balance: f32,
    };
    //Below is wrong way to do things
    //the thread closure might outlive the current function
    // which borrows jpmorgan which is owned by main
    // fn withdraw(the_bank: &mut Bank, amt: f32) {
    //     the_bank.balance -= amt;
    // }
    // let mut jpmogran = Bank { balance: 100.0 };
    // withdraw(&mut jpmogran, 50.0);
    // println!("My balance :{}", jpmogran.balance);
    // fn customer(the_bank: &mut Bank) {
    //     withdraw(the_bank, 5.00)
    // }
    // thread::spawn(||{
    //     customer(&mut jpmogran)
    // }).join().unwrap()
    // how to fix the above?
    fn withdraw(bank: &Arc<Mutex<Bank>>, amt: f32) {
        let mut bank_ref = bank.lock().unwrap();
        if bank_ref.balance < amt {
            println!(
                "Current Balance: {} does not allow you to withdraw the {} amount",
                bank_ref.balance, amt
            )
        } else {
            bank_ref.balance -= amt;
            println!(
                "You successfully withdrew {}, you have {} left in your balance",
                amt, bank_ref.balance
            )
        }
    }
    fn customer(bank: Arc<Mutex<Bank>>) {
        // is the below possibile 
        // take input on every thread
        // let mut bank_ref = bank.lock().unwrap();
        // println!("You currently have {} in your balance", bank_ref.balance);
        // println!("Enter the amount to withdraw in numbers");
        // io::stdin()
        //     .read_line(&mut amt)
        //     .expect("You did not enter a correct amount");
        withdraw(&bank, 5.00)
    }
    let jpmogran: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank { balance:20.00 }));
    let handles = (0..10).map(|_| {
        let bank_ref = jpmogran.clone();
        thread::spawn(|| customer(bank_ref))
    });

    for handle in handles {
        handle.join().unwrap()
    }
    println!("Total {}", jpmogran.lock().unwrap().balance)
}

fn trees() {
    //Box a smart pointer for large amount of data in heap
    //stores data on a heap rather than a stack
    //stack stores value in last in first out format
    //data on stack must have a defined fixed size
    //with a heap you request an amount of memory with the os allocates for you
    let b_int1 = Box::new(10);
    println!("bint1 {}", b_int1);

    #[derive(Debug)]
    struct TreeNode<T> {
        //this is a usual tree
        //left:TreeNode<T>
        //right:TreeNode<T>
        //rust does not like null values
        //leafs of a tree have no right/left
        //this fixes it
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }
    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode {
                left: None,
                right: None,
                key,
            }
        }
        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }

        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }
    let node1 = TreeNode::new(1)
        .left(TreeNode::new(2))
        .right(TreeNode::new(5));
    println!("Tree Printing : {:?}", node1)
}

fn closures() {
    // a closure
    // let var_name = |parameters| -> return_type {body}
    let can_vote = |age: i32| age >= 18;
    println!("Can this guy vote?: {}", can_vote(78));
    let mut samp = 5;
    let print_var = || println!("samp = {}", samp);
    print_var();
    samp = 10;
    //closure is an easy way to change value in a function
    //harder to do this in a normal function
    //if you declare closure as mut, you can change values inside of it
    let mut change_var = || samp *= 2;
    change_var();
    println!("new samp = {}", samp);
    //passing a closure to a function
    fn use_func<T>(a: i32, b: i32, func: T) -> i32
    where
        T: Fn(i32, i32) -> i32,
    {
        func(a, b)
    };
    let sum = |a: i32, b: i32| a + b;
    let prod = |a: i32, b: i32| a * b;
    println!("using a use function to sum : {}", use_func(5, 4, sum));
    println!("using a use function to mult: {}", use_func(5, 4, prod));
}

fn iters() {
    let mut arr_it = [1, 2, 3, 4];
    // you can't change the value in the below examples
    for val in arr_it.iter() {
        println!("{}", val)
    }
    let mut iter = arr_it.iter();
    println!("1st: {:?}", iter.next());
    //can consume but it goes away
    for val in arr_it.into_iter() {}
}

fn files_errors() {
    //Result has 2 variants Ok and Err
    // Ok(T),
    // Err(E),
    // }
    // Where T represents the data typeof the value
    // and E the type of error
    let path = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => panic!("Problem creating a file: {:?}", error),
    };
    write!(output, "Just Some random stuff\n next line stuff")
        .expect("Failed to write to the file");

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    let output1 = File::create("random.txt");
    let output1 = match output1 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("random.txt") {
                Ok(fc) => fc,
                Err(err) => panic!("Could not create a file: {:?}", err),
            },
            _err => panic!("Problem occured: {:?}", _err),
        },
    };
}

fn structs() {
    //this lets you print instantly
    #[derive(Debug)]
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }
    let mut bob = Customer {
        name: String::from("Bob Smith"),
        address: String::from("Downtown Dubai"),
        balance: 250.50,
    };

    bob.address = String::from("Jumairah Street");
    println!("Details: {:?}", bob);

    const PI: f32 = 3.141592;
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }
    struct Rectangle {
        length: f32,
        width: f32,
    };
    struct Circle {
        length: f32,
        width: f32,
    };
    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            Rectangle { length, width }
        }
        fn area(&self) -> f32 {
            &self.length * self.width
        }
    }
    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            Circle { length, width }
        }
        fn area(&self) -> f32 {
            (&self.length / 2.0) * PI
        }
    }

    let rec: Rectangle = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);
    println!("Rectangle Area: {}", rec.area());
    println!("Circle Area: {}", circ.area())
}

fn hashmap() {
    let mut heroes: HashMap<&str, &str> = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("The Flash", "Barry Allen");

    for (k, v) in heroes.iter() {
        println!(" key: {}, value: {}", k, v)
    }
    if heroes.contains_key(&"Batman") {
        let the_batman = heroes.get(&"Batman");
        match the_batman {
            Some(x) => println!("Batman is a hero"),
            None => println!("Batman is not here today"),
        }
    }

    println!("Length: {}", heroes.len())
}

fn functions() {
    fn print_str(x: String) {
        println!("A String {}", x);
    }

    fn print_return_str(x: String) -> String {
        println!("A String {}", x);
        x
    }
    fn change_string(x: &mut String) {
        x.push_str(" modified string");
        println!("New {}", x)
    }
    let mut str1 = String::from("Bahaa");
    let str2 = str1.clone();
    let str3 = print_return_str(str2);
    println!("str3 = {}", str3);
    fn get_2(x: i32) -> (i32, i32) {
        return (x + 1, x + 2);
    }

    fn get_sum(x: i32, y: i32) -> i32 {
        x + y
        // or this
        // return x + y
    }

    fn sum_list(list: &[i32]) -> i32 {
        let mut sum = 0;
        for &i in list.iter() {
            sum += &i
        }

        return sum;
    }

    println!("{}", get_sum(5, 2));
    println!("{:?}", get_2(2));
    let num_list: Vec<i32> = vec![1, 2, 3, 4, 5, 5, 6];
    println!("Sum of list {:?}: {}", num_list, sum_list(&num_list));

    fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
        return x + y;
    }

    println!("{} + {} = {}", 5, 4, get_sum_gen(5, 4));
    println!("{} + {} = {}", 3.2, 4.5, get_sum_gen(3.2, 4.5));
}

fn vectors() {
    let vec: Vec<i32> = Vec::new();
    let mut vec1 = vec![1, 2, 3, 4];
    vec1.push(5);
    println!("1st: {}", vec1[0]);
    let second = &vec1[1];
    match vec1.get(1) {
        Some(second) => println!("2nd: {}", second),
        None => println!("No 2nd Value"),
    };
    for i in &mut vec1 {
        *i *= 2
    }
    for i in &vec1 {
        println!("{}", i)
    }
    println!("Vector: {:?}", vec1);
    println!("Vector Length: {}", vec1.len());
    println!("Pop: {:?}", vec1.pop())
}

fn enums() {
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    };

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false,
            }
        }
    }
    let today = Day::Monday;
    match today {
        Day::Monday => println!("Hard Work Monday!"),
        Day::Tuesday => println!("Keep Pushing Tuesday!"),
        Day::Wednesday => println!("Gym Wednesday!"),
        Day::Thursday => println!("Do not slow down Thursday!"),
        Day::Friday => println!("Everyone excited about weekends Friday!"),
        Day::Saturday => println!("Family Saturday!"),
        Day::Sunday => println!("Sauna Sunday!"),
    }
    println!("Is today the weekend ? {}", today.is_weekend())
}

fn strings() {
    let my_tuple = (47, "Derek".to_string(), 50_000);
    println!("Name: {}", my_tuple.1);
    let (v1, v2, v3) = my_tuple;

    let mut str1 = String::new();
    str1.push('A');
    str1.push_str(" Word");
    for word in str1.split_whitespace() {
        println!("{}", word)
    }
    let str2 = str1.replace("A", "Another");
    println!("{}", str2);

    let str3 = String::from("x a s d f a w s d h g k k a m c");
    let mut v1: Vec<char> = str3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char)
    }
    let str4 = "Random String";
    let mut str5 = str4.to_string();
    println!("{}", str5);
    let byte_arr1 = str5.as_bytes();
    let str6 = &str5[0..6];
    println!("String Lenght : {}", str6.len());
    str5.clear();
    let str6 = String::from("Just some");
    let str7 = String::from("words");
    let str8 = str6 + &str7;
    //Borrowed checker, the below line doesn't work as str6 is used above
    // println!("{}",str6);
    println!("{}", str8);
    for char in str8.bytes() {
        println!("{}", char)
    }
}

fn loops() {
    let arr_1 = [1, 2, 3, 4];
    println!("1st: {}", arr_1[0]);
    println!("Length: {}", arr_1.len());

    let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut loop_idx = 0;
    println!("Looping");
    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr_2[loop_idx] == 9 {
            break;
        }
        println!("Val : {}", arr_2[loop_idx]);
        loop_idx += 1;
    }
    loop_idx = 0;
    println!("While loop");
    while loop_idx < arr_2.len() {
        println!("Arr : {}", arr_2[loop_idx]);
        loop_idx += 1;
    }
    println!("For loop");
    for val in arr_2.iter() {
        println!("Val : {}", val)
    }
}

fn name_input() {
    println!("What is your name?");
    let mut name = String::new();
    let greeting = "Nice to meet you";
    io::stdin()
        .read_line(&mut name)
        .expect("Didn't Recieve Input");
    println!("Hello {}! {}", name.trim_end(), greeting);
}

fn number_types() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age = "47";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a numebr");
    age = age + 1;

    println!("I'm {} and i want ${}", age, ONE_MIL);

    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max usize: {}", usize::MAX);

    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random : {}", random_num);
    //casting
    let int_u8: u8 = 5;
    let int1_u8: u16 = 4;
    let int2_u32 = int_u8 as u32;
}

fn if_match() {
    let age = 8;
    if (age >= 1) && (age <= 18) {
        println!("Important Birthday");
    } else if (age == 2) || (age == 50) {
        println!("Important Birthday");
    } else if age >= 65 {
        println!("Important Birthday");
    } else {
        println!("Not an Importnat Birthday");
    }

    let mut my_age = 47;
    let can_vote = if my_age >= 18 { true } else { false };
    println!("Can Vote : {}", can_vote);

    let age2 = 21;
    match age2 {
        1..=18 => println!("Not an Important Birthday"),
        21 | 50 => println!("Important Birthday"),
        65..=i32::MAX => println!("Important Birthday"),
        _ => println!("Not an important birthday"),
    };

    let my_age = 19;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("You sadly can't vote yet"),
        Ordering::Greater => println!("You are allowed to vote"),
        Ordering::Equal => println!("You just gained your right to vote"),
    };
}
