use super::misc::Length;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn song_new(){
        let song = Song::new("All Star", 120);

        assert_eq!(song.title, "All Star");
        assert_eq!(song.length.get_length(), 120);
    }

    #[test]
    fn song_pretty_print(){
        let song = Song::new("All Star", 120);

        assert_eq!(song.pretty_print(), "All Star : 0:2:0".to_string());
    }

    #[test]
    fn song_from_title() {
        let song = Song::from_title("All Star");

        assert_eq!(song.title, "All Star".to_string())
    }

    #[test]
    fn getters() {
        let song = Song::new("All Star", 120);

        assert_eq!(song.get_title(), "All Star".to_string());
        assert_eq!(song.get_length().get_length(), 120);
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

    pub fn from_title(title: &str) -> Song{
        Song {
            title: title.to_string(),
            length: Length::from_int(0)
        }
    }

    pub fn pretty_print(&self) -> String {
        format!("{} : {}", self.title, self.length.pretty_print())
    }


    //Getters
    pub fn get_title(&self) -> String { self.title.to_string() }

    pub fn get_length(&self) -> Length { self.length.clone() }
}