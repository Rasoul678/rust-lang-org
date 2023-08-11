pub fn drop_trait() {
    // Running Code on Cleanup with the Drop Trait
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    drop(c);

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // Dropping a Value Early with std::mem::drop
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
