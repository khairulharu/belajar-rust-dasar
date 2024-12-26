mod alok;
mod aurora;
mod layla;
mod model;
mod zilong;

use model::{User, Weapon};
//or
// use model::*;
use alok::tembak_musuh;
use layla::tembak_musuh as tembak_musuh_layla;
use zilong::zilong_speaking;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_use_modul() {
    tembak_musuh();
    tembak_musuh_layla();
    zilong_speaking();
    zilong::hero::skill::tembak_musuh();
}

#[test]
fn test_public_model() {
    let user: User = User {
        first_name: String::from("alok"),
        last_name: String::from("tuti"),
        username: String::from("alokskik"),
        email: String::from("alok@gmail.com"),
        age: 80,
    };

    let m416: Weapon = Weapon {
        name: "m416".to_string(),
        magazine: 16,
        single_mode: true,
    };

    println!("weapon name: {}", m416.name);
    println!("weapon single mode: {}", m416.single_mode);

    user.show_my_credential();

    user.show_my_credential();
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
    let range_inclusive = 0..=6;

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

// fn get_ownerships(name: String) {
//     println!("{}", name);
// }

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

struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

fn print_person(person: &Person) {
    println!("{}", person.first_name);
    println!("{}", person.last_name);
    println!("{}", person.age);
}

#[test]
fn struct_in_rust() {
    let first_name = String::from("Khairul");
    let last_name = String::from("Aswad");

    //init shorthand
    let khairul: Person = Person {
        first_name,
        last_name,
        age: 20,
    };

    print_person(&khairul);

    //if we use Struct Update Syntax value will move if data on heap;
    let mut khairul2 = Person { ..khairul };

    //if we dosnt want move the value, use clone traits, to move variable on heap so the ownership dont move;
    let khairul3: Person = Person {
        last_name: khairul2.last_name.clone(),
        first_name: khairul2.first_name.clone(),
        ..khairul2
    };

    print_person(&khairul3);

    khairul2.first_name = String::from("Alok");

    print_person(&khairul2);
}

//implement in tuple struct
struct GeoPointTuple(f64, f64);

struct GeoPointStruct {
    longitude: f64,
    latitude: f64,
}

#[test]
fn test_tuple_struct_and_normal_struct() {
    let geo_point1: GeoPointTuple = GeoPointTuple(-3.716471834, 197.31442313);

    let geo_point: GeoPointStruct = GeoPointStruct {
        longitude: -3.716471834,
        latitude: 197.31442313,
    };

    println!("{}", geo_point1.0);
    println!("{}", geo_point1.1);
    println!("{}", geo_point.latitude);
    println!("{}", geo_point.longitude);
}

struct Nothing;

#[test]
fn test_struct_nothing() {
    let _nothing: Nothing = Nothing;

    let _nothing1: Nothing = Nothing {};
}

struct Hero {
    name: String,
    skill: String,
    hp: u32,
}

impl Hero {
    //associated function
    fn new(name: String, skill: String, hp: u32) -> Hero {
        Hero { name, skill, hp }
    }

    fn show_name(&self) {
        println!("name: {}", self.name);
    }

    fn show_skill(&self) {
        println!("skill: {}", self.skill);
    }

    fn show_hp(&self) {
        println!("skill: {}", self.hp);
    }

    // fn show_name_and_move(self) {
    //     println!("name and move: {}", self.name);
    // }
}

#[test]
fn test_method_in_struct() {
    let alok_no_hand: Hero = Hero {
        name: String::from("nur alok wijaya"),
        skill: String::from("menambahkan hp dan kecepatan lari"),
        hp: 200,
    };
    // alok_no_hand.show_name_and_move();

    alok_no_hand.show_name();
    alok_no_hand.show_hp();
    alok_no_hand.show_skill();
}

#[test]
fn test_associated_function_to_create_new_hero() {
    let layla: Hero = Hero::new(
        String::from("Layla"),
        String::from("tembak tembak physical damage"),
        200,
    );

    layla.show_hp();
    layla.show_name();
    layla.show_skill();
}

enum Level {
    High,
    Medium,
    Low,
}

#[test]
fn test_enum() {
    let _level0: Level = Level::High;
    let _level1: Level = Level::Medium;
    let _level3: Level = Level::Low;
}

enum Payment {
    CreditCard(String),
    BankTransfer(String, String),
    Ewallet(String, String),
}

impl Payment {
    fn pay(&self, amount: u32) {
        match self {
            Payment::CreditCard(number) => {
                println!(
                    "Pembayaran dilakukan menggunakan Credit Card dengan nomor {} dengan jumlah {}",
                    number, amount
                );
            }
            Payment::BankTransfer(bank, number) => {
                println!("Pembayaran dilakukan menggunakan bank Transfer dengan tipe bank {} dan nomor rekening {} dengan nominal: {}", bank, number, amount);
            }
            Payment::Ewallet(ewallet, number) => {
                println!("Pembayaran di lakukan dengan menggunakan Ewallet dengan nama {} nomor {} dengan nominal {}", ewallet, number, amount);
            }
        }
    }
}

#[test]
fn test_payment() {
    let _payment1: Payment = Payment::BankTransfer(String::from("BCA"), String::from("12413131"));

    _payment1.pay(10810313);

    let _payment2: Payment = Payment::CreditCard(String::from("1391839174"));

    _payment2.pay(891219213);

    let payment3: Payment = Payment::Ewallet(String::from("GOPAY"), String::from("13117484819"));

    payment3.pay(1923241312);
}

enum Skill {
    PhysicalDamage,
    MagicDamage,
    TrueDamage,
}

fn what_typical_damage_do_i_have(hero: Skill) {
    match hero {
        Skill::PhysicalDamage => {
            println!("hero ini physical damage");
        }
        Skill::MagicDamage => {
            println!("hero ini magic damage");
        }
        Skill::TrueDamage => {
            println!("hero ini memakai true  damage");
        }
    }
}

#[test]
fn pattern_matching() {
    let layla: Skill = Skill::PhysicalDamage;
    let eudora: Skill = Skill::MagicDamage;
    let lesley: Skill = Skill::TrueDamage;

    what_typical_damage_do_i_have(layla);
    what_typical_damage_do_i_have(eudora);
    what_typical_damage_do_i_have(lesley);
}

#[test]
fn test_match_value() {
    let name = "tolod";

    match name {
        "tulu" => {
            println!("halo tulu: kamu {} kan?", name);
        }
        "tolo" => {
            println!("wahh tolo selemat datang");
        }
        other => {
            println!("yahhh kamu lagi {}", other);
        }
    }

    match name {
        "tulu" | "tolo" => {
            println!("halo tulu: kamu {} kan?", name);
        }
        other => {
            println!("yahhh kamu lagi {}", other);
        }
    }
}

#[test]
fn test_range_pattern_matching() {
    let value = 100;

    match value {
        90..=100 => {
            println!("Great");
        }
        80..=89 => {
            println!("imposiibela")
        }
        75..=79 => {
            println!("ccuihh")
        }
        other => {
            println!("Blok belajar lagi nilai kok {}", other);
        }
    }
}

#[test]
fn test_destructuring_tuple_using_match() {
    let geo_point = GeoPointTuple(121.13131, -113.1313);

    match geo_point {
        GeoPointTuple(0.0, 0.0) => {
            println!("please input actual longitude and latitude");
        }
        GeoPointTuple(long, 0.0) => {
            println!("latitude must filled out longitude:{}, latitude: ", long);
        }
        GeoPointTuple(0.0, lati) => {
            println!("longitude mus filled longitude: , latitude: {}", lati);
        }
        GeoPointTuple(longitude, latitude) => {
            println!("longitude: {}, latitude: {}", longitude, latitude);
        }
    }
}

#[test]
fn test_struct_destructuring_using_match() {
    let layla: Hero = Hero::new(
        String::from("layla"),
        String::from("bullet drop with phisycal"),
        212,
    );

    match layla {
        Hero { name, skill, .. } => {
            println!("name: {}, skill: {}", name, skill);
        }
    }
}
#[test]
fn test_tuple_enum_ignoring() {
    let geo_point = GeoPointTuple(121.13131, -113.1313);

    match geo_point {
        GeoPointTuple(_, lati) => {
            println!("latitude: {}", lati);
        }
    }
}

#[test]
fn test_range_ignore_match() {
    let value = 0;

    match value {
        90..=100 => {
            println!("Great");
        }
        80..=89 => {
            println!("imposiibela")
        }
        75..=79 => {
            println!("ccuihh")
        }
        _ => {
            println!("Blok belajar lagi nilai kok");
        }
    }
}

#[test]
fn test_match_expression() {
    let value: i32 = 10;

    let result = match value {
        0 => "nol",
        1 => "satu",
        _ => "invalid",
    };

    println!("{}", result);
}

type Age = u8;
type IdentityNumber = String;

struct Customer {
    id: IdentityNumber,
    name: String,
    age: Age,
}

impl Customer {
    fn print_value(&self) {
        println!("id: {}, name: {}, age: {}", self.id, self.name, self.age)
    }
}

type Pelanggan = Customer;

#[test]
fn test_identity_number_string() {
    let customer = Customer {
        id: String::from("013131331312"),
        name: String::from("hilda"),
        age: 20,
    };

    let pelanggan: Pelanggan = Pelanggan {
        id: String::from("97913134"),
        name: String::from("pelanggan"),
        age: 93,
    };

    pelanggan.print_value();
    customer.print_value();
}

trait CanSayHello {
    fn hello(&self) -> String {
        String::from("this is the implementation for u")
    }

    fn say_hello(&self) -> String;
    fn say_hello_to(&self, name: &str) -> String;
}

trait CanSayGoodBye {
    fn say_good_bye(&self) -> String;
    fn say_good_bye_to(&self, name: &str) -> String;
}

struct Human {
    name: String,
}

impl Human {
    fn say_hello(&self) {
        println!("halo my name is {}", self.name);
    }
}

impl CanSayHello for Human {
    fn say_hello(&self) -> String {
        format!("hello my name is {}", self.name)
    }

    fn say_hello_to(&self, name: &str) -> String {
        format!("hello {} my name is {}", name, self.name)
    }
}

impl CanSayGoodBye for Human {
    fn say_good_bye(&self) -> String {
        format!("{}: good bay bro have a nice day", self.name)
    }

    fn say_good_bye_to(&self, name: &str) -> String {
        format!("{}: have a nice day bro, {}", self.name, name)
    }
}

fn say_hello_trait(person: &impl CanSayHello) {
    println!("{}", person.say_hello_to("layla flicker"));
}

fn say_hello_and_good_bay(value: &(impl CanSayHello + CanSayGoodBye)) {
    println!("{}", value.say_hello());
    println!("{}", value.say_good_bye());
}

#[test]
fn test_trait_implementation() {
    let alok_jumpshot: Human = Human {
        name: "alok jumpshot".to_string(),
    };

    let no_hand: Human = Human {
        name: String::from("no hand"),
    };

    say_hello_trait(&no_hand);
    say_hello_trait(&alok_jumpshot);

    println!("\n");

    println!("{}", no_hand.hello());
    println!("{}", CanSayHello::say_hello(&no_hand));
    println!("{}", no_hand.say_hello_to("agus"));

    println!("-------------");

    println!("{}", alok_jumpshot.hello());
    println!("{}", CanSayHello::say_hello(&no_hand));
    println!("{}", alok_jumpshot.say_hello_to("khairul aswad"));

    println!("\n");

    println!("{}", no_hand.say_good_bye());
    println!("{}", no_hand.say_good_bye_to("brody"));

    println!("\n");

    say_hello_and_good_bay(&alok_jumpshot);
    say_hello_and_good_bay(&no_hand);

    //solve when method and implement trait have a same name function
    println!("{}", CanSayHello::say_hello(&no_hand));
    Human::say_hello(&no_hand);
}

trait SkillAndStatus {
    fn show_skill(&self) -> String;
    fn show_hp_bar(&self) -> u8;
    fn show_all_status(&self);
}

struct Aurora {
    name: String,
    skill: String,
    hp: u8,
}

impl Aurora {
    fn show_skill(&self) {
        println!("show skill method: {}", self.skill)
    }
}

impl SkillAndStatus for Aurora {
    fn show_skill(&self) -> String {
        format!("show skill: {}", self.skill)
    }

    fn show_hp_bar(&self) -> u8 {
        self.hp
    }

    fn show_all_status(&self) {
        println!(
            "name: {}, skill: {}, hp: {}",
            self.name, self.skill, self.hp
        );
    }
}

fn create_aurora_skill_and_status(value: Aurora) -> impl SkillAndStatus {
    Aurora {
        name: value.name,
        skill: value.skill,
        hp: value.hp,
    }
}

#[test]
fn test_return_implementation_trait() {
    let aurora: Aurora = Aurora {
        name: "aur".to_string(),
        skill: "aur".to_string(),
        hp: 200,
    };

    let aurora_imple = create_aurora_skill_and_status(aurora);

    println!("{}", aurora_imple.show_skill());    
    aurora_imple.show_all_status();
    println!("hp: {}", aurora_imple.show_hp_bar());
    println!("skill: {}", aurora_imple.show_skill());
}
