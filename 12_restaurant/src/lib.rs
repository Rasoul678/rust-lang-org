use std::fmt;
use std::io;
use std::io::Result as IoResult;

mod back_of_house;
mod front_of_house;
mod customer;
pub use front_of_house::hosting;
// Calling by external code
// restaurant::hosting::add_to_waitlist()

#[allow(dead_code)]
fn deliver_order() {}

#[allow(dead_code)]
fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}

#[allow(dead_code)]
fn function2() -> io::Result<()> {
    // --snip--
    Ok(())
}

#[allow(dead_code)]
fn function3() -> IoResult<()> {
    // --snip--
    Ok(())
}
