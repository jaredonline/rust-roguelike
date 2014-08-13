pub struct Point {
    pub x: int,
    pub y: int
}

impl Point {
    pub fn offset_x(&self, offset: int) -> Point {
        Point { x: self.x + offset, y: self.y }
    }

    pub fn offset_y(&self, offset: int) -> Point {
        Point { x: self.x, y: self.y + offset }
    }

    pub fn offset(&self, offset: Point) -> Point {
        Point { x: self.x + offset.x, y: self.y + offset.y }
    }
}

pub enum Contains {
    DoesContain,
    DoesNotContain
}

pub struct Bound {
    pub min: Point,
    pub max: Point
}

impl Bound {
    pub fn contains(&self, point: Point) -> Contains {
        if 
            point.x >= self.min.x &&
            point.x <= self.max.x &&
            point.y >= self.min.y &&
            point.y <= self.max.y
        {
            DoesContain
        } else {
            DoesNotContain
        }
    }
}
