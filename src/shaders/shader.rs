use crate::device::Frame;

pub trait Shader<'a, 'b> where 'a: 'b {
    fn apply(&'a mut self, frame: &mut Frame<'b>);
}
