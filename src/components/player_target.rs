use hecs::{With, World};

use crate::assets::Assets;
use crate::components::{
    Material, Mesh, Player, RENDER_TAG_HIDDEN, RENDER_TAG_SCENE, RenderOrder, RenderTags, Transform,
};
use crate::graphics::Graphics;
use crate::materials;
use crate::math::Vec3;

// A visual guide showing the current focus point of the player
pub struct PlayerTarget;

impl PlayerTarget {
    pub fn spawn(gfx: &Graphics, world: &mut World, assets: &mut Assets) {
        let mat = assets.add_color_material(gfx);
        if let materials::Material::Color(m) = assets.material_mut(mat) {
            m.set_color(gfx, Vec3::new(1.0, 1.0, 0.0))
        }

        world.spawn((
            PlayerTarget,
            Transform::default(),
            Mesh(assets.box_mesh),
            Material(mat),
            RenderOrder(0),
            RenderTags(RENDER_TAG_HIDDEN),
        ));
    }

    pub fn update(world: &mut World) {
        let (pos, player_pos) = {
            let (_, (player, player_tr)) = world
                .query_mut::<(&Player, &Transform)>()
                .into_iter()
                .next()
                .unwrap();
            (player.focus().map(|f| f.point), player_tr.position())
        };

        let (new_tag, new_pos, new_scale) = if let Some(pos) = pos {
            let dist_to_camera = (player_pos - pos).magnitude();
            let scale = (dist_to_camera / 10.0).clamp(0.01, 0.1);
            (RENDER_TAG_SCENE, pos, scale)
        } else {
            (RENDER_TAG_HIDDEN, Vec3::zeros(), 1.0)
        };

        let (_, (tr, tags)) = world
            .query_mut::<With<(&mut Transform, &mut RenderTags), &PlayerTarget>>()
            .into_iter()
            .next()
            .unwrap();
        tr.set_position(new_pos);
        tr.set_scale(Vec3::from_element(new_scale));
        tags.0 = new_tag;
    }
}
