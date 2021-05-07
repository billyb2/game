use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// Used for being able to get random abilities, guns, etc.
use rand::Rng;
use rand::distributions::{Distribution, Standard};

use crate::components::*;
use crate::ProjectileType;

//Each player has a unique player id
#[derive(Bundle, Debug)]
pub struct Player {
    pub id: PlayerID,
    pub health: Health,
    pub speed: PlayerSpeed,
    pub requested_movement: RequestedMovement,
    pub movement_type: MovementType,
    pub distance_traveled: DistanceTraveled,
    pub ability: Ability,
    pub ability_charge: AbilityCharge,
    pub ability_completed: AbilityCompleted,
    pub using_ability: UsingAbility,
    pub can_respawn: RespawnTimer,

}

impl Player {
    pub fn new(id: u8, ability: Ability) -> Player {
        Player {
            id: PlayerID(id),
            health: Health(100),
            speed: match ability {
                // Stim players have a faster default running speed
                Ability::Stim => PlayerSpeed(13.0),
                _ => PlayerSpeed(11.0),
            },
            requested_movement: RequestedMovement::new(0.0, 0.0),
            movement_type: MovementType::SingleFrame,
            distance_traveled: DistanceTraveled(0.0),
            ability,
            ability_charge: AbilityCharge(Timer::from_seconds(5.0, false)),
            ability_completed: AbilityCompleted(Timer::from_seconds(3.0, false)),
            using_ability: UsingAbility(false),
            can_respawn: RespawnTimer(Timer::from_seconds(2.5, false))

        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Ability {
    Stim,
    Phase,
    Wall,
    Engineer, //Should be default
}

impl Distribution<Ability> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Ability {
        let rand_num: u8 = rng.gen_range(0..=3);

        match rand_num {
            0 => Ability::Stim,
            1 => Ability::Phase,
            2 => Ability::Wall,
            3 => Ability::Engineer,
            // This can't happen, but I need it for the match arm
            _ => Ability::Stim,

        }
    }
}


#[derive(Debug)]
pub struct PlayerSpeed(pub f32);

#[derive(Debug)]
pub struct AbilityCharge(pub Timer);

#[derive(Debug)]
pub struct AbilityCompleted(pub Timer);

#[derive(Debug)]
pub struct RespawnTimer(pub Timer);

#[derive(Debug)]
pub struct UsingAbility(pub bool);


#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Model {
    Pistol,
    Shotgun,
    Speedball,
    BurstRifle,
    AssaultRifle,
}


#[derive(Bundle, Debug)]
pub struct Gun {
    pub model: Model,
    pub time_since_last_shot: TimeSinceLastShot,
    pub time_since_start_reload: TimeSinceStartReload,
    pub ammo_in_mag: AmmoInMag,
    pub max_ammo: MaxAmmo,
    pub max_distance: MaxDistance,
    pub projectile_type: ProjectileType,
    pub projectile_speed: Speed,
    pub recoil_range: RecoilRange,
    pub bursting: Bursting,
    pub damage: Damage,
    pub projectile_size: Size,

}


impl Gun {
    pub fn new(model: Model, ability: Ability) -> Gun {
        let mut gun = Gun {
            model,
            time_since_last_shot: match model {
                Model::Pistol => TimeSinceLastShot(Timer::from_seconds(0.5, false)),
                Model::Shotgun => TimeSinceLastShot(Timer::from_seconds(1.5, false)),
                Model::Speedball => TimeSinceLastShot(Timer::from_seconds(1.5, false)),
                Model::BurstRifle => TimeSinceLastShot(Timer::from_seconds(0.5, false)),
                Model::AssaultRifle => TimeSinceLastShot(Timer::from_seconds(0.08, false)),

            },
            time_since_start_reload: TimeSinceStartReload {
                timer: match model {
                    Model::Pistol => Timer::from_seconds(2.0, false),
                    Model::Shotgun => Timer::from_seconds(5.0, false),
                    Model::Speedball => Timer::from_seconds(3.0, false),
                    Model::BurstRifle => Timer::from_seconds(3.25, false),
                    Model::AssaultRifle => Timer::from_seconds(3.75, false),

                },
                reloading: false,

            },
            ammo_in_mag: match model {
                Model::Pistol=> AmmoInMag(16),
                Model::Shotgun => AmmoInMag(8),
                Model::Speedball => AmmoInMag(6),
                Model::BurstRifle => AmmoInMag(21),
                Model::AssaultRifle => AmmoInMag(25),

            },
            max_ammo: match model {
                Model::Pistol=> MaxAmmo(16),
                Model::Shotgun => MaxAmmo(8),
                Model::Speedball => MaxAmmo(6),
                Model::BurstRifle => MaxAmmo(21),
                Model::AssaultRifle => MaxAmmo(25),

            },
            max_distance: match model {
                Model::Pistol => MaxDistance(900.0),
                Model::Shotgun => MaxDistance(300.0),
                Model::Speedball => MaxDistance(3000.0),
                Model::BurstRifle => MaxDistance(1000.0),
                Model::AssaultRifle => MaxDistance(1000.0),

            },

            recoil_range: match model {
                Model::Shotgun => RecoilRange(0.2),
                Model::Speedball => RecoilRange(0.0),
                Model::BurstRifle => RecoilRange(0.025),
                _ => RecoilRange(0.075),

            },
            projectile_type: match model {
                Model::Speedball => ProjectileType::Speedball,
                _ => ProjectileType::Regular,
            },
            projectile_speed: match model {
                Model::Pistol => Speed(15.0),
                Model::Shotgun => Speed(14.0),
                Model::Speedball => Speed(0.5),
                Model::BurstRifle => Speed(15.0),
                Model::AssaultRifle => Speed(16.0),

            },
            projectile_size: Size::new(5.0, 5.0),

            damage: match model {
                Model::Pistol => Damage(45),
                Model::Shotgun => Damage(25),
                Model::Speedball => Damage(1),
                Model::BurstRifle => Damage(13),
                Model::AssaultRifle => Damage(15),

            },
            // The bursting component only matters for burst rifles
            bursting: Bursting(false),

        };

        if ability == Ability::Engineer {
            // Cut the reload time in half
            gun.time_since_start_reload.timer.set_duration(gun.time_since_start_reload.timer.duration() / 2);

            // Increase the speed
            gun.projectile_speed.0 *= 1.25;

            // Increase the size of speedball slightly
            if gun.model == Model::Speedball {
                gun.projectile_size *= 1.25;

            }


        }

        gun
    }

}
