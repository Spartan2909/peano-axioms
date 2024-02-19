//! Type-level numbers based on an extension of the Peano axioms.

#![no_std]
#![recursion_limit = "256"]
#![cfg_attr(test, allow(unused_parens))]

use core::fmt;
use core::hash;
use core::marker::PhantomData;

/// Types which can be converted to a runtime value.
pub trait Reify<T> {
    /// The runtime representation of this type.
    const REIFIED: T;

    /// A convience method for getting the runtime representation of this type.
    ///
    /// The default implementation should always be sufficient.
    #[inline(always)]
    fn reify(&self) -> T {
        Self::REIFIED
    }
}

/// Types which are part of a sequence.
pub trait Sequence {
    /// The next type in the sequence.
    type Next;

    /// The previous type in the sequence.
    type Prev;
}

/// The number zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Zero;

impl Zero {
    /// The value associated with this type, i.e. 0.
    pub const VALUE: Zero = Zero;
}

impl Sequence for Zero {
    type Next = Next<Zero>;

    type Prev = Prev<Zero>;
}

impl fmt::Display for Zero {
    #[inline]
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

            impl From<Zero> for $ty {
                #[inline(always)]
                fn from(_: Zero) -> Self {
                    0
                }
            }
        )*
    };
}

reify_zero![u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];

impl hash::Hash for Zero {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        state.write_i128(0);
    }
}

/// The successor to some number.
///
/// Some of the traits in this crate require that `Next<T>` is positive. If this
/// causes errors, the [`Simplify`] trait can be used to remove redundancies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Next<T>(PhantomData<T>);

impl<T> Next<T> {
    /// The value associated with this type.
    pub const VALUE: Next<T> = Next(PhantomData);
}

impl<T> Sequence for Next<T> {
    type Next = Next<Next<T>>;

    type Prev = T;
}

impl<T> fmt::Display for Next<T>
where
    Self: Reify<u128>,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::REIFIED)
    }
}

impl<T> hash::Hash for Next<T>
where
    Self: Reify<i128>,
{
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        state.write_i128(Self::REIFIED);
    }
}

/// The predecessor to some number.
///
/// Some of the traits in this crate require that `Prev<T>` is negative. If this
/// causes errors, the [`Simplify`] trait can be used to remove redundancies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Prev<T>(PhantomData<T>);

impl<T> Prev<T> {
    /// The value associated with this type.
    pub const VALUE: Prev<T> = Prev(PhantomData);
}

impl<T> Sequence for Prev<T> {
    type Next = T;

    type Prev = Prev<Prev<T>>;
}

impl<T> fmt::Display for Prev<T>
where
    Self: Reify<i128>,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::REIFIED)
    }
}

impl<T> hash::Hash for Prev<T>
where
    Self: Reify<i128>,
{
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        state.write_i128(Self::REIFIED);
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

            impl<T> From<$name<T>> for $ty
            where
                T: Reify<$ty>
            {
                #[inline(always)]
                fn from(_: $name<T>) -> $ty {
                    <$name<T> as Reify<$ty>>::REIFIED
                }
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

/// The number 1.
pub type One = Next<Zero>;

/// The number 2.
pub type Two = Next<One>;

/// The number 3.
pub type Three = Next<Two>;

/// The number 4.
pub type Four = Next<Three>;

/// The number 5.
pub type Five = Next<Four>;

/// The number 6.
pub type Six = Next<Five>;

/// The number 7.
pub type Seven = Next<Six>;

/// The number 8.
pub type Eight = Next<Seven>;

/// The number 9.
pub type Nine = Next<Eight>;

/// The number 10.
pub type Ten = Next<Nine>;

/// Type-level addition.
pub trait Add<T> {
    /// The result of adding `Self` to `T`.
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
    Sum<T, U>: Sub<Two>,
{
    type Result = Difference<Sum<T, U>, Two>;
}

/// The sum of `T` and `U`.
pub type Sum<T, U> = <T as Add<U>>::Result;

/// A convience method for adding two types through their values.
#[inline(always)]
#[must_use]
pub fn sum<T, U, V>(_: T, _: U) -> Sum<T, U>
where
    T: Add<U>,
    Sum<T, U>: Default,
{
    Default::default()
}

/// Type-level multiplication.
pub trait Mul<T> {
    /// The result of multiplying `Self` by `T`.
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

/// The product of `T` and `U`.
pub type Product<T, U> = <T as Mul<U>>::Result;

/// A convience method for multiplying two types through their values.
#[inline(always)]
#[must_use]
pub fn product<T, U, V>(_: T, _: U) -> Product<T, U>
where
    T: Mul<U>,
    Product<T, U>: Default,
{
    Default::default()
}

/// Type-level subtraction.
pub trait Sub<T> {
    /// The result of subtracting `T` from `Self`.
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
    type Result = Difference<T, U>;
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

/// The difference between `T` and `U`.
///
/// Note that this is not absolute difference. For the absolute difference
/// between `T` and `U`, use [`Absolute<Difference<T, U>>`][Absolute].
pub type Difference<T, U> = <T as Sub<U>>::Result;

/// A convience method for subtracting two types through their values.
#[inline(always)]
#[must_use]
pub fn difference<T, U, V>(_: T, _: U) -> Difference<T, U>
where
    T: Sub<U>,
    Difference<T, U>: Default,
{
    Default::default()
}

/// Type-level negation.
pub trait Neg {
    /// The negation of `Self`.
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

/// The negation of `Self`.
pub type Negation<T> = <T as Neg>::Result;

/// A convience method for negating a type through its value.
#[inline(always)]
#[must_use]
pub fn negation<T>(_: T) -> Negation<T>
where
    T: Neg,
    Negation<T>: Default,
{
    Default::default()
}

/// Type-level division.
pub trait Div<T> {
    /// The result of dividing `Self` by `T`.
    type Result;

    /// The remainder when dividing `Self` by `T`.
    type Remainder;
}

mod div_internals {
    use crate::Add;
    use crate::Div;
    use crate::Negation;
    use crate::Next;
    use crate::One;
    use crate::Prev;
    use crate::Sum;
    use crate::Zero;

    pub trait ToDivHelper<Divisor> {
        type Result;
    }

    impl<Divisor> ToDivHelper<Divisor> for Zero {
        type Result = Self;
    }

    impl<T, Divisor> ToDivHelper<Divisor> for Next<T> {
        type Result = Self;
    }

    impl<T, Divisor> ToDivHelper<Divisor> for Prev<T>
    where
        Divisor: Add<Self>,
    {
        type Result = DivHelper<Sum<Divisor, Self>>;
    }

    pub struct DivHelper<T>(T);

    impl<T, U> Div<U> for DivHelper<T> {
        type Result = Negation<One>;

        type Remainder = T;
    }

    pub(crate) type DivHelperOf<T, Divisor> = <T as ToDivHelper<Divisor>>::Result;
}

use div_internals::DivHelperOf;
use div_internals::ToDivHelper;

impl<T> Div<Next<T>> for Zero {
    type Result = Zero;

    type Remainder = Zero;
}

impl<T> Div<Prev<T>> for Zero {
    type Result = Zero;

    type Remainder = Zero;
}

impl<T, U> Div<Next<U>> for Next<T>
where
    Next<U>: Positive,
    Next<T>: Sub<Next<U>> + Positive,
    Difference<Next<T>, Next<U>>: ToDivHelper<Next<U>>,
    DivHelperOf<Difference<Next<T>, Next<U>>, Next<U>>: Div<Next<U>>,
    Quotient<DivHelperOf<Difference<Next<T>, Next<U>>, Next<U>>, Next<U>>: Add<One>,
{
    type Result = Sum<Quotient<DivHelperOf<Difference<Next<T>, Next<U>>, Next<U>>, Next<U>>, One>;

    type Remainder = Remainder<DivHelperOf<Difference<Next<T>, Next<U>>, Next<U>>, Next<U>>;
}

impl<T, U> Div<Prev<U>> for Next<T>
where
    Prev<U>: Neg + Negative,
    Next<T>: Div<Negation<Prev<U>>, Remainder = Zero> + Positive,
    Quotient<Next<T>, Negation<Prev<U>>>: Neg,
{
    type Result = Negation<Quotient<Next<T>, Negation<Prev<U>>>>;

    type Remainder = Zero;
}

impl<T, U> Div<Next<U>> for Prev<T>
where
    Next<U>: Positive,
    Prev<T>: Neg + Negative,
    Negation<Prev<T>>: Div<Next<U>, Remainder = Zero>,
    Quotient<Negation<Prev<T>>, Next<U>>: Neg,
{
    type Result = Negation<Quotient<Negation<Prev<T>>, Next<U>>>;

    type Remainder = Zero;
}

impl<T, U> Div<Prev<U>> for Prev<T>
where
    Prev<T>: Neg + Negative,
    Prev<U>: Neg + Negative,
    Negation<Prev<T>>: Div<Negation<Prev<U>>, Remainder = Zero>,
{
    type Result = Quotient<Negation<Prev<T>>, Negation<Prev<U>>>;

    type Remainder = Zero;
}

/// The quotient of `T` and `U`.
pub type Quotient<T, U> = <T as Div<U>>::Result;

/// The remainder when dividing `T` by `U`.
pub type Remainder<T, U> = <T as Div<U>>::Remainder;

/// A convience method for dividing two types through their values.
#[inline(always)]
#[must_use]
pub fn quotient<T, U, V>(_: T, _: U) -> Quotient<T, U>
where
    T: Div<U>,
    Quotient<T, U>: Default,
{
    Default::default()
}

/// Type-level absolute value.
pub trait Absolute {
    /// The absolute value of `Self`.
    type Result;
}

impl Absolute for Zero {
    type Result = Zero;
}

impl<T> Absolute for Next<T>
where
    Self: Simplify + Positive,
{
    type Result = Simplified<Self>;
}

impl<T> Absolute for Prev<T>
where
    Self: Simplify + Negative,
{
    type Result = Simplified<Self>;
}

/// Simplification of redundancies in type-level numbers.
pub trait Simplify {
    /// The simplification of `Self`.
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

/// The simplification of `T`.
pub type Simplified<T> = <T as Simplify>::Result;

/// Positive type-level numbers.
pub trait Positive: NonNegative + NonZero {}

impl<T> Positive for T where T: NonNegative + NonZero {}

/// Negative type-level numbers.
pub trait Negative: NonPositive + NonZero {}

impl<T> Negative for T where T: NonPositive + NonZero {}

/// Non-negative type-level numbers.
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

/// Non-positive type-level numbers.
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

/// Non-zero type-level numbers.
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

/// Reverse polish notation representation for type-level numerical expressions.
///
/// Numbers from 0-10 inclusive can be represented with numerals, but others
/// must be represented with parenthesised types (e.g. `(Next<Next<Ten>>)`).
///
/// Negation is not supported. Subtract the number from 0 instead.
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
    (@ ((%)       $($rest:tt)*) ($a:tt $b:tt $($stack:tt)*)) => { $crate::rpn!(@ ($($rest)*) (($crate::Remainder<$b, $a> ) $($stack)*)) };

    // Operands
    (@ (($val:ty) $($rest:tt)*) (            $($stack:tt)*)) => { $crate::rpn!(@ ($($rest)*) ($val                         $($stack)*)) };

    // Done
    (@ (                      ) ($val:tt                  )) => { $val };

    // Entry point
    ($($t:tt)+)                                              => { $crate::rpn!(@ ($(($t))+)  (                                       )) };
}

#[cfg(test)]
mod test;
