#![deny(clippy::all)]
#![allow(unused_assignments)]
#![allow(clippy::type_complexity)]

#![feature(format_args_capture)]

mod info;

use std::f32::consts::PI;
use bevy::prelude::*;

use map::*;
use helper_functions::*;
use game_types::*;
use game_types::player_attr::*;

use rapier2d::prelude::*;
use rapier2d::na::Vector2;

pub use info::*;

pub struct AggroBot {
    my_id: PlayerID,
    my_player: TruncatedPlayer,
    internal_angle: Angle,
    players: Vec<TruncatedPlayer>,
    current_health: Health,
    prev_health: Health,
}

impl Bot for AggroBot {
    fn new(map: &Map, my_player_id: PlayerID) -> (Self, Ability, Model) {
        let bot = AggroBot {
            my_id: my_player_id,
            internal_angle: Angle(PI),
            players: Vec::new(),
            my_player: TruncatedPlayer::empty(my_player_id),
            prev_health: Health(100.0),
            current_health: Health(100.0),
        };

        (bot, Ability::Cloak, Model::SubmachineGun)

    }
    // Our bot (currently) doesn't do depending on map info
    fn update_map_info(&mut self, map: &Map) {}
    fn update_player_info(&mut self, players: &Vec<TruncatedPlayer>) {
        // Only let players be the other players, not this one
        self.players = players.iter().filter(|p| p.id.0 != self.my_id.0).cloned().collect();
        self.my_player = players.iter().find(|p| p.id.0 == self.my_id.0).unwrap().clone();

        let enemy_player = self.players[0].pos;

        // Face the opponent
        let angle = get_angle(self.my_player.pos.x, self.my_player.pos.y, enemy_player.x, enemy_player.y);
        self.internal_angle = Angle(match self.my_player.pos.x > enemy_player.x {
            true => angle - PI,
            false => angle,
        });

    }
    fn update_health(&mut self, new_health: Health) {
        self.prev_health = self.current_health;
        self.current_health = new_health;
    }

    // Since our bot is relatively simple, we don't need to do any misc. updates
    fn misc_update(&mut self) {}
    fn movement(&self) -> Option<(Angle, Dashing)> { 
        let enemy_player = self.players[0].pos;

        let distance = self.my_player.pos.distance(enemy_player);

        // Only run towards players if they're relatively close
        match distance >= 350.0 && distance <= 450.0 {
            true => Some((Angle(self.internal_angle.0), Dashing(false))), 
            false => None,

        }

    }

    // Use the internal angle that we got earlier to set our rotation
    fn update_direction(&self) -> Option<Angle> {
        Some(self.internal_angle)

    }
    fn use_ability(&self) -> bool {
        // If the bot takes damage, cloak
        self.current_health.0 < self.prev_health.0
        
    }

    fn should_shoot(&self) -> bool {
        true

    }
}
