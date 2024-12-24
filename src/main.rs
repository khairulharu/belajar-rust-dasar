
fn main() {
    println!("Hello, world!");
}

#[test]
fn loop_label() {
    let mut number = 0;

    let name: [&str; 2] = ["khairul", "aswad"];

    'outer: loop {
        let mut i = 1;
        loop {
            if number > 1 {
                break 'outer;
            }

            println!("{}:{}", i, name[number]);
            i += 1;

            if i > 10 {
                break;
            }
        }
        number += 1;
    }
}

#[test]
fn while_loop() {
    let mut under_age = 30;

    while under_age > 18 {
        println!("decrease under age: {}", under_age);

        under_age -= 1;
    }
}

#[test]
fn array_iteration() {
    //use while loop
    let array: [&str; 5] = ["a", "b", "c", "d", "e"];
    let mut index = 0;
    while index < array.len() {
        println!("index ke: {} value: {}", index, array[index]);

        index += 1;
    }

    //use for loop
    for item in array {
        println!("value: {item}")
    }
}

#[test]

fn range() {
    let array: [&str; 5] = ["a", "b", "c", "d", "e"];
    //range exclusive, the end of range not included
    let range = 0..5;

    println!("Start Range {}", range.start);
    println!("End range {}", range.end);

    for i in range {
        println!("Index ke {} value {}", i, array[i])
    }

    for i in 0..5 {
        println!("Index ke {} value {}", i, array[i])
    }
    
    //range inclusive
    let range_inclusive= 0..=6;

    println!("start {}", range_inclusive.start());

    println!("start {}", range_inclusive.end());

    for i in 0..=4 {
        println!("{}", i);
    }
}

fn say_hello() {
    println!("hello world")
}

fn say_goodbye(first_name: &str, last_name: &str) {
    println!("Goodbye {} {}", first_name, last_name);
}

fn factorial_loop(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }

    let mut result = 1;

    for i in 1..=n {
        result *= i;
    }

    return result;
}

#[test]
fn test_say_hello() {
    say_hello();
    say_hello();

    say_goodbye("Khairul", "aswad");
    say_goodbye("first_name", "array_iteration");

    let factorial = factorial_loop(5);
    println!("{}", factorial);

    let result = factorial_loop(-5);
    println!("{}", result);
}

fn print_my_name_ten(name: &str, times: u32) {
    if times == 0 {
        return;
    } else {
        println!("{}", name);
    }

    print_my_name_ten(name, times - 1);
}

#[test]
fn test_recursive_function() {
    print_my_name_ten("Khairul Aswad", 10);
}

fn factorial_recursive(value: i32) -> i32 {
    if value <= 1 {
        return 1;
    }

    return value * factorial_recursive(value - 1);
}

fn print_my_name_five_times(name: String) {
    let mut five_time = 5;

    // loop {
    //     if five_time != 0 {
    //         five_time -= 1;

    //         println!("Print: {}", name)
    //     } else {
    //         break;
    //     }
    // }

    while five_time != 0 {
        five_time -= 1;
        println!("Print with while: {}", name);
    }
}

#[test]
fn test_factorial_recursive() {
    let result = factorial_recursive(5);
    println!("{}", result);
}

#[test]
fn ownership_in_function() {
    let factorial_number = 10;

    let result = factorial_recursive(factorial_number);

    println!("{}", result);

    println!("ini tidak pindah ownershipnya: {}", factorial_number);

    //jika data tersebut di simpan di stack maka data tersebut akan di copy
    //kalau datanya di heap maka ownership nya pindah cara ngakalinnya adalah menggunakan refence

    let my_name = String::from("Khairul Aswad");

    print_my_name_five_times(my_name);

    // println!("This value is move: {}", my_name);
}

fn full_name(first_name: String, last_name: String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name() {
    let first_name = String::from("Khairul");
    let last_name = String::from("Aswad");

    let full_name = full_name(first_name, last_name);

    println!("{}", full_name);
}

fn full_name_tuple(first_name: String, last_name: String) -> (String, String, String) {
    let full_name = format!("{} {}", first_name, last_name);

    (first_name, last_name, full_name)
}

#[test]
fn test_full_name_tuple() {
    let first_name = String::from("Khairul");
    let last_name = String::from("Aswad");

    let (first_name1, last_name1, full_name1) = full_name_tuple(first_name, last_name);

    println!("{}", first_name1);
    println!("{}", last_name1);
    println!("{}", full_name1);
}

fn full_name_references(first_name: &String, last_name: &String) -> String {
    return format!("{} {}", first_name, last_name);
}

#[test]
fn test_full_name_references() {
    let first_name = String::from("Khairul");
    let last_name = String::from("Aswad");

    let full_name = full_name_references(&first_name, &last_name);

    println!("{}", first_name);
    println!("{}", last_name);
    println!("{}", full_name);
}

fn change_value(value: &mut String) {
    value.push_str(" change");
}

#[test]
fn test_change_value() {
    let mut value = String::from("Khairul Aswad");

    // change_value(&mut value);

    // println!("{}", value);
    // change_value(&mut value);
    // println!("{}", value);
    // change_value(&mut value);

    // println!("{}", value);


    let borrowed_value = &mut value;

    borrowed_value.push_str("string");

    change_value(borrowed_value);
    change_value(borrowed_value);
    change_value(borrowed_value);
    change_value(borrowed_value);

    println!("borrwed value :{}", value)
}

fn get_full_name(first_name: &String, last_name: &String) -> String {
    let full_name = format!("{} {}", first_name, last_name);

    return full_name;
}

#[test]
fn test_get_full_name() {
    let first_name = String::from("Khairul");
    let last_name = String::from("Aswad");

    let full_name = get_full_name(&first_name, &last_name);

    println!("{}", full_name);
}

#[test]
fn slice_reference() {

    //fixed size array itu tidak di pindahkan ownershipnya dikarenakan di tipe data yang pada saat compile kita tau ukurannya.
    let array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    //slice itu adalah references jadi tidak pindah juga ownershipnya hanya references ke string aslinya.
    let slice1: &[i32] = &array[..];
    println!("{:?}", slice1);

    let slice5 = slice1;

    println!("{:?}", slice5);

    let slice2: &[i32] = &array[0..5];
    println!("{:?}", slice2);

    let slice3: &[i32] = &array[0..=5];
    println!("{:?}", slice3);

    let slice4: &[i32] = &array[5..];
    println!("{:?}", slice4);
}

fn get_ownerships(name: String) {
    println!("{}", name);
}

#[test]
fn string_slice() {
    let name: String = String::from("Halo Dek Abang Lagi Nyantai");

    // get_ownerships(name);

    // string slice adalah references jadi jika yang di reference hilang atau pindah ownership maka string slice nya akan tidak berguna
    let get_halo = &name[0..4];
    println!("{}", get_halo);

    let another_string = get_halo.trim();

    println!("{}", get_halo);
    println!("{}", another_string);
}