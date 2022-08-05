use euclid::*;


pub struct Position{};

pub type Point = Point2D<Point, Position>;

/// A 2D vector for working in [`DrawingSpace`].
pub type Vector = Vector2D<Point, Position>;

/// A length in [`DrawingSpace`].
pub type Length = Length;

pub type Line = Point2D<Point, Position>;