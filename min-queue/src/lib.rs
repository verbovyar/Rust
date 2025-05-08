#![forbid(unsafe_code)]

use std::collections::VecDeque;

#[derive(Default)]
pub struct MinQueue<T> {
    last_used_id: usize,
    min_window: VecDeque<(usize, T)>,
    window: VecDeque<(usize, T)>,
}

impl<T: Clone + Ord> MinQueue<T> {
    pub fn new() -> Self {
        Self {
            last_used_id: 0,
            min_window: VecDeque::<(usize, T)>::new(),
            window: VecDeque::<(usize, T)>::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        while !self.min_window.is_empty() && self.min_window.back().unwrap().1 >= val {
            self.min_window.pop_back();
        }
        self.last_used_id += 1;
        self.min_window.push_back((self.last_used_id, val.clone()));
        self.window.push_back((self.last_used_id, val));
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = self.window.pop_front()?;

        assert!(!self.min_window.is_empty());

        let min_window_head = self.min_window.front().unwrap();

        if head.0 == min_window_head.0 {
            self.min_window.pop_front();
        }

        Some(head.1)
    }

    pub fn front(&self) -> Option<&T> {
        let head = self.window.front()?;

        Some(&head.1)
    }

    pub fn min(&self) -> Option<&T> {
        let min_window_head = self.min_window.front()?;

        Some(&min_window_head.1)
    }

    pub fn len(&self) -> usize {
        self.window.len()
    }

    pub fn is_empty(&self) -> bool {
        self.window.is_empty()
    }
}
