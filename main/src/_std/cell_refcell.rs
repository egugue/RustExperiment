pub fn main() {
    utils::println_file_name!();
    mut_smart_pointers::main();
}

/// this code is referenced in https://qiita.com/wada314/items/24249418983312795c08
mod mut_smart_pointers {
    use std::cell::{Cell, Ref, RefCell};
    use std::rc::Rc;

    pub fn main() {
        utils::println_function_name!();
        mutable_reference();
        reference_counter();
        cell();
        ref_cell();
    }

    fn mutable_reference() {
        utils::println_function_name!();

        // a &mut value can change the actual value it references to.
        {
            let mut x: i32 = 1;
            let x_ref: &mut i32 = &mut x;
            *x_ref += 1;
            assert_eq!(x, 2);
        }

        // a mut reference can change the reference destination.
        {
            let mut x: i32 = 1;
            let mut x_ref: &i32 = &x;
            assert_eq!(*x_ref, x);

            let mut y: i32 = 2;
            x_ref = &y;
            assert_eq!(*x_ref, y);
        }
    }

    fn reference_counter() {
        utils::println_function_name!();

        // Rc can change the reference destination.
        {
            let mut x: Rc<i32> = Rc::new(1);
            x = Rc::new(2);
            assert_eq!(*x, 2);
        }

        // Rc cannot define mutable value as the generic type parameter.
        {
            // let mut x: Rc<mut i32> = Rc::new(1);
        }

        // Instead, Rc::get_mut can be used to change the actual value.
        {
            let mut rc: Rc<i32> = Rc::new(1);
            match Rc::get_mut(&mut rc) {
                None => {}
                Some(x) => *x += 1,
            }
            assert_eq!(*rc, 2);
        }

        // But Rc::get_mut can be used only in the same situation as using a normal variable.
        {
            let mut rc: Rc<i32> = Rc::new(1);
            let immutable_reference = Rc::clone(&rc);
            let mutable_reference_option: Option<&mut i32> = Rc::get_mut(&mut rc);
            // Because there is a immutable reference, a mutable reference cannot be created.
            assert_eq!(mutable_reference_option, None);
        }

        // Rc<Cell> can solve the issue.
    }

    fn cell() {
        utils::println_function_name!();

        // Cell can change the value even if not defined as mutable.
        {
            let cell: Cell<i32> = Cell::new(1);
            cell.set(2);
            assert_eq!(cell.get(), 2);
        }

        // Rc<Cell> can change the value it references to even if there are some immutable references to it.
        {
            let rc_cell: Rc<Cell<i32>> = Rc::new(Cell::new(1));
            let reference1 = Rc::clone(&rc_cell);
            let reference2 = Rc::clone(&rc_cell);
            rc_cell.set(rc_cell.get() + 1);
            assert_eq!(rc_cell.get(), 2);
        }

        // But Cell cannot have a value which does not implement Copy trait.
        {
            let cell: Cell<NonCopy> = Cell::new(NonCopy { x: 1 });
            // cell.get();
        }

        // RefCell can have such value.
    }

    fn ref_cell() {
        utils::println_function_name!();

        // RefCell can have a value which does not implement Copy trait  and change the value it references to.
        // This is useful in some situations.
        {
            let ref_cell: RefCell<NonCopy> = RefCell::new(NonCopy { x: 1 });
            ref_cell.borrow_mut().x = 2;
            let ref_non_copy: Ref<NonCopy> = ref_cell.borrow();
            assert_eq!((*ref_non_copy).x, 2);
        }

        // But unlike Cell, there is some constraints on borrowing a mutable reference.
        // One reason why Cell has not such constraint is that a type a Cell has should implement Copy trait,
        // which means a return value by Cell.set is a copied value and the value is not borrowed.
        {
            let ref_cell: RefCell<NonCopy> = RefCell::new(NonCopy { x: 1 });
            let ref_non_copy: Ref<NonCopy> = ref_cell.borrow();
            let borrow_result = ref_cell.try_borrow_mut();
            assert_eq!(borrow_result.is_err(), true);
        }
    }

    struct NonCopy {
        x: i32,
    }
}
