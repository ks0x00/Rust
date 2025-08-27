use std::ops::Deref;

use crate::mymod::A;

// Box는 자료를 힙에 저장한다.
fn main() {
    // box is a smart pointer
    let b = Box::new(123);
    println!("{}", b);
    println!("{} {}", *b, b.deref());

    let mut c = Box::new(456);
    *c += 1;
    println!("{}", c);

    let a = A(2, 3);
    println!("{:?}", *a.deref());
}

mod mybox {
    use std::ops::Deref;

    #[allow(dead_code)]
    pub struct MyBox<T>(pub T);

    impl<T> MyBox<T> {
        #[allow(dead_code)]
        pub fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}

mod mymod {
    use std::ops::Deref;

    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct A(pub i32, pub usize);

    impl Deref for A {
        type Target = i32;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}
