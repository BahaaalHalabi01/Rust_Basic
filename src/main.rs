#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn functions() {
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

}

fn main() {
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
