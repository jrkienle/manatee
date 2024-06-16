use image::{open, GenericImageView};
use std::{
    env::current_dir,
    io::{Error, ErrorKind, Result},
    path::PathBuf,
};
use wgpu::{Extent3d, Sampler, TextureView};

use super::Gpu;

// TODO: I'll probably want to pull this out and make some sort of asset manager struct with dir
// configuration at some point soon, but this'll work for how big this PR already is. This function
// is probably also very flaky and prone to some sort of injection somewhere but, eh
fn get_assets_dir() -> Result<PathBuf> {
    let path = current_dir()?;
    let mut path_ancestors = path.as_path().ancestors();
    while let Some(current_path) = path_ancestors.next() {
        let assets_path = current_path.join("assets");
        let assets_folder_exists = assets_path.exists() && assets_path.is_dir();
        if assets_folder_exists {
            return Ok(assets_path);
        }
    }
    Err(Error::new(
        ErrorKind::NotFound,
        "Could not find \"assets\" directory in project root. Please ensure you followed Manatee's setup instructions",
    ))
}

// Note: This is probably nowhere near the abstraction that I want it to be, however the wgpu
// tutorial files were getting fucking HUGE and I really wanted to pull some of them out and start
// to think about abstractions before I got too deep
pub struct Texture {
    pub sampler: Sampler,
    pub texture: wgpu::Texture,
    pub view: TextureView,
}

impl Texture {
    pub fn new(name: &str, gpu: &Gpu) -> Self {
        let assets_dir = get_assets_dir().expect("Assets Directory Not Found");
        let texture_path = assets_dir.join("textures").join(&name);

        let image = open(&texture_path).expect(&format!(
            "Failed to load texture at \"{}\"",
            &texture_path.to_string_lossy()
        ));
        let rgba = image.to_rgba8();
        let dimensions = image.dimensions();

        let texture_size = Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        // TODO: I need to figure out if I want to abstract this to the Gpu struct and make texture
        // structs data-only (probably) or if I want to leave this here to keep code contextual.
        // Ugh decisions are so hard when you have no idea what you're doing
        let texture = gpu.device.create_texture(&wgpu::TextureDescriptor {
            // TODO: Should these be configurable?
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some(&name),
            view_formats: &[],
        });

        gpu.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            // TODO: This should also almost definitely be configurable lol
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            texture_size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        // TODO: More things to probably make configurable
        let sampler = gpu.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
        }
    }
}
