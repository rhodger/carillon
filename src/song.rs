use super::misc::Length;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn song_new() {
        let song = Song::new("All Star", 120);

        assert_eq!(song.title, "All Star");
        assert_eq!(song.length.get_length(), 120);
    }

    #[test]
    fn song_pretty_print() {
        let song = Song::new("All Star", 120);

        assert_eq!(song.pretty_print(), "All Star : 0:2:0".to_string());
    }
}

pub struct Song {
    title: String,
    length: Length
}

impl Song {
    pub fn new(t: &str, l: u32) -> Song {
        let song = Song {
            title: t.to_string(),
            length: Length::from_int(l)
        };

        return song;
    }

    pub fn pretty_print(&self) -> String {
        format!("{} : {}", self.title, self.length.pretty_print())
    }
}