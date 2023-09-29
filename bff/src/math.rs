use std::io::{Read, Seek, Write};
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Range, RangeInclusive, Sub};

use binrw::{BinRead, BinResult, BinWrite, Endian};
use derive_more::{Deref, DerefMut};
use num_traits::{cast, MulAdd, NumCast, PrimInt, Signed, Unsigned};
use serde::{Deserialize, Serialize};

use crate::name::Name;

pub type Vec<const SIZE: usize, InnerType = f32> = [InnerType; SIZE];
pub type Vec2<T = f32> = Vec<2, T>;
pub type Vec2f = Vec2;
pub type Vec2i16 = Vec2<i16>;
pub type Vec3<T = f32> = Vec<3, T>;
pub type Vec3f = Vec3;
pub type RGB = Vec3;
pub type Vec4<T = f32> = Vec<4, T>;
pub type Vec4f = Vec4;
pub type Vec4i16 = Vec4<i16>;
pub type Quat = Vec4;
pub type RGBA = Vec4;

pub type Mat<const ROWS: usize, const COLUMNS: usize = ROWS, InnerType = f32> =
    [[InnerType; COLUMNS]; ROWS];
pub type Mat3f = Mat<3>;
pub type Mat4f = Mat<4>;

// A fixed precision float with a variable numerator and constant denominator.
#[derive(BinRead, Deref, DerefMut, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NumeratorFloat<
    T: NumCast + BinRead + BinWrite,
    const DENOMINATOR: usize,
    F: NumCast + Div<Output = F> + Mul<Output = F> = f32,
>(
    #[deref_mut]
    #[deref]
    #[br(map = |x: T| cast::<T, F>(x).unwrap() / cast::<usize, F>(DENOMINATOR).unwrap())]
    F,
    #[serde(skip)] PhantomData<T>,
)
where
    for<'a> <T as BinRead>::Args<'a>: Default,
    for<'a> <T as BinWrite>::Args<'a>: Clone + Default;

impl<
        T: NumCast + BinRead + BinWrite,
        const DENOMINATOR: usize,
        F: NumCast + Div<Output = F> + Mul<Output = F> + Clone,
    > BinWrite for NumeratorFloat<T, DENOMINATOR, F>
where
    for<'a> <T as BinRead>::Args<'a>: Default,
    for<'a> <T as BinWrite>::Args<'a>: Clone + Default,
{
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        cast::<F, T>(self.0.clone() * cast::<usize, F>(DENOMINATOR).unwrap())
            .unwrap()
            .write_options(writer, endian, <_>::default())
    }
}

// A fixed precision normal float between -1 and 1. (x / x.max_value()) * 2 + -1.
#[derive(BinRead, Deref, DerefMut, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SignedNormalFloat<
    T: NumCast + Div<F, Output = F> + Unsigned + PrimInt + BinRead + BinWrite,
    F: NumCast + MulAdd<f32, f32, Output = F> + Signed = f32,
>(
    #[deref_mut]
    #[deref]
    #[br(map = |x: T| (x / cast::<T, F>(T::max_value()).unwrap()).mul_add(2., -1.))]
    F,
    #[serde(skip)] PhantomData<T>,
)
where
    for<'a> <T as BinRead>::Args<'a>: Default,
    for<'a> <T as BinWrite>::Args<'a>: Clone + Default;

impl<
        T: NumCast + Div<F, Output = F> + Unsigned + PrimInt + BinRead + BinWrite,
        F: NumCast + MulAdd<f32, f32, Output = F> + Signed + Clone,
    > BinWrite for SignedNormalFloat<T, F>
where
    for<'a> <T as BinRead>::Args<'a>: Default,
    for<'a> <T as BinWrite>::Args<'a>: Clone + Default,
{
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        cast::<F, T>(
            (self.0.clone() + cast::<f32, F>(1.).unwrap()) / cast::<f32, F>(2.).unwrap()
                * cast::<T, F>(T::max_value()).unwrap(),
        )
        .unwrap()
        .write_options(writer, endian, <_>::default())
    }
}

// Range whose first element is first and last element is last. [first, last].
// We intentionally use the names first and last instead of begin and end to avoid confusion with
// C++ iterators.
#[derive(Debug, Serialize, Deref, DerefMut, Deserialize)]
#[serde(rename = "range_inclusive")]
pub struct RangeFirstLast<T = u16>(RangeInclusive<T>);

impl<T: BinRead + Add<Output = T> + Copy> BinRead for RangeFirstLast<T>
where
    for<'a> <T as BinRead>::Args<'a>: Default,
{
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let first = T::read_options(reader, endian, <_>::default())?;
        let last = T::read_options(reader, endian, <_>::default())?;
        Ok(RangeFirstLast(first..=last))
    }
}

impl<T: BinWrite> BinWrite for RangeFirstLast<T>
where
    for<'a> <T as BinWrite>::Args<'a>: Default,
{
    type Args<'a> = ();

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        T::write_options(self.start(), writer, endian, <_>::default())?;
        T::write_options(self.end(), writer, endian, <_>::default())?;
        Ok(())
    }
}

// Range whose first element is first and contains size elements. [first, first + size).
#[derive(Debug, Serialize, Deref, DerefMut, Deserialize)]
#[serde(rename = "range")]
pub struct RangeBeginSize<T = u16>(Range<T>);

impl<T: BinRead + Add<Output = T> + Copy> BinRead for RangeBeginSize<T>
where
    for<'a> <T as BinRead>::Args<'a>: Default,
{
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let first = T::read_options(reader, endian, <_>::default())?;
        let size = T::read_options(reader, endian, <_>::default())?;
        Ok(RangeBeginSize(first..first + size))
    }
}

impl<T: BinWrite + Sub<Output = T> + Copy> BinWrite for RangeBeginSize<T>
where
    for<'a> <T as BinWrite>::Args<'a>: Default,
{
    type Args<'a> = ();

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        T::write_options(&self.start, writer, endian, <_>::default())?;
        T::write_options(&(self.end - self.start), writer, endian, <_>::default())?;
        Ok(())
    }
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
pub struct Sphere {
    pub center: Vec3f,
    pub radius: f32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
pub struct DynSphere {
    pub sphere: Sphere,
    pub flags: u32,
    pub name: Name,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize)]
pub struct DynBox {
    pub matrix: Mat4f,
    pub flags: u32,
    pub name: Name,
}

#[derive(BinRead, Debug, Serialize, Deserialize)]
pub struct Rect<T: BinRead + BinWrite + 'static = i32>
where
    for<'a> <T as BinRead>::Args<'a>: Default + Clone,
    for<'a> <T as BinWrite>::Args<'a>: Default + Clone,
{
    pub top_left: Vec2<T>,
    pub bottom_right: Vec2<T>,
}

impl<T: BinRead + BinWrite + 'static> BinWrite for Rect<T>
where
    for<'a> <T as BinRead>::Args<'a>: Default + Clone,
    for<'a> <T as BinWrite>::Args<'a>: Default + Clone,
{
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        self.top_left
            .write_options(writer, endian, <_>::default())?;
        self.bottom_right
            .write_options(writer, endian, <_>::default())?;
        Ok(())
    }
}
