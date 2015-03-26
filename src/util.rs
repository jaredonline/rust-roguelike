#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

pub enum XRelation {
    Left,
    Right,
    On
}

pub enum YRelation {
    Above,
    Below,
    On
}

pub enum PointEquality {
    Equal,
    NotEqual
}

impl Point {
    pub fn offset_x(&self, offset: i32) -> Point {
        Point { x: self.x + offset, y: self.y }
    }

    pub fn offset_y(&self, offset: i32) -> Point {
        Point { x: self.x, y: self.y + offset }
    }

    pub fn offset(&self, offset: &Point) -> Point {
        Point { x: self.x + offset.x, y: self.y + offset.y }
    }

    pub fn compare_x(&self, point: &Point) -> XRelation {
        if self.x < point.x {
            XRelation::Left
        } else if self.x > point.x {
            XRelation::Right
        } else {
            XRelation::On
        }
    }

    pub fn compare_y(&self, point: &Point) -> YRelation {
        if self.y < point.y {
            YRelation::Above
        } else if self.y > point.y {
            YRelation::Below
        } else {
            YRelation::On
        }
    }

    pub fn compare(&self, point: &Point) -> PointEquality {
        if self.x == point.x && self.y == point.y {
            PointEquality::Equal
        } else {
            PointEquality::NotEqual
        }
    }
}

pub enum Contains {
    DoesContain,
    DoesNotContain
}

#[derive(Clone)]
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

    pub fn contains(&self, point: &Point) -> Contains {
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

