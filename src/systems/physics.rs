use voxelviewer::view_system::components::PositionComponent;
use specs::prelude::*;

use nalgebra::{Vector3, Point3};
use specs::{System, WriteStorage, Component,ReadStorage, Entities, VecStorage, Entity, WriteExpect};
use rapier3d::prelude::*;
use specs::Join;


pub struct PhysicsWorldResource{
    // rigid_body_set: RigidBodySet,
    // collider_set: ColliderSet,

    gravity: Vector3<f32>,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,

    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,

    ccd_solver: CCDSolver,
}

impl PhysicsWorldResource {
    pub fn new() -> Self {
        let gravity = vector![0.0, -9.81, 0.0];
        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = BroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();
    

        Self{
            gravity,
            integration_parameters,
            physics_pipeline,
            island_manager,
            broad_phase,
            narrow_phase,
            impulse_joint_set,
            multibody_joint_set,
            ccd_solver
        }
    } 
    
    fn step(&mut self, 
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet
    ) {
        let physics_hooks = ();
        let event_handler = ();

        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            rigid_body_set,
            collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            &physics_hooks,
            &event_handler,
          );
    }
}

pub struct RigidBodyComponent(pub RigidBodyHandle);
impl Component for RigidBodyComponent { type Storage = VecStorage<Self>;}
pub struct VelocityComponent(pub Vector3<f32>);
impl Component for VelocityComponent { type Storage = VecStorage<Self>;}

pub struct PhysicsSystem;
impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (
        WriteExpect<'a, PhysicsWorldResource>,
        WriteExpect<'a, RigidBodySet>,
        WriteExpect<'a, ColliderSet>,
        WriteStorage<'a, VelocityComponent>,
        WriteStorage<'a, PositionComponent>,
        ReadStorage<'a, RigidBodyComponent>
    );

    fn run(&mut self, (mut physics_world, mut rb_set, mut coll_set, mut vel_s, mut pos_s, rb_s): Self::SystemData) {
        PhysicsSystem::sync_physics(&mut rb_set, &mut vel_s, &mut pos_s, &rb_s);
        physics_world.step(&mut rb_set, &mut coll_set);
        PhysicsSystem::sync_ecs(&rb_set, &mut vel_s, &mut pos_s, &rb_s);
    }
}

impl PhysicsSystem {
    pub fn new() -> Self {
        Self{}
    }

    pub fn sync_physics(rb_set: &mut WriteExpect<RigidBodySet>, vel_s: &mut WriteStorage<VelocityComponent>, pos_s: &mut WriteStorage<PositionComponent>, rb_s: &ReadStorage<RigidBodyComponent>) {
        for (vel, rb) in (vel_s, rb_s).join() {
            let phy_rigid = rb_set.get_mut(rb.0).unwrap();
            let wake = vel.0 != Vector3::zeros();
            phy_rigid.set_linvel(vel.0, wake);
        }
        for (pos, rb) in (pos_s, rb_s).join() {
            let phy_rigid = rb_set.get_mut(rb.0).unwrap();
            phy_rigid.set_translation(pos.0.coords, false);
        }
    }

    pub fn sync_ecs(rb_set: &WriteExpect<RigidBodySet>, vel_s: &mut WriteStorage<VelocityComponent>, pos_s: &mut WriteStorage<PositionComponent>, rb_s: &ReadStorage<RigidBodyComponent>) {
        for (vel, rb) in (vel_s, rb_s).join() {
            let phy_rigid_body = rb_set.get(rb.0).unwrap();
            vel.0 = *phy_rigid_body.linvel();
        }
        for (pos, rb) in (pos_s, rb_s).join() {
            let phy_rigid_body = rb_set.get(rb.0).unwrap();
            pos.0 = Point3::from(phy_rigid_body.position().translation.vector);
        }
    }
}

// **************** MANAGER
#[derive(Default)]
pub struct AddRigidBodyCubeFlag(pub f32);
impl Component for AddRigidBodyCubeFlag { type Storage = HashMapStorage<Self>;}

pub struct PhysicsWorldManagerSystem;
impl<'a> System<'a> for PhysicsWorldManagerSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, RigidBodySet>,
        WriteExpect<'a, ColliderSet>,
        ReadStorage<'a, PositionComponent>,
        WriteStorage<'a, RigidBodyComponent>,
        WriteStorage<'a, AddRigidBodyCubeFlag>,
    );

    fn run(&mut self, (ents, mut rb_set, mut coll_set, pos_s, mut rb_s, mut arb_cube_s): Self::SystemData) {
        PhysicsWorldManagerSystem::add_rigid_body_cube(&ents, &mut rb_set, &mut coll_set, &pos_s, &mut rb_s, &mut arb_cube_s);
    }
} 

impl PhysicsWorldManagerSystem {
    pub fn add_rigid_body_cube(
        ents: &Entities, 
        rb_set: &mut WriteExpect<RigidBodySet>, 
        coll_set: &mut WriteExpect<ColliderSet>, 
        pos_s: &ReadStorage<PositionComponent>, 
        rb_s: &mut WriteStorage<RigidBodyComponent>, 
        arb_cube_s: &mut WriteStorage<AddRigidBodyCubeFlag>
    ) {
        let mut ents_added:Vec<Entity> = Vec::new();
        for (ent, pos, rb_f) in (ents, pos_s, &*arb_cube_s).join(){
           
            let rb = RigidBodyBuilder::dynamic()
                .translation(pos.0.coords)
                .build();
            let collider = ColliderBuilder::cuboid(
                rb_f.0, rb_f.0, rb_f.0,
            ).build();

            let rb_handle = rb_set.insert(rb);
            coll_set.insert_with_parent(
                collider,
                rb_handle.clone(),
                rb_set
            );

            rb_s.insert(ent, RigidBodyComponent(rb_handle)).unwrap();
            ents_added.push(ent);
        }
        for ent in ents_added {
            arb_cube_s.remove(ent);
        }
    }
}