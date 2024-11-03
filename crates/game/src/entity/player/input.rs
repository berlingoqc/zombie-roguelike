
use bevy::{
    input::gamepad::{GamepadEvent, GamepadEventType},
    prelude::*
};

use bevy_ggrs::{Rollback, RollbackIdProvider};
use bytemuck::{Pod, Zeroable};
use ggrs::{Config, InputStatus, P2PSession, PlayerHandle, SpectatorSession, SyncTestSession};
use std::{hash::Hash};

use crate::shared::{game::{GameState, GameSpeed}, character::{CharacterMovementState, LookingAt, Death, Velocity}, collider::{MovementCollider, is_colliding}, utils::get_cursor_location, weapons::weapons::GameButton};

use super::{Player, MainCamera, PLAYER_SIZE};

// You can also register resources. If your Component / Resource implements Hash, you can make use of `#[reflect(Hash)]`
// in order to allow a GGRS `SyncTestSession` to construct a checksum for a world snapshot
#[derive(Default, Reflect, Hash, Component)]
#[reflect(Hash)]
pub struct FrameCount {
    pub frame: u32,
}

/// You need to define a config struct to bundle all the generics of GGRS. You can safely ignore `State` and leave it as u8 for all GGRS functionality.
/// TODO: Find a way to hide the state type.
#[derive(Debug)]
pub struct GGRSConfig;
impl ggrs::Config for GGRSConfig {
    type Input = BoxInput;
    type State = i32;
    type Address = String;
}

const INPUT_UP: i32 = 1 << 0;
const INPUT_DOWN: i32 = 1 << 1;
const INPUT_LEFT: i32 = 1 << 2;
const INPUT_RIGHT: i32 = 1 << 3;
pub const INPUT_FIRE: i32 = 1 << 4;
pub const INPUT_JUST_FIRE: i32 = 1 << 5;

pub const INPUT_WEAPON_RELOAD: i32 = 1 << 6;

pub const INPUT_WEAPON_CHANGED: i32 = 1 << 7;

pub const INPUT_INTERACTION_PRESSED: i32 = 1 << 8;


pub const INPUT_FROM_GAMEPAD: i32 = 1 << 31;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Pod, Zeroable, Default)]
pub struct BoxInput {
    // 0 : UP
    // 1 : DOWN
    // 2 : LEFT
    // 3 : RIGHT
    // 4 : Fire
    // 5 : Reload
    // 6 : Change Weapon
    // 7 : Action
    // ..
    // ..
    // 
    pub inp: i32,
    pub right_x: i32,
    pub right_y: i32,
}

#[derive(Default, PartialEq, Debug, Clone)]
pub enum SupportedController {
	#[default]
	Keyboard,
	Gamepad
}

#[derive(Component, Default, Debug, Clone)]
pub struct PlayerCurrentInput {
	pub input_source: SupportedController,
	pub gamepad: Option<Gamepad>,


    pub movement: Vec2,
    pub looking_at: Vec2,
    pub relative: bool,
}

pub struct AvailableGameController {
    pub keyboard_mouse: bool,
    pub gamepad: Vec<Gamepad>,
}


fn vec_moving(vec: &Vec2) -> bool {
    return vec.x != 0. && vec.y != 0.;
}


pub fn system_gamepad_event(
    mut q_player_input: Query<&mut PlayerCurrentInput, With<Player>>,
    mut gamepad_evr: EventReader<GamepadEvent>,

    mut available_controller: ResMut<AvailableGameController>,
) {
    for GamepadEvent(id, kind) in gamepad_evr.iter() {
        match kind {
            GamepadEventType::Connected => {
                available_controller.gamepad.push(id.clone());
            },
            GamepadEventType::Disconnected => {
                available_controller.gamepad = available_controller.gamepad.iter().filter(|x| x.0 != id.0).map(|x| x.clone()).collect();
            },
            _ => {
                //info!("OTHER EVENT I GUESS {:?}", id);
            }
        }
    }

}



pub fn input(
    handle: In<PlayerHandle>,

    q_player: Query<(&PlayerCurrentInput, &Player)>,
    
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,

    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,


    buttons: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,


) -> BoxInput {

    let mut input: i32 = 0;
    let mut mouse_position: Vec2 = Vec2::default();

    for (player_input, player) in q_player.iter() {

        if (player.handle == handle.0) {

            if player_input.input_source == SupportedController::Keyboard {

                if keyboard_input.pressed(KeyCode::W) {
                    input |= INPUT_UP;
                }
                if keyboard_input.pressed(KeyCode::A) {
                    input |= INPUT_LEFT;
                }
                if keyboard_input.pressed(KeyCode::S) {
                    input |= INPUT_DOWN;
                }
                if keyboard_input.pressed(KeyCode::D) {
                    input |= INPUT_RIGHT;
                }
                if mouse_input.pressed(MouseButton::Left) {
                    input |= INPUT_FIRE
                }
                if mouse_input.just_pressed(MouseButton::Left) {
                    input |= INPUT_JUST_FIRE;
                }
                if keyboard_input.just_pressed(KeyCode::Tab) {
                    input |= INPUT_WEAPON_CHANGED;
                }
                if keyboard_input.just_pressed(KeyCode::R) {
                    input |= INPUT_WEAPON_RELOAD;
                }
                if keyboard_input.pressed(KeyCode::F) {
                    input |= INPUT_INTERACTION_PRESSED;
                }

                mouse_position = get_cursor_location(&wnds, &q_camera);
            } else {
                input |= INPUT_FROM_GAMEPAD;


                let player_gamepad = player_input.gamepad.unwrap();

                let axis_lx = GamepadAxis(player_gamepad, GamepadAxisType::LeftStickX);
                let axis_ly = GamepadAxis(player_gamepad, GamepadAxisType::LeftStickY);

                let axis_rx = GamepadAxis(player_gamepad, GamepadAxisType::RightStickX);
                let axis_ry = GamepadAxis(player_gamepad, GamepadAxisType::RightStickY);


                if let (Some(x), Some(y), Some(rx), Some(ry)) = (axes.get(axis_lx), axes.get(axis_ly), axes.get(axis_rx), axes.get(axis_ry)) {
                    let left_stick_pos = Vec2::new(x, y);
                    let right_stick_pos = Vec2::new(rx * 100., ry * 100.);

                    mouse_position = right_stick_pos;

                    if x > 0.1 {
                        input |= INPUT_RIGHT;
                    } else if x < -0.1 {
                        input |= INPUT_LEFT;
                    }

                    if y > 0.1 {
                        input |= INPUT_UP;
                    } else if (y) < -0.1 {
                        input |= INPUT_DOWN;
                    }
                }

                let reload_button = GamepadButton(player_gamepad, GamepadButtonType::West);
                let change_weapon_button = GamepadButton(player_gamepad, GamepadButtonType::North);
                let interaction_button = GamepadButton(player_gamepad, GamepadButtonType::South);
                let weapon_trigger_button = GamepadButton(player_gamepad, GamepadButtonType::RightTrigger);

                if buttons.pressed(weapon_trigger_button) {
                    input |= INPUT_FIRE
                }
                if buttons.just_pressed(weapon_trigger_button) {
                    input |= INPUT_JUST_FIRE;
                }
                if buttons.just_pressed(change_weapon_button) {
                    input |= INPUT_WEAPON_CHANGED;
                }
                if buttons.just_pressed(reload_button) {
                    input |= INPUT_WEAPON_RELOAD;
                }
                if buttons.pressed(interaction_button) {
                    input |= INPUT_INTERACTION_PRESSED;
                }

            }
        }
    }

    BoxInput { inp: input, right_x: mouse_position.x as i32, right_y: mouse_position.y as i32}
}


pub fn apply_input_players(

    mut query: Query<(&mut PlayerCurrentInput, &Player), Without<Death>>,

    inputs: Res<Vec<(BoxInput, InputStatus)>>,

    mut game_state: ResMut<State<GameState>>
) {

    for (
		mut current_input,
		player,
	) in query.iter_mut() {

        if inputs.len() <= player.handle {
            continue;
        }

        let box_input = match inputs[player.handle].1 {
            InputStatus::Confirmed => inputs[player.handle].0,
            InputStatus::Predicted => inputs[player.handle].0,
            InputStatus::Disconnected => BoxInput::default(), // disconnected players do nothing
        };


        let mut movement = Vec2::default();

        if box_input.inp & INPUT_DOWN == 0 && box_input.inp & INPUT_UP != 0 {
            movement += Vec2::new(0., 1.);
        } else if box_input.inp & INPUT_DOWN != 0 && box_input.inp & INPUT_UP == 0 {
            movement += Vec2::new(0., -1.);
        }

        if box_input.inp & INPUT_LEFT == 0 && box_input.inp & INPUT_RIGHT != 0 {
            movement += Vec2::new(1., 0.);
        } else if box_input.inp & INPUT_LEFT != 0 && box_input.inp & INPUT_RIGHT == 0 {
            movement += Vec2::new(-1., 0.);
        }

        current_input.movement = movement;
        if box_input.inp & INPUT_FROM_GAMEPAD == INPUT_FROM_GAMEPAD {
            current_input.relative = true;
            let vec = Vec2::new((box_input.right_x as f32) / 100., (box_input.right_y as f32) / 100.);
            if vec_moving(&vec) {
                current_input.looking_at = vec;
            }
        } else {
            current_input.looking_at = Vec2::new(box_input.right_x as f32, box_input.right_y as f32);
        }
    }
}

pub fn update_velocity_player(
    mut query: Query<(&Transform, &mut Velocity, &PlayerCurrentInput,)>
) {
    for (t, mut v, c) in query.iter_mut() {
        v.v = c.movement;
    }
}

pub fn move_players(
    mut query: Query<(&mut Transform, &mut LookingAt, &mut CharacterMovementState, &Velocity, &PlayerCurrentInput), With<Player>>,

    collider_query: Query<
        (Entity, &Transform, &MovementCollider),
        (Without<Player>, Without<Death>),
    >,

    game_speed: Res<GameSpeed>,
) {
    for (mut player_transform, mut looking_at, mut character_movement_state, v, c) in query.iter_mut() {
        looking_at.0 = c.looking_at;
        looking_at.1 = c.relative;
        if v.v.x != 0. || v.v.y != 0. {
			character_movement_state.state = "walking".to_string();

			let dest = player_transform.translation + (v.v.extend(0.) * game_speed.0 * 125.);

			if !is_colliding(dest, PLAYER_SIZE, "player",&collider_query) {
				player_transform.translation = dest;
			}
		} else if character_movement_state.state == "walking" {
            character_movement_state.state = "standing".to_string();
		}
    }

}