use core::ops::{Bound, RangeBounds, RangeInclusive};

pub trait Plus<Rhs> {
    #[must_use]
    fn plus(self, rhs: Rhs) -> Self;
}

impl Plus<i16> for u16 {
    fn plus(self, rhs: i16) -> Self {
        if rhs < 0 {
            self - rhs.unsigned_abs()
        } else {
            self + rhs.unsigned_abs()
        }
    }
}
impl Plus<i8> for u16 {
    fn plus(self, rhs: i8) -> Self {
        if rhs < 0 {
            self - u16::from(rhs.unsigned_abs())
        } else {
            self + u16::from(rhs.unsigned_abs())
        }
    }
}
impl Plus<u16> for u16 {
    fn plus(self, rhs: u16) -> Self {
        self + rhs
    }
}
impl Plus<u8> for u16 {
    fn plus(self, rhs: u8) -> Self {
        self + u16::from(rhs)
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct OutOfBoundsError;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Bounds {
    pub x_start: u16,
    pub x_end: u16,
    pub y_start: u16,
    pub y_end: u16,
}

impl Bounds {
    pub fn new_within<X, Y>(x: X, y: Y, parent: &Bounds) -> Result<Self, OutOfBoundsError>
    where
        X: RangeBounds<u16>,
        Y: RangeBounds<u16>,
    {
        let x_start = match x.start_bound() {
            Bound::Included(&v) => v,
            Bound::Excluded(&v) => v + 1,
            Bound::Unbounded => 0,
        };
        if x_start < parent.x_start {
            return Err(OutOfBoundsError);
        }
        let x_end = match x.end_bound() {
            Bound::Included(&v) => v,
            Bound::Excluded(&v) => v - 1,
            Bound::Unbounded => parent.x_end,
        };
        if x_end > parent.x_end {
            return Err(OutOfBoundsError);
        }
        let y_start = match y.start_bound() {
            Bound::Included(&v) => v,
            Bound::Excluded(&v) => v + 1,
            Bound::Unbounded => 0,
        };
        if y_start < parent.y_start {
            return Err(OutOfBoundsError);
        }
        let y_end = match y.end_bound() {
            Bound::Included(&v) => v,
            Bound::Excluded(&v) => v - 1,
            Bound::Unbounded => parent.y_end - 1,
        };
        if y_end > parent.y_end {
            return Err(OutOfBoundsError);
        }
        Ok(Self {
            x_start,
            x_end,
            y_start,
            y_end,
        })
    }
    pub fn width(&self) -> u16 {
        self.x_end - self.x_start + 1
    }
    pub fn height(&self) -> u16 {
        self.y_end - self.y_start + 1
    }
    pub fn range_horiz(&self) -> RangeInclusive<u16> {
        self.x_start..=self.x_end
    }
    pub fn range_vert(&self) -> RangeInclusive<u16> {
        self.y_start..=self.y_end
    }
    pub fn area(&self) -> u32 {
        u32::from(self.width()) * u32::from(self.height())
    }
    pub fn move_by<Px, Py>(&mut self, x: Px, y: Py) -> &mut Self
    where
        u16: Plus<Px>,
        u16: Plus<Py>,
        Px: Copy,
        Py: Copy,
    {
        self.x_start = self.x_start.plus(x);
        self.x_end = self.x_end.plus(x);
        self.y_start = self.y_start.plus(y);
        self.y_end = self.y_end.plus(y);
        self
    }
    #[must_use]
    pub fn intersect(&self, other: &Self) -> Option<Self> {
        use core::cmp::{max, min};
        if other.x_start > self.x_end || other.x_end < self.x_start || other.y_start > self.y_end || other.y_end < self.y_start {
            return None;
        }

        Some(Self {
            x_start: max(self.x_start, other.x_start),
            x_end: min(self.x_end, other.x_end),
            y_start: max(self.y_start, other.y_start),
            y_end: min(self.y_end, other.y_end),
        })
    }
    pub fn append<Px, Py>(&mut self, x: Px, y: Py) -> &mut Self
    where
        u16: Plus<Px>,
        u16: Plus<Py>,
        Px: Copy,
        Py: Copy,
    {
        self.x_end = self.x_end.plus(x);
        self.y_end = self.y_end.plus(y);
        self
    }
    pub fn set_height(&mut self, y: u16) -> &mut Self {
        assert!(y >= 1); // minimum height is 1 when y_start == y_end
        self.y_end = self.y_start + y - 1;
        self
    }
    pub fn set_width(&mut self, x: u16) -> &mut Self {
        assert!(x >= 1); // minimum width is 1 when x_start == x_end
        self.x_end = self.x_start + x - 1;
        self
    }

    pub fn is_within(&self, parent: Self) -> bool {
        self.x_start >= parent.x_start && self.x_end <= parent.x_end && self.y_start >= parent.y_start && self.y_end <= parent.y_end
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn b(x_start: u16, x_end: u16, y_start: u16, y_end: u16) -> Bounds {
        Bounds {
            x_start,
            x_end,
            y_start,
            y_end,
        }
    }

    #[test]
    fn set_height() {
        assert_eq!(b(0u16, 0u16, 0u16, 0u16).set_height(2u16), &b(0, 0, 0, 1));
    }
    #[test]
    fn set_width() {
        assert_eq!(b(0u16, 0u16, 0u16, 0u16).set_width(2u16), &b(0, 1, 0, 0));
    }
    #[test]
    fn append() {
        assert_eq!(b(0u16, 0u16, 0u16, 0u16).append(5u16, 5u16), &b(0, 5, 0, 5));
        assert_eq!(b(0u16, 1u16, 0u16, 1u16).append(-1i16, -1i16), &b(0, 0, 0, 0));
    }
    #[test]
    fn width() {
        assert_eq!(b(0u16, 1u16, 0u16, 1u16).width(), 2);
    }
    #[test]
    fn height() {
        assert_eq!(b(0u16, 1u16, 0u16, 1u16).height(), 2);
    }
    #[test]
    fn intersect() {
        assert_eq!(b(0u16, 0u16, 0u16, 0u16).intersect(&b(1, 1, 1, 1)), None);
        assert_eq!(b(0u16, 2u16, 0u16, 2u16).intersect(&b(1, 10, 1, 10)), Some(b(1, 2, 1, 2)));
    }
}
