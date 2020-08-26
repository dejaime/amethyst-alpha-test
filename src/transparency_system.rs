use amethyst::{
    core::timing::Time,
    ecs::{Join, Read, System, WriteStorage},
    renderer::resources::Tint,
};

pub struct TransparencySystem;

impl<'s> System<'s> for TransparencySystem {
    type SystemData = (WriteStorage<'s, Tint>, Read<'s, Time>);

    fn run(&mut self, (mut tints, time): Self::SystemData) {
        let mut count = 0;
        for tint in (&mut tints).join() {
            let time: f32 = time.absolute_time().as_secs_f32() as f32 * 4.0;

            let value = (0.5 + time.sin() * 0.51).max(0.0).min(1.0);

            //left
            if count < 3 {
                tint.0.red = value;
                tint.0.green = value;
                tint.0.blue = value;
            
            //middle
            } else if count < 6 {
                tint.0.red = value;
                tint.0.green = value;
                tint.0.blue = value;
                tint.0.alpha = value;
                
            //right
            } else {
                tint.0.alpha = value;
            }

            count += 1;
        }
    }
}
