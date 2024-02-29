use bevy_math::{Rect, Vec2};

#[derive(Default, Clone, Debug)]
pub struct QuadTree {
    depth: usize,
    boundary: Rect,
    max_depth: usize,
    children: Option<Box<[QuadTree; 4]>>,

    player: bool,
}

impl QuadTree {
    pub fn new(max_depth: usize, boundary: Rect) -> Self {
        QuadTree {
            depth: 0,
            max_depth,
            boundary,
            children: None,
            player: false,
        }
    }

    fn new_child(boundary: Rect, max_depth: usize, depth: usize) -> Self {
        Self {
            boundary,
            depth,
            max_depth,
            children: None,
            player: false,
        }
    }

    fn subdivide(&mut self) {
        let rects = subdivide_rect(self.boundary);
        self.children = Some(Box::new([
            QuadTree::new_child(rects.0, self.max_depth, self.depth + 1),
            QuadTree::new_child(rects.1, self.max_depth, self.depth + 1),
            QuadTree::new_child(rects.2, self.max_depth, self.depth + 1),
            QuadTree::new_child(rects.3, self.max_depth, self.depth + 1),
        ]));
    }

    pub fn insert(&mut self, position: Vec2) -> bool {
        if !self.boundary.contains(position) {
            return false;
        }

        if self.depth >= self.max_depth {
            self.player = true;
            return true;
        }

        if self.children.is_none() {
            self.subdivide();
        }

        let children = self.children.as_mut().unwrap();

        for child in children.iter_mut() {
            if child.insert(position) {
                return true;
            }
        }

        return false;
    }
}

fn subdivide_rect(rect: Rect) -> (Rect, Rect, Rect, Rect) {
    (
        Rect::new(
            rect.min.x - rect.max.x / 4.0,
            rect.min.y - rect.min.y / 4.0,
            rect.max.x / 2.0,
            rect.max.y / 2.0,
        ),
        Rect::new(
            rect.min.x + rect.max.x / 4.0,
            rect.min.y - rect.min.y / 4.0,
            rect.max.x / 2.0,
            rect.max.y / 2.0,
        ),
        Rect::new(
            rect.min.x - rect.max.x / 4.0,
            rect.min.y + rect.min.y / 4.0,
            rect.max.x / 2.0,
            rect.max.y / 2.0,
        ),
        Rect::new(
            rect.min.x + rect.max.x / 4.0,
            rect.min.y + rect.min.y / 4.0,
            rect.max.x / 2.0,
            rect.max.y / 2.0,
        ),
    )
}
