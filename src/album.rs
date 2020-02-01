use super::song::Song;

struct Album {
    title: String,
    songs: Vec<Song>
}

impl Album {
    pub fn new(t: &str) -> Album {
        Album {
            title: t.to_string(),
            songs: Vec::new()
        }
    }

    pub fn get_title(&self) -> String { self.title.to_string() }

    pub fn get_song(&self, i: u32) -> &Song { &self.songs.get(i) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn album_new(){
        let album = Album::new("Flowerboy");

        assert_eq!(album.title, "Flowerboy");
        assert_eq!(album.songs.len(), 0);
    }

    #[test]
    fn album_getters(){
        let album = Album::new("Flowerboy");

        album.songs.push(Song {title: "Flowerboy".to_string(), length: Length::from_int(120)})
    }
}