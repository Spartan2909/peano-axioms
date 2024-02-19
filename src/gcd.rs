use crate::Next;
use crate::Remainder;
use crate::Zero;

use core::ops::Rem;

pub trait Reduce<T> {
    type This;
    type Arg;
}

impl<T> Reduce<Zero> for Next<T> {
    type This = Self;
    type Arg = Zero;
}

impl<T> Reduce<Next<T>> for Zero {
    type This = Self;
    type Arg = Next<T>;
}

impl<T, U> Reduce<Next<U>> for Next<T>
where
    T: Reduce<U>,
{
    type This = <T as Reduce<U>>::This;
    type Arg = <T as Reduce<U>>::Arg;
}

pub trait GcdInner<T, U> {
    type Result;
}

impl<T, U> GcdInner<Next<T>, Next<U>> for Zero
where
    Next<U>: Rem<Next<T>>,
    Next<T>: Gcd<Remainder<Next<U>, Next<T>>>,
{
    type Result = GreatestCommonDivisor<Next<T>, Remainder<Next<U>, Next<T>>>;
}

impl<T, U, V> GcdInner<Next<T>, Next<U>> for Next<V>
where
    Zero: GcdInner<Next<U>, Next<T>>,
{
    type Result = <Zero as GcdInner<Next<U>, Next<T>>>::Result;
}

/// Type-level greatest common divisor.
pub trait Gcd<T> {
    /// The greatest common divisor of `Self` and `T`.
    type Result;
}

impl Gcd<Zero> for Zero {
    type Result = Zero;
}

impl<T> Gcd<Next<T>> for Zero {
    type Result = Next<T>;
}

impl<T> Gcd<Zero> for Next<T> {
    type Result = Next<T>;
}

impl<T, U> Gcd<Next<U>> for Next<T>
where
    Next<T>: Reduce<Next<U>>,
    <Next<T> as Reduce<Next<U>>>::This: GcdInner<Next<T>, Next<U>>,
{
    type Result = <<Next<T> as Reduce<Next<U>>>::This as GcdInner<Next<T>, Next<U>>>::Result;
}

/// The greatest common divisor of `T` and `U`.
pub type GreatestCommonDivisor<T, U> = <T as Gcd<U>>::Result;
