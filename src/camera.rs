use bevy::prelude::*;

use crate::{
    level_manager::LevelManager,
    player::{Player, PlayerState},
    util::CardinalDirection,
};

#[derive(Component)]
pub struct MainCamera {
    pub direction: CardinalDirection,
    pub angle_change: f32,
}
impl Default for MainCamera {
    fn default() -> Self {
        Self {
            direction: CardinalDirection::North,
            angle_change: 0.0,
        }
    }
}

pub fn camera_rotation(
    mut camera_query: Query<(&mut Transform, &mut MainCamera)>,
    player_query: Query<&Player>,
    level_manager: Res<LevelManager>,
    time: Res<Time>,
) {
    let angle_increment = 4.0;
    let arc = 1.2;
    if let Ok(player) = player_query.get_single() {
        if let Ok((mut camera_transform, mut camera)) = camera_query.get_single_mut() {
            let direction = if let PlayerState::Climbing(climb_state) = &player.state {
                climb_state.direction
            } else {
                CardinalDirection::North
            };
            if (camera.direction == CardinalDirection::North
                && direction == CardinalDirection::East)
                || (camera.direction == CardinalDirection::West
                    && direction == CardinalDirection::North)
            {
                // move camera clockwise
                camera.angle_change += arc;
                camera.direction = direction;
            } else if (camera.direction == CardinalDirection::North
                && direction == CardinalDirection::West)
                || (camera.direction == CardinalDirection::East
                    && direction == CardinalDirection::North)
            {
                // move camera anticlockwise
                camera.angle_change -= arc;
                camera.direction = direction;
            }
            if camera.angle_change < 0.0 {
                let angle = (angle_increment * time.delta_seconds())
                    .min(camera.angle_change * camera.angle_change);
                let midpoint = level_manager.get_current_level().map.midpoint();
                camera.angle_change += angle;
                camera_transform.rotate_around(
                    Vec3::new(midpoint.0, 0.0, midpoint.1),
                    Quat::from_rotation_y(angle),
                );
            } else if camera.angle_change > 0.0 {
                let angle = (angle_increment * time.delta_seconds())
                    .min(camera.angle_change * camera.angle_change);
                let midpoint = level_manager.get_current_level().map.midpoint();
                camera.angle_change -= angle;
                camera_transform.rotate_around(
                    Vec3::new(midpoint.0, 0.0, midpoint.1),
                    Quat::from_rotation_y(-angle),
                );
            }
        }
    }
}

pub fn camera_spin(mut camera_query: Query<&mut Transform, With<MainCamera>>, time: Res<Time>) {
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        camera_transform.rotate_around(
            Vec3::new(3.0, 0.0, 2.5),
            Quat::from_rotation_y(time.delta_seconds() * -0.05),
        );
    }
}
