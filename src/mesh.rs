use alloc::vec::Vec;

use crate::{buffer::Buffer, Renderer, VertexFormat};

pub struct Mesh {
    pub(crate) vertex_buffers: Vec<Buffer>,
    pub(crate) strides: Vec<usize>,
    pub(crate) index_buffer: Buffer,
    pub(crate) vertex_formats: Vec<VertexFormat>,
}

impl Mesh {
    pub fn new(renderer: &Renderer, vertex_data: &[&[u8]], strides: &[usize], index_data: &[u8], vertex_formats: Vec<VertexFormat>) -> Self {
        let mut vertex_buffers = Vec::with_capacity(vertex_data.len());
        for vertex_datum in vertex_data {
            let buffer = renderer.buffer_pool.alloc(vertex_datum.len());
            buffer.write(vertex_datum);

            vertex_buffers.push(buffer);
        }
        let index_buffer = renderer.buffer_pool.alloc(index_data.len());
        index_buffer.write(index_data);

        Self {
            vertex_buffers,
            strides: Vec::from(strides),
            index_buffer,
            vertex_formats,
        }
    }
}
