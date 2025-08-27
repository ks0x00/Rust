use std::{cell::RefCell, rc::Rc};

// 참조자 대여 규칙
//   1. 어떠한 경우이든 간에, 하나의 가변 참조자 혹은 여러 개의 불변 참조자 중 (둘 다가 아니고) 하나만 가질 수 있습니다.
//   2. 참조자는 항상 유효해야 합니다.
// 참조자와 Box<T>를 이용할 때, 대여 규칙의 불변성은 컴파일 타임에 집행됩니다.
//   참조자를 가지고서 이 규칙을 어기면 컴파일러 에러를 얻게 될 것입니다.
// RefCell<T>를 이용할 때, 이 불변성은 런타임에 집행됩니다.
//   RefCell<T>를 가지고서 이 규칙을 어기면, 프로그램은 panic!을 일으키고 종료될 것입니다.
// https://doc.rust-kr.org/ch15-05-interior-mutability.html

fn main() {
    let a = RefCell::new(123);
    println!("{:?}, {}, {}", a, a.borrow(), *a.borrow());

    *a.borrow_mut() += 456;
    println!("{:?}", *a.borrow());

    println!(
        "===================================================================================="
    );
    let b = RefCell::new(vec![1]);
    let b1 = b.clone();
    b1.borrow_mut().push(2);
    println!("{:?}, {:?}", b, b1);
    // RefCell { value: [1] }, RefCell { value: [1, 2] }
    // clone()이 deep copy처럼 보이지만, 사실은 그렇지 않다.
    // b.clone()이 Vec의 clone을 호출하기 때문에 deep copy처럼 보인다.
    // 조심해서 사용해야 한다.

    println!("===================================================================================");
    let c = Rc::new(RefCell::new(123));
    println!("{:?}", c);

    let c1 = c.clone(); // RefCell의 주소만 복사한다.
    *c1.borrow_mut() += 456;
    println!("{:?} {:?}", c, c1);
}

/*
// 대여 규칙 위반
fn panic_func() {
    let mut a = 123;
    let p1 = &a;
    let p2 = &mut a;
    *p2 += 1;
    println!("{}", *p1);
}
 */