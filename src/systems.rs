use std::borrow::BorrowMut;
use crate::data::*;
use crate::generate::GenManager;
use crate::store::EcsStore; // trait
use termion::raw::RawTerminal;
use termion::{color, cursor};

pub fn move_sys<V: EcsStore<ObjectVelocity>, P: EcsStore<ObjectPosition>>(vel: &V, pos: &mut P) {
    pos.for_each_mut(|g, p| {
        if let Some(obj) = vel.get(g) {
            p.point.0 = obj.x_vel;
            p.point.1 = obj.y_vel;
        }
    });
}