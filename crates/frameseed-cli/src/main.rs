use frameseed_core::{RenderContext, welcome_message};
use std::{error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", welcome_message());


    let ctx = RenderContext::new(0, 120, 24.0, 42);
    println!(
        "frame {}: t={:.3}, seconds={:.2}",
        ctx.frame_index, ctx.normalized_time, ctx.time_seconds
    );

    Ok(())
}
