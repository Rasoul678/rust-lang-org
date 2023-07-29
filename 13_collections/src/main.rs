use std::collections::HashMap;

fn main() {
    // Storing Lists of Values with Vectors
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: Option<&i32> = v.get(3);

    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v2 = vec![1, 2, 3];
    let third: &i32 = &v2[2];
    println!("The third element is {third}");

    let v3 = vec![1, 2, 3, 4, 5];
    // Will panic
    // let does_not_exist = &v3[100];
    let _does_not_exist = v3.get(100);

    #[allow(unused_mut)]
    let mut v4 = vec![1, 2, 3, 4, 5];

    // immutable borrow
    let first = &v4[0];

    // mutable borrow
    // v4.push(6);

    println!("The first element is: {first}");

    let v5 = vec![100, 32, 57];
    for i in &v5 {
        println!("{i}");
    }

    let mut v6 = vec![100, 32, 57];
    for i in &mut v6 {
        // dereference
        *i += 50;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    {
        let _v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here

    // Storing UTF-8 Encoded Text with Strings
    let mut s1 = String::new();

    let data = "initial contents";

    let s2 = data.to_string();

    // the method also works on a literal directly:
    let s3 = "initial contents".to_string();
    let _test = &s3[..];

    // Appending to a String with push_str and push
    let mut s4 = String::from("initial contents");
    s4.push_str("bar");

    let mut s5 = String::from("lo");
    s5.push('l');

    // Concatenation with the + Operator or the format! Macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let _s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    let _tic_tac_toe = tic + "-" + &tac + "-" + &toe;

    let tic1 = String::from("tic");
    let tac2 = String::from("tac");
    let toe3 = String::from("toe");

    let _tic_tac_toe = format!("{tic1}-{tac2}-{toe3}");

    // Indexing into Strings
    // Won't compile
    // let s1 = String::from("hello");
    // let h = s1[0];

    // Internal Representation
    let hello = "Здравствуйте";

    let _s = &hello[0..4];
    // With slicing, program may crash in run time

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }

    // Storing Keys with Associated Values in Hash Maps
    // Creating a New Hash Map

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let _score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // Updating a Hash Map
    // Overwriting a Value
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // Adding a Key and Value Only If a Key Isn’t Present
    let mut scores2 = HashMap::new();

    scores2.insert(String::from("Blue"), 10);
    scores2.entry(String::from("Yellow")).or_insert(70);
    scores2.entry(String::from("Blue")).or_insert(70);

    println!("{:?}", scores2);

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
