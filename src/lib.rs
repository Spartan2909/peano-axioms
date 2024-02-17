#![no_std]
#![recursion_limit = "256"]
#![cfg_attr(test, allow(unused_parens))]

use core::fmt;
use core::marker::PhantomData;

pub trait Reify<T> {
    const REIFIED: T;

    fn reify(&self) -> T {
        Self::REIFIED
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Zero;

impl fmt::Display for Zero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("0")
    }
}

macro_rules! reify_zero {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Reify<$ty> for Zero {
                const REIFIED: $ty = 0;
            }
        )*
    };
}

reify_zero![u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Next<T>(PhantomData<T>);

impl<T> Next<T> {
    #[inline(always)]
    pub const fn new() -> Next<T> {
        Next(PhantomData)
    }
}

impl<T> fmt::Display for Next<T>
where
    Self: Reify<u128>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::REIFIED)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Prev<T>(PhantomData<T>);

impl<T> Prev<T> {
    #[inline(always)]
    pub const fn new() -> Prev<T> {
        Prev(PhantomData)
    }
}

impl<T> fmt::Display for Prev<T>
where
    Self: Reify<i128>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::REIFIED)
    }
}

macro_rules! reify_generic {
    ($name:ident, $op:tt, $($ty:ty),* $(,)?) => {
        $(
            impl<T> Reify<$ty> for $name<T>
            where
                T: Reify<$ty>
            {
                const REIFIED: $ty = T::REIFIED $op 1;
            }
        )*
    };
}

reify_generic![Next, +, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];

reify_generic![Prev, -, i8, i16, i32, i64, i128, isize];

trait IsNotNext {}

impl IsNotNext for Zero {}

impl<T> IsNotNext for Prev<T> {}

trait IsNotPrev {}

impl IsNotPrev for Zero {}

impl<T> IsNotPrev for Next<T> {}

trait IsNotZero {}

impl<T> IsNotZero for Next<T> {}

impl<T> IsNotZero for Prev<T> {}

pub type One = Next<Zero>;

pub type Two = Next<One>;

pub type Three = Next<Two>;

pub type Four = Next<Three>;

pub type Five = Next<Four>;

pub type Six = Next<Five>;

pub type Seven = Next<Six>;

pub type Eight = Next<Seven>;

pub type Nine = Next<Eight>;

pub type Ten = Next<Nine>;

pub trait Add<T> {
    type Result;
}

impl Add<Zero> for Zero {
    type Result = Zero;
}

impl<T> Add<Zero> for Next<T> {
    type Result = Self;
}

impl<T> Add<Zero> for Prev<T> {
    type Result = Self;
}

impl<T> Add<Next<T>> for Zero {
    type Result = Next<T>;
}

impl<T> Add<Prev<T>> for Zero {
    type Result = Prev<T>;
}

impl<T, U> Add<Next<U>> for Next<T>
where
    T: Add<U>,
{
    type Result = Next<Next<Sum<T, U>>>;
}

impl<T, U> Add<Prev<U>> for Next<T>
where
    T: Add<U>,
{
    type Result = Sum<T, U>;
}

impl<T, U> Add<Next<U>> for Prev<T>
where
    T: Add<U>,
{
    type Result = Sum<T, U>;
}

impl<T, U> Add<Prev<U>> for Prev<T>
where
    T: Add<U>,
{
    type Result = Prev<Prev<Sum<T, U>>>;
}

pub type Sum<T, U> = <T as Add<U>>::Result;

#[inline(always)]
pub fn sum<T, U, V>(_: T, _: U) -> Sum<T, U>
where
    T: Add<U>,
    Sum<T, U>: Default,
{
    Default::default()
}

pub trait Mul<T> {
    type Result;
}

impl Mul<Zero> for Zero {
    type Result = Zero;
}

impl<T> Mul<Zero> for Next<T> {
    type Result = Zero;
}

impl<T> Mul<Zero> for Prev<T> {
    type Result = Zero;
}

impl<T> Mul<Next<T>> for Zero {
    type Result = Zero;
}

impl<T> Mul<Prev<T>> for Zero {
    type Result = Zero;
}

impl<T, U> Mul<Next<U>> for Next<T>
where
    Next<T>: Mul<U> + Add<Product<Next<T>, U>>,
{
    type Result = Sum<Next<T>, Product<Next<T>, U>>;
}

impl<T, U> Mul<Prev<U>> for Next<T>
where
    T: Mul<U>,
    Product<T, U>: Sub<T>,
    Difference<Product<T, U>, T>: Add<U>,
    Sum<Difference<Product<T, U>, T>, U>: Sub<One>,
{
    type Result = Difference<Sum<Difference<Product<T, U>, T>, U>, One>;
}

impl<T, U> Mul<Next<U>> for Prev<T>
where
    Next<U>: Mul<Prev<T>>,
{
    type Result = Product<Next<U>, Prev<T>>;
}

impl<T, U> Mul<Prev<U>> for Prev<T>
where
    T: Mul<U>,
    Product<T, U>: Sub<T>,
    Difference<Product<T, U>, T>: Sub<U>,
    Difference<Difference<Product<T, U>, T>, U>: Add<One>,
{
    type Result = Sum<Difference<Difference<Product<T, U>, T>, U>, One>;
}

pub type Product<T, U> = <T as Mul<U>>::Result;

#[inline(always)]
pub fn product<T, U, V>(_: T, _: U) -> Product<T, U>
where
    T: Mul<U>,
    Product<T, U>: Default,
{
    Default::default()
}

pub trait Sub<T> {
    type Result;
}

impl Sub<Zero> for Zero {
    type Result = Zero;
}

impl<T: Neg> Sub<Next<T>> for Zero {
    type Result = Prev<Negation<T>>;
}

impl<T: Neg> Sub<Prev<T>> for Zero {
    type Result = Next<Negation<T>>;
}

impl<T> Sub<Zero> for Next<T> {
    type Result = Self;
}

impl<T> Sub<Zero> for Prev<T> {
    type Result = Self;
}

impl<T, U> Sub<Next<U>> for Next<T>
where
    T: Sub<U>,
{
    type Result = <T as Sub<U>>::Result;
}

impl<T, U> Sub<Prev<U>> for Next<T>
where
    Prev<U>: Neg,
    Next<T>: Add<Negation<Prev<U>>>,
{
    type Result = Sum<Next<T>, Negation<Prev<U>>>;
}

impl<T, U> Sub<Next<U>> for Prev<T>
where
    Prev<Prev<T>>: Sub<U>,
{
    type Result = Difference<Prev<Prev<T>>, U>;
}

impl<T, U> Sub<Prev<U>> for Prev<T>
where
    T: Sub<U>,
{
    type Result = Difference<T, U>;
}

pub type Difference<T, U> = <T as Sub<U>>::Result;

#[inline(always)]
pub fn difference<T, U, V>(_: T, _: U) -> Difference<T, U>
where
    T: Sub<U>,
    Difference<T, U>: Default,
{
    Default::default()
}

pub trait Neg {
    type Result;
}

impl Neg for Zero {
    type Result = Zero;
}

impl<T: Neg> Neg for Next<T> {
    type Result = Prev<Negation<T>>;
}

impl<T: Neg> Neg for Prev<T> {
    type Result = Next<Negation<T>>;
}

pub type Negation<T> = <T as Neg>::Result;

#[inline(always)]
pub fn negation<T>(_: T) -> Negation<T>
where
    T: Neg,
    Negation<T>: Default,
{
    Default::default()
}

pub trait Div<T> {
    type Result;
}

impl<T> Div<Next<T>> for Zero {
    type Result = Zero;
}

impl<T> Div<Prev<T>> for Zero {
    type Result = Zero;
}

impl<T, U> Div<Next<U>> for Next<T>
where
    Next<U>: Positive,
    Next<T>: Sub<Next<U>> + Positive,
    Difference<Next<T>, Next<U>>: Div<Next<U>> + NonNegative,
    Quotient<Difference<Next<T>, Next<U>>, Next<U>>: Add<One>,
{
    type Result = Sum<Quotient<Difference<Next<T>, Next<U>>, Next<U>>, One>;
}

impl<T, U> Div<Prev<U>> for Next<T>
where
    Prev<U>: Neg + Negative,
    Next<T>: Div<Negation<Prev<U>>> + Positive,
    Quotient<Next<T>, Negation<Prev<U>>>: Neg,
{
    type Result = Negation<Quotient<Next<T>, Negation<Prev<U>>>>;
}

impl<T, U> Div<Next<U>> for Prev<T>
where
    Next<U>: Positive,
    Prev<T>: Neg + Negative,
    Negation<Prev<T>>: Div<Next<U>>,
    Quotient<Negation<Prev<T>>, Next<U>>: Neg,
{
    type Result = Negation<Quotient<Negation<Prev<T>>, Next<U>>>;
}

impl<T, U> Div<Prev<U>> for Prev<T>
where
    Prev<T>: Neg + Negative,
    Prev<U>: Neg + Negative,
    Negation<Prev<T>>: Div<Negation<Prev<U>>>,
{
    type Result = Quotient<Negation<Prev<T>>, Negation<Prev<U>>>;
}

pub type Quotient<T, U> = <T as Div<U>>::Result;

pub trait Simplify {
    type Result;
}

impl Simplify for Zero {
    type Result = Zero;
}

impl Simplify for Next<Zero> {
    type Result = Self;
}

impl Simplify for Prev<Zero> {
    type Result = Self;
}

impl<T> Simplify for Next<Next<T>>
where
    Next<T>: Simplify,
    Simplified<Next<T>>: Add<One>,
{
    type Result = Sum<Simplified<Next<T>>, One>;
}

impl<T> Simplify for Prev<Prev<T>>
where
    Prev<T>: Simplify,
    Simplified<Prev<T>>: Sub<One>,
{
    type Result = Difference<Simplified<Prev<T>>, One>;
}

impl<T> Simplify for Next<Prev<T>>
where
    T: Simplify,
{
    type Result = Simplified<T>;
}

impl<T> Simplify for Prev<Next<T>>
where
    T: Simplify,
{
    type Result = Simplified<T>;
}

pub type Simplified<T> = <T as Simplify>::Result;

pub trait Positive: NonNegative + NonZero {}

impl<T> Positive for T where T: NonNegative + NonZero {}

pub trait Negative: NonPositive + NonZero {}

impl<T> Negative for T where T: NonPositive + NonZero {}

pub trait NonNegative {}

impl NonNegative for Zero {}

impl<T> NonNegative for Next<T>
where
    Self: Simplify,
    Simplified<Self>: IsNotPrev,
{
}

impl<T> NonNegative for Prev<T>
where
    Self: Simplify,
    Simplified<Self>: IsNotPrev,
{
}

pub trait NonPositive {}

impl NonPositive for Zero {}

impl<T> NonPositive for Next<T>
where
    Self: Simplify,
    Simplified<Self>: IsNotNext,
{
}

impl<T> NonPositive for Prev<T>
where
    Self: Simplify,
    Simplified<Self>: IsNotNext,
{
}

pub trait NonZero {}

impl<T> NonZero for Next<T>
where
    Self: Simplify,
    Simplified<Self>: IsNotZero,
{
}

impl<T> NonZero for Prev<T>
where
    Self: Simplify,
    Simplified<Self>: IsNotZero,
{
}

#[macro_export]
macro_rules! rpn {
    // Numerals
    (@ ((0)       $($rest:tt)*) (            $($stack:tt)*)) => { $crate::rpn!(@ ($($rest)*) (($crate::Zero              ) $($stack)*)) };
    (@ ((1)       $($rest:tt)*) (            $($stack:tt)*)) => { $crate::rpn!(@ ($($rest)*) (($crate::One               ) $($stack)*)) };
    (@ ((2)       $($rest:tt)*) (            $($stack:tt)*)) => { $crate::rpn!(@ ($($rest)*) (($crate::Two               ) $($stack)*)) };
    (@ ((3)       $($rest:tt)*) (            $($stack:tt)*)) => { $crate::rpn!(@ ($($rest)*) (($crate::Three             ) $($stack)*)) };
    (@ ((4)       $($rest:tt)*) (            $($stack:tt)*)) => { $crate::rpn!(@ ($($rest)*) (($crate::Four              ) $($stack)*)) };
    (@ ((5)       $($rest:tt)*) (            $($stack:tt)*)) => { $crate::rpn!(@ ($($rest)*) (($crate::Five              ) $($stack)*)) };
    (@ ((6)       $($rest:tt)*) (            $($stack:tt)*)) => { $crate::rpn!(@ ($($rest)*) (($crate::Six               ) $($stack)*)) };
    (@ ((7)       $($rest:tt)*) (            $($stack:tt)*)) => { $crate::rpn!(@ ($($rest)*) (($crate::Seven             ) $($stack)*)) };
    (@ ((8)       $($rest:tt)*) (            $($stack:tt)*)) => { $crate::rpn!(@ ($($rest)*) (($crate::Eight             ) $($stack)*)) };
    (@ ((9)       $($rest:tt)*) (            $($stack:tt)*)) => { $crate::rpn!(@ ($($rest)*) (($crate::Nine              ) $($stack)*)) };
    (@ ((10)      $($rest:tt)*) (            $($stack:tt)*)) => { $crate::rpn!(@ ($($rest)*) (($crate::Ten               ) $($stack)*)) };

    // Infix operators
    (@ ((+)       $($rest:tt)*) ($a:tt $b:tt $($stack:tt)*)) => { $crate::rpn!(@ ($($rest)*) (($crate::Sum<$b, $a>       ) $($stack)*)) };
    (@ ((-)       $($rest:tt)*) ($a:tt $b:tt $($stack:tt)*)) => { $crate::rpn!(@ ($($rest)*) (($crate::Difference<$b, $a>) $($stack)*)) };
    (@ ((*)       $($rest:tt)*) ($a:tt $b:tt $($stack:tt)*)) => { $crate::rpn!(@ ($($rest)*) (($crate::Product<$b, $a>   ) $($stack)*)) };
    (@ ((/)       $($rest:tt)*) ($a:tt $b:tt $($stack:tt)*)) => { $crate::rpn!(@ ($($rest)*) (($crate::Quotient<$b, $a>  ) $($stack)*)) };

    // Operands
    (@ (($val:ty) $($rest:tt)*) (            $($stack:tt)*)) => { $crate::rpn!(@ ($($rest)*) ($val                         $($stack)*)) };

    // Done
    (@ (                      ) ($val:tt                  )) => { $val };

    // Entry point
    ($($t:tt)+)                                              => { $crate::rpn!(@ ($(($t))+)  (                                       )) };
}

#[cfg(test)]
mod test;
