use rapier2d::prelude::*;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use crate::home::types::{NodeId, NodeRegistry, Position};
use super::viewport::Viewport;

/// Manager for rigid bodies in the physics simulation
pub struct BodyManager {
    pub bodies: RigidBodySet,
    pub colliders: ColliderSet,
    pub body_map: HashMap<NodeId, RigidBodyHandle>,
}

impl BodyManager {
    pub fn new() -> Self {
        Self {
            bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            body_map: HashMap::new(),
        }
    }

    /// Create rigid bodies for all nodes in the registry
    pub fn create_bodies_from_registry(
        &mut self,
        node_registry: Rc<RefCell<NodeRegistry>>,
        viewport: &Viewport,
    ) {
        let registry = node_registry.borrow();
        
        for (id, pos) in &registry.positions {
            self.create_body_for_node(*id, *pos, &registry, viewport);
        }
    }

    /// Create a rigid body for a specific node
    pub fn create_body_for_node(
        &mut self,
        node_id: NodeId,
        position: Position,
        registry: &NodeRegistry,
        viewport: &Viewport,
    ) {
        // Create dynamic rigid body
        let rigid_body = RigidBodyBuilder::dynamic()
            .linear_damping(3.0)
            .angular_damping(6.0)
            .position(viewport.screen_to_physics(&position))
            .build();
        
        let handle = self.bodies.insert(rigid_body);

        // Create collider with physics radius
        let physics_radius = registry.calculate_physics_radius(node_id);
        let restitution = if registry.is_author_node(node_id) { 0.3 } else { 0.7 };
        
        let collider = ColliderBuilder::ball(physics_radius)
            .restitution(restitution)
            .build();
        
        self.colliders.insert_with_parent(collider, handle, &mut self.bodies);
        self.body_map.insert(node_id, handle);
    }

    /// Update node positions from physics bodies
    pub fn update_node_positions(
        &self,
        node_registry: Rc<RefCell<NodeRegistry>>,
        viewport: &Viewport,
    ) {
        let mut registry = node_registry.borrow_mut();
        
        for (id, handle) in &self.body_map {
            if let Some(body) = self.bodies.get(*handle) {
                if let Some(pos) = registry.positions.get_mut(id) {
                    *pos = viewport.physics_to_screen(body.position());
                }
            }
        }
    }

    /// Set a node to kinematic mode (for dragging)
    pub fn set_node_kinematic(&mut self, node_id: NodeId) {
        if let Some(&handle) = self.body_map.get(&node_id) {
            if let Some(body) = self.bodies.get_mut(handle) {
                body.set_body_type(RigidBodyType::KinematicPositionBased, true);
            }
        }
    }

    /// Set a node to dynamic mode (normal physics)
    pub fn set_node_dynamic(&mut self, node_id: NodeId) {
        if let Some(&handle) = self.body_map.get(&node_id) {
            if let Some(body) = self.bodies.get_mut(handle) {
                body.set_body_type(RigidBodyType::Dynamic, true);
            }
        }
    }

    /// Set node position directly
    pub fn set_node_position(&mut self, node_id: NodeId, pos: &Position, viewport: &Viewport) {
        if let Some(&handle) = self.body_map.get(&node_id) {
            if let Some(body) = self.bodies.get_mut(handle) {
                body.set_position(viewport.screen_to_physics(pos), true);
            }
        }
    }

    /// Update node size (collider)
    pub fn update_node_size(
        &mut self,
        node_id: NodeId,
        _new_radius: i32,
        registry: &NodeRegistry,
        island_manager: &mut IslandManager,
    ) {
        if let Some(&body_handle) = self.body_map.get(&node_id) {
            // Find and remove the old collider
            let mut collider_handle_to_remove = None;
            for (collider_handle, collider) in self.colliders.iter() {
                if collider.parent() == Some(body_handle) {
                    collider_handle_to_remove = Some(collider_handle);
                    break;
                }
            }

            // Remove old collider and add new one with updated size
            if let Some(old_collider_handle) = collider_handle_to_remove {
                self.colliders.remove(
                    old_collider_handle,
                    island_manager,
                    &mut self.bodies,
                    true,
                );

                // Create new collider with updated radius
                let physics_radius = registry.calculate_physics_radius(node_id);
                let restitution = if registry.is_author_node(node_id) { 0.3 } else { 0.7 };

                let collider = ColliderBuilder::ball(physics_radius)
                    .restitution(restitution)
                    .build();
                
                self.colliders.insert_with_parent(collider, body_handle, &mut self.bodies);
            }
        }
    }

    /// Get body handle for a node
    pub fn get_body_handle(&self, node_id: NodeId) -> Option<RigidBodyHandle> {
        self.body_map.get(&node_id).copied()
    }

    /// Get body for a node
    pub fn get_body(&self, node_id: NodeId) -> Option<&RigidBody> {
        self.body_map.get(&node_id)
            .and_then(|&handle| self.bodies.get(handle))
    }

    /// Get mutable body for a node
    pub fn get_body_mut(&mut self, node_id: NodeId) -> Option<&mut RigidBody> {
        self.body_map.get(&node_id)
            .and_then(|&handle| self.bodies.get_mut(handle))
    }
}

impl Default for BodyManager {
    fn default() -> Self {
        Self::new()
    }
}