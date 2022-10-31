use std::cell::RefCell;
use std::rc::{Rc, Weak};

// An Rc pointer from Owner to Gadget introduces a cycle.
// This means that their reference counts can never reach 0,
// and the allocation will never be destroyed: a memory leak.

struct Owner {
    name: String,
    // Weak RC gets around the memory leak problem.
    // a Weak reference does not count towards ownership,
    // it will not prevent the value stored in the allocation from being dropped,
    // and Weak itself makes no guarantees about the value still being present.
    // Thus it may return None when upgraded.
    // Note however that a Weak reference does prevent the allocation itself
    // (the backing store) from being deallocated.

    // Weak pointer is useful for keeping a temporary reference to the allocation managed by Rc without preventing its inner value from being dropped.
    // It is also used to prevent circular references between Rc pointers,
    // since mutual owning references would never allow either Rc to be dropped.
    gadgets: RefCell<Vec<Weak<Gadget>>>,
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
        // Without RefCell the RC items inside Vec cannot be mutated.
        // You cannot generally obtain a mutable reference to something inside an Rc.
        // If you need mutability, put a Cell or RefCell inside the Rc;
        let mut gadgets = gadget_owner.gadgets.borrow_mut();
        gadgets.push(Rc::downgrade(&gadget1));
        gadgets.push(Rc::downgrade(&gadget2));

        // `RefCell` dynamic borrow ends here.
    }

    for gadget in gadget_owner.gadgets.borrow().iter() {
        // `gadget` is a `Weak<Gadget>.
        // Upgrade the Weak RC before accessing containing data.
        // Since `Weak` pointers can't guarantee the allocation still exists,
        // we need to call `upgrade`, which returns an `Option<Rc<Gadget>>.

        // In this case we know the allocation still exists,
        // so `unwrap` the `Option` is acceptable.
        // In a more complicated program,
        // graceful error handling might be required for a `None` result.

        let gadget = gadget.upgrade().unwrap();
        println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
    }
}
