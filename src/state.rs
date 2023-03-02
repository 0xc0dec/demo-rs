use std::iter;
use cgmath::{Vector3};
use crate::camera::Camera;
use crate::input::Input;
use crate::material::{Material, MaterialParams, RenderMaterial};
use crate::model::{self, DrawModel, load_model};
use crate::renderer::Renderer;
use crate::texture::Texture;

pub struct State {
    material: Material,
    camera: Camera,
    obj_model: model::Model,
    depth_texture: Texture,
}

impl State {
    pub async fn new(renderer: &Renderer) -> State {
        let texture = Texture::from_file("cube-diffuse.jpg", renderer).await.unwrap();

        let material = Material::diffuse(renderer, MaterialParams {
            texture
        }).await;

        let obj_model = load_model("cube.obj", renderer).await.unwrap();

        let camera = Camera::new(
            Vector3::new(5.0, 5.0, 5.0),
            Vector3::new(0.0, 0.0, 0.0),
            renderer.canvas_size().into()
        );

        let depth_texture = Texture::new_depth_texture(renderer);

        Self {
            material,
            camera,
            obj_model,
            depth_texture
        }
    }

    pub fn update(&mut self, input: &Input, dt: f32) {
        self.camera.update(input, dt);
    }

    pub fn resize(&mut self, renderer: &Renderer) {
        self.depth_texture = Texture::new_depth_texture(renderer);
    }

    // TODO Don't pass Renderer
    pub fn render(&mut self, renderer: &Renderer) -> Result<(), wgpu::SurfaceError> {
        let output = renderer.surface().get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = renderer.device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: None
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.5,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    }
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                })
            });

            render_pass.apply_material(renderer, &mut self.material, &self.camera);
            render_pass.draw_model(&self.obj_model);
        }

        renderer.queue().submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
