use generics_traits_lifetimes::{NewsArticle, Summary, Tweet};
use std::fmt::Display;

fn main() {
    // Generic Types, Traits, and Lifetimes
    // Removing Duplication by Extracting a Function
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let result = get_largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    //10.1 Generic Data Types
    // In Function Definitions
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let result = find_largest(&number_list);
    println!("The largest number is {}", result);

    let result = find_largest(&char_list);
    println!("The largest char is {}", result);

    // In Struct Definitions
    let integer = Point { x: 5, y: 1.0 };
    let float = Point {
        x: 1.0f32,
        y: 4.0f32,
    };

    // In Enum Definitions
    let _person = Person::Me { name: "Rasoul" };

    // In Method Definitions
    println!("p.x = {}", integer.x());
    println!("distance from origin = {}", float.distance_from_origin());

    let p1 = MyPoint { x: 5, y: 10.4 };
    let p2 = MyPoint { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    //10.2 Traits: Defining Shared Behavior
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    notify(&article);
    notify2(&tweet);
    notify3(&tweet, &article);
    notify4(&tweet, &tweet);
    notify4(&article, &article);
    notify5(&tweet);
    notify6(&article);
    notify7(&tweet);

    let returned_tweet = returns_summarizable();
    println!("{}", returned_tweet.summarize());

    // Validating References with Lifetimes
    // let r;

    // {
    //     let x = 5;
    //     r = &x;
    // }

    // println!("r: {}", r);

    // Generic Lifetimes in Functions
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string11 = String::from("long string is long");

    {
        let string12 = String::from("xyz");
        let result: &str = longest(string11.as_str(), string12.as_str());
        println!("The longest string is {}", result);
    }

    // let string21 = String::from("long string is long");
    // let result;
    // {
    //     let string22 = String::from("xyz");
    //     result = longest(string21.as_str(), string22.as_str());
    // }
    // println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let _i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[allow(dead_code)]
struct Point<T, U> {
    x: T,
    y: U,
}

#[allow(dead_code)]
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[allow(dead_code)]
enum Person<T> {
    Me { name: T },
    You { name: T },
}

fn get_largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn find_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct MyPoint<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> MyPoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: MyPoint<X2, Y2>) -> MyPoint<X1, Y2> {
        MyPoint {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn notify(item: &impl Summary) {
    println!("Inside notify function");
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary>(item: &T) {
    println!("Inside notify2 function");
    println!("Breaking news! {}", item.summarize());
}

// To have different types (as long as both types implement Summary).
pub fn notify3(item1: &impl Summary, item2: &impl Summary) {
    println!("Inside notify3 function");
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

// Force both parameters to have the same type.
pub fn notify4<T: Summary>(item1: &T, item2: &T) {
    println!("Inside notify4 function");
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

// Specifying Multiple Trait Bounds with the + Syntax
pub fn notify5(item: &(impl Summary + Display)) {
    println!("Inside notify5 function");
    println!("Breaking news! {}", item.summarize());
    println!("Breaking news! {}", item);
}

pub fn notify6<T: Summary + Display>(item: &T) {
    println!("Inside notify6 function");
    println!("Breaking news! {}", item.summarize());
    println!("Breaking news! {}", item);
}

// Clearer Trait Bounds with where Clauses
pub fn notify7<T>(item: &T)
where
    T: Summary + Display,
{
    println!("Inside notify6 function");
    println!("Breaking news! {}", item.summarize());
    println!("Breaking news! {}", item);
}

// Returning Types that Implement Traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("returned_tweet"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// Lifetime Annotations in Struct Definitions
#[allow(dead_code)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}
