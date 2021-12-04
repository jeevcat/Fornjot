mod config_ui;
mod draw_config;
mod drawables;
mod geometries;
mod mesh;
mod pipelines;
mod renderer;
mod shaders;
mod transform;
mod uniforms;

pub use self::{
    draw_config::DrawConfig,
    renderer::{DrawError, Renderer},
    transform::{FIELD_OF_VIEW_IN_X, NEAR_PLANE},
};

const COLOR_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;
const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
