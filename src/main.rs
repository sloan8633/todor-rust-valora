use valora::prelude::*;

fn main() -> Result<()> {
    run_fn(Options::from_args(), |_gpu, world, _rng| {
        Ok(move |ctx: Context, canvas: &mut Canvas| {
            // set colro for circle
            canvas.set_color(LinSrgb::new(153./255., 104./255., 255./255.));
            canvas.paint(Filled(ctx.world));

            // TODO: add more functions
            let max_radius = world.width / 3.;
            let radius = ctx.time.as_secs_f32().cos().abs() * max_radius;
            // set background color
            canvas.set_color(LinSrgb::new(255./255., 153./255., 51./255.));
            canvas.paint(Filled(Ellipse::circle(world.center() * ctx.time.as_secs_f32().cos().abs(), radius)));
        })
    })
}
