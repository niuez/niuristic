pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Pos { x, y }
    }
}

impl From<(usize, usize)> for Pos {
    fn from(value: (usize, usize)) -> Self {
        Pos::new(value.0, value.1)
    }
}

pub struct Field2d {
    h: usize,
    w: usize,
}

impl Field2d {
    pub fn new(h: usize, w: usize) -> Self {
        Field2d { h, w }
    }

    pub fn size(&self) -> usize {
        self.h * self.w
    }

    pub fn idx<T: Into<Pos>>(&self, t: T) -> usize {
        let p = t.into();
        p.x * self.w + p.y
    }

    pub fn step<P: Into<Pos>, X: Into<i64>, Y: Into<i64>>(&self, p: P, x: X, y: Y) -> Option<Pos> {
        let p = p.into();
        let x = p.x as i64 + x.into();
        let y = p.y as i64 + y.into();
        if 0 <= x && x < self.h as i64 && 0 <= y && y < self.w as i64 {
            Some(Pos::new(x as usize, y as usize))
        }
        else {
            None
        }
    }
}
