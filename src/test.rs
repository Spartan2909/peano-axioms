use super::*;

#[test]
fn neg() {
    assert_eq!(reify(Negation::<Five>::new()), -5);
    assert_eq!(reify(Negation::<Negation<Four>>::new()), 4);
}

#[test]
fn add() {
    assert_eq!(reify(Sum::<One, Three>::new()), 4);
    assert_eq!(reify(Sum::<Three, Negation<One>>::new()), 2);
    assert_eq!(reify(Sum::<Three, Negation<Four>>::new()), -1);
    assert_eq!(reify(Sum::<Negation<Two>, Four>::new()), 2);
    assert_eq!(reify(Sum::<Negation<Five>, Three>::new()), -2);
    assert_eq!(reify(Sum::<Negation<Seven>, Negation<Eight>>::new()), -15)
}

#[test]
fn mul() {
    assert_eq!(reify(Product::<Seven, Eight>::new()), 56);
    assert_eq!(reify(Product::<Three, Negation<Four>>::new()), -12);
    assert_eq!(reify(Product::<Negation<Six>, Seven>::new()), -42);
    assert_eq!(reify(Product::<Negation<Four>, Negation<Five>>::new()), 20);
}

#[test]
fn sub() {
    assert_eq!(reify(Difference::<Five, Two>::new()), 3);
    assert_eq!(reify(Difference::<Three, Seven>::new()), -4);
}

#[test]
fn div() {
    assert_eq!(reify(Quotient::<Six, Two>::new()), 3);
    assert_eq!(reify(Quotient::<Six, Negation<Two>>::new()), -3);
    assert_eq!(reify(Quotient::<Negation<Ten>, Two>::new()), -5);
    assert_eq!(reify(Quotient::<Negation<Three>, Negation<One>>::new()), 3);
}

#[test]
fn rpn() {
    assert_eq!(reify(<rpn!(3 4 5 + *)>::new()), 27);
}
