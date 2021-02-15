pub const PNG: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
pub const IHDR: [u8; 4] = [0x49, 0x48, 0x44, 0x52];

// fn slice_to_array(slice: &[u8]) -> [u8] {
    
// }

pub struct IhdrChunk {
    width: [u8; 4],
    height: [u8; 4],
    depth: [u8; 1],
    color_type: [u8; 1],
    compression: [u8; 1],
    filter: [u8; 1],
    interlaced: [u8; 1],
}

impl IhdrChunk {
    pub fn build(data: [u8; 13]) -> IhdrChunk {
        let mut temp: PngImage = PngImage::new_empty();

        let mut width = [0; 4];
        let mut height = [0; 4];
        let mut depth = [0; 1];
        let mut color_type = [0; 1];
        let mut compression = [0; 1];
        let mut filter = [0; 1];
        let mut interlaced = [0; 1];
        
        width.copy_from_slice(&data[0..4]);
        height.copy_from_slice(&data[4..8]);
        depth.copy_from_slice(&data[8..9]);
        color_type.copy_from_slice(&data[9..10]);
        compression.copy_from_slice(&data[10..11]);
        filter.copy_from_slice(&data[11..12]);
        interlaced.copy_from_slice(&data[12..13]);

        return IhdrChunk {
            width,
            height,
            depth,
            color_type,
            compression,
            filter,
            interlaced
        }
    }
}

pub struct Chunk {
    length: [u8; 4],
    type_: [u8; 4],
    data: Vec<u8>,
    crc: [u8; 4]
}

pub struct PngImage {
    header: [u8; 8],
    ihdr: IhdrChunk,
    chunks: Vec<Chunk>,
}

impl PngImage {
    pub fn new_empty() -> PngImage {
        let empty_chunk: Chunk = Chunk {
            length: [0,0,0,0],
            type_: [0,0,0,0],
            data: vec![0x0],
            crc: [0,0,0,0]
        };

        let ihdr = IhdrChunk {
            width: [0,0,0,0],
            height: [0,0,0,0],
            depth: [0],
            color_type: [0],
            compression: [0],
            filter: [0],
            interlaced: [0],
        };

        let empty: PngImage = PngImage {
            header: [0,0,0,0,0,0,0,0],
            ihdr: ihdr,
            chunks: vec![empty_chunk]
        };

        empty
    }
    pub fn new(ihdr: IhdrChunk) -> PngImage {
        let empty_chunk: Chunk = Chunk {
            length: [0,0,0,0],
            type_: [0,0,0,0],
            data: vec![0x0],
            crc: [0,0,0,0]
        };

        let empty: PngImage = PngImage {
            header: [0,0,0,0,0,0,0,0],
            ihdr: ihdr,
            chunks: vec![empty_chunk]
        };

        empty
    }
}