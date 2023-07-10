pub type Vec<const SIZE: usize, InnerType = f32> = [InnerType; SIZE];
pub type Vec3f = Vec<3>;
pub type Vec4f = Vec<4>;
pub type Quat = Vec<4>;

pub type Mat<const ROWS: usize, const COLUMNS: usize = ROWS, InnerType = f32> = [[InnerType; COLUMNS]; ROWS];
pub type Mat3f = Mat<3>;
pub type Mat4f = Mat<4>;
