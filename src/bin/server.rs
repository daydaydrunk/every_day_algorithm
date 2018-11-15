extern crate minigrep;

use std::rc::Rc;

struct Owner {
    name: String
}

struct Gadget {
    id: i32,
 owner: Rc<Owner>
}

#[derive(Debug)]
struct Demo {
    name:&'static str,
}

fn main() {
    // Create a reference counted Owner.
    let gadget_owner : Rc<Owner> = Rc::new(
        Owner { name: String::from("Gadget Man") }
    );
    // Create Gadgets belonging to gadget_owner. To increment th

    // count we clone the `Rc<T>` object.
    let gadget1 = Gadget { id: 1, owner: gadget_owner.clone() };
    let gadget2 = Gadget { id: 2, owner: gadget_owner.clone() };
    drop(gadget_owner);

    println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
    println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);

    let a = String::from("charlie");


    
    let r1 = Demo { name:&a };
    let r2 = Demo { name:&a };

    println!("{:?} {:?}", r1, r2);
}