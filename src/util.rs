use bevy::{
    prelude::{Component, Vec3},
    reflect::Reflect,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}
impl CardinalDirection {
    pub fn reverse(&self) -> Self {
        match self {
            CardinalDirection::North => CardinalDirection::South,
            CardinalDirection::East => CardinalDirection::West,
            CardinalDirection::South => CardinalDirection::North,
            CardinalDirection::West => CardinalDirection::East,
        }
    }
}

impl From<CardinalDirection> for Vec3 {
    fn from(value: CardinalDirection) -> Self {
        match value {
            CardinalDirection::North => Vec3::NEG_Z,
            CardinalDirection::East => Vec3::X,
            CardinalDirection::South => Vec3::Z,
            CardinalDirection::West => Vec3::NEG_X,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Alignment {
    Xaxis,
    Yaxis,
}

impl From<CardinalDirection> for Alignment {
    fn from(value: CardinalDirection) -> Self {
        match value {
            CardinalDirection::North => Alignment::Yaxis,
            CardinalDirection::East => Alignment::Xaxis,
            CardinalDirection::South => Alignment::Yaxis,
            CardinalDirection::West => Alignment::Xaxis,
        }
    }
}

pub fn cubic_ease_in_out(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    let t = t * 2.0;

    if t < 1.0 {
        return 0.5 * t * t * t;
    }

    let t = t - 2.0;
    0.5 * (t * t * t + 2.0)
}

#[derive(Component)]
pub struct Spin(pub f32);
