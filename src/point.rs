use std::{cmp::Ordering, fmt::Debug};

use crate::graham_hull::{Convex, Turn};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }
}
impl Convex<f32> for Point {
    fn get_angle(a: &Self, b: &Self) -> f32 {
        let dx = b.x - a.x;
        let dy = b.y - a.y;

        dy.atan2(dx)
    }
    fn get_turn(a: &Self, b: &Self, c: &Self) -> Turn {
        let crossprod = (b.x - a.x) * (c.y - b.y) - (b.y - a.y) * (c.x - b.x);
        if crossprod < 0.0 {
            return Turn::Clockwise;
        }
        Turn::CounterClockwise
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let order = self.y.partial_cmp(&other.y).unwrap();

        if order != Ordering::Equal {
            return Some(order);
        } else {
            return Some(self.x.partial_cmp(&other.x).unwrap());
        }
    }
}

impl Eq for Point {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.partial_cmp(other).unwrap();
    }
}
