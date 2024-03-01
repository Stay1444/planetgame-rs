use bevy::ecs::entity::Entity;
use bevy_math::{Rect, Vec2};

use super::resources::LODSettings;

#[derive(Default, Clone, Debug)]
pub struct LODTree {
    pub depth: usize,
    pub boundary: Rect,
    pub max_depth: usize,
    pub leaf: LODLeaf,
}

#[derive(Default, Clone, Debug)]
pub enum LODLeaf {
    Children(Box<[LODTree; 4]>),
    Chunk(Entity),
    #[default]
    Pending,
}

impl LODTree {
    pub fn new(max_depth: usize, boundary: Rect) -> Self {
        LODTree {
            depth: 0,
            max_depth,
            boundary,
            leaf: LODLeaf::Pending,
        }
    }

    fn new_child(boundary: Rect, max_depth: usize, depth: usize) -> Self {
        Self {
            boundary,
            depth,
            max_depth,
            leaf: LODLeaf::Pending,
        }
    }

    pub fn collapse(&mut self) -> bool {
        if self.depth >= self.max_depth {
            return false;
        }

        if let LODLeaf::Children(_) = self.leaf {
            return false;
        }

        let rects = subdivide_rect(self.boundary);
        self.leaf = LODLeaf::Children(Box::new([
            LODTree::new_child(rects.0, self.max_depth, self.depth + 1),
            LODTree::new_child(rects.1, self.max_depth, self.depth + 1),
            LODTree::new_child(rects.2, self.max_depth, self.depth + 1),
            LODTree::new_child(rects.3, self.max_depth, self.depth + 1),
        ]));

        return true;
    }

    pub fn should_collapse(&self, settings: &LODSettings, point: Vec2) -> bool {
        let distance = f32::max(
            settings.max + -(self.depth as f32) * settings.layer_penalty,
            settings.min,
        );

        self.boundary.center().distance_squared(point) / (self.boundary.size().length() * 100.0)
            < distance
    }

    pub fn can_collapse(&self) -> bool {
        if self.depth >= self.max_depth {
            return false;
        }

        if let LODLeaf::Children(_) = self.leaf {
            return false;
        }

        return true;
    }

    pub fn get_child_chunks_recursive(&self, out: &mut Vec<Entity>) {
        match &self.leaf {
            LODLeaf::Children(children) => {
                for child in children.iter() {
                    child.get_child_chunks_recursive(out);
                }
            }
            LODLeaf::Chunk(entity) => {
                out.push(entity.clone());
            }
            LODLeaf::Pending => (),
        }
    }
}

fn subdivide_rect(rect: Rect) -> (Rect, Rect, Rect, Rect) {
    (
        Rect::new(
            rect.min.x,
            rect.min.y,
            rect.min.x + (rect.max.x - rect.min.x) / 2.0,
            rect.min.y + (rect.max.y - rect.min.y) / 2.0,
        ),
        Rect::new(
            rect.min.x + (rect.max.x - rect.min.x) / 2.0,
            rect.min.y,
            rect.max.x,
            rect.min.y + (rect.max.y - rect.min.y) / 2.0,
        ),
        Rect::new(
            rect.min.x,
            rect.min.y + (rect.max.y - rect.min.y) / 2.0,
            rect.min.x + (rect.max.x - rect.min.x) / 2.0,
            rect.max.y,
        ),
        Rect::new(
            rect.min.x + (rect.max.x - rect.min.x) / 2.0,
            rect.min.y + (rect.max.y - rect.min.y) / 2.0,
            rect.max.x,
            rect.max.y,
        ),
    )
}
