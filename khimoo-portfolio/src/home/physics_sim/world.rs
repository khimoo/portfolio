use rapier2d::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::home::types::{NodeId, NodeRegistry, ForceSettings, ContainerBound, Position};
use super::{Viewport, ForceCalculator, ForceApplicator, BodyManager, JointManager};

/// Main physics world that orchestrates the simulation
pub struct PhysicsWorld {
    gravity: Vector<f32>,
    integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    multibody_joints: MultibodyJointSet,
    ccd_solver: CCDSolver,
    
    // Managers
    body_manager: BodyManager,
    joint_manager: JointManager,
    
    // State
    node_registry: Rc<RefCell<NodeRegistry>>,
    force_settings: ForceSettings,
    container_bound: ContainerBound,
}

impl PhysicsWorld {
    pub fn new(
        node_registry: Rc<RefCell<NodeRegistry>>,
        viewport: &Viewport,
        force_settings: ForceSettings,
        container_bound: ContainerBound,
    ) -> Self {
        let mut body_manager = BodyManager::new();
        let mut joint_manager = JointManager::new();

        // Create bodies for all nodes
        body_manager.create_bodies_from_registry(Rc::clone(&node_registry), viewport);

        // Create joints for all edges
        joint_manager.create_joints_from_registry(
            Rc::clone(&node_registry),
            &body_manager.body_map,
            &force_settings,
        );

        Self {
            gravity: vector![0.0, 0.0],
            integration_parameters: IntegrationParameters::default(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            multibody_joints: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            body_manager,
            joint_manager,
            node_registry,
            force_settings,
            container_bound,
        }
    }

    /// Step the physics simulation
    pub fn step(&mut self, viewport: &Viewport) {
        let physics_hooks = ();
        let event_handler = ();

        self.integration_parameters.dt = 1.0 / 12.0;

        // Apply forces
        self.apply_all_forces(viewport);

        // Run physics step
        let mut pipeline = PhysicsPipeline::new();
        pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.body_manager.bodies,
            &mut self.body_manager.colliders,
            &mut self.joint_manager.impulse_joints,
            &mut self.multibody_joints,
            &mut self.ccd_solver,
            None,
            &physics_hooks,
            &event_handler,
        );

        // Update node positions from physics bodies
        self.body_manager.update_node_positions(Rc::clone(&self.node_registry), viewport);
    }

    /// Apply all forces to the simulation
    fn apply_all_forces(&mut self, viewport: &Viewport) {
        let dt = self.integration_parameters.dt;

        // Apply center force to author node only
        self.apply_center_force_to_author(viewport, dt);

        // Apply repulsion forces
        self.apply_repulsion_forces();

        // Apply category attraction forces
        self.apply_category_forces(dt);
    }

    /// Apply center force to the author node
    fn apply_center_force_to_author(&mut self, _viewport: &Viewport, dt: f32) {
        let (author_id, center_pos) = {
            let registry = self.node_registry.borrow();
            let author_id = registry.get_author_node_id();
            let center = Position {
                x: self.container_bound.x + self.container_bound.width / 2.0,
                y: self.container_bound.y + self.container_bound.height / 2.0,
            };
            (author_id, center)
        };

        if let Some(author_id) = author_id {
            if let Some(current_pos) = self.node_registry.borrow().positions.get(&author_id).copied() {
                if let Some(body) = self.body_manager.get_body_mut(author_id) {
                    ForceApplicator::apply_center_force(
                        body,
                        current_pos,
                        center_pos,
                        &self.force_settings,
                        dt,
                    );
                }
            }
        }
    }

    /// Apply repulsion forces between all nodes
    fn apply_repulsion_forces(&mut self) {
        let registry = self.node_registry.borrow();
        let forces = ForceCalculator::calculate_repulsion_forces(&registry, &self.force_settings);
        drop(registry);

        ForceApplicator::apply_repulsion_forces(
            &mut self.body_manager.bodies,
            &self.body_manager.body_map,
            forces,
        );
    }

    /// Apply category-based attraction forces
    fn apply_category_forces(&mut self, dt: f32) {
        let registry = self.node_registry.borrow();
        let forces = ForceCalculator::calculate_category_attraction_forces(
            &registry,
            &self.force_settings,
            dt,
        );
        drop(registry);

        ForceApplicator::apply_category_forces(
            &mut self.body_manager.bodies,
            &self.body_manager.body_map,
            forces,
        );
    }

    /// Update force settings
    pub fn update_force_settings(&mut self, new_settings: ForceSettings) {
        self.force_settings = new_settings;
        
        // Update joint strengths
        self.joint_manager.update_joint_strengths(
            Rc::clone(&self.node_registry),
            &self.body_manager.body_map,
            &self.force_settings,
        );
    }

    /// Update container bounds
    pub fn update_container_bound(&mut self, new_bound: ContainerBound) {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(
            &format!(
                "Updating container bound: ({}, {}, {}x{})",
                new_bound.x, new_bound.y, new_bound.width, new_bound.height
            ).into(),
        );
        self.container_bound = new_bound;
    }

    /// Set node position directly
    pub fn set_node_position(&mut self, id: NodeId, pos: &Position, viewport: &Viewport) {
        self.body_manager.set_node_position(id, pos, viewport);
    }

    /// Set node to kinematic mode
    pub fn set_node_kinematic(&mut self, id: NodeId) {
        self.body_manager.set_node_kinematic(id);
    }

    /// Set node to dynamic mode
    pub fn set_node_dynamic(&mut self, id: NodeId) {
        self.body_manager.set_node_dynamic(id);
    }

    /// Update node size
    pub fn update_node_size(&mut self, node_id: NodeId, new_radius: i32) {
        // Update registry
        {
            let mut registry = self.node_registry.borrow_mut();
            registry.update_node_radius(node_id, new_radius);
        }

        // Update physics collider
        let registry = self.node_registry.borrow();
        self.body_manager.update_node_size(
            node_id,
            new_radius,
            &registry,
            &mut self.island_manager,
        );
    }

    /// Update all node sizes based on article data
    pub fn update_all_node_sizes(&mut self, article_data: &HashMap<NodeId, (Option<u8>, usize)>) {
        let node_ids: Vec<NodeId> = {
            let registry = self.node_registry.borrow();
            registry.positions.keys().cloned().collect()
        };

        for node_id in node_ids {
            if let Some((importance, inbound_count)) = article_data.get(&node_id) {
                let new_radius = {
                    let registry = self.node_registry.borrow();
                    registry.calculate_dynamic_radius(node_id, *importance, *inbound_count)
                };
                self.update_node_size(node_id, new_radius);
            }
        }
    }

    /// Enable/disable category clustering
    pub fn set_category_clustering_enabled(&mut self, enabled: bool) {
        self.force_settings.enable_category_clustering = enabled;
    }

    /// Set debug mode
    pub fn set_debug_mode(&mut self, debug_mode: bool) {
        self.force_settings.debug_mode = debug_mode;

        let mut registry = self.node_registry.borrow_mut();
        registry.set_connection_line_visibility(
            debug_mode && self.force_settings.show_connection_lines,
        );
    }

    /// Set connection lines visibility
    pub fn set_connection_lines_visible(&mut self, visible: bool) {
        self.force_settings.show_connection_lines = visible;

        let mut registry = self.node_registry.borrow_mut();
        registry.set_connection_line_visibility(visible && self.force_settings.debug_mode);
    }
}