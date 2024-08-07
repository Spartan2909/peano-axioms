//! Type-level numbers based on an extension of the Peano axioms.

#![no_std]
#![recursion_limit = "256"]
#![cfg_attr(test, allow(unused_parens))]

mod div;
pub use div::Quotient;
pub use div::Remainder;

mod fraction;
pub use fraction::Fraction;
pub use fraction::Inverse;
pub use fraction::Reciprocal;
pub use fraction::ToInt;

mod gcd;
pub use gcd::Gcd;
pub use gcd::GreatestCommonDivisor;

use core::fmt;
use core::hash;
use core::marker::PhantomData;
use core::ops::Add;
use core::ops::Div;
use core::ops::Mul;
use core::ops::Neg;
use core::ops::Sub;

/// Types which can be converted to a runtime value.
pub trait Reify<T> {
    /// The runtime representation of this type.
    const REIFIED: T;

    /// A convenience method for getting the runtime representation of this type.
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

impl Add<Zero> for Zero {
    type Output = Zero;

    #[inline(always)]
    fn add(self, _: Zero) -> Self::Output {
        Zero
    }
}

impl<T> Add<Zero> for Next<T> {
    type Output = Self;

    #[inline(always)]
    fn add(self, _: Zero) -> Self::Output {
        Self::VALUE
    }
}

impl<T> Add<Zero> for Prev<T> {
    type Output = Self;

    #[inline(always)]
    fn add(self, _: Zero) -> Self::Output {
        Self::VALUE
    }
}

impl<T> Add<Next<T>> for Zero {
    type Output = Next<T>;

    #[inline(always)]
    fn add(self, _: Next<T>) -> Self::Output {
        Self::Output::VALUE
    }
}

impl<T> Add<Prev<T>> for Zero {
    type Output = Prev<T>;

    #[inline(always)]
    fn add(self, _: Prev<T>) -> Self::Output {
        Self::Output::VALUE
    }
}

impl<T, U> Add<Next<U>> for Next<T>
where
    T: Add<U>,
{
    type Output = Next<Next<Sum<T, U>>>;

    #[inline(always)]
    fn add(self, _: Next<U>) -> Self::Output {
        Self::Output::VALUE
    }
}

impl<T, U> Add<Prev<U>> for Next<T>
where
    T: Add<U>,
    Sum<T, U>: Default,
{
    type Output = Sum<T, U>;

    #[inline(always)]
    fn add(self, _: Prev<U>) -> Self::Output {
        Self::Output::default()
    }
}

impl<T, U> Add<Next<U>> for Prev<T>
where
    T: Add<U>,
    Sum<T, U>: Default,
{
    type Output = Sum<T, U>;

    #[inline(always)]
    fn add(self, _: Next<U>) -> Self::Output {
        Self::Output::default()
    }
}

impl<T, U> Add<Prev<U>> for Prev<T>
where
    T: Add<U>,
    Sum<T, U>: Sub<Two>,
    Difference<Sum<T, U>, Two>: Default,
{
    type Output = Difference<Sum<T, U>, Two>;

    #[inline(always)]
    fn add(self, _: Prev<U>) -> Self::Output {
        Self::Output::default()
    }
}

/// The sum of `T` and `U`.
pub type Sum<T, U> = <T as Add<U>>::Output;

impl Mul<Zero> for Zero {
    type Output = Zero;

    #[inline(always)]
    fn mul(self, _: Zero) -> Self::Output {
        Zero
    }
}

impl<T> Mul<Zero> for Next<T> {
    type Output = Zero;

    #[inline(always)]
    fn mul(self, _: Zero) -> Self::Output {
        Zero
    }
}

impl<T> Mul<Zero> for Prev<T> {
    type Output = Zero;

    #[inline(always)]
    fn mul(self, _: Zero) -> Self::Output {
        Zero
    }
}

impl<T> Mul<Next<T>> for Zero {
    type Output = Zero;

    #[inline(always)]
    fn mul(self, _: Next<T>) -> Self::Output {
        Zero
    }
}

impl<T> Mul<Prev<T>> for Zero {
    type Output = Zero;

    #[inline(always)]
    fn mul(self, _: Prev<T>) -> Self::Output {
        Zero
    }
}

impl<T, U> Mul<Next<U>> for Next<T>
where
    Next<T>: Mul<U> + Add<Product<Next<T>, U>>,
    Sum<Next<T>, Product<Next<T>, U>>: Default,
{
    type Output = Sum<Next<T>, Product<Next<T>, U>>;

    #[inline(always)]
    fn mul(self, _: Next<U>) -> Self::Output {
        Self::Output::default()
    }
}

impl<T, U> Mul<Prev<U>> for Next<T>
where
    T: Mul<U>,
    Product<T, U>: Sub<T>,
    Difference<Product<T, U>, T>: Add<U>,
    Sum<Difference<Product<T, U>, T>, U>: Sub<One>,
    Difference<Sum<Difference<Product<T, U>, T>, U>, One>: Default,
{
    type Output = Difference<Sum<Difference<Product<T, U>, T>, U>, One>;

    #[inline(always)]
    fn mul(self, _: Prev<U>) -> Self::Output {
        Self::Output::default()
    }
}

impl<T, U> Mul<Next<U>> for Prev<T>
where
    Next<U>: Mul<Prev<T>>,
    Product<Next<U>, Prev<T>>: Default,
{
    type Output = Product<Next<U>, Prev<T>>;

    #[inline(always)]
    fn mul(self, _: Next<U>) -> Self::Output {
        Self::Output::default()
    }
}

impl<T, U> Mul<Prev<U>> for Prev<T>
where
    T: Mul<U>,
    Product<T, U>: Sub<T>,
    Difference<Product<T, U>, T>: Sub<U>,
    Difference<Difference<Product<T, U>, T>, U>: Add<One>,
    Sum<Difference<Difference<Product<T, U>, T>, U>, One>: Default,
{
    type Output = Sum<Difference<Difference<Product<T, U>, T>, U>, One>;

    #[inline(always)]
    fn mul(self, _: Prev<U>) -> Self::Output {
        Self::Output::default()
    }
}

/// The product of `T` and `U`.
pub type Product<T, U> = <T as Mul<U>>::Output;

impl Sub<Zero> for Zero {
    type Output = Zero;

    #[inline(always)]
    fn sub(self, _: Zero) -> Self::Output {
        Zero
    }
}

impl<T: Neg> Sub<Next<T>> for Zero {
    type Output = Prev<Negation<T>>;

    #[inline(always)]
    fn sub(self, _: Next<T>) -> Self::Output {
        Self::Output::VALUE
    }
}

impl<T: Neg> Sub<Prev<T>> for Zero {
    type Output = Next<Negation<T>>;

    #[inline(always)]
    fn sub(self, _: Prev<T>) -> Self::Output {
        Self::Output::VALUE
    }
}

impl<T> Sub<Zero> for Next<T> {
    type Output = Self;

    #[inline(always)]
    fn sub(self, _: Zero) -> Self::Output {
        Self::Output::VALUE
    }
}

impl<T> Sub<Zero> for Prev<T> {
    type Output = Self;

    #[inline(always)]
    fn sub(self, _: Zero) -> Self::Output {
        Self::Output::VALUE
    }
}

impl<T, U> Sub<Next<U>> for Next<T>
where
    T: Sub<U>,
    Difference<T, U>: Default,
{
    type Output = Difference<T, U>;

    #[inline(always)]
    fn sub(self, _: Next<U>) -> Self::Output {
        Self::Output::default()
    }
}

impl<T, U> Sub<Prev<U>> for Next<T>
where
    Prev<U>: Neg,
    Next<T>: Add<Negation<Prev<U>>>,
    Sum<Next<T>, Negation<Prev<U>>>: Default,
{
    type Output = Sum<Next<T>, Negation<Prev<U>>>;

    #[inline(always)]
    fn sub(self, _: Prev<U>) -> Self::Output {
        Self::Output::default()
    }
}

impl<T, U> Sub<Next<U>> for Prev<T>
where
    Prev<Prev<T>>: Sub<U>,
    Difference<Prev<Prev<T>>, U>: Default,
{
    type Output = Difference<Prev<Prev<T>>, U>;

    #[inline(always)]
    fn sub(self, _: Next<U>) -> Self::Output {
        Self::Output::default()
    }
}

impl<T, U> Sub<Prev<U>> for Prev<T>
where
    T: Sub<U>,
    Difference<T, U>: Default,
{
    type Output = Difference<T, U>;

    #[inline(always)]
    fn sub(self, _: Prev<U>) -> Self::Output {
        Self::Output::default()
    }
}

/// The difference between `T` and `U`.
///
/// Note that this is not absolute difference. For the absolute difference
/// between `T` and `U`, use [`Absolute<Difference<T, U>>`][Absolute].
pub type Difference<T, U> = <T as Sub<U>>::Output;

/// Type-level exponentiation.
pub trait Exp<T> {
    /// `Self` raised to the power of `T`.
    type Result;
}

impl Exp<Zero> for Zero {
    type Result = One;
}

impl<T> Exp<Zero> for Next<T> {
    type Result = One;
}

impl<T> Exp<Next<T>> for Zero {
    type Result = Zero;
}

impl<T> Exp<Zero> for Prev<T> {
    type Result = One;
}

impl<T> Exp<Prev<T>> for Zero {
    type Result = Zero;
}

impl<T, U> Exp<Next<U>> for Next<T>
where
    Next<U>: Positive,
    Next<T>: Exp<U>,
    Exponent<Next<T>, U>: Mul<Next<T>>,
{
    type Result = Product<Exponent<Next<T>, U>, Next<T>>;
}

impl<T, U> Exp<Prev<U>> for Next<T>
where
    Prev<U>: Negative + Abs,
    Next<T>: Exp<Absolute<Prev<U>>>,
    Exponent<Next<T>, Absolute<Prev<U>>>: Inverse,
{
    type Result = Reciprocal<Exponent<Next<T>, Absolute<Prev<U>>>>;
}

impl<T, U> Exp<Next<U>> for Prev<T>
where
    Next<U>: Positive,
    Prev<T>: Exp<U>,
    Exponent<Prev<T>, U>: Mul<Prev<T>>,
{
    type Result = Product<Exponent<Prev<T>, U>, Prev<T>>;
}

impl<T, U> Exp<Prev<U>> for Prev<T>
where
    Prev<U>: Negative + Abs,
    Prev<T>: Exp<Absolute<Prev<U>>>,
    Exponent<Prev<T>, Absolute<Prev<U>>>: Inverse,
{
    type Result = Reciprocal<Exponent<Prev<T>, Absolute<Prev<U>>>>;
}

/// `T` raised to the power of `U`.
pub type Exponent<T, U> = <T as Exp<U>>::Result;

impl Neg for Zero {
    type Output = Zero;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Zero
    }
}

impl<T: Neg> Neg for Next<T> {
    type Output = Prev<Negation<T>>;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self::Output::VALUE
    }
}

impl<T: Neg> Neg for Prev<T> {
    type Output = Next<Negation<T>>;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self::Output::VALUE
    }
}

/// The negation of `Self`.
pub type Negation<T> = <T as Neg>::Output;

/// Type-level least common multiple.
pub trait Lcm<T> {
    /// The least common multiple of `Self` and `T`.
    type Result;
}

impl Lcm<Zero> for Zero {
    type Result = Zero;
}

impl<T> Lcm<Next<T>> for Zero {
    type Result = Zero;
}

impl<T> Lcm<Zero> for Next<T> {
    type Result = Zero;
}

impl<T> Lcm<Prev<T>> for Zero {
    type Result = Zero;
}

impl<T> Lcm<Zero> for Prev<T> {
    type Result = Zero;
}

macro_rules! impl_lcm {
    ($(($name1:ident, $name2:ident)),* $(,)?) => {
        $(
            #[allow(unused_parens)]
            impl<T, U> Lcm<$name2<U>> for $name1<T>
            where
                $name1<T>: Abs,
                $name2<U>: Abs,
                Absolute<$name1<T>>: Mul<Absolute<$name2<U>>>,
                Absolute<$name1<T>>: Gcd<Absolute<$name2<U>>>,
                Product<Absolute<$name1<T>>, Absolute<$name2<U>>>:
                    Div<GreatestCommonDivisor<Absolute<$name1<T>>, Absolute<$name2<U>>>>,
            {
                type Result = rpn!(
                    ($name1<T>) abs ($name2<U>) abs * ($name1<T>) abs ($name2<U>) abs gcd /
                );
            }
        )*
    };
}

impl_lcm![(Next, Next), (Next, Prev), (Prev, Next), (Prev, Prev)];

/// The least common multiple of `T` and `U`.
pub type LeastCommonMultiple<T, U> = <T as Lcm<U>>::Result;

/// Type-level absolute value.
pub trait Abs {
    /// The absolute value of `Self`.
    type Result: NonNegative;
}

impl Abs for Zero {
    type Result = Zero;
}

impl<T> Abs for Next<T>
where
    Self: Simplify,
    Simplified<Self>: Positive,
{
    type Result = Simplified<Self>;
}

impl<T> Abs for Prev<T>
where
    Self: Simplify,
    Simplified<Self>: Neg,
    Negation<Simplified<Self>>: Positive,
{
    type Result = Negation<Simplified<Self>>;
}

/// The absolute value of `T`.
pub type Absolute<T> = <T as Abs>::Result;

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

#[doc(hidden)]
#[macro_export]
macro_rules! rpn_impl {
   // Numerals
   (((0            ) $($rest:tt)*) (            $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) (($crate::Zero                         ) $($stack)*)) };
   (((1            ) $($rest:tt)*) (            $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) (($crate::One                          ) $($stack)*)) };
   (((2            ) $($rest:tt)*) (            $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) (($crate::Two                          ) $($stack)*)) };
   (((3            ) $($rest:tt)*) (            $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) (($crate::Three                        ) $($stack)*)) };
   (((4            ) $($rest:tt)*) (            $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) (($crate::Four                         ) $($stack)*)) };
   (((5            ) $($rest:tt)*) (            $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) (($crate::Five                         ) $($stack)*)) };
   (((6            ) $($rest:tt)*) (            $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) (($crate::Six                          ) $($stack)*)) };
   (((7            ) $($rest:tt)*) (            $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) (($crate::Seven                        ) $($stack)*)) };
   (((8            ) $($rest:tt)*) (            $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) (($crate::Eight                        ) $($stack)*)) };
   (((9            ) $($rest:tt)*) (            $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) (($crate::Nine                         ) $($stack)*)) };
   (((10           ) $($rest:tt)*) (            $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) (($crate::Ten                          ) $($stack)*)) };

   // Standard operators
   (((+            ) $($rest:tt)*) ($a:tt $b:tt $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) (($crate::Sum<$b, $a>                  ) $($stack)*)) };
   (((-            ) $($rest:tt)*) ($a:tt $b:tt $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) (($crate::Difference<$b, $a>           ) $($stack)*)) };
   (((*            ) $($rest:tt)*) ($a:tt $b:tt $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) (($crate::Product<$b, $a>              ) $($stack)*)) };
   (((/            ) $($rest:tt)*) ($a:tt $b:tt $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) (($crate::Quotient<$b, $a>             ) $($stack)*)) };
   (((%            ) $($rest:tt)*) ($a:tt $b:tt $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) (($crate::Remainder<$b, $a>            ) $($stack)*)) };
   (((~            ) $($rest:tt)*) ($a:tt       $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) (($crate::Negation<$a>                 ) $($stack)*)) };
   (((^            ) $($rest:tt)*) ($a:tt $b:tt $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) (($crate::Exponent<$b, $a>             ) $($stack)*)) };

   (((abs          ) $($rest:tt)*) ($a:tt       $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) (($crate::Absolute<$a>                 ) $($stack)*)) };
   (((gcd          ) $($rest:tt)*) ($a:tt $b:tt $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) (($crate::GreatestCommonDivisor<$b, $a>) $($stack)*)) };
   (((lcm          ) $($rest:tt)*) ($a:tt $b:tt $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) (($crate::LeastCommonMultiple<$b, $a>  ) $($stack)*)) };
   (((simplify     ) $($rest:tt)*) ($a:tt       $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) (($crate::Simplified<$a>               ) $($stack)*)) };
   (((fract        ) $($rest:tt)*) ($a:tt $b:tt $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) (($crate::Fraction<$b, $a>             ) $($stack)*)) };
   (((inv          ) $($rest:tt)*) ($a:tt       $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) (($crate::Reciprocal<$a>               ) $($stack)*)) };
   (((int          ) $($rest:tt)*) ($a:tt       $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) (($crate::ToInt<$a>                    ) $($stack)*)) };
   (((dup          ) $($rest:tt)*) ($a:tt       $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) (($a               ) ($a               ) $($stack)*)) };

   // Custom operators
   ((([1 $op:ident]) $($rest:tt)*) ($a:tt       $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) (($op<$a>                              ) $($stack)*)) };
   ((([2 $op:ident]) $($rest:tt)*) ($a:tt $b:tt $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) (($op<$b, $a>                          ) $($stack)*)) };

   // Operands
   ((($val:ty      ) $($rest:tt)*) (            $($stack:tt)*)) => { $crate::rpn_impl!(($($rest)*) ($val                                    $($stack)*)) };

   // Done
   ((                             ) ($val:tt                 )) => { $val };
}

/// Reverse polish notation representation for type-level numerical expressions.
///
/// Numbers from 0-10 inclusive can be represented with numerals, but others
/// must be represented with parenthesised types (e.g. `(Next<Next<Ten>>)`).
///
/// The available operators are `+`, `-`, `*`, `/`, `%`, `~`, `^`, `abs` (absolute value),
/// `gcd` (greatest common divisor), `lcm` (least common multiple), `simplify`,
/// `fract` (precise division with fractions), `inv` (reciprocal),
/// `int` (precise fraction-integer conversion), `dup` (duplicates the operand).
///
/// `[n Ty]` will use `Ty` as an operator by giving it `n` type parameters formed from the top `n`
/// operands on the stack. The operand at the top of the stack will be the last type parameter, and
/// will continue down the stack until `n` operands have been consumed. Currently only `n=1` and
/// `n=2` are supported.
#[macro_export]
macro_rules! rpn {
    // Entry point
    ($($t:tt)+) => { $crate::rpn_impl!(($(($t))+) ()) };
}

#[cfg(test)]
mod test;
