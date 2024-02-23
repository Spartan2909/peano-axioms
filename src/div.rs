use crate::rpn;
use crate::Difference;
use crate::Negation;
use crate::Negative;
use crate::Next;
use crate::One;
use crate::Positive;
use crate::Prev;
use crate::Sum;
use crate::Zero;

use core::ops::Add;
use core::ops::Div;
use core::ops::Neg;
use core::ops::Rem;
use core::ops::Sub;

use local_type_alias::local_alias;

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
    type Output = Negation<One>;

    fn div(self, _: U) -> Self::Output {
        Self::Output::VALUE
    }
}

impl<T: Default, U> Rem<U> for DivHelper<T> {
    type Output = T;

    fn rem(self, _: U) -> Self::Output {
        Self::Output::default()
    }
}

pub type DivHelperOf<T, Divisor> = <T as ToDivHelper<Divisor>>::Result;

impl<T> Div<Next<T>> for Zero {
    type Output = Zero;

    #[inline(always)]
    fn div(self, _: Next<T>) -> Self::Output {
        Zero
    }
}

impl<T> Rem<Next<T>> for Zero {
    type Output = Zero;

    #[inline(always)]
    fn rem(self, _: Next<T>) -> Self::Output {
        Zero
    }
}

impl<T> Div<Prev<T>> for Zero {
    type Output = Zero;

    #[inline(always)]
    fn div(self, _: Prev<T>) -> Self::Output {
        Zero
    }
}

impl<T> Rem<Prev<T>> for Zero {
    type Output = Zero;

    #[inline(always)]
    fn rem(self, _: Prev<T>) -> Self::Output {
        Zero
    }
}

#[allow(unused_parens)]
#[local_alias(macros)]
impl<T, U> Div<Next<U>> for Next<T>
where
    Next<U>: Positive,
    Next<T>: Sub<Next<U>> + Positive,
    Difference<Next<T>, Next<U>>: ToDivHelper<Next<U>>,
    alias!(DivHelper = DivHelperOf<Difference<Next<T>, Next<U>>, Next<U>>):,
    DivHelper: Div<Next<U>> + Rem<Next<U>>,
    Quotient<DivHelper, Next<U>>: Add<One>,
    Sum<Quotient<DivHelper, Next<U>>, One>: Default,
{
    type Output = rpn!(({{DivHelper}}) (Next<U>) / 1 +);

    #[inline(always)]
    fn div(self, _: Next<U>) -> Self::Output {
        Self::Output::default()
    }
}

#[local_alias]
impl<T, U> Rem<Next<U>> for Next<T>
where
    Next<U>: Positive,
    Next<T>: Sub<Next<U>> + Positive,
    Difference<Next<T>, Next<U>>: ToDivHelper<Next<U>>,
    alias!(DivHelper = DivHelperOf<Difference<Next<T>, Next<U>>, Next<U>>):,
    DivHelper: Div<Next<U>> + Rem<Next<U>>,
    Quotient<DivHelper, Next<U>>: Add<One>,
    Remainder<DivHelper, Next<U>>: Default,
{
    type Output = Remainder<DivHelper, Next<U>>;

    #[inline(always)]
    fn rem(self, _: Next<U>) -> Self::Output {
        Self::Output::default()
    }
}

impl<T, U> Div<Prev<U>> for Next<T>
where
    Prev<U>: Neg + Negative,
    Next<T>: Div<Negation<Prev<U>>> + Rem<Output = Zero> + Positive,
    Quotient<Next<T>, Negation<Prev<U>>>: Neg,
    Negation<Quotient<Next<T>, Negation<Prev<U>>>>: Default,
{
    type Output = Negation<Quotient<Next<T>, Negation<Prev<U>>>>;

    #[inline(always)]
    fn div(self, _: Prev<U>) -> Self::Output {
        Self::Output::default()
    }
}

impl<T, U> Rem<Prev<U>> for Next<T>
where
    Prev<U>: Neg + Negative,
    Next<T>: Div<Negation<Prev<U>>> + Rem<Output = Zero> + Positive,
    Quotient<Next<T>, Negation<Prev<U>>>: Neg,
{
    type Output = Zero;

    #[inline(always)]
    fn rem(self, _: Prev<U>) -> Self::Output {
        Zero
    }
}

impl<T, U> Div<Next<U>> for Prev<T>
where
    Next<U>: Positive,
    Prev<T>: Neg + Negative,
    Negation<Prev<T>>: Div<Next<U>> + Rem<Output = Zero>,
    Quotient<Negation<Prev<T>>, Next<U>>: Neg,
    Negation<Quotient<Negation<Prev<T>>, Next<U>>>: Default,
{
    type Output = Negation<Quotient<Negation<Prev<T>>, Next<U>>>;

    #[inline(always)]
    fn div(self, _: Next<U>) -> Self::Output {
        Self::Output::default()
    }
}

impl<T, U> Rem<Next<U>> for Prev<T>
where
    Next<U>: Positive,
    Prev<T>: Neg + Negative,
    Negation<Prev<T>>: Div<Next<U>> + Rem<Output = Zero>,
    Quotient<Negation<Prev<T>>, Next<U>>: Neg,
{
    type Output = Zero;

    #[inline(always)]
    fn rem(self, _: Next<U>) -> Self::Output {
        Zero
    }
}

impl<T, U> Div<Prev<U>> for Prev<T>
where
    Prev<T>: Neg + Negative,
    Prev<U>: Neg + Negative,
    Negation<Prev<T>>: Div<Negation<Prev<U>>> + Rem<Output = Zero>,
    Quotient<Negation<Prev<T>>, Negation<Prev<U>>>: Default,
{
    type Output = Quotient<Negation<Prev<T>>, Negation<Prev<U>>>;

    #[inline(always)]
    fn div(self, _: Prev<U>) -> Self::Output {
        Self::Output::default()
    }
}

impl<T, U> Rem<Prev<U>> for Prev<T>
where
    Prev<T>: Neg + Negative,
    Prev<U>: Neg + Negative,
    Negation<Prev<T>>: Div<Negation<Prev<U>>> + Rem<Output = Zero>,
{
    type Output = Zero;

    #[inline(always)]
    fn rem(self, _: Prev<U>) -> Self::Output {
        Zero
    }
}

/// The quotient of `T` and `U`.
pub type Quotient<T, U> = <T as Div<U>>::Output;

/// The remainder when dividing `T` by `U`.
pub type Remainder<T, U> = <T as Rem<U>>::Output;
