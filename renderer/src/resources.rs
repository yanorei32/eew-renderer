use crate::vertex::MapVertex;
use renderer_types::Vertex;

use glium::backend::Facade;
use glium::index::PrimitiveType;
use glium::texture::{MipmapsOption, RawImage2d, UncompressedFloatFormat};
use glium::{IndexBuffer, Program, Texture2d, VertexBuffer};

#[derive(Debug)]
pub struct Resources {
    pub shader: Shader,
    pub buffer: Buffer,
    pub texture: Texture,
}

impl Resources {
    pub fn load<F: ?Sized + Facade>(facade: &F) -> Self {
        let shader = Shader::load(facade);
        let buffer = Buffer::load(facade);
        let texture = Texture::load(facade);

        Self {
            shader,
            buffer,
            texture,
        }
    }
}

#[derive(Debug)]
pub struct Buffer {
    pub vertex: VertexBuffer<MapVertex>,
    pub area_line: IndexBuffer<u32>,
    pub pref_line: IndexBuffer<u32>,
    pub map: IndexBuffer<u32>,
}

impl Buffer {
    fn load<F: ?Sized + Facade>(facade: &F) -> Self {
        let geom = renderer_assets::QueryInterface::geometries();

        let vertices: Vec<_> = geom
            .vertices
            .iter()
            .map(|v| MapVertex::new(Vertex::from(*v)))
            .collect();

        let vertex = VertexBuffer::new(facade, &vertices).unwrap();

        let map =
            IndexBuffer::new(facade, PrimitiveType::TrianglesList, geom.map_triangles).unwrap();

        let area_line =
            IndexBuffer::new(facade, PrimitiveType::LinesList, geom.area_lines).unwrap();

        let pref_line =
            IndexBuffer::new(facade, PrimitiveType::LinesList, geom.pref_lines).unwrap();

        Buffer {
            vertex,
            map,
            area_line,
            pref_line,
        }
    }
}

#[derive(Debug)]
pub struct Shader {
    pub map: Program,
    pub intensity_icon: Program,
    pub textured: Program,
}

impl Shader {
    fn load<F: ?Sized + Facade>(facade: &F) -> Self {
        let map = glium::Program::from_source(
            facade,
            include_str!("../../assets/shader/map.vsh"),
            include_str!("../../assets/shader/map.fsh"),
            None,
        )
        .unwrap();

        let intensity_icon = glium::Program::from_source(
            facade,
            include_str!("../../assets/shader/intensity_icon.vsh"),
            include_str!("../../assets/shader/intensity_icon.fsh"),
            Some(include_str!("../../assets/shader/intensity_icon.gsh")),
        )
        .unwrap();

        let textured = glium::Program::from_source(
            facade,
            include_str!("../../assets/shader/textured.vsh"),
            include_str!("../../assets/shader/textured.fsh"),
            None,
        )
        .unwrap();

        Self {
            map,
            intensity_icon,
            textured,
        }
    }
}

#[derive(Debug)]
pub struct Texture {
    pub intensity: Texture2d,
    pub overlay: Texture2d,
}

impl Texture {
    fn load<F: ?Sized + Facade>(facade: &F) -> Self {
        use image::ImageFormat;

        let load_png = |buf: &[u8]| -> Texture2d {
            let image = image::load_from_memory_with_format(buf, ImageFormat::Png).unwrap();
            let image = image.as_rgba8().unwrap();
            let dimension = image.dimensions();
            let image = RawImage2d::from_raw_rgba_reversed(image.as_raw(), dimension);

            Texture2d::with_format(
                facade,
                image,
                UncompressedFloatFormat::U8U8U8U8,
                MipmapsOption::AutoGeneratedMipmaps,
            )
            .unwrap()
        };

        let intensity = load_png(include_bytes!("../../assets/image/intensity.png"));
        let overlay = load_png(include_bytes!("../../assets/image/overlay.png"));

        Self { intensity, overlay }
    }
}