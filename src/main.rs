use std::cell::RefCell;
use std::rc::Rc;

// An Rc pointer from Owner to Gadget introduces a cycle.
// This means that their reference counts can never reach 0,
// and the allocation will never be destroyed: a memory leak.
struct Owner {
    name: String,
    gadgets: RefCell<Vec<Rc<Gadget>>>,
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>,
}

fn main() {
    let gadget_owner: Rc<Owner> = Rc::new(Owner {
        name: "Gadget Man".to_string(),
        // Rc enforces memory safety by only giving out shared references to the value it wraps,
        // and these don’t allow direct mutation.
        // We need to wrap the part of the value we wish to mutate in a RefCell.
        // which provides interior mutability:
        // a method to achieve mutability through a shared reference.
        gadgets: RefCell::new(vec![]),
    });

    let gadget1 = Rc::new(Gadget {
        id: 1,
        owner: Rc::clone(&gadget_owner),
    });

    let gadget2 = Rc::new(Gadget {
        id: 2,
        owner: Rc::clone(&gadget_owner),
    });

    {
        let mut gadgets = gadget_owner.gadgets.borrow_mut();
        gadgets.push(gadget1);
        gadgets.push(gadget2);

        // `RefCell` dynamic borrow ends here.
    }

    for gadget in gadget_owner.gadgets.borrow().iter() {
        println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
    }
}
