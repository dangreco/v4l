extern crate v4l;
use neon::prelude::*;

mod context;


#[neon::main]
fn main(mut ctx: ModuleContext) -> NeonResult<()> {
    let _context = context::context(&mut ctx);
    ctx.export_value("Context", _context?);
    Ok(())
}