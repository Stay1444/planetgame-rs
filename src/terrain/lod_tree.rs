use bevy_math::Rect;

#[derive(Default, Clone, Debug)]
pub struct LODTree {
    depth: usize,
    boundary: Rect,
    max_depth: usize,
    children: Option<Box<[LODTree; 4]>>,
}

impl LODTree {
    pub fn new(max_depth: usize, boundary: Rect) -> Self {
        LODTree {
            depth: 0,
            max_depth,
            boundary,
            children: None,
        }
    }

    pub fn boundary(&self) -> Rect {
        self.boundary
    }

    pub fn children(&self) -> Option<&Box<[LODTree; 4]>> {
        self.children.as_ref()
    }

    pub fn children_mut(&mut self) -> Option<&mut Box<[LODTree; 4]>> {
        self.children.as_mut()
    }

    fn new_child(boundary: Rect, max_depth: usize, depth: usize) -> Self {
        Self {
            boundary,
            depth,
            max_depth,
            children: None,
        }
    }

    pub fn subdivide(&mut self) -> bool {
        if self.depth >= self.max_depth || self.children.is_some() {
            return false;
        }

        let rects = subdivide_rect(self.boundary);
        self.children = Some(Box::new([
            LODTree::new_child(rects.0, self.max_depth, self.depth + 1),
            LODTree::new_child(rects.1, self.max_depth, self.depth + 1),
            LODTree::new_child(rects.2, self.max_depth, self.depth + 1),
            LODTree::new_child(rects.3, self.max_depth, self.depth + 1),
        ]));

        return true;
    }

    pub fn clear(&mut self) {
        self.children = None;
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
