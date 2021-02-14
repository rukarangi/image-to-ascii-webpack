const PNG: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
const IHDR: [u8; 4] = [0x49, 0x48, 0x44, 0x52];


pub struct IhdrChunk {
    width: [u8; 4],
    height: [u8; 4],
    depth: [u8; 1],
    color_type: [u8; 1],
    compression: [u8; 1],
    filter: [u8; 1],
    interlaced: [u8; 1],
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