use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub enum ImageType {
    UBYTE(Image<u8>),
    SBYTE(Image<i8>),
    USHORT(Image<u16>),
    SSHORT(Image<i16>),
    UINT(Image<u32>),
    SINT(Image<i32>),
    ULONG(Image<u64>),
    SLONG(Image<i64>),
    FLOAT(Image<f32>),
    DOUBLE(Image<f64>)
}

pub enum ImageError {
    ShapeMismatchError,
    ImageError
}

pub struct Image<T> {
    data: Vec<T>,
    shape: Vec<usize>,
    spacing: Vec<f64>,
    origin: Vec<f64>,
    orientation: Vec<f64>,
}

impl<T> Image<T> {
    fn new(
        data: Vec<T>,
        shape: Vec<usize>,
        spacing: Vec<f64>,
        origin: Vec<f64>,
        orientation: Vec<f64>
    ) -> Self {
        Image::<T>{
            data,
            shape,
            spacing,
            origin,
            orientation
        }
    }

    fn default() -> Self {
        Image::<T>{
            data: vec![],
            shape: vec![0, 0, 0],
            spacing: vec![1.0, 1.0, 1.0],
            origin: vec![0.0, 0.0, 0.0],
            orientation: vec![0.0, 0.0, 0.0]
        }
    }
}

impl<T> Add<Image<T>> for Image<T>
    where T: Add<Output=T> + Copy
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.shape != rhs.shape {
            panic!("Image shapes do not match")
        } else {
            let new_data: Vec<T> = self.data.iter()
                .zip(rhs.data.iter())
                .map(|(&a, &b)| {
                    a + b
                })
                .collect();

            Image {
                data: new_data,
                ..self
            }
        }
    }
}

impl<T> AddAssign<Image<T>> for Image<T>
    where T: Add<Output=T> + Copy
{
    fn add_assign(&mut self, rhs: Image<T>) {
        todo!()
    }
}

impl<T> Sub<Image<T>> for Image<T>
    where T: Sub<Output=T> + Copy
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.shape != rhs.shape {
            panic!("Image shapes do not match")
        } else {
            let new_data: Vec<T> = self.data.iter()
                .zip(rhs.data.iter())
                .map(|(&a, &b)| {
                    a - b
                })
                .collect();

            Image {
                data: new_data,
                ..self
            }
        }
    }
}

impl<T> SubAssign<Image<T>> for Image<T>
    where T: Sub<Output=T> + Copy
{
    fn sub_assign(&mut self, rhs: Image<T>) {
        todo!()
    }
}

impl<T> Div<Image<T>> for Image<T>
    where T: Div<Output=T> + Copy
{
    type Output = Self;

    fn div(self, rhs: Image<T>) -> Self::Output {
        if self.shape != rhs.shape {
            panic!("Image shapes do not match")
        } else {
            let new_data: Vec<T> = self.data.iter()
                .zip(rhs.data.iter())
                .map(|(&a, &b)| {
                    a / b
                })
                .collect();

            Image {
                data: new_data,
                ..self
            }
        }
    }
}

impl<T> DivAssign<Image<T>> for Image<T>
    where T: Div<Output=T> + Copy
{
    fn div_assign(&mut self, rhs: Image<T>) {
        todo!()
    }
}

impl<T> Mul<Image<T>> for Image<T>
    where T:Mul<Output=T> + Copy
{
    type Output = Self;

    fn mul(self, rhs: Image<T>) -> Self::Output {
        if self.shape != rhs.shape {
            panic!("Image shapes do not match")
        } else {
            let new_data: Vec<T> = self.data.iter()
                .zip(rhs.data.iter())
                .map(|(&a, &b)| {
                    a * b
                })
                .collect();

            Image {
                data: new_data,
                ..self
            }
        }
    }
}

impl<T> MulAssign<Image<T>> for Image<T>
    where T:Mul<Output=T> + Copy
{
    fn mul_assign(&mut self, rhs: Image<T>) {
        todo!()
    }
}
