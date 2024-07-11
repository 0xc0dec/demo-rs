use std::collections::VecDeque;
use std::time::Instant;

pub struct FrameTime {
    pub delta: f32,
    queue: VecDeque<f32>,
    last_frame_instant: Instant,
}

const DT_FILTER_WIDTH: usize = 10;

impl FrameTime {
    pub fn new() -> Self {
        let queue = VecDeque::with_capacity(DT_FILTER_WIDTH);
        let last_frame_instant = Instant::now();

        Self {
            queue,
            last_frame_instant,
            delta: 0.0,
        }
    }

    pub fn advance(self) -> Self {
        // Stolen from Kajiya
        let now = Instant::now();
        let dt = (now - self.last_frame_instant).as_secs_f32();

        let mut queue = self.queue;
        if queue.len() >= DT_FILTER_WIDTH {
            queue.pop_front();
        }
        queue.push_back(dt);

        let delta = queue.iter().copied().sum::<f32>() / queue.len() as f32;

        Self {
            last_frame_instant: now,
            delta,
            queue,
        }
    }
}
