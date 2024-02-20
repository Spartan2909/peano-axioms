use super::*;

fn reify_i32<T: Reify<i32>>(_: T) -> i32 {
    T::REIFIED
}

fn reify_i32_i32<T: Reify<(i32, i32)>>(_: T) -> (i32, i32) {
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
    assert_eq!(reify_i32(Quotient::<Four, Four>::VALUE), 1);
    assert_eq!(reify_i32(Remainder::<Four, Four>::VALUE), 0);
}

#[test]
fn gcd() {
    assert_eq!(reify_i32(GreatestCommonDivisor::<Six, Three>::VALUE), 3);
    assert_eq!(reify_i32(GreatestCommonDivisor::<Ten, Three>::VALUE), 1);
    assert_eq!(reify_i32(GreatestCommonDivisor::<Five, Five>::VALUE), 5);
}

#[test]
fn lcm() {
    type TwentyOne = Sum<Product<Ten, Two>, One>;

    assert_eq!(reify_i32(LeastCommonMultiple::<TwentyOne, Six>::VALUE), 42);
    assert_eq!(reify_i32(LeastCommonMultiple::<Four, Four>::VALUE), 4);
}

#[test]
fn fraction() {
    assert_eq!(reify_i32_i32(Fraction::<Three, Two>::VALUE), (3, 2));
    assert_eq!(reify_i32_i32(<rpn!(1 2 fract 1 3 fract +)>::VALUE), (5, 6));
    assert_eq!(reify_i32_i32(<rpn!(2 3 fract 1 3 fract /)>::VALUE), (2, 1));
}

#[test]
fn rpn() {
    assert_eq!(reify_i32(<rpn!(3 4 5 + *)>::VALUE), 27);
}
