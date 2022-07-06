mod image;
mod phantoms;
mod filters;
mod io;
mod metrics;
mod draw;

pub use image::Image;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
