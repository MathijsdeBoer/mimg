use crate::filters::{Filter, FilterError};
use crate::filters::kernel::KernelType;
use crate::image::ImageType;

struct ClosingFilter {
    kernel_type: KernelType,
    output: Result<ImageType, FilterError>
}

impl Filter for ClosingFilter {
    fn execute() {
        todo!()
    }

    fn get_output() -> Result<ImageType, FilterError> {
        todo!()
    }
}