use specs::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Point(pub i32, pub i32);


/// Something which can be drawn on the screen.
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectPosition {
    pub point: Point,
    /// The [`Layer`] this [`DrawingObject`] is attached to.
    pub layer: Entity,
}

impl Component for ObjectPosition {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

pub struct ObjectVelocity {
    pub x_vel: i32,
    pub y_vel: i32,
}




