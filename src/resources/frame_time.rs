use std::collections::VecDeque;
use std::time::Instant;

use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct FrameTime {
    pub delta: f32,
    queue: VecDeque<f32>,
    last_frame_instant: Instant,
}

impl FrameTime {
    const DT_FILTER_WIDTH: usize = 10;

    pub fn new() -> Self {
        let queue = VecDeque::with_capacity(Self::DT_FILTER_WIDTH);
        let last_frame_instant = Instant::now();

        Self {
            queue,
            last_frame_instant,
            delta: 0.0,
        }
    }

    pub fn update(&mut self) {
        // Stolen from Kajiya
        let now = Instant::now();
        let dt = now - self.last_frame_instant;
        self.last_frame_instant = now;

        let dt = dt.as_secs_f32();

        if self.queue.len() >= FrameTime::DT_FILTER_WIDTH {
            self.queue.pop_front();
        }
        self.queue.push_back(dt);

        self.delta = self.queue.iter().copied().sum::<f32>() / self.queue.len() as f32;
    }
}
