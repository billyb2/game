#![deny(clippy::all)]
#![allow(unused_assignments)]
#![allow(clippy::type_complexity)]

#![feature(format_args_capture)]

mod info;

use std::f32::consts::PI;

use map::*;
use helper_functions::*;
use game_types::*;

pub use info::*;

pub struct AggroBot {
    my_id: PlayerID,
    my_player: TruncatedPlayer,
    internal_angle: Angle,
    players: Vec<TruncatedPlayer>,
    current_health: Health,
    prev_health: Health,
}

#[allow(unused_variables)]
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

        (bot, Ability::Cloak, Model::AssaultRifle)

    }
    // Our bot (currently) doesn't do depending on map info
    fn update_map_info(&mut self, map: &Map) {}
    fn update_player_info(&mut self, players: &Vec<TruncatedPlayer>) {
        // Only let players be the other players, not this one
        self.players = players.iter().filter(|p| p.id.0 != self.my_id.0).cloned().collect();
        self.my_player = players.iter().find(|p| p.id.0 == self.my_id.0).unwrap().clone();

        if self.players.len() > 0 {
            let enemy_player = self.players[0].pos;

            // Face the opponent
            let angle = get_angle(self.my_player.pos.x, self.my_player.pos.y, enemy_player.x, enemy_player.y);
            self.internal_angle = Angle(angle);
        }

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


// A bot that does literally nothing, useful for testing weapons and abilities
pub struct StandStillBot;

#[allow(unused_variables)]
impl Bot for StandStillBot {
    fn new(map: &Map, my_player_id: PlayerID) -> (Self, Ability, Model) { (StandStillBot, Ability::Cloak, Model::AssaultRifle) }
    fn update_map_info(&mut self, map: &Map) {}
    fn update_player_info(&mut self, players: &Vec<TruncatedPlayer>) {}
    fn update_health(&mut self, new_health: Health) {}
    fn misc_update(&mut self) {}
    fn movement(&self) -> Option<(Angle, Dashing)> {None}
    fn update_direction(&self) -> Option<Angle> {None}
    fn use_ability(&self) -> bool {false}
    fn should_shoot(&self) -> bool {false}
}


pub struct Xx69N00bSlay3rxX {
    enemy_players: Vec<TruncatedPlayer>,
    targeted_player: Option<TruncatedPlayer>,
    damage_taken: Health,
    my_player: Option<TruncatedPlayer>,
    my_id: PlayerID,
    angle: Angle,

}

impl Bot for Xx69N00bSlay3rxX {
    fn new(map: &Map, my_player_id: PlayerID) -> (Self, Ability, Model) {
        (Xx69N00bSlay3rxX {
            enemy_players: Vec::new(),
            damage_taken: Health(0.0),
            targeted_player: None,
            my_player: None,
            my_id: my_player_id,
            angle: Angle(0.0),

        }, Ability::Cloak, Model::SniperRifle)
    }

    fn update_map_info(&mut self, map: &Map) {

    }

    fn update_player_info(&mut self, players: &Vec<TruncatedPlayer>) {
        // Only let players be the other players, not this one
        self.enemy_players = players.iter().filter(|p| p.id.0 != self.my_id.0).cloned().collect();
        self.my_player = Some(players.iter().find(|p| p.id.0 == self.my_id.0).unwrap().clone());

        self.targeted_player = self.enemy_players.iter().find(|p| {
            self.my_player.unwrap().pos.distance(p.pos) >= 500.0
        }).cloned();

        if let Some(target) = self.targeted_player {
            let angle = get_angle(self.my_player.unwrap().pos.x, self.my_player.unwrap().pos.y, target.pos.x, target.pos.y);
            self.angle = Angle(angle);

        }
        

    }

    fn update_health(&mut self, new_health: Health) {

    }

    fn misc_update(&mut self) {

    }

    fn movement(&self) -> Option<(Angle, Dashing)> {
        None
    }

    fn update_direction(&self) -> Option<Angle> {
        /*if let Some(target_player) = self.targeted_player {
            let my_pos = self.my_player.unwrap().pos;
            let targeted_player_pos = target_player.pos;
            let angle = get_angle(my_pos.x, my_pos.y, targeted_player_pos.x, targeted_player_pos.y);
find
            Some(Angle(match my_pos.x > targeted_player_pos.x {
                true => angle - PI,
                false => angle,
            }))
        } else {
            None
        }*/
        Some(self.angle)



    }

    fn use_ability(&self) -> bool {
        true
    }
    fn should_shoot(&self) -> bool {
        self.targeted_player.is_some()
    }
}