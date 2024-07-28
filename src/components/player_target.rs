use hecs::{With, World};

use crate::assets::{Assets, MaterialHandle};
use crate::components::{
    Material, Mesh, Player, RENDER_TAG_HIDDEN, RENDER_TAG_SCENE, RenderOrder, RenderTags, Transform,
};
use crate::math::Vec3;

// A visual guide showing the current focus point of the player
pub struct PlayerTarget;

impl PlayerTarget {
    pub fn spawn(material: MaterialHandle, world: &mut World, assets: &Assets) {
        world.spawn((
            PlayerTarget,
            Transform::default(),
            Mesh(assets.box_mesh_handle),
            Material(material),
            RenderOrder(0),
            RenderTags(RENDER_TAG_HIDDEN),
        ));
    }

    pub fn update(world: &mut World) {
        let (player_focus_at_pt, player_pos) = {
            let (_, (player, player_tr)) = world
                .query_mut::<(&Player, &Transform)>()
                .into_iter()
                .next()
                .unwrap();
            (player.focus_point(), player_tr.position())
        };

        let (new_tag, new_pos, new_scale) = if let Some(pos) = player_focus_at_pt {
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
