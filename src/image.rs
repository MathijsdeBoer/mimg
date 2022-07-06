use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use ndarray::prelude::*;
use ndarray_rand::rand_distr::num_traits::Zero;

pub enum ImageType {
    Ubyte(Image<u8>),
    Sbyte(Image<i8>),
    Ushort(Image<u16>),
    Sshort(Image<i16>),
    Uint(Image<u32>),
    Sint(Image<i32>),
    Ulong(Image<u64>),
    Slong(Image<i64>),
    Float(Image<f32>),
    Double(Image<f64>)
}

pub enum ImageDim {
    Nodim = 0,
    Onedim = 1,
    Twodim = 2,
    Threedim = 3,
    Fourdim = 4,
    Fivedim = 5
}

pub struct Image<T> {
    data: ArrayD<T>,
    shape: Vec<usize>,
    spacing: Vec<f64>,
    origin: Vec<f64>,
    orientation: Vec<f64>,
}

impl<T> Image<T>
    where T: Zero + Copy{
    fn new(
        data: ArrayD<T>,
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
            data: ArrayD::zeros(ndarray::IxDynImpl::default()),
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
            let new_data: ArrayD<T> = self.data + rhs.data;

            Image {
                data: new_data,
                ..self
            }
        }
    }
}

impl<T> AddAssign<Image<T>> for Image<T>
    where T: AddAssign + Copy
{
    fn add_assign(&mut self, rhs: Image<T>) {
        if self.shape != rhs.shape {
            panic!("Image shapes do not match")
        } else {
            self.data += &rhs.data;
        }
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
            let new_data: ArrayD<T> = self.data - rhs.data;

            Image {
                data: new_data,
                ..self
            }
        }
    }
}

impl<T> SubAssign<Image<T>> for Image<T>
    where T: SubAssign + Copy
{
    fn sub_assign(&mut self, rhs: Image<T>) {
        if self.shape != rhs.shape {
            panic!("Image shapes do not match")
        } else {
            self.data -= &rhs.data;
        }
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
            let new_data: ArrayD<T> = self.data / rhs.data;

            Image {
                data: new_data,
                ..self
            }
        }
    }
}

impl<T> DivAssign<Image<T>> for Image<T>
    where T: DivAssign + Copy
{
    fn div_assign(&mut self, rhs: Image<T>) {
        if self.shape != rhs.shape {
            panic!("Image shapes do not match")
        } else {
            self.data /= &rhs.data;
        }
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
            let new_data: ArrayD<T> = self.data * rhs.data;

            Image {
                data: new_data,
                ..self
            }
        }
    }
}

impl<T> MulAssign<Image<T>> for Image<T>
    where T:MulAssign + Copy
{
    fn mul_assign(&mut self, rhs: Image<T>) {
        if self.shape != rhs.shape {
            panic!("Image shapes do not match")
        } else {
            self.data *= &rhs.data;
        }
    }
}
