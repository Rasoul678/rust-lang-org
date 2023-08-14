mod message_passing;
mod shared_state;
mod threads;

use message_passing::message_passing;
use shared_state::shared_state;
use threads::threads;

fn main() {
    // threads();
    // message_passing();
    shared_state();
}
