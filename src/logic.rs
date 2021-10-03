use game_types::RequestedMovement;
use rapier2d::prelude::*;
use rapier2d::na::Vector2;
use bevy::prelude::*;

use bevy_networking_turbulence::NetworkResource;

pub fn move_objects(mut physics_pipeline: ResMut<PhysicsPipeline>, mut island_manager: ResMut<IslandManager>, mut broad_phase: ResMut<BroadPhase>, mut narrow_phase: ResMut<NarrowPhase>, mut joint_set: ResMut<JointSet>, mut ccd_solver: ResMut<CCDSolver>, mut rigid_body_set: ResMut<RigidBodySet>, mut collider_set: ResMut<ColliderSet>, mut net: ResMut<NetworkResource>, mut movable_objects: Query<(&RigidBodyHandle, &mut Transform)>) {
    // Update the locations of all rigid bodies to their graphics counterparts
    movable_objects.iter_mut().for_each(|(rigid_body_handle, mut transform)| {
        let rigid_body = rigid_body_set.get(*rigid_body_handle).unwrap();
        let rigid_body_translation = rigid_body.translation().component_mul(&Vector2::new(250.0, 250.0));

        transform.translation = Vec3::new(rigid_body_translation.x, rigid_body_translation.y, transform.translation.z);

    });

    const GRAVITY: Vector2<f32> = Vector2::new(0.0, 0.0);
    const INTEGRATION_PARAMETERS: IntegrationParameters = IntegrationParameters {
        dt: 1.0 / 60.0,
        min_ccd_dt: 1.0 / 60.0 / 100.0,
        erp: 0.2,
        joint_erp: 0.2,
        velocity_solve_fraction: 1.0,
        velocity_based_erp: 0.0,
        warmstart_coeff: 1.0,
        warmstart_correction_slope: 10.0,
        allowed_linear_error: 0.005,
        prediction_distance: 0.002,
        allowed_angular_error: 0.001,
        max_linear_correction: 0.2,
        max_angular_correction: 0.2,
        max_velocity_iterations: 4,
        max_position_iterations: 1,
        min_island_size: 128,
        max_ccd_substeps: 20,
    };

    physics_pipeline.step(
        &GRAVITY,
        &INTEGRATION_PARAMETERS,
        &mut island_manager,
        &mut broad_phase,
        &mut narrow_phase,
        &mut rigid_body_set,
        &mut collider_set,
        &mut joint_set,
        &mut ccd_solver,
        &(),
        &()
    );

}
