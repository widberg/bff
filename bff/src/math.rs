use std::io::{Read, Seek};
use std::marker::PhantomData;
use std::ops::{Add, Div, Range, RangeInclusive};

use binrw::{BinRead, Endian};
use derive_more::{Deref, DerefMut};
use num_traits::{cast, MulAdd, NumCast, PrimInt, Signed, Unsigned};
use serde::Serialize;

use crate::name::Name;

pub type Vec<const SIZE: usize, InnerType = f32> = [InnerType; SIZE];
pub type Vec2<T = f32> = Vec<2, T>;
pub type Vec2f = Vec2;
pub type Vec3<T = f32> = Vec<3, T>;
pub type Vec3f = Vec3;
pub type Vec4<T = f32> = Vec<4, T>;
pub type Vec4f = Vec4;
pub type Quat = Vec4;
pub type RGBA = Vec4;

pub type Mat<const ROWS: usize, const COLUMNS: usize = ROWS, InnerType = f32> =
    [[InnerType; COLUMNS]; ROWS];
pub type Mat3f = Mat<3>;
pub type Mat4f = Mat<4>;

// A fixed precision float with a variable numerator and constant denominator.
#[derive(BinRead, Deref, DerefMut, Debug, Serialize)]
#[serde(transparent)]
pub struct NumeratorFloat<
    T: NumCast + BinRead,
    const DENOMINATOR: usize,
    F: NumCast + Div<Output = F> = f32,
>(
    #[deref_mut]
    #[deref]
    #[br(map = |x: T| cast::<T, F>(x).unwrap() / cast::<usize, F>(DENOMINATOR).unwrap())]
    F,
    #[serde(skip)] PhantomData<T>,
)
where
    for<'a> <T as BinRead>::Args<'a>: Default;

// A fixed precision normal float between -1 and 1. (x / x.max_value()) * 2 + -1.
#[derive(BinRead, Deref, DerefMut, Debug, Serialize)]
#[serde(transparent)]
pub struct SignedNormalFloat<
    T: NumCast + Div<F, Output = F> + Unsigned + PrimInt + BinRead,
    F: NumCast + MulAdd<f32, f32, Output = F> + Signed = f32,
>(
    #[deref_mut]
    #[deref]
    #[br(map = |x: T| (x / cast::<T, F>(T::max_value()).unwrap()).mul_add(2., -1.))]
    F,
    #[serde(skip)] PhantomData<T>,
)
where
    for<'a> <T as BinRead>::Args<'a>: Default;

// Range whose first element is first and last element is last. [first, last].
// We intentionally use the names first and last instead of begin and end to avoid confusion with
// C++ iterators.
#[derive(Debug, Serialize, Deref, DerefMut)]
#[serde(rename = "range_inclusive")]
pub struct RangeFirstLast<T: BinRead = u16>(RangeInclusive<T>)
where
    for<'a> <T as BinRead>::Args<'a>: Default;

impl<T: BinRead + Add<Output = T> + Copy> BinRead for RangeFirstLast<T>
where
    for<'a> <T as BinRead>::Args<'a>: Default,
{
    type Args<'a> = (
        <T as binrw::BinRead>::Args<'a>,
        <T as binrw::BinRead>::Args<'a>,
    );

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let first = T::read_options(reader, endian, args.0)?;
        let last = T::read_options(reader, endian, args.1)?;
        Ok(RangeFirstLast(first..=last))
    }
}

// Range whose first element is first and contains size elements. [first, first + size).
#[derive(Debug, Serialize, Deref, DerefMut)]
#[serde(rename = "range")]
pub struct RangeBeginSize<T: BinRead + Add<Output = T> + Copy = u16>(Range<T>)
where
    for<'a> <T as BinRead>::Args<'a>: Default;

impl<T: BinRead + Add<Output = T> + Copy> BinRead for RangeBeginSize<T>
where
    for<'a> <T as BinRead>::Args<'a>: Default,
{
    type Args<'a> = (
        <T as binrw::BinRead>::Args<'a>,
        <T as binrw::BinRead>::Args<'a>,
    );

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let first = T::read_options(reader, endian, args.0)?;
        let size = T::read_options(reader, endian, args.1)?;
        Ok(RangeBeginSize(first..first + size))
    }
}

#[derive(BinRead, Debug, Serialize)]
pub struct Sphere {
    pub center: Vec3f,
    pub radius: f32,
}

#[derive(BinRead, Debug, Serialize)]
pub struct DynSphere {
    pub sphere: Sphere,
    pub flags: u32,
    pub name: Name,
}

#[derive(BinRead, Debug, Serialize)]
pub struct DynBox {
    pub matrix: Mat4f,
    pub flags: u32,
    pub name: Name,
}

#[derive(BinRead, Debug, Serialize)]
pub struct Rect<T: BinRead = i32>
where
    for<'a> <T as BinRead>::Args<'a>: Default + Clone,
{
    pub top_left: Vec2<T>,
    pub bottom_right: Vec2<T>,
}
