use std::ops::{Add, Div, Mul, Sub};

pub enum ImageError {
    ShapeMismatchError
}

pub struct Image<T> {
    data: Vec<T>,
    shape: Vec<usize>,
    spacing: Vec<f64>,
    origin: Vec<f64>,
    orientation: Vec<f64>,
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
