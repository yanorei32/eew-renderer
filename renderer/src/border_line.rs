use glium::{DrawParameters, Surface, uniform};
use renderer_types::GeoDegree;

const PREFECTURAL_BORDER_WIDTH: f32 = 5.0;
const AREA_BORDER_WIDTH: f32 = 2.0;

const PREFECTURAL_BORDER_COLOR: [f32; 3] = [0.35, 0.25, 0.19];
const AREA_BORDER_COLOR: [f32; 3] = [0.35, 0.25, 0.19];

pub fn draw<S: ?Sized + Surface>(
    offset: renderer_types::Vertex<GeoDegree>,
    aspect_ratio: f32,
    scale: f32,
    resources: &crate::resources::Resources,
    surface: &mut S,
    params: &DrawParameters,
) {
    let mut params = params.clone();
    params.line_width = Some(AREA_BORDER_WIDTH);

    surface.draw(
        &resources.buffer.vertex,
        &resources.buffer.area_line,
        &resources.shader.map,
        &uniform! {
            aspect_ratio: aspect_ratio,
            offset: offset.to_slice(),
            zoom: scale,
            color: AREA_BORDER_COLOR,
        },
        &params,
    ).unwrap();

    let mut params = params.clone();
    params.line_width = Some(PREFECTURAL_BORDER_WIDTH);

    surface.draw(
        &resources.buffer.vertex,
        &resources.buffer.pref_line,
        &resources.shader.map,
        &uniform! {
            aspect_ratio: aspect_ratio,
            offset: offset.to_slice(),
            zoom: scale,
            color: PREFECTURAL_BORDER_COLOR,
        },
        &params,
    ).unwrap();
}
