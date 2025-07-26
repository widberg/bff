use std::marker::PhantomData;
use std::ops::{Div, Mul, Range, RangeInclusive, Sub};

use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite, binrw};
use derive_more::{Deref, DerefMut};
use num_traits::{CheckedAdd, Float, NumCast, PrimInt, Signed, Unsigned, cast};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::names::Name;

pub type Vec<const SIZE: usize, InnerType = f32> = [InnerType; SIZE];
pub type Vec2<T = f32> = Vec<2, T>;
pub type Vec2f = Vec2;
pub type Vec2i16 = Vec2<i16>;
pub type Vec3<T = f32> = Vec<3, T>;
pub type Vec3f = Vec3;
pub type Vec3i16 = Vec3<i16>;
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
pub type Mat3x4f = Mat<3, 4>;

// A fixed precision float with a variable numerator and constant denominator.
#[derive(
    BinRead, BinWrite, Deref, DerefMut, Debug, Serialize, Deserialize, ReferencedNames, JsonSchema,
)]
#[serde(transparent)]
pub struct NumeratorFloat<
    T: NumCast + BinRead + BinWrite,
    const DENOMINATOR: usize,
    F: NumCast + Div<Output = F> + Mul<Output = F> + Copy + Float = f32,
>(
    #[deref_mut]
    #[deref]
    #[br(map = |x: T| cast::<T, F>(x).unwrap() / cast::<usize, F>(DENOMINATOR).unwrap())]
    #[bw(map = |x: &F| cast::<F, T>((*x * cast::<usize, F>(DENOMINATOR).unwrap()).round()).unwrap())]
    F,
    #[serde(skip)] PhantomData<T>,
)
where
    for<'a> <T as BinRead>::Args<'a>: Default,
    for<'a> T: BinWrite<Args<'a> = ()>;

// A fixed precision normal float between -1 and 1. (x / x.max_value()) * 2 + -1.
#[derive(
    BinRead, BinWrite, Deref, DerefMut, Debug, Serialize, Deserialize, ReferencedNames, JsonSchema,
)]
#[serde(transparent)]
pub struct SignedNormalFloat<
    T: NumCast + Div<F, Output = F> + Unsigned + PrimInt + BinRead + BinWrite,
    F: NumCast + Signed + Copy + Float = f32,
>(
    #[deref_mut]
    #[deref]
    #[br(map = |x: T| (x / cast::<T, F>(T::max_value()).unwrap()).mul_add(cast::<f32, F>(2.).unwrap(), cast::<f32, F>(-1.).unwrap()))]
    #[bw(map = |x: &F| cast::<F, T>(((*x + cast::<f32, F>(1.).unwrap()) / cast::<f32, F>(2.).unwrap() * cast::<T, F>(T::max_value()).unwrap()).round()).unwrap())]
    F,
    #[serde(skip)] PhantomData<T>,
)
where
    for<'a> <T as BinRead>::Args<'a>: Default,
    for<'a> T: BinWrite<Args<'a> = ()>;

// Range whose first element is first and last element is last. [first, last].
// We intentionally use the names first and last instead of begin and end to avoid confusion with
// C++ iterators.
#[binrw]
#[derive(Debug, Serialize, Deref, DerefMut, Deserialize, ReferencedNames, JsonSchema)]
#[serde(rename = "range_inclusive")]
pub struct RangeFirstLast<T = u16>
where
    T: Copy,
    for<'a> <T as BinRead>::Args<'a>: Default,
    for<'a> T: BinRead + BinWrite<Args<'a> = ()>,
{
    #[br(temp)]
    #[bw(calc = *inner.start())]
    start: T,
    #[br(temp)]
    #[bw(calc = *inner.end())]
    end: T,
    #[br(calc = start..=end)]
    #[bw(ignore)]
    inner: RangeInclusive<T>,
}

// Range whose first element is first and contains size elements. [first, first + size).
#[binrw]
#[derive(Debug, Serialize, Deref, DerefMut, Deserialize, ReferencedNames, JsonSchema)]
#[serde(rename = "range")]
pub struct RangeBeginSize<T = u16>
where
    T: Copy + CheckedAdd<Output = T> + Sub<Output = T>,
    for<'a> <T as BinRead>::Args<'a>: Default,
    for<'a> T: BinRead + BinWrite<Args<'a> = ()>,
{
    #[br(temp)]
    #[bw(calc = inner.start)]
    first: T,
    #[br(temp)]
    #[bw(calc = inner.end - inner.start)]
    size: T,
    #[br(try_calc = first.checked_add(&size).map(|last| first..last).ok_or("Addition overflow in RangeBeginSize"))]
    #[bw(ignore)]
    inner: Range<T>,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
pub struct Sphere {
    pub center: Vec3f,
    pub radius: f32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
pub struct BffBox {
    pub matrix: Mat3x4f,
    pub vec: Vec3f,
    pub scale: f32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
pub struct Segment {
    pub origin: Vec3f,
    pub length: f32,
    pub direction: Vec3f,
    pub pad: f32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
pub struct Cylindre {
    pub seg: Segment,
    pub radius: f32,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
pub struct DynSphere {
    pub sphere: Sphere,
    pub flags: u32,
    pub name: Name,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
pub struct DynBox {
    pub matrix: Mat4f,
    pub flags: u32,
    pub name: Name,
}

#[derive(BinRead, BinWrite, Debug, Serialize, Deserialize, ReferencedNames, JsonSchema)]
pub struct Rect<T: BinRead + BinWrite + 'static = i32>
where
    for<'a> <T as BinRead>::Args<'a>: Default + Clone,
    for<'a> T: BinWrite<Args<'a> = ()>,
{
    pub top_left: Vec2<T>,
    pub bottom_right: Vec2<T>,
}
