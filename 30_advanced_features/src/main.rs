mod advanced_faunctions;
mod advanced_traits;
mod advanced_types;
mod unsafe_rust;

use advanced_faunctions::advanced_functions;
use advanced_traits::advanced_traits;
use advanced_types::advanced_types;
use unsafe_rust::unsafe_rust;

fn main() {
    unsafe_rust();
    advanced_traits();
    advanced_types();
    advanced_functions();
}
