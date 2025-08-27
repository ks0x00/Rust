use crate::{mod1::A, mod2::B};



fn main() {
    let a = A(2, 3);
    // Deref Coercion, *a = *a.deref()
    println!("{:?}", *a);

    // DerefMut
    let mut b = A(0, 1);
    *b = 99;
    println!("{:?}", b);

    // B<f64>
    let a1 = B(2.0, 3);
    println!("{:?}", *a1);

    let mut b1 = B(0.0, 1);
    *b1 = 99.0;
    println!("{:?}", b1);
}

mod mod1 {
    use std::ops::{Deref, DerefMut};

    #[derive(Debug)]
    #[allow(dead_code)]
    pub struct A(pub i32, pub usize);

    impl Deref for A {
        type Target = i32;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for A {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
}

mod mod2 {
    use std::ops::{Deref, DerefMut};

    #[derive(Debug)]
    #[allow(dead_code)]
    pub struct B<T>(pub T, pub usize);

    impl<T> Deref for B<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T> DerefMut for B<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
}
