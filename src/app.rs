use crate::buffer::Buffer;

pub struct Window {
    width: usize,
    height: usize,
    buffers: Vec<Buffer>
}

impl Window {
    pub fn new(width: usize, height: usize) -> Self {
        return Window {
            width: width as usize,
            height: height as usize,
            buffers: vec![]
        };
    }

    pub fn render() {
        
    }
}
