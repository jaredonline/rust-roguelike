pub enum XPointRelation {
    LeftOfPoint,
    RightOfPoint,
    OnPointX
}

pub enum YPointRelation {
    AbovePoint,
    BelowPoint,
    OnPointY
}

pub enum PointEquality {
    PointsEqual,
    PointsNotEqual
}

#[deriving(Clone, Show, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    pub fn offset_x(&self, offset: i32) -> Point {
        Point { x: self.x + offset, y: self.y }
    }

    pub fn offset_y(&self, offset: i32) -> Point {
        Point { x: self.x, y: self.y + offset }
    }

    pub fn offset(&self, offset: Point) -> Point {
        Point { x: self.x + offset.x, y: self.y + offset.y }
    }

    pub fn compare_x(&self, point: Point) -> XPointRelation {
        if self.x > point.x {
            XPointRelation::RightOfPoint
        } else if self.x < point.x {
            XPointRelation::LeftOfPoint
        } else {
            XPointRelation::OnPointX
        }
    }

    pub fn compare_y(&self, point: Point) -> YPointRelation {
        if self.y > point.y {
            YPointRelation::BelowPoint
        } else if self.y < point.y {
            YPointRelation::AbovePoint
        } else {
            YPointRelation::OnPointY
        }
    }

    pub fn compare(&self, point: Point) -> PointEquality {
        if self.x == point.x && self.y == point.y {
            PointEquality::PointsEqual
        } else {
            PointEquality::PointsNotEqual
        }
    }
}

pub enum Contains {
    DoesContain,
    DoesNotContain
}

#[deriving(Clone, Show)]
pub struct Bound {
    pub min: Point,
    pub max: Point
}

impl Bound {
    pub fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Bound {
        Bound {
            min: Point { x: min_x, y: min_y },
            max: Point { x: max_x, y: max_y }
        }
    }

    pub fn contains(&self, point: Point) -> Contains {
        if 
            point.x >= self.min.x &&
            point.x <  self.max.x &&
            point.y >= self.min.y &&
            point.y <  self.max.y
        {
            Contains::DoesContain
        } else {
            Contains::DoesNotContain
        }
    }
}
