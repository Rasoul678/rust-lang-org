pub fn smart_box() {
    use super::smart_box::List::{Cons, Nil};
    // Using Box<T> to Point to Data on the Heap
    let int_on_heap = Box::new(5);
    println!("b = {} is on the heap now!", int_on_heap);

    // Enabling Recursive Types with Boxes
    // let list = Cons(1, Cons(2, Cons(3, Nil)));

    // Using Box<T> to Get a Recursive Type with a Known Size
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

// Not working
// enum List {
//     Cons(i32, List),
//     Nil,
// }

enum List {
    Cons(i32, Box<List>),
    Nil,
}
