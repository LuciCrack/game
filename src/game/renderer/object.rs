use wgpu::util::DeviceExt;

use super::{vertex::{VERTICES, INDICES}, Renderer};
use super::texture::Texture;

pub struct Object {
    pub texture: Texture,
    pub texture_bind_group: wgpu::BindGroup,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl Object {
    pub fn new(renderer: &Renderer) -> Self {
        let device = renderer.get_device();
        let diffuse_bytes= include_bytes!("../../../assets/Chara - BlueIdle00000.png");
        let texture: Texture =
            Texture::from_bytes(device, renderer.get_queue(), diffuse_bytes, "Character.png").unwrap();
        let texture_bind_group = 
            texture.create_bind_group(device, renderer.bind_group_layout());

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = INDICES.len() as u32;

        Self {
            texture,
            texture_bind_group,
            index_buffer,
            num_indices,
            vertex_buffer
        }
    }
}
