#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/bindings.rs"));

mod lerc {
    use super::*;
    use std::fs::File;
    use std::io::{Error, Read, ErrorKind};

    #[derive(Debug)]
    pub struct BlobInfo {
        pub version: u32,
        pub data_type: u32,
        pub n_values_per_pixel: u32,
        pub n_cols: u32,
        pub n_rows: u32,
        pub n_bands: u32,
        pub n_valid_pixels: u32,
        pub blob_size: u32,
        pub n_masks: u32,
    }

    impl BlobInfo {
        fn new(vec: &Vec<u32>) -> BlobInfo {
            BlobInfo {
                version: vec[0],
                data_type: vec[1],
                n_values_per_pixel: vec[2],
                n_cols: vec[3],
                n_rows: vec[4],
                n_bands: vec[5],
                n_valid_pixels: vec[6],
                blob_size: vec[7],
                n_masks: vec[8],
            }
        }
    }

    #[derive(Debug)]
    pub struct DataRange {
        pub z_min: f64,
        pub z_max: f64,
        pub max_z_err_used: f64,
    }

    impl DataRange {
        fn new(vec: &Vec<f64>) -> DataRange {
            DataRange {
                z_min: vec[0],
                z_max: vec[1],
                max_z_err_used: vec[2],
            }
        }
    }

    #[derive(Debug)]
    pub struct LercDataset {
        pub info: BlobInfo,
        pub data_range: DataRange,
        pub data: Vec<f64>,
    }

    pub fn decode_file(mut f: File) -> Result<LercDataset, Error> {
        let mut buf = Vec::new();
        match f.read_to_end(&mut buf) {
            Ok(_) => decode(buf),
            Err(e) => Err(e),
        }
    }

    pub fn decode(buf: Vec<u8>) -> Result<LercDataset, Error> {
        unsafe {
            let mut info_vec = vec![0; 9];
            let mut data_range_vec = vec![0f64; 3];
            let info_result = lerc_getBlobInfo(
                buf.as_ptr(),
                buf.len() as u32,
                info_vec.as_mut_ptr(),
                data_range_vec.as_mut_ptr(),
                9,
                3,
            );
            if info_result > 0 {
                return Err(Error::new(ErrorKind::Other, format!("Failed to get info from blob: {}", info_result)));
            }
            let blob_info = BlobInfo::new(&info_vec);
            let data_range = DataRange::new(&data_range_vec);

            let mut valid_bytes =
                vec![0; (blob_info.n_cols * blob_info.n_rows * blob_info.n_masks) as usize];
            let mut p_data =
                vec![0f64; (blob_info.n_cols * blob_info.n_rows * blob_info.n_bands) as usize];

            let decode_result = lerc_decodeToDouble(
                buf.as_ptr(),
                blob_info.blob_size,
                blob_info.n_masks as i32,
                valid_bytes.as_mut_ptr(),
                blob_info.n_values_per_pixel as i32,
                blob_info.n_cols as i32,
                blob_info.n_rows as i32,
                blob_info.n_bands as i32,
                p_data.as_mut_ptr(),
            );
            if decode_result > 0 {
                return Err(Error::new(ErrorKind::Other, format!("Failed to decode blob: {}", decode_result)));
            }
            Ok(LercDataset {
                info: blob_info,
                data_range: data_range,
                data: p_data,
            })
        }
    }

    // TODO: Encoder
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn decode_blob() {
        let file = File::open("example_file").unwrap();
        if let Ok(blob) = lerc::decode_file(file) {
            println!("Data: {:?}", blob.data.len());
            println!("Info: {:?}", blob.info);
            println!("Data Range: {:?}", blob.data_range);
        } else {
            panic!("Oops :(");
        }
    }
}
