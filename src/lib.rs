#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/bindings.rs"));

use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct LercBlobInfo {
    pub version: u32,
    pub dataType: u32,
    pub nDim: u32,
    pub nCols: u32,
    pub nRows: u32,
    pub nBands: u32,
    pub nValidPixels: u32,
    pub blobSize: u32,
    pub nMasks: u32,
}

impl LercBlobInfo {
    fn new(vec: &Vec<u32>) -> LercBlobInfo {
        LercBlobInfo {
            version: vec[0],
            dataType: vec[1],
            nDim: vec[2],
            nCols: vec[3],
            nRows: vec[4],
            nBands: vec[5],
            nValidPixels: vec[6],
            blobSize: vec[7],
            nMasks: vec[8],
        }
    }
}

pub struct LercBlob {
    pub info: LercBlobInfo,
    pub data: Vec<f64>,
}

pub fn decode_lerc_file(mut f: File) -> LercBlob {
    unsafe {
        let mut buf = Vec::new();
        let size = f.read_to_end(&mut buf).unwrap();

        let mut infoArr = vec![0; 9];
        let mut dataRangeArr = vec![0f64; 3];
        lerc_getBlobInfo(
            buf.as_ptr(),
            size as u32,
            infoArr.as_mut_ptr(),
            dataRangeArr.as_mut_ptr(),
            9,
            3,
        );
        let blob_info = LercBlobInfo::new(&infoArr);

        let mut valid_bytes = vec![0; blob_info.nValidPixels as usize];
        let mut p_data = vec![0f64; blob_info.nValidPixels as usize];

        lerc_decodeToDouble(
            buf.as_ptr(),
            blob_info.blobSize,
            1,
            valid_bytes.as_mut_ptr(),
            blob_info.nDim as i32,
            blob_info.nCols as i32,
            blob_info.nRows as i32,
            1,
            p_data.as_mut_ptr(),
        );
        LercBlob {
            info: blob_info,
            data: p_data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_blob() {
        let file = File::open("example_file").unwrap();
        let blob = decode_lerc_file(file);

        println!("Info: {:?}", blob.data);
        println!("Info: {:?}", blob.info);
    }
}
