use neon::prelude::*;
use v4l::context::{ Node };

pub fn node_to_object<'a>(node: &Node, ctx: &mut FunctionContext<'a>) -> JsResult<'a, JsObject>
{
  let object = ctx.empty_object();

  let name = ctx.string(&node.name().unwrap_or("<unknown>".to_string()));
  object.set(
    ctx,
    "name",
    name
  )?;

  let path = ctx.string(&node.path().to_str().unwrap_or("<unknown>"));
  object.set(
    ctx,
    "path",
    path
  )?;

  let index = ctx.number(*&node.index() as f64);
  object.set(
    ctx,
    "index",
    index
  )?;

  Ok(object)
}

pub fn enum_devices(mut ctx: FunctionContext) -> JsResult<JsArray> 
{
  let devices = v4l::context::enum_devices();

  let devices_js = JsArray::new(&mut ctx, devices.len() as u32);

  for (i, s) in devices.iter().enumerate() {
    let v = node_to_object(s, &mut ctx);
    devices_js.set(&mut ctx, i as u32, v?)?;
  }

  Ok(devices_js)
}

pub fn context<'a>(ctx: &mut ModuleContext<'a>) -> JsResult<'a, JsObject>
{
  let context = ctx.empty_object();
  let _enum_devices = JsFunction::new(ctx, enum_devices);
  context.set(ctx, "devices", _enum_devices?);

  Ok(context)
}