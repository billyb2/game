#![deny(clippy::all)]
#![allow(clippy::type_complexity)]

// This file is for storing all systems that are used as setups, such as setting up cameras, drawing the map, etc
use std::convert::TryInto;

use bevy::prelude::*;
use bevy::math::const_vec2;

use game_types::*;

use map::MapCRC32;
use single_byte_hashmap::*;
use bots::*;
use map::*;

use rapier2d::prelude::*;
use rapier2d::na::Vector2;

#[cfg(feature = "graphics")]
pub use setup_graphics::*;

#[allow(clippy::too_many_arguments)]
pub fn setup_players(mut commands: Commands, _materials: Option<Res<Skin>>, (maps, map_crc32): (Res<Maps>, Res<MapCRC32>), mut _deathmatch_score: ResMut<DeathmatchScore>, my_gun_model: Option<Res<Model>>, my_ability: Option<Res<Ability>>, my_perk: Option<Res<Perk>>, (mut _rigid_body_set, mut _collider_set): (Option<ResMut<RigidBodySet>>, Option<ResMut<ColliderSet>>), num_of_bots: Res<NumOfBots>, my_player_name: Option<Res<PlayerName>>, hosting: Res<Hosting>) {
    let mut available_player_ids: Vec<PlayerID> = Vec::with_capacity(10);
    let mut player_entities: HashMap<u8, Entity> = HashMap::with_capacity_and_hasher(10, BuildHasher::default());

    let mut remaining_bots_to_add = num_of_bots.0;

    #[allow(unused_mut)]
    let mut online_player_ids = HashMap::with_capacity_and_hasher(10, BuildHasher::default());

    #[allow(unused_mut)]
    let mut local_players = Vec::with_capacity(5);


    let map = maps.0.get(&map_crc32.0).unwrap();
    
    map.spawn_points.iter().enumerate().for_each(|(i, coords)| {
        let i: u8 = i.try_into().unwrap();

        let ability = match &my_ability {
            Some(ability) => **ability,
            None => Ability::Engineer,

        };

        let gun_model = match &my_gun_model {
            Some(gun_model) => **gun_model,
            None => Model::Pistol,

        };

        let perk = match &my_perk {
            Some(perk) => **perk,
            None => Perk::ExtendedMag,

        };

        let player_name = match &my_player_name {
            Some(name) => Some((**name).clone()),
            None => None,

        };

        let player = Player::new(i, ability, perk, false, player_name);

        let mut entity = commands.spawn_bundle(player);

        entity
            .insert_bundle(Gun::new(gun_model, ability, perk))
            .insert(Transform::from_translation(coords.extend(101.0)));

        #[cfg(feature = "graphics")]
        let (material, size) = 
            match i == 1 {
                true => _materials.as_ref().unwrap().player[0].clone(),
                false => _materials.as_ref().unwrap().enemy.clone(),
            };

        #[cfg(not(feature = "graphics"))]
        let size = 
            match i == 1 {
                true => _materials.as_ref().unwrap().player[0].clone(),
                false => _materials.as_ref().unwrap().enemy.clone(),
            };

        #[cfg(feature = "graphics")]
        entity
            .insert_bundle(SpriteBundle {
                texture: material,
                sprite: Sprite {
                    custom_size: Some(size),
                    flip_x: true, 
                    ..Default::default()
                },
                visibility: Visibility {
                    is_visible: false,
                },
                transform: Transform::from_translation(coords.extend(101.0)),
                ..Default::default()
            })
            .insert(GameRelated);
            
        player_entities.insert(i, entity.id());

        #[cfg(feature = "graphics")]
        { 
            let rigid_body_set = _rigid_body_set.as_mut().unwrap();
            let collider_set = _collider_set.as_mut().unwrap();

            let rigid_body = RigidBodyBuilder::new(RigidBodyType::Dynamic)
                .translation(Vector2::new(coords.x, coords.y).component_div(&Vector2::new(250.0, 250.0)))
                .linvel(Vector2::new(0.0, 0.0))
                .gravity_scale(0.0)
                .linear_damping(80.0)
                .user_data(u128::MAX)
                .ccd_enabled(true)
                .additional_mass(0.36)
                .build();

            let collider_size = size / const_vec2!([500.0; 2]);

            let collider = ColliderBuilder::cuboid(collider_size.x, collider_size.x)
                .collision_groups(InteractionGroups::none())
                .restitution(0.000001)
                .friction(0.4)
                // A user_data set to u128::MAX is an indicator that this is a player
                .user_data(u128::MAX)
                .density(0.0)
                .build();

            let rigid_body_handle = rigid_body_set.insert(rigid_body);
            let collider_handle = collider_set.insert_with_parent(collider, rigid_body_handle, rigid_body_set);

            entity.insert(RigidBodyHandleWrapper(rigid_body_handle));
            entity.insert(ColliderHandleWrapper(collider_handle));

            if remaining_bots_to_add > 0 {
                let (bot, ability, model) = AggroBot::new(map, PlayerID(i));

                entity.insert_bundle(Gun::new(model, ability, perk));

                entity.insert(BotWrapper(Box::new(bot)));
                entity.insert(ability);

                remaining_bots_to_add -= 1;

                online_player_ids.insert(i, None);
                _deathmatch_score.0.insert(i, 0);
                local_players.push(i);
                // Bots get random names
                entity.insert(PlayerName::get_random_name());


            } else {
                available_player_ids.push(PlayerID(i));


            }

        }

        #[cfg(not(feature = "graphics"))]
        available_player_ids.push(PlayerID(i));

    });

    #[cfg(all(feature = "native", feature = "graphics"))]
    {
        if hosting.0 {
            let p_id = available_player_ids.remove(0);
            commands.insert_resource(MyPlayerID(Some(p_id)));

            online_player_ids.insert(p_id.0, None);
            _deathmatch_score.0.insert(p_id.0, 0);
            local_players.push(p_id.0);

        }
        
    }

    #[cfg(feature = "web")]
    commands.insert_resource(MyPlayerID(None));

    commands.insert_resource(OnlinePlayerIDs(online_player_ids));

    commands.insert_resource(available_player_ids);
    commands.insert_resource(player_entities);
    commands.insert_resource(LocalPlayers(local_players));

    commands.insert_resource(WidowMakerHeals(HashMap::with_capacity_and_hasher(10, BuildHasher::default())));
}

#[cfg(not(feature = "graphics"))]
pub struct AssetsLoading;

pub fn setup_physics(mut commands: Commands) {
    commands.insert_resource(RigidBodySet::new());
    commands.insert_resource(ColliderSet::new());
    commands.insert_resource(PhysicsPipeline::new());
    commands.insert_resource(IslandManager::new());
    commands.insert_resource(BroadPhase::new());
    commands.insert_resource(NarrowPhase::new());
    commands.insert_resource(JointSet::new());
    commands.insert_resource(CCDSolver::new());

}
