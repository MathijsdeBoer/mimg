use crate::image::ImageType;

pub enum FilterError {
    ShapeMismatchError,
    FilterError
}

pub trait Filter {
    fn execute();
    fn get_output() -> Result<ImageType, FilterError>;
}