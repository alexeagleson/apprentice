use crate::components::particle_lifetime::ParticleLifetime;

use specs::{Join, System, Entities, WriteStorage};

pub struct UpdateParticleEffectsSystem {
    pub elapsed_time: f32,
}

impl<'a> System<'a> for UpdateParticleEffectsSystem {
    type SystemData = (Entities<'a>, WriteStorage<'a, ParticleLifetime>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut lifetimes) = data;

        for (_ent, mut lifetime) in (&entities, &mut lifetimes).join() {
            lifetime.duration -= self.elapsed_time;
        }
    }
}
