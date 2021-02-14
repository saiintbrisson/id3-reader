pub mod decoder;
pub mod encoding;
pub mod frame;
pub mod header;

pub use encoding::EncodingTypes;

#[derive(Debug)]
pub struct ID3v2 {
    header: header::Header,
    frames: Vec<frame::Frame>
}

#[cfg(test)]
mod tests {
    use std::fs::{File, read_dir};
    use super::decoder;
    use test::Bencher;

    #[test]
    fn test_id3_v2() {
        let entries: Vec<_> = read_dir("../../")
            .unwrap()
            .filter_map(Result::ok)
            .collect();

        for entry in entries {
            let _: Option<()> = try {
                let metadata = entry.metadata().ok()?;
                if !metadata.is_file() {
                    continue;
                }

                let name = entry.file_name().to_str()?.to_owned();
                if !name.ends_with(".mp3") {
                    continue;
                }

                let mut file = match File::open(entry.path()) {
                    Ok(file) => file,
                    Err(err) => {
                        println!("Could not open file, {:?}", err);
                        continue;
                    }
                };

                let _ = decoder::read(&mut file)
                    .map_err(|err| eprintln!("failed to read {}, {:?}", name, err));
            };
        }
    }

    #[bench]
    fn bench_id3_v2(b: &mut Bencher) {
        let mut file = File::open("../../id3v2.mp3").unwrap();
        b.iter(move || decoder::read(&mut file));
    }
}
