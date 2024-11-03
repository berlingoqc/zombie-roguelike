use bevy::prelude::*;
use ggrs::InputStatus;

//use crate::shared::{map::{MapElementPosition, Window, WindowPanel, Size}, health::Health, weapons::weapons::{PlayerInputs, INTERACTION_BTN}};

use super::{Player, input::{PlayerCurrentInput, INPUT_INTERACTION_PRESSED, BoxInput}};

#[derive(Component)]
pub struct PlayerCurrentInteraction {
    // tell if or not there is an interaction available for the user
    pub interaction: bool,
    // cooldown between each interaction
    pub interaction_cooldown: f32,
    // entity that has the interaction component
    pub entity: Entity,
    pub child_entity: Entity,
    // type of interaction
    pub interaction_type: PlayerInteractionType,

    // tell if the player is doing the interaction
    pub interacting: bool,

    // when the user last trigger the interaction
    pub interaction_trigger_at: f32,
}

#[derive(Default, Clone, Copy)]
pub enum PlayerInteractionType {
    #[default]
    None = 0,

    RepairWindow,
}

#[derive(Default, Component)]
pub struct PlayerInteraction {
    pub interaction_available: bool,
    // the type of interaction , use to find the right handler for the action
    pub interaction_type: PlayerInteractionType,
    // the size of the zone where the player can trigger the animatin arround
    // the position of the interaction entity
    pub interaction_size: Vec2,
    // timeout before the interaction can be trigger again
    pub interaction_timeout: f32
}

pub fn system_interaction_player(
    mut query_player: Query<(&Transform, &mut PlayerCurrentInteraction, &PlayerCurrentInput, &Player)>,
    time: Res<Time>,
    interaction_query: Query<
        (Entity, &Transform, &MapElementPosition, &PlayerInteraction),
        (
            With<MapElementPosition>,
            Without<Player>,
        ),
    >,

    inputs: Res<Vec<(BoxInput, InputStatus)>>,

    mut query_window: Query<(&mut Window, &mut Health, &Children)>,
    mut query_panel: Query<(&mut WindowPanel, &Size, &mut Sprite)>
) {

    for (player_transform, mut interaction, current_input, player) in query_player.iter_mut() {

        if inputs.len() <= player.handle {
            continue;
        }

        let box_input = match inputs[player.handle].1 {
            InputStatus::Confirmed => inputs[player.handle].0,
            InputStatus::Predicted => inputs[player.handle].0,
            InputStatus::Disconnected => BoxInput::default(), // disconnected players do nothing
        };


        for (entity, transform, info, player_interaction) in interaction_query.iter() {
            let collision = collide(player_transform.translation, Vec2::new(25., 25.), info.position.extend(10.),  player_interaction.interaction_size);
            if collision.is_some() && player_interaction.interaction_available {
                // notify use that key perform action
                interaction.interaction = true;
                interaction.entity = entity.clone();
                interaction.interaction_type = player_interaction.interaction_type;
                interaction.interaction_cooldown = player_interaction.interaction_timeout;
            } else {
                if entity.id() == interaction.entity.id() {
                    match interaction.interaction_type {
                        PlayerInteractionType::RepairWindow => {
                            if interaction.interacting == true {
                                // SEND CANCEL REPARATRION EVENT
                                let (_,size, mut sprite) = query_panel.get_mut(interaction.child_entity).unwrap();
                                interaction.interacting = false;
                                sprite.custom_size = Some(Vec2::new(0.,0.));
                            }
                        },
                        _ => {}
                    }
                    interaction.interaction = false;
                    interaction.interacting = false;
                    interaction.entity = Entity::from_raw(0);
                }
            }
        } 


        if interaction.interaction {
            if box_input.inp & INPUT_INTERACTION_PRESSED == INPUT_INTERACTION_PRESSED {
                match interaction.interaction_type {
                    PlayerInteractionType::RepairWindow => {
                        if interaction.interacting == true {
                            // repair the window
                            let time_since_startup = time.time_since_startup().as_secs_f32();
                            if interaction.interaction_trigger_at + interaction.interaction_cooldown <= time_since_startup {
                                let (_,size, mut sprite) = query_panel.get_mut(interaction.child_entity).unwrap();
                                sprite.custom_size = Some(size.0);
                                interaction.interacting = false;

                                if let Ok((mut window, mut health, _)) = query_window.get_mut(interaction.entity) {
                                    health.tmp_health += 1.0
                                }
                            } else {
                                let (_,size, mut sprite) = query_panel.get_mut(interaction.child_entity).unwrap();
                                let time_diff = time_since_startup - (interaction.interaction_trigger_at + interaction.interaction_cooldown);
                                let percentage_time_diff_cooldown = 1. - (time_diff / interaction.interaction_cooldown);
                                sprite.custom_size = Some(size.0 / percentage_time_diff_cooldown);
                            }
                        } else {
                            let (_, health, children) = query_window.get(interaction.entity).unwrap();

                            if health.current_health < health.max_health {
                                for &child_entity in children.iter() {
                                    let (_,size , mut sprite) = query_panel.get_mut(child_entity).unwrap();
                                    if sprite.custom_size.unwrap().x == 0. {
                                        // there is a panel to repair
                                        interaction.interacting = true;
                                        interaction.child_entity = child_entity.clone();
                                        interaction.interaction_trigger_at = time.time_since_startup().as_secs_f32();
                                        break;
                                    }
                                }
                            }
                        }
                    },
                    _ => {}
                }
            } else {
                if interaction.interacting {
                    interaction.interacting = false;
                    match interaction.interaction_type {
                        PlayerInteractionType::RepairWindow => {
                            let (_,size, mut sprite) = query_panel.get_mut(interaction.child_entity).unwrap();
                            sprite.custom_size = Some(Vec2::new(0.,0.));
                        },
                        _ => {}
                    }
                }
            }
        }
    }

}

