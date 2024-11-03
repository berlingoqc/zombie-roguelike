//pub mod interaction;
//pub mod input;

use bevy::prelude::*;



#[derive(Component)]
pub struct Player {
    pub handle: usize,
}


#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    //#[bundle]
    pub sprite: SpriteSheetBundle,

    //pub velocity: Velocity,
    //pub player_current_input: PlayerCurrentInput,
    /*
    pub interaction: PlayerCurrentInteraction,
    pub looking_direction: LookingAt,
    pub animation_timer: AnimationTimer,
    pub map_element_position: MapElementPosition,
    pub movement_collider: MovementCollider,
    pub health: Health,
    pub health_regeneration: HealthRegeneration,
    pub character_movement_state: CharacterMovementState,
    */
}