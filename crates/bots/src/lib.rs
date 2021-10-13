#![deny(clippy::all)]
#![allow(unused_assignments)]
#![allow(clippy::type_complexity)]

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
    internal_angle: f32,
    players: Vec<TruncatedPlayer>,
}

impl Bot for AggroBot {
    fn new(map: &Map, players: &Vec<TruncatedPlayer>, my_player_id: PlayerID) -> Self {
        AggroBot {
            my_id: my_player_id,
            internal_angle: PI,
            players: Vec::new(),
            my_player: TruncatedPlayer::empty(my_player_id),
        }

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
        self.internal_angle = match self.my_player.pos.x > enemy_player.x {
            true => angle - PI,
            false => angle,
        };

    }

    // Since our bot is relatively simple, we don't need to do any misc. updates
    fn misc_update(&mut self) {}
    fn movement(&self) -> Option<(Angle, Dashing)> { 
        let enemy_player = self.players[0].pos;

        let distance = self.my_player.pos.distance(enemy_player);

        // Only run towards players if they're relatively close
        match distance >= 350.0 && distance <= 450.0 {
            true => {
                Some((Angle(self.internal_angle), Dashing(false)))

            }, 
            false => None,


        }

    }

    // Use the internal angle that we got earlier to set our rotation
    fn update_direction(&self) -> Option<Angle> {
        Some(Angle(self.internal_angle))

    }
    fn use_ability(&self) -> Option<Angle> {
        None

    }

    fn should_shoot(&self) -> bool {
        true

    }
}
