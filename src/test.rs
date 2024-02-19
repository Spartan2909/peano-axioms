use super::*;

fn reify_i32<T: Reify<i32>>(_: T) -> i32 {
    T::REIFIED
}

#[test]
fn neg() {
    assert_eq!(reify_i32(Negation::<Five>::VALUE), -5);
    assert_eq!(reify_i32(Negation::<Negation<Four>>::VALUE), 4);
}

#[test]
fn add() {
    assert_eq!(reify_i32(Sum::<One, Three>::VALUE), 4);
    assert_eq!(reify_i32(Sum::<Three, Negation<One>>::VALUE), 2);
    assert_eq!(reify_i32(Sum::<Three, Negation<Four>>::VALUE), -1);
    assert_eq!(reify_i32(Sum::<Negation<Two>, Four>::VALUE), 2);
    assert_eq!(reify_i32(Sum::<Negation<Five>, Three>::VALUE), -2);
    assert_eq!(
        reify_i32(Sum::<Negation<Seven>, Negation<Eight>>::VALUE),
        -15
    );
}

#[test]
fn mul() {
    assert_eq!(reify_i32(Product::<Seven, Eight>::VALUE), 56);
    assert_eq!(reify_i32(Product::<Three, Negation<Four>>::VALUE), -12);
    assert_eq!(reify_i32(Product::<Negation<Six>, Seven>::VALUE), -42);
    assert_eq!(
        reify_i32(Product::<Negation<Four>, Negation<Five>>::VALUE),
        20
    );
}

#[test]
fn sub() {
    assert_eq!(reify_i32(Difference::<Five, Two>::VALUE), 3);
    assert_eq!(reify_i32(Difference::<Three, Seven>::VALUE), -4);
}

#[test]
fn div() {
    assert_eq!(reify_i32(Quotient::<Six, Two>::VALUE), 3);
    assert_eq!(reify_i32(Remainder::<Six, Two>::VALUE), 0);
    assert_eq!(reify_i32(Quotient::<Six, Negation<Two>>::VALUE), -3);
    assert_eq!(reify_i32(Quotient::<Negation<Ten>, Two>::VALUE), -5);
    assert_eq!(
        reify_i32(Quotient::<Negation<Three>, Negation<One>>::VALUE),
        3
    );
    assert_eq!(reify_i32(Quotient::<Four, Three>::VALUE), 1);
    assert_eq!(reify_i32(Remainder::<Four, Three>::VALUE), 1);
    assert_eq!(reify_i32(Quotient::<Next<Ten>, Four>::VALUE), 2);
    assert_eq!(reify_i32(Remainder::<Next<Ten>, Four>::VALUE), 3);
}

#[test]
fn rpn() {
    assert_eq!(reify_i32(<rpn!(3 4 5 + *)>::VALUE), 27);
}
