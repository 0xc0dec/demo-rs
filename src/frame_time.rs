use std::collections::VecDeque;
use std::time::Instant;

pub struct FrameTime {
    queue: VecDeque<f32>,
    prev_time: Instant,
}

const DT_FILTER_WIDTH: usize = 10;

impl FrameTime {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::with_capacity(DT_FILTER_WIDTH),
            prev_time: Instant::now(),
        }
    }

    pub fn advance(&mut self) -> f32 {
        // Stolen from Kajiya
        let now = Instant::now();
        let dt = (now - self.prev_time).as_secs_f32();
        self.prev_time = now;

        if self.queue.len() >= DT_FILTER_WIDTH {
            self.queue.pop_front();
        }
        self.queue.push_back(dt);

        self.queue.iter().copied().sum::<f32>() / self.queue.len() as f32
    }
}
