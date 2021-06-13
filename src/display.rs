use core::ops::RangeBounds;

pub trait Display {
    const WIDTH: u16;
    const HEIGHT: u16;
    type Color;
    type Error;

    fn fill_area<X, Y>(&mut self, x: X, y: Y, it: &mut dyn Iterator<Item = Self::Color>) -> Result<(), Self::Error>
    where
        X: RangeBounds<u16>,
        Y: RangeBounds<u16>;
}

pub trait ReadArea: Display {
    type Iter<'r>: Iterator<Item = Result<Self::Color, Self::Error>>;
    fn read_area<'a, X, Y>(&'a mut self, x: X, y: Y) -> Result<Self::Iter<'a>, Self::Error>
    where
        X: RangeBounds<u16>,
        Y: RangeBounds<u16>;
}

pub trait CopyArea: Display {
    fn copy_area<X, Y>(&mut self, x: X, y: Y, horiz_by: i16, vert_by: i16, buffer: &mut [Self::Color]) -> Result<(), Self::Error>
    where
        X: RangeBounds<u16>,
        Y: RangeBounds<u16>;
}
