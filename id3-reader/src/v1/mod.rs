pub mod decoder;

#[derive(Debug, Default)]
pub struct ID3v1 {
    song_name: String,
    artist: String,
    album_name: String,
    year: String,
    comment: String,
    album_track: u8,
    song_genre: u8,
    song_sub_genre: Option<String>,
}

impl ID3v1 {
    pub fn merge(mut self, ext: ID3v1Ext) -> Self {
        self.song_name.push_str(&ext.song_name);
        self.artist.push_str(&ext.artist);
        self.album_name.push_str(&ext.album_name);
        self.comment.push_str(&ext.comment);
        self.song_sub_genre = Some(ext.sub_genre);
        self
    }
}

#[derive(Debug, Default)]
pub struct ID3v1Ext {
    song_name: String,
    artist: String,
    album_name: String,
    comment: String,
    sub_genre: String,
}

#[cfg(test)]
mod tests {
    use std::fs::{File, read_dir};
    use super::decoder;
    use test::Bencher;

    #[test]
    fn test_id3_v1() {
        let entries: Vec<_> = read_dir("../")
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
    fn bench_id3_v1(b: &mut Bencher) {
        let mut file = File::open("../id3v1.mp3").unwrap();
        b.iter(move || decoder::read(&mut file));
    }
}
