use crate::property::Region;

// Rust std::collections::BinaryHeap has some limitations
pub(super) struct RegionHeap<T: Region> {
    pub(super) data: Vec<T>,
}

impl<T: Region> Default for RegionHeap<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl<T: Region> RegionHeap<T> {
    fn adjust_down(&mut self, mut node: usize) {
        let len = self.data.len();
        let node_key = self.data[node].end();
        while node < len {
            let mut candidate = node;
            let mut candidate_key = node_key;
            for child_id in 0..2 {
                let child_node = child_id + 1 + node * 2;
                if let Some(child_data) = self.data.get(child_node) {
                    if candidate_key > child_data.end() {
                        candidate_key = child_data.end();
                        candidate = child_node;
                    }
                }
            }
            if candidate != node {
                self.data.swap(candidate, node);
                node = candidate;
            } else {
                break;
            }
        }
    }

    fn adjust_up(&mut self, mut node: usize) {
        let node_key = self.data[node].end();
        while node > 0 && self.data[node / 2].end() > node_key {
            self.data.swap(node / 2, node);
            node /= 2;
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut ret = self.data.pop()?;
        if !self.data.is_empty() {
            std::mem::swap(&mut self.data[0], &mut ret);
            self.adjust_down(0);
        }
        Some(ret)
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
        self.adjust_up(self.data.len() - 1);
    }

    pub fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}
