use rapier2d::prelude::*;
use crate::home::types::Position;

/// Viewport for coordinate transformation between screen and physics space
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
    pub offset: Position,
    pub scale: f32,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            offset: Position::default(),
            scale: 1.0,
        }
    }
}

impl Viewport {
    pub fn new() -> Self {
        Self::default()
    }

    /// Convert screen coordinates to physics coordinates
    pub fn screen_to_physics(&self, screen_pos: &Position) -> Isometry<f32> {
        let world_x = (screen_pos.x - self.offset.x) / self.scale;
        let world_y = (screen_pos.y - self.offset.y) / self.scale;
        Isometry::new(vector![world_x, world_y], 0.0)
    }

    /// Convert physics coordinates to screen coordinates
    pub fn physics_to_screen(&self, physics_pos: &Isometry<f32>) -> Position {
        let screen_x = physics_pos.translation.x * self.scale + self.offset.x;
        let screen_y = physics_pos.translation.y * self.scale + self.offset.y;
        Position {
            x: screen_x,
            y: screen_y,
        }
    }

    /// Update viewport offset
    pub fn set_offset(&mut self, offset: Position) {
        self.offset = offset;
    }

    /// Update viewport scale
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale.max(0.1).min(10.0); // Clamp scale to reasonable bounds
    }

    /// Pan the viewport by a delta
    pub fn pan(&mut self, delta: Position) {
        self.offset.x += delta.x;
        self.offset.y += delta.y;
    }

    /// Zoom the viewport by a factor
    pub fn zoom(&mut self, factor: f32, center: Position) {
        let old_scale = self.scale;
        self.set_scale(self.scale * factor);
        
        // Adjust offset to zoom around the center point
        let scale_ratio = self.scale / old_scale;
        self.offset.x = center.x - (center.x - self.offset.x) * scale_ratio;
        self.offset.y = center.y - (center.y - self.offset.y) * scale_ratio;
    }
}