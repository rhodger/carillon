use super::song::Song;
use super::misc::Length;

pub struct Album {
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

    pub fn get_song(&self, i: u32) -> Option<&Song> {
        match &self.songs.get(i as usize) {
            Some(x) => Some(x),
            None => None
        }
    }
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
        let mut album = Album::new("Flowerboy");

        album.songs.push(Song::new("Flowerboy", 120));

        assert_eq!(album.get_title(), "Flowerboy");
        assert_eq!(album.get_song(0).unwrap().get_title(), "Flowerboy");
    }
}