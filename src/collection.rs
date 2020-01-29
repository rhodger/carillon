mod misc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_song_new() {
        let song = Song::new("All Star", 120);

        assert_eq!(song.title, "All Star");
        assert_eq!(song.length.get_length(), 120);
    }
}

pub struct Song {
    title: String,
    length: misc::Length
}

impl Song {
    pub fn new(t: &str, l: u32) -> Song {
        let song = Song {
            title: t.to_string(),
            length: misc::Length::from_int(l)
        };

        return song;
    }
}