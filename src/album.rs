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
}