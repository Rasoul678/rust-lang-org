mod deref_trait;
mod drop_trait;
mod rc_smart_pointer;
mod ref_cell;
mod smart_box;

use deref_trait::deref_trait;
use drop_trait::drop_trait;
use rc_smart_pointer::rc_smart_pointer;
use ref_cell::ref_cell;
use smart_box::smart_box;

fn main() {
    smart_box();
    deref_trait();
    drop_trait();
    rc_smart_pointer();
    ref_cell();
}
