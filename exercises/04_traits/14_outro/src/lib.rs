// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::convert::From;
use std::ops::Add;
use std::cmp::PartialEq;


#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct SaturatingU16 {
    value: u16
}

impl SaturatingU16 {
    pub fn new(a: u16) -> Self {
        Self {
            value: a
        }
    }
}

impl Add<Self> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: Self) -> Self::Output {
        let a = self.value + rhs.value;

        return SaturatingU16::new(a)
    }
}

impl Add<&Self> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &Self) -> Self::Output {
        let a = self.value + rhs.value;

        return SaturatingU16::new(a)
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: u16) -> Self::Output {
        let a = self.value + rhs;

        return SaturatingU16::new(a)
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &u16) -> Self::Output {
        let a = self.value + *rhs;

        return SaturatingU16::new(a)
    }
}

impl PartialEq<Self> for SaturatingU16 {
    fn eq(&self, other: &Self) -> bool {
        return self.value == other.value;
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        return self.value == *other;
    }
}

impl From<u16> for SaturatingU16 {
    fn from(a: u16) -> SaturatingU16 {
        SaturatingU16::new(a)
    }
}

impl From<u8> for SaturatingU16 {
    fn from(a: u8) -> SaturatingU16 {
        SaturatingU16::new(a as u16)
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(a: &u16) -> SaturatingU16 {
        SaturatingU16::new(*a)
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(a: &u8) -> SaturatingU16 {
        SaturatingU16::new(*a as u16)
    }
}
