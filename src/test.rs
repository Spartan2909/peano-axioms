use super::*;

fn reify_i32<T: Reify<i32>>(_: T) -> i32 {
    T::REIFIED
}

#[test]
fn neg() {
    assert_eq!(reify_i32(Negation::<Five>::new()), -5);
    assert_eq!(reify_i32(Negation::<Negation<Four>>::new()), 4);
}

#[test]
fn add() {
    assert_eq!(reify_i32(Sum::<One, Three>::new()), 4);
    assert_eq!(reify_i32(Sum::<Three, Negation<One>>::new()), 2);
    assert_eq!(reify_i32(Sum::<Three, Negation<Four>>::new()), -1);
    assert_eq!(reify_i32(Sum::<Negation<Two>, Four>::new()), 2);
    assert_eq!(reify_i32(Sum::<Negation<Five>, Three>::new()), -2);
    assert_eq!(
        reify_i32(Sum::<Negation<Seven>, Negation<Eight>>::new()),
        -15
    )
}

#[test]
fn mul() {
    assert_eq!(reify_i32(Product::<Seven, Eight>::new()), 56);
    assert_eq!(reify_i32(Product::<Three, Negation<Four>>::new()), -12);
    assert_eq!(reify_i32(Product::<Negation<Six>, Seven>::new()), -42);
    assert_eq!(
        reify_i32(Product::<Negation<Four>, Negation<Five>>::new()),
        20
    );
}

#[test]
fn sub() {
    assert_eq!(reify_i32(Difference::<Five, Two>::new()), 3);
    assert_eq!(reify_i32(Difference::<Three, Seven>::new()), -4);
}

#[test]
fn div() {
    assert_eq!(reify_i32(Quotient::<Six, Two>::new()), 3);
    assert_eq!(reify_i32(Quotient::<Six, Negation<Two>>::new()), -3);
    assert_eq!(reify_i32(Quotient::<Negation<Ten>, Two>::new()), -5);
    assert_eq!(
        reify_i32(Quotient::<Negation<Three>, Negation<One>>::new()),
        3
    );
}

#[test]
fn rpn() {
    assert_eq!(reify_i32(<rpn!(3 4 5 + *)>::new()), 27);
}
