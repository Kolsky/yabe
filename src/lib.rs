//! Yet another bitcast API endeavour.
//! 
//! See [Bb] for more information.
#![no_std]

use core::marker::PhantomData;
use core::ops::Deref;
use core::ops::Mul;

/// Bit bashing struct.
/// 
/// `x: int ⇒ x * Bb = x`.
/// 
/// Chain `Bb` fields to apply consecutive operations.
/// 
/// ### Fields
/// - `Bb.s` — change type to its signed counterpart of the same bit width.
/// - `Bb.u` — change type to its unsigned counterpart.
/// - `Bb.t{width}` — truncate to a type with equivalent signedness and specified bit width.
/// - `Bb.e{width}` — extend to a type with equivalent signedness and specified bit width.
/// Depending on type signedness, this can be either zero (unsigned) or signed extension.
/// 
/// ### Examples
/// 
/// ```
/// # use yabe::Bb;
/// // it's highly recommended to ascribe the type of left hand side, as shown here
/// assert_eq!(-1i32 * Bb.u.e64.s, 4294967295);
/// ```
pub struct Bb;

impl Deref for Bb {
    type Target = CastSigned<Bb>;

    fn deref(&self) -> &Self::Target {
        &CastSigned {
            s: RL(PhantomData)
        }
    }
}

#[doc(hidden)]
pub struct RL<T, U>(PhantomData<(T, U)>);

impl<T, U> Copy for RL<T, U> {}

impl<T, U> Clone for RL<T, U> {
    fn clone(&self) -> Self {
        *self
    }
}

#[doc(hidden)]
pub struct CastSigned<U = ()> {
    pub s: RL<CastSigned, U>,
}

impl<T, U> Deref for RL<T, U> {
    type Target = CastSigned<Self>;

    fn deref(&self) -> &Self::Target {
        &CastSigned {
            s: RL(PhantomData),
        }
    }
}

#[doc(hidden)]
pub struct CastUnsigned<U = ()> {
    pub u: RL<CastUnsigned, U>,
}

impl<U> Deref for CastSigned<U> {
    type Target = CastUnsigned<U>;

    fn deref(&self) -> &Self::Target {
        &CastUnsigned {
            u: RL(PhantomData),
        }
    }
}

#[doc(hidden)]
pub struct Truncate8<U = ()> {
    pub t8: RL<Truncate8, U>,
}

#[doc(hidden)]
pub struct Truncate16<U = ()> {
    pub t16: RL<Truncate16, U>,
}

#[doc(hidden)]
pub struct Truncate32<U = ()> {
    pub t32: RL<Truncate32, U>,
}

#[doc(hidden)]
pub struct Truncate64<U = ()> {
    pub t64: RL<Truncate64, U>,
}

#[doc(hidden)]
pub struct Extend64<U = ()> {
    pub e64: RL<Extend64, U>,
}

#[doc(hidden)]
pub struct Extend32<U = ()> {
    pub e32: RL<Extend32, U>,
}

#[doc(hidden)]
pub struct Extend16<U = ()> {
    pub e16: RL<Extend16, U>,
}

#[doc(hidden)]
pub struct Extend128<U = ()> {
    pub e128: RL<Extend128, U>,
}

impl<U> Deref for CastUnsigned<U> {
    type Target = Truncate8<U>;

    fn deref(&self) -> &Self::Target {
        &Truncate8 {
            t8: RL(PhantomData),
        }
    }
}

impl<U> Deref for Truncate8<U> {
    type Target = Extend64<U>;

    fn deref(&self) -> &Self::Target {
        &Extend64 {
            e64: RL(PhantomData),
        }
    }
}

impl<U> Deref for Extend64<U> {
    type Target = Truncate16<U>;

    fn deref(&self) -> &Self::Target {
        &Truncate16 {
            t16: RL(PhantomData),
        }
    }
}

impl<U> Deref for Truncate16<U> {
    type Target = Extend32<U>;

    fn deref(&self) -> &Self::Target {
        &Extend32 {
            e32: RL(PhantomData),
        }
    }
}

impl<U> Deref for Extend32<U> {
    type Target = Truncate32<U>;

    fn deref(&self) -> &Self::Target {
        &Truncate32 {
            t32: RL(PhantomData),
        }
    }
}

impl<U> Deref for Truncate32<U> {
    type Target = Extend16<U>;

    fn deref(&self) -> &Self::Target {
        &Extend16 {
            e16: RL(PhantomData),
        }
    }
}

impl<U> Deref for Extend16<U> {
    type Target = Truncate64<U>;

    fn deref(&self) -> &Self::Target {
        &Truncate64 {
            t64: RL(PhantomData),
        }
    }
}

impl<U> Deref for Truncate64<U> {
    type Target = Extend128<U>;

    fn deref(&self) -> &Self::Target {
        &Extend128 {
            e128: RL(PhantomData),
        }
    }
}

type MulT<This, U> = <This as Mul<U>>::Output;

/// VALIDITY: T must be inhabited. This is the case for all instantiated types in this library.
/// 
/// SAFETY: T's existence conforms to soundness invariants. This is the case for all instantiated types in this library.
const unsafe fn make_zst<T>() -> T {
    assert!(core::mem::size_of::<T>() == 0, "T is not zero sized type");
    core::mem::zeroed()
}

#[doc(hidden)]
macro_rules! impl_id {
    ($($T:ty)*) => {
        $(
            impl Mul<Bb> for $T {
                type Output = Self;

                fn mul(self, _: Bb) -> Self {
                    self
                }
            }
        )*
    };
}

impl_id!(i8 u8 i16 u16 i32 u32 i64 u64 i128 u128);

#[doc(hidden)]
pub trait MkSigned {
    type Signed;

    fn cast_signed(self) -> Self::Signed;
}

#[doc(hidden)]
pub trait MkUnsigned {
    type Unsigned;

    fn cast_unsigned(self) -> Self::Unsigned;
}

#[doc(hidden)]
macro_rules! impl_su {
    ($($S:ty > $U:ty),*) => {
        $(
            impl MkSigned for $S {
                type Signed = $S;

                fn cast_signed(self) -> $S {
                    self
                }
            }

            impl MkSigned for $U {
                type Signed = $S;

                fn cast_signed(self) -> $S {
                    self as $S
                }
            }

            impl MkUnsigned for $S {
                type Unsigned = $U;

                fn cast_unsigned(self) -> $U {
                    self as $U
                }
            }

            impl MkUnsigned for $U {
                type Unsigned = $U;

                fn cast_unsigned(self) -> $U {
                    self
                }
            }

            impl<U> Mul<RL<CastSigned, U>> for $S where $S: Mul<U>, MulT<$S, U>: MkSigned {
                type Output = <MulT<$S, U> as MkSigned>::Signed;

                fn mul(self, _: RL<CastSigned, U>) -> Self::Output {
                    (self * unsafe { make_zst::<U>() }).cast_signed()
                }
            }

            impl<U> Mul<RL<CastSigned, U>> for $U where $U: Mul<U>, MulT<$U, U>: MkSigned {
                type Output = <MulT<$U, U> as MkSigned>::Signed;

                fn mul(self, _: RL<CastSigned, U>) -> Self::Output {
                    (self * unsafe { make_zst::<U>() }).cast_signed()
                }
            }

            impl<U> Mul<RL<CastUnsigned, U>> for $S where $S: Mul<U>, MulT<$S, U>: MkUnsigned {
                type Output = <MulT<$S, U> as MkUnsigned>::Unsigned;

                fn mul(self, _: RL<CastUnsigned, U>) -> Self::Output {
                    (self * unsafe { make_zst::<U>() }).cast_unsigned()
                }
            }

            impl<U> Mul<RL<CastUnsigned, U>> for $U where $U: Mul<U>, MulT<$U, U>: MkUnsigned {
                type Output = <MulT<$U, U> as MkUnsigned>::Unsigned;

                fn mul(self, _: RL<CastUnsigned, U>) -> Self::Output {
                    (self * unsafe { make_zst::<U>() }).cast_unsigned()
                }
            }
        )*
    };
}

impl_su!(i8 > u8, i16 > u16, i32 > u32, i64 > u64, i128 > u128);

#[doc(hidden)]
pub trait AppliedTo<Src> {
    type Output;

    fn apply(src: Src) -> Self::Output;
}

#[doc(hidden)]
macro_rules! applied_to {
    ($Op:ty => $U:ty: $($T:ty)*) => {
        $(
            impl AppliedTo<$T> for $Op {
                type Output = $U;

                fn apply(src: $T) -> $U {
                    src as $U
                }
            }

            impl<U> Mul<RL<$Op, U>> for $T where $T: Mul<U>, $Op: AppliedTo<MulT<$T, U>> {
                type Output = <$Op as AppliedTo<MulT<$T, U>>>::Output;

                fn mul(self, _: RL<$Op, U>) -> Self::Output {
                    <$Op>::apply(self * unsafe { make_zst::<U>() })
                }
            }
        )*
    };
}

applied_to!(Truncate8 => i8: i16 i32 i64 i128);
applied_to!(Truncate8 => u8: u16 u32 u64 u128);
applied_to!(Truncate16 => i16: i32 i64 i128);
applied_to!(Truncate16 => u16: u32 u64 u128);
applied_to!(Truncate32 => i32: i64 i128);
applied_to!(Truncate32 => u32: u64 u128);
applied_to!(Truncate64 => i64: i128);
applied_to!(Truncate64 => u64: u128);

// signed extension
applied_to!(Extend128 => i128: i64 i32 i16 i8);
applied_to!(Extend64 => i64: i32 i16 i8);
applied_to!(Extend32 => i32: i16 i8);
applied_to!(Extend16 => i16: i8);

// zero extension
applied_to!(Extend128 => u128: u64 u32 u16 u8);
applied_to!(Extend64 => u64: u32 u16 u8);
applied_to!(Extend32 => u32: u16 u8);
applied_to!(Extend16 => u16: u8);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn minimal_coverage() {
        assert_eq!(5u8 * Bb, 5);
        assert_eq!(255u8 * Bb.s.u.s, -1);
        assert_eq!(u64::MAX * Bb.t8, 255);
        assert_eq!(0xfeu8 * Bb.s.e16.u, 0xfffe);
    }
}
