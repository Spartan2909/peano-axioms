use crate::rpn;
use crate::Abs;
use crate::Absolute;
use crate::Exp;
use crate::Exponent;
use crate::Gcd;
use crate::Lcm;
use crate::LeastCommonMultiple;
use crate::Negation;
use crate::Negative;
use crate::Next;
use crate::NonNegative;
use crate::NonPositive;
use crate::NonZero;
use crate::One;
use crate::Positive;
use crate::Prev;
use crate::Product;
use crate::Quotient;
use crate::Reify;
use crate::Simplified;
use crate::Simplify;
use crate::Sum;
use crate::Zero;

use core::fmt;
use core::marker::PhantomData;
use core::ops::Add;
use core::ops::Div;
use core::ops::Mul;
use core::ops::Neg;
use core::ops::Rem;
use core::ops::Sub;

use local_type_alias::local_alias;

pub trait Sign {
    type Result;
}

impl Sign for Next<Zero> {
    type Result = Next<Zero>;
}

impl<T> Sign for Next<Next<T>>
where
    Self: Positive,
    Next<T>: Sign,
{
    type Result = <Next<T> as Sign>::Result;
}

impl Sign for Prev<Zero> {
    type Result = Prev<Zero>;
}

impl<T> Sign for Prev<Prev<T>>
where
    Self: Negative,
    Prev<T>: Sign,
{
    type Result = <Prev<T> as Sign>::Result;
}

type SignOf<T> = <T as Sign>::Result;

/// A ratio of two numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Fraction<Num, Dem: Positive>(PhantomData<(Num, Dem)>);

impl<Num, Dem: Positive> Fraction<Num, Dem> {
    /// The value associated with this type.
    pub const VALUE: Self = Fraction(PhantomData);
}

impl<Num, Dem> fmt::Display for Fraction<Num, Dem>
where
    Dem: Positive,
    Self: Reify<(i128, i128)>,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (num, dem) = Self::REIFIED;
        write!(f, "{num}/{dem}")
    }
}

macro_rules! refiy_fraction {
    ($($ty:ty),* $(,)?) => {
        $(
            impl<Num, Dem: Positive> Reify<($ty, $ty)> for Fraction<Num, Dem>
            where
                Num: Reify<$ty>,
                Dem: Reify<$ty>,
            {
                const REIFIED: ($ty, $ty) = (Num::REIFIED, Dem::REIFIED);
            }
        )*
    };
}

refiy_fraction![u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];

#[allow(unused_parens)]
#[local_alias(macros)]
impl<Num, Dem> Simplify for Fraction<Num, Dem>
where
    Num: Simplify,
    Dem: Simplify + Positive,
    Simplified<Num>: Abs + Sign,
    Simplified<Dem>: Abs + Sign,
    SignOf<Simplified<Num>>: Mul<SignOf<Simplified<Dem>>>,
    alias!(SimplifiedGcd = rpn!(Num simplify abs Dem simplify abs gcd)):,
    Absolute<Simplified<Num>>: Gcd<Absolute<Simplified<Dem>>> + Div<SimplifiedGcd>,
    Absolute<Simplified<Dem>>: Div<SimplifiedGcd>,
    rpn!(Dem simplify abs ({{SimplifiedGcd}}) /): Positive,
    rpn!(Num simplify abs ({{SimplifiedGcd}}) /):
        Mul<rpn!(Num simplify [1 SignOf] Dem simplify [1 SignOf] *)>,
{
    type Result = rpn!(
                Num simplify abs ({{SimplifiedGcd}}) /
                Num simplify [1 SignOf] Dem simplify [1 SignOf] *
            *
            Dem simplify abs ({{SimplifiedGcd}}) /
        fract
    );
}

#[allow(unused_parens)]
impl<Num1, Dem1, Num2, Dem2> Add<Fraction<Num2, Dem2>> for Fraction<Num1, Dem1>
where
    Dem1: Lcm<Dem2> + Positive,
    Dem2: Positive,
    LeastCommonMultiple<Dem1, Dem2>: Div<Dem1> + Div<Dem2> + Positive, // All trivial
    Num1: Mul<rpn!(Dem1 Dem2 lcm Dem1 /)>,
    Num2: Mul<rpn!(Dem1 Dem2 lcm Dem2 /)>,
    rpn!(Num1 Dem1 Dem2 lcm Dem1 / *): Add<rpn!(Num2 Dem1 Dem2 lcm Dem2 / *)>,
    rpn!(
            Num1 Dem1 Dem2 lcm Dem1 / * Num2 Dem1 Dem2 lcm Dem2 / * +
            Dem1 Dem2 lcm
        fract
    ): Simplify,
    rpn!(
            Num1 Dem1 Dem2 lcm Dem1 / * Num2 Dem1 Dem2 lcm Dem2 / * +
            Dem1 Dem2 lcm
        fract simplify
    ): Default,
{
    type Output = rpn!(
            Num1 Dem1 Dem2 lcm Dem1 / * Num2 Dem1 Dem2 lcm Dem2 / * +
            Dem1 Dem2 lcm
        fract simplify
    );

    #[inline(always)]
    fn add(self, _: Fraction<Num2, Dem2>) -> Self::Output {
        Self::Output::default()
    }
}

impl<Num1, Dem1, Num2, Dem2> Sub<Fraction<Num2, Dem2>> for Fraction<Num1, Dem1>
where
    Dem1: Positive,
    Dem2: Positive,
    Fraction<Num2, Dem2>: Neg,
    Fraction<Num1, Dem1>: Add<Negation<Fraction<Num2, Dem2>>>,
    Sum<Fraction<Num1, Dem1>, Negation<Fraction<Num2, Dem2>>>: Default,
{
    type Output = Sum<Fraction<Num1, Dem1>, Negation<Fraction<Num2, Dem2>>>;

    #[inline(always)]
    fn sub(self, _: Fraction<Num2, Dem2>) -> Self::Output {
        Self::Output::default()
    }
}

#[allow(unused_parens)]
impl<Num1, Dem1, Num2, Dem2> Mul<Fraction<Num2, Dem2>> for Fraction<Num1, Dem1>
where
    Num1: Mul<Num2>,
    Dem1: Mul<Dem2> + Positive,
    Dem2: Positive,
    Product<Dem1, Dem2>: Positive, // Trivial
    rpn!(Num1 Num2 * Dem1 Dem2 * fract): Simplify,
    rpn!(Num1 Num2 * Dem1 Dem2 * fract simplify): Default,
{
    type Output = rpn!(Num1 Num2 * Dem1 Dem2 * fract simplify);

    #[inline(always)]
    fn mul(self, _: Fraction<Num2, Dem2>) -> Self::Output {
        Self::Output::default()
    }
}

#[allow(unused_parens)]
impl<Num1, Dem1, Num2, Dem2> Div<Fraction<Num2, Dem2>> for Fraction<Num1, Dem1>
where
    Dem1: Positive,
    Dem2: Positive,
    Fraction<Num2, Dem2>: Inverse,
    Fraction<Num1, Dem1>: Mul<Reciprocal<Fraction<Num2, Dem2>>>,
    rpn!(Num1 Dem1 fract Num2 Dem2 fract inv *): Default,
{
    type Output = rpn!(Num1 Dem1 fract Num2 Dem2 fract inv *);

    #[inline(always)]
    fn div(self, _: Fraction<Num2, Dem2>) -> Self::Output {
        Self::Output::default()
    }
}

impl<Num, Dem: Positive> Exp<Zero> for Fraction<Num, Dem> {
    type Result = One;
}

impl<Num, Dem, T> Exp<Next<T>> for Fraction<Num, Dem>
where
    Dem: Positive,
    Fraction<Num, Dem>: Exp<T>,
    Exponent<Fraction<Num, Dem>, T>: Mul<Fraction<Num, Dem>>,
{
    type Result = Product<Exponent<Fraction<Num, Dem>, T>, Fraction<Num, Dem>>;
}

impl<Num, Dem, T> Exp<Prev<T>> for Fraction<Num, Dem>
where
    Dem: Positive,
    Prev<T>: Negative + Abs,
    Fraction<Num, Dem>: Exp<Absolute<Prev<T>>>,
    Exponent<Fraction<Num, Dem>, Absolute<Prev<T>>>: Inverse,
{
    type Result = Reciprocal<Exponent<Fraction<Num, Dem>, Absolute<Prev<T>>>>;
}

impl<Num, Dem: Positive> Add<Zero> for Fraction<Num, Dem> {
    type Output = Self;

    #[inline(always)]
    fn add(self, _: Zero) -> Self::Output {
        Self::Output::VALUE
    }
}

impl<Num, Dem: Positive> Add<Fraction<Num, Dem>> for Zero {
    type Output = Fraction<Num, Dem>;

    #[inline(always)]
    fn add(self, _: Fraction<Num, Dem>) -> Self::Output {
        Self::Output::VALUE
    }
}

impl<Num, Dem: Positive> Sub<Zero> for Fraction<Num, Dem> {
    type Output = Self;

    #[inline(always)]
    fn sub(self, _: Zero) -> Self::Output {
        Self::Output::VALUE
    }
}

impl<Num, Dem: Positive> Sub<Fraction<Num, Dem>> for Zero {
    type Output = Fraction<Num, Dem>;

    #[inline(always)]
    fn sub(self, _: Fraction<Num, Dem>) -> Self::Output {
        Self::Output::VALUE
    }
}

impl<Num, Dem: Positive> Mul<Zero> for Fraction<Num, Dem> {
    type Output = Zero;

    #[inline(always)]
    fn mul(self, _: Zero) -> Self::Output {
        Zero
    }
}

impl<Num, Dem: Positive> Mul<Fraction<Num, Dem>> for Zero {
    type Output = Zero;

    #[inline(always)]
    fn mul(self, _: Fraction<Num, Dem>) -> Self::Output {
        Zero
    }
}

impl<Num, Dem: Positive> Div<Fraction<Num, Dem>> for Zero {
    type Output = Zero;

    #[inline(always)]
    fn div(self, _: Fraction<Num, Dem>) -> Self::Output {
        Zero
    }
}

impl<Num: Neg, Dem: Positive> Neg for Fraction<Num, Dem> {
    type Output = Fraction<Negation<Num>, Dem>;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self::Output::VALUE
    }
}

impl<Num: NonZero, Dem: Positive> NonZero for Fraction<Num, Dem> {}

impl<Num: NonNegative, Dem: Positive> NonNegative for Fraction<Num, Dem> {}

impl<Num: NonPositive, Dem: Positive> NonPositive for Fraction<Num, Dem> {}

/// Type level fraction-to-integer conversion.
///
/// Also implemented from integers to themselves for convenience.
pub trait ToInt {
    /// `Self` as an integer.
    type Result;
}

impl ToInt for Zero {
    type Result = Zero;
}

impl<T> ToInt for Next<T>
where
    Self: Simplify,
{
    type Result = Simplified<Self>;
}

impl<T> ToInt for Prev<T>
where
    Self: Simplify,
{
    type Result = Simplified<Self>;
}

impl<Num, Dem> ToInt for Fraction<Num, Dem>
where
    Dem: Positive,
    Num: Div<Dem> + Rem<Dem, Output = Zero>,
{
    type Result = Quotient<Num, Dem>;
}

/// Type-level inverse.
pub trait Inverse {
    /// The reciprocal of `Self`.
    type Result;
}

impl<T> Inverse for Next<T>
where
    Self: Positive,
{
    type Result = Fraction<One, Self>;
}

impl<T> Inverse for Prev<T>
where
    Self: Negative + Abs,
    Absolute<Self>: Positive, // Trivial
{
    type Result = Fraction<Negation<One>, Absolute<Self>>;
}

#[allow(unused_parens)]
impl<Num, Dem> Inverse for Fraction<Num, Dem>
where
    Num: Simplify + NonZero,
    Simplified<Num>: Abs + Sign,
    Dem: Mul<SignOf<Simplified<Num>>> + Positive,
    Absolute<Simplified<Num>>: Positive, // Trivial
{
    type Result = Fraction<rpn!(Dem Num simplify [1 SignOf] *), Absolute<Simplified<Num>>>;
}

/// The reciprocal of `T`.
pub type Reciprocal<T> = <T as Inverse>::Result;
