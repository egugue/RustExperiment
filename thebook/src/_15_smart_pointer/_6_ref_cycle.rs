use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub fn main() {
    utils::println_file_name!();
    cyclic_list();
    tree_node();
    why_wrapping_refcell::tree_node_raw();
}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn next(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            List::Cons(_, item) => Some(item),
            List::Nil => None,
        }
    }
}

fn cyclic_list() {
    use self::List::{Cons, Nil};
    utils::println_function_name!();

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.next());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.next());

    if let Some(link) = a.next() {
        // Rc<List> in RefCell is replaced with an Rc pointing to `b`,
        // meaning that links the next list of 'a' to b.
        // As a result, a cyclic list is created.
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.next());
}

// --------------------------------------------------

/// `parent` should be a Weak.
///
/// If it is an Rc instead, the node is pointing to the parent and the parent is pointing to the node.
/// This is a cyclic reference.
/// But a reference to the parent does not need to be a strong reference.
/// Because if a node is dropped, we do not need to drop the parent.
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            value: 0,
            parent: RefCell::default(),
            children: RefCell::default(),
        }
    }
}

fn tree_node() {
    utils::println_function_name!();

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // creates `branch` as the parent of leaf.
    {
        let reference_to_leaf = Rc::clone(&leaf);
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![reference_to_leaf]),
        });

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!(
            "leaf parent is Some = {:?}",
            leaf.parent.borrow().upgrade().is_some()
        );

        // It will prints not infinitely because Weak<Node> prints just (Weak).
        // This indicates this code doesn't create a reference cycle.
        println!("leaf = {:?}", leaf);
        println!("branch = {:?}", branch);

        assert_eq!(Rc::strong_count(&leaf), 2); // leaf itself and a branch's child
        assert_eq!(Rc::weak_count(&leaf), 0);
        assert_eq!(Rc::strong_count(&branch), 1); // branch itself
        assert_eq!(Rc::weak_count(&branch), 1); // the leaf's parent
    }

    // leaf itself because `branch` goes out of scope
    assert_eq!(Rc::strong_count(&leaf), 1);

    // Because `branch` goes out and leaf.parent is a Weak, leaf.parent references to nothing
    assert_eq!(leaf.parent.borrow().upgrade().is_none(), true);
}

/// This is code for me to understand why the official code uses RefCell
/// In summary,
///  - Rc itself doesn't implement `DerefMut` so the inner value cannot mutate.
///  - RefCell can get us `&mut Node` using `borrow_mut`.
mod why_wrapping_refcell {
    use std::ops::DerefMut;
    use std::rc::{Rc, Weak};

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: Weak<Node>,
        children: Vec<Rc<Node>>,
        // parent: RefCell<Weak<Node>>,
        // children: RefCell<Vec<Rc<Node>>>,
    }

    impl Node {
        fn new(value: i32, parent: Weak<Node>, children: Vec<Rc<Node>>) -> Node {
            Self {
                value,
                parent,
                children,
            }
        }
    }

    pub fn tree_node_raw() {
        utils::println_function_name!();
        let mut leaf = Rc::new(Node::new(3, Weak::new(), vec![]));
        let reference_to_leaf = Rc::clone(&leaf);
        let mut branch = Rc::new(Node::new(5, Weak::new(), vec![reference_to_leaf]));

        // Rc cannot change the inner value directly because Rc doesn't implement DerefMut.
        // Therefore both code cannot compile.
        {
            // leaf.parent = Rc::downgrade(&branch);
            // leaf.deref_mut().parent = Rc::downgrade(&branch);
            // *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        }

        // Using Rc::get_mut method can change the inner value directly.
        // But `leaf` is already referenced by `branch`.
        // As a result, Rc::get_mut() will fail because of Rust borrowing rule.
        if false {
            // this will panic
            Rc::get_mut(&mut leaf).unwrap().parent = Rc::downgrade(&branch);
        }

        // But RefCell can get us &mut reference
        let node = super::Node::default();
        let rc = Rc::new(node);
        {
            let mut weak: &mut Weak<_> = rc.parent.borrow_mut().deref_mut();
        }
        // and change the inner value
        let parent = Rc::new(super::Node::default());
        *rc.parent.borrow_mut() = Rc::downgrade(&parent);
    }
}
