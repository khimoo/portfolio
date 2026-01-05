use rapier2d::prelude::*;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use crate::home::types::{NodeId, NodeRegistry, ForceSettings};

/// Manager for joints (springs) between nodes
pub struct JointManager {
    pub impulse_joints: ImpulseJointSet,
    pub edge_joint_handles: Vec<ImpulseJointHandle>,
}

impl JointManager {
    pub fn new() -> Self {
        Self {
            impulse_joints: ImpulseJointSet::new(),
            edge_joint_handles: Vec::new(),
        }
    }

    /// Create joints for all edges in the node registry
    pub fn create_joints_from_registry(
        &mut self,
        node_registry: Rc<RefCell<NodeRegistry>>,
        body_map: &HashMap<NodeId, RigidBodyHandle>,
        force_settings: &ForceSettings,
    ) {
        let registry = node_registry.borrow();
        
        for (from, to) in &registry.edges {
            if let (Some(&handle_a), Some(&handle_b)) = (body_map.get(from), body_map.get(to)) {
                self.create_spring_joint(handle_a, handle_b, force_settings);
            }
        }
    }

    /// Create a spring joint between two bodies
    pub fn create_spring_joint(
        &mut self,
        handle_a: RigidBodyHandle,
        handle_b: RigidBodyHandle,
        force_settings: &ForceSettings,
    ) {
        let joint_params = SpringJointBuilder::new(
            0.0, // Natural length
            force_settings.link_strength,
            force_settings.direct_link_damping,
        )
        .local_anchor1(point![0.0, 0.0])
        .local_anchor2(point![0.0, 0.0])
        .build();
        
        let handle = self.impulse_joints.insert(handle_a, handle_b, joint_params, true);
        self.edge_joint_handles.push(handle);
    }

    /// Update joint strengths based on new force settings
    pub fn update_joint_strengths(
        &mut self,
        node_registry: Rc<RefCell<NodeRegistry>>,
        body_map: &HashMap<NodeId, RigidBodyHandle>,
        force_settings: &ForceSettings,
    ) {
        // Remove existing joints
        for handle in &self.edge_joint_handles {
            self.impulse_joints.remove(*handle, true);
        }
        self.edge_joint_handles.clear();

        // Recreate joints with new settings
        self.create_joints_from_registry(node_registry, body_map, force_settings);
    }

    /// Remove all joints
    pub fn clear_joints(&mut self) {
        for handle in &self.edge_joint_handles {
            self.impulse_joints.remove(*handle, true);
        }
        self.edge_joint_handles.clear();
    }

    /// Get joint count
    pub fn joint_count(&self) -> usize {
        self.edge_joint_handles.len()
    }
}

impl Default for JointManager {
    fn default() -> Self {
        Self::new()
    }
}