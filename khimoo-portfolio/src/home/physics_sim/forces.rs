use rapier2d::prelude::*;
use std::collections::HashMap;
use crate::home::types::{NodeId, Position, ForceSettings, NodeRegistry};

/// Calculator for various physics forces
pub struct ForceCalculator;

impl ForceCalculator {
    /// Calculate center force for a body
    pub fn calculate_center_force(
        current_pos: Position,
        center_pos: Position,
        velocity: Vector<f32>,
        force_settings: &ForceSettings,
        dt: f32,
    ) -> Vector<f32> {
        let dx = center_pos.x - current_pos.x;
        let dy = center_pos.y - current_pos.y;

        let fx = force_settings.center_strength * dx - force_settings.center_damping * velocity.x;
        let fy = force_settings.center_strength * dy - force_settings.center_damping * velocity.y;

        vector![fx * dt, fy * dt]
    }

    /// Calculate repulsion forces between all nodes
    pub fn calculate_repulsion_forces(
        registry: &NodeRegistry,
        force_settings: &ForceSettings,
    ) -> HashMap<NodeId, Vector<f32>> {
        let mut forces = HashMap::new();

        for (id1, pos1) in &registry.positions {
            for (id2, pos2) in &registry.positions {
                if id1 == id2 {
                    continue;
                }

                let dx = pos2.x - pos1.x;
                let dy = pos2.y - pos1.y;
                let distance = ((dx * dx + dy * dy) as f32).sqrt();

                if distance < 1.0 {
                    continue;
                }

                let radius1 = registry.radii.get(id1).copied().unwrap_or(30) as f32;
                let radius2 = registry.radii.get(id2).copied().unwrap_or(30) as f32;
                
                let base_min_distance = if registry.is_author_node(*id1) || registry.is_author_node(*id2) {
                    force_settings.author_repulsion_min_distance
                } else {
                    force_settings.repulsion_min_distance
                };
                
                let min_distance = radius1 + radius2 + base_min_distance;

                if distance < min_distance {
                    let force_magnitude = force_settings.repulsion_strength
                        * (min_distance - distance)
                        / min_distance;

                    let force_x = (dx as f32 / distance) * force_magnitude;
                    let force_y = (dy as f32 / distance) * force_magnitude;

                    // Apply opposite forces to each node
                    let force1 = forces.entry(*id1).or_insert(vector![0.0, 0.0]);
                    force1.x -= force_x;
                    force1.y -= force_y;

                    let force2 = forces.entry(*id2).or_insert(vector![0.0, 0.0]);
                    force2.x += force_x;
                    force2.y += force_y;
                }
            }
        }

        forces
    }

    /// Calculate category-based attraction forces
    pub fn calculate_category_attraction_forces(
        registry: &NodeRegistry,
        force_settings: &ForceSettings,
        dt: f32,
    ) -> HashMap<NodeId, Vector<f32>> {
        let mut forces = HashMap::new();

        if !force_settings.enable_category_clustering {
            return forces;
        }

        let categories = registry.get_all_categories();

        for category in categories {
            let nodes_in_category = registry.get_nodes_by_category(&category);

            for i in 0..nodes_in_category.len() {
                for j in (i + 1)..nodes_in_category.len() {
                    let node1 = nodes_in_category[i];
                    let node2 = nodes_in_category[j];

                    // Skip author nodes
                    if registry.is_author_node(node1) || registry.is_author_node(node2) {
                        continue;
                    }

                    if let (Some(pos1), Some(pos2)) = (
                        registry.positions.get(&node1),
                        registry.positions.get(&node2),
                    ) {
                        let dx = pos2.x - pos1.x;
                        let dy = pos2.y - pos1.y;
                        let distance = (dx * dx + dy * dy).sqrt();

                        if distance > 0.0 && distance < force_settings.category_attraction_range {
                            let force_magnitude = force_settings.category_attraction_strength
                                / (distance + 50.0);

                            let fx = (dx / distance) * force_magnitude * dt;
                            let fy = (dy / distance) * force_magnitude * dt;

                            let force1 = forces.entry(node1).or_insert(vector![0.0, 0.0]);
                            force1.x += fx;
                            force1.y += fy;

                            let force2 = forces.entry(node2).or_insert(vector![0.0, 0.0]);
                            force2.x -= fx;
                            force2.y -= fy;
                        }
                    }
                }
            }
        }

        forces
    }
}

/// Applicator for physics forces to rigid bodies
pub struct ForceApplicator;

impl ForceApplicator {
    /// Apply center force to a specific body
    pub fn apply_center_force(
        body: &mut RigidBody,
        current_pos: Position,
        center_pos: Position,
        force_settings: &ForceSettings,
        dt: f32,
    ) {
        let velocity = body.linvel();
        let impulse = ForceCalculator::calculate_center_force(
            current_pos,
            center_pos,
            *velocity,
            force_settings,
            dt,
        );
        body.apply_impulse(impulse, true);
    }

    /// Apply repulsion forces to all bodies
    pub fn apply_repulsion_forces(
        bodies: &mut RigidBodySet,
        body_map: &HashMap<NodeId, RigidBodyHandle>,
        forces: HashMap<NodeId, Vector<f32>>,
    ) {
        for (node_id, force) in forces {
            if let Some(&handle) = body_map.get(&node_id) {
                if let Some(body) = bodies.get_mut(handle) {
                    body.apply_impulse(force, true);
                }
            }
        }
    }

    /// Apply category attraction forces to all bodies
    pub fn apply_category_forces(
        bodies: &mut RigidBodySet,
        body_map: &HashMap<NodeId, RigidBodyHandle>,
        forces: HashMap<NodeId, Vector<f32>>,
    ) {
        for (node_id, force) in forces {
            if let Some(&handle) = body_map.get(&node_id) {
                if let Some(body) = bodies.get_mut(handle) {
                    body.apply_impulse(force, true);
                }
            }
        }
    }
}