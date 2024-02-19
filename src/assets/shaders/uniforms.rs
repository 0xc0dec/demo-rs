use crate::math::{Mat4, OPENGL_TO_WGPU_MATRIX};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct WorldViewProjUniform {
    world: [[f32; 4]; 4],
    view_proj: [[f32; 4]; 4],
}

impl WorldViewProjUniform {
    pub fn new() -> Self {
        Self {
            world: Mat4::identity().into(),
            view_proj: Mat4::identity().into(),
        }
    }

    pub fn update(&mut self, world: &Mat4, view: &Mat4, proj: &Mat4) {
        self.world = (*world).into();
        self.view_proj = (OPENGL_TO_WGPU_MATRIX * proj * view).into();
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ViewInvProjUniform {
    // Couldn't make it work with Matrix3, probably something to do with alignment and padding
    view_mat: [[f32; 4]; 4],
    proj_mat_inv: [[f32; 4]; 4],
}

impl ViewInvProjUniform {
    pub fn new() -> Self {
        Self {
            view_mat: Mat4::identity().into(),
            proj_mat_inv: Mat4::identity().into(),
        }
    }

    pub fn update(&mut self, view: &Mat4, proj: &Mat4) {
        self.view_mat = (*view).into();
        self.proj_mat_inv = (OPENGL_TO_WGPU_MATRIX * proj).try_inverse().unwrap().into();
    }
}
