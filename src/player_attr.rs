#![deny(clippy::all)]
#![allow(clippy::type_complexity)]

use std::convert::From;
use std::mem::variant_count;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// Used for being able to get random abilities, guns, etc.
use rand::Rng;
use rand::distributions::{Distribution, Standard};

#[cfg(feature = "parallel")]
use rayon::join;

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
    pub ability: Ability,
    pub ability_charge: AbilityCharge,
    pub ability_completed: AbilityCompleted,
    pub using_ability: UsingAbility,
    pub can_respawn: RespawnTimer,
    pub dashing_info: DashingInfo,
    pub perk: Perk,

}

pub const DEFAULT_PLAYER_SPEED: f32 = 13.0;


pub fn set_ability_player_attr(ability_charge: &mut AbilityCharge, ability_completed: &mut AbilityCompleted, ability: Ability) {

    let mut set_ability_charge = || *ability_charge = match ability {
        Ability::Hacker => AbilityCharge(Timer::from_seconds(15.0, false)),
        Ability::Stim => AbilityCharge(Timer::from_seconds(7.5, false)),
        Ability::Warp => AbilityCharge(Timer::from_seconds(5.0, false)),
        Ability::Wall => AbilityCharge(Timer::from_seconds(5.0, false)),
        Ability::Engineer => AbilityCharge(Timer::from_seconds(1.0, false)),
        Ability::Inferno => AbilityCharge(Timer::from_seconds(15.0, false)),
        Ability::Cloak => AbilityCharge(Timer::from_seconds(17.0, false)),
        Ability::PulseWave => AbilityCharge(Timer::from_seconds(10.0, false)),

    };

    let mut set_ability_completed = || *ability_completed = match ability {
        Ability::Stim => AbilityCompleted(Timer::from_seconds(3.0, false)),
        Ability::Cloak => AbilityCompleted(Timer::from_seconds(5.0, false)),
        // Only stim and cloak have a duration, so this variable can be set to whatever for the other abilities
        _ => AbilityCompleted(Timer::from_seconds(3.0, false)),
    };

    #[cfg(feature = "parallel")]
    join(set_ability_charge, set_ability_completed);

    #[cfg(not(feature = "parallel"))]
    {
        set_ability_charge();
        set_ability_completed();

    };

}

pub fn set_perk_player_attr(health: &mut Health, speed: &mut PlayerSpeed, perk: Perk) {
    if perk == Perk::HeavyArmor {
        health.0 *= 1.25;
        speed.0 *= 0.8;

    } else if perk == Perk::LightArmor {
        health.0 *= 0.8;
        speed.0 *= 1.3;

    }

}

pub fn set_perk_gun_attr(mut max_ammo: &mut MaxAmmo, mut ammo_in_mag: &mut AmmoInMag, perk: Perk) {
    if perk == Perk::ExtendedMag {
        max_ammo.0 = (max_ammo.0 as f32 * 1.5).ceil() as u8;
        ammo_in_mag.0 = max_ammo.0;

    }

}


impl Player {
    pub fn new(id: u8, ability: Ability, perk: Perk, living: bool) -> Player {
        let mut player = Player {
            id: PlayerID(id),
            health: match living {
                true => Health(100.0),
                false => Health(0.0),
            },
            speed: match ability {
                // Stim players have a faster default running speed
                Ability::Stim => PlayerSpeed(DEFAULT_PLAYER_SPEED + 1.0),
                _ => PlayerSpeed(DEFAULT_PLAYER_SPEED),
            },
            requested_movement: RequestedMovement::new(0.0, 0.0),
            movement_type: MovementType::SingleFrame,
            ability,
            ability_charge: AbilityCharge(Timer::from_seconds(0.0, false)),
            // The AbilityCompleted timer is just the duration of how long the ability lasts (if it has an affect over time)
            ability_completed: AbilityCompleted(Timer::from_seconds(0.0, false)),
            using_ability: UsingAbility(false),
            can_respawn: RespawnTimer(Timer::from_seconds(2.5, false)),
            dashing_info: DashingInfo {
                time_till_can_dash: Timer::from_seconds(3.0, false),
                time_till_stop_dash: Timer::from_seconds(0.2, false),
                dashing: false,
            },
            perk,

        };

        set_perk_player_attr(&mut player.health, &mut player.speed, player.perk);
        set_ability_player_attr(&mut player.ability_charge, &mut player.ability_completed, player.ability);

        // The ability charge is ready on game start
        finish_timer(&mut player.ability_charge.0);
        finish_timer(&mut player.dashing_info.time_till_can_dash);

        player
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Ability {
    Stim,
    Warp,
    Wall,
    Engineer, //Should be default
    Hacker,
    Inferno,
    Cloak,
    PulseWave,

}


#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Perk {
    ExtendedMag,
    HeavyArmor,
    LightArmor,

}

pub const NUM_OF_ABILITIES: u8 = variant_count::<Ability>() as u8;

impl From<u8> for Ability {
    fn from(ability: u8)  -> Self {
        match ability {
            0 => Ability::Stim,
            1 => Ability::Warp,
            2 => Ability::Wall,
            3 => Ability::Engineer,
            4 => Ability::Hacker,
            5 => Ability::Inferno,
            6 => Ability::Cloak,
            7 => Ability::PulseWave,
            _ => panic!("Ability conversion out of bounds: {} was requested, max is {}", ability, NUM_OF_ABILITIES),

        }

    }

}

impl From<Ability> for u8 {
    fn from(ability: Ability)  -> Self {
        match ability {
            Ability::Stim => 0,
            Ability::Warp => 1,
            Ability::Wall => 2,
            Ability::Engineer => 3,
            Ability::Hacker => 4,
            Ability::Inferno => 5,
            Ability::Cloak => 6,
            Ability::PulseWave => 7,

        }

    }

}

impl Distribution<Ability> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Ability {
        let rand_num: u8 = rng.gen_range(0..NUM_OF_ABILITIES);
        let ability: Ability = rand_num.into();

        ability
    }
}

pub const NUM_OF_PERKS: u8 = variant_count::<Perk>() as u8;

impl From<u8> for Perk {
    fn from(perk: u8)  -> Self {
        match perk {
            0 => Perk::ExtendedMag,
            1 => Perk::HeavyArmor,
            2 => Perk::LightArmor,
            _ => panic!("Perk conversion out of bounds: {} was requested, max is {}", perk, NUM_OF_PERKS),

        }

    }

}

impl From<Perk> for u8 {
    fn from(perk: Perk)  -> Self {
        match perk {
            Perk::ExtendedMag => 0,
            Perk::HeavyArmor => 1,
            Perk::LightArmor => 2,

        }

    }

}

impl Distribution<Perk> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Perk {
        let rand_num: u8 = rng.gen_range(0..NUM_OF_PERKS);
        let perk: Perk = rand_num.into();

        perk
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
    SubmachineGun,
    ClusterShotgun,
    Flamethrower,

}

pub const NUM_OF_GUN_MODELS: u8 = variant_count::<Model>() as u8;

impl From<u8> for Model {
    fn from(model: u8)  -> Self {
        match model {
            0 => Model::Pistol,
            1 => Model::Shotgun,
            2 => Model::Speedball,
            3 => Model::BurstRifle,
            4 => Model::AssaultRifle,
            5 => Model::SubmachineGun,
            6 => Model::ClusterShotgun,
            7 => Model::Flamethrower,
            _ => panic!("Gun model conversion out of bounds: {} was requested, max is {}", model, NUM_OF_GUN_MODELS),

        }

    }

}

impl From<Model> for u8 {
    fn from(model: Model)  -> Self {
        match model {
            Model::Pistol=> 0,
            Model::Shotgun => 1,
            Model::Speedball=> 2,
            Model::BurstRifle => 3,
            Model::AssaultRifle => 4,
            Model::SubmachineGun => 5,
            Model::ClusterShotgun => 6,
            Model::Flamethrower => 7,

        }

    }

}

impl Distribution<Model> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Model {
        let rand_num: u8 = rng.gen_range(0..NUM_OF_GUN_MODELS);
        let gun_model: Model = rand_num.into();

        gun_model
    }
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
    pub fn new(model: Model, ability: Ability, perk: Perk) -> Gun {
        let mut gun = Gun {
            model,
            time_since_last_shot: match model {
                Model::Pistol => TimeSinceLastShot(Timer::from_seconds(0.5, false)),
                Model::Shotgun => TimeSinceLastShot(Timer::from_seconds(0.8, false)),
                Model::Speedball => TimeSinceLastShot(Timer::from_seconds(1.5, false)),
                Model::BurstRifle => TimeSinceLastShot(Timer::from_seconds(0.5, false)),
                Model::AssaultRifle => TimeSinceLastShot(Timer::from_seconds(0.12, false)),
                Model::SubmachineGun => TimeSinceLastShot(Timer::from_seconds(0.07, false)),
                Model::ClusterShotgun => TimeSinceLastShot(Timer::from_seconds(1.3, false)),
                Model::Flamethrower => TimeSinceLastShot(Timer::from_seconds(0.1, false)),

            },
            time_since_start_reload: TimeSinceStartReload {
                timer: match model {
                    Model::Pistol => Timer::from_seconds(1.0, false),
                    Model::Shotgun => Timer::from_seconds(4.0, false),
                    Model::Speedball => Timer::from_seconds(3.0, false),
                    Model::BurstRifle => Timer::from_seconds(3.25, false),
                    Model::AssaultRifle => Timer::from_seconds(3.75, false),
                    Model::SubmachineGun => Timer::from_seconds(2.0, false),
                    Model::ClusterShotgun => Timer::from_seconds(4.0, false),
                    Model::Flamethrower => Timer::from_seconds(2.0, false),

                },
                reloading: false,

            },
            // Ammo in mag and max_ammo should match
            ammo_in_mag: match model {
                Model::Pistol=> AmmoInMag(16),
                Model::Shotgun => AmmoInMag(8),
                Model::Speedball => AmmoInMag(6),
                Model::BurstRifle => AmmoInMag(21),
                Model::AssaultRifle => AmmoInMag(25),
                Model::SubmachineGun => AmmoInMag(35),
                Model::ClusterShotgun => AmmoInMag(5),
                Model::Flamethrower => AmmoInMag(30),

            },
            max_ammo: match model {
                Model::Pistol=> MaxAmmo(16),
                Model::Shotgun => MaxAmmo(8),
                Model::Speedball => MaxAmmo(6),
                Model::BurstRifle => MaxAmmo(21),
                Model::AssaultRifle => MaxAmmo(25),
                Model::SubmachineGun => MaxAmmo(35),
                Model::ClusterShotgun => MaxAmmo(5),
                Model::Flamethrower => MaxAmmo(30),

            },
            max_distance: match model {
                Model::Pistol => MaxDistance(1250.0),
                Model::Shotgun => MaxDistance(700.0),
                Model::Speedball => MaxDistance(3000.0),
                Model::BurstRifle => MaxDistance(1000.0),
                Model::AssaultRifle => MaxDistance(1000.0),
                Model::SubmachineGun => MaxDistance(600.0),
                Model::ClusterShotgun => MaxDistance(275.0),
                Model::Flamethrower => MaxDistance(200.0),

            },
            // The recoil range is in radians
            recoil_range: match model {
                Model::Shotgun => RecoilRange(0.2),
                Model::Speedball => RecoilRange(0.0),
                Model::BurstRifle => RecoilRange(0.025),
                Model::SubmachineGun => RecoilRange(0.12),
                Model::ClusterShotgun => RecoilRange(0.07),
                Model::Flamethrower => RecoilRange(0.15),
                _ => RecoilRange(0.075),

            },
            projectile_type: match model {
                Model::Speedball => ProjectileType::Speedball,
                Model::Flamethrower => ProjectileType::Flame,
                _ => ProjectileType::Regular,
            },
            projectile_speed: match model {
                Model::Pistol => Speed(20.0),
                Model::Shotgun => Speed(16.0),
                Model::Speedball => Speed(0.7),
                Model::BurstRifle => Speed(17.0),
                Model::AssaultRifle => Speed(18.0),
                Model::SubmachineGun => Speed(16.0),
                Model::ClusterShotgun => Speed(11.0),
                Model::Flamethrower => Speed(15.0),

            },
            projectile_size: match model {
                Model::SubmachineGun => Size::new(3.0, 3.0),
                _ => Size::new(6.0, 6.0),

            },

            damage: match model {
                Model::Pistol => Damage(45.0),
                Model::Shotgun => Damage(6.0),
                Model::Speedball => Damage(1.0),
                Model::BurstRifle => Damage(15.0),
                Model::AssaultRifle => Damage(13.0),
                Model::SubmachineGun => Damage(7.5),
                Model::ClusterShotgun => Damage(17.0),
                Model::Flamethrower => Damage(3.25),


            },
            // The bursting component only matters for burst rifles
            bursting: Bursting(false),

        };

        if ability == Ability::Engineer {
            // Cut the reload time in half
            gun.time_since_start_reload.timer.set_duration(gun.time_since_start_reload.timer.duration() / 2);

            // The recoil of engineers is higher
            gun.recoil_range.0 *= 1.25;

            // Increase the speed
            gun.projectile_speed.0 *= 1.2;

            // Increase the size of speedball slightly
            if gun.model == Model::Speedball {
                gun.projectile_size *= 1.2;

            }


        } else if ability == Ability::Inferno {
            // Inferno's bullets do less damage to make up for the fact that his fire does so much
            gun.damage.0 *= 0.7;

        }

        set_perk_gun_attr(&mut gun.max_ammo, &mut gun.ammo_in_mag, perk);

        gun
    }

}


fn finish_timer(timer: &mut Timer){
    timer.set_elapsed(timer.duration());

}
