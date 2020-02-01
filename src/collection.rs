use super::album::Album;
use super::misc::AlbumError;

pub struct Collection {
    albums: Vec<Album>
}

impl Collection {
    pub fn new() -> Collection { Collection { albums: Vec::new() } }

    pub fn get_album(&self, x: u32) -> Option<&Album> {
        match self.albums.get(x as usize) {
            Some(x) => Some(x),
            None => None
        }
    }

    pub fn add(&mut self, album: Album) {
        self.albums.push(album)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn Collection_getters() {
        let mut collection = Collection::new();
        collection.albums.push(Album::new("Flowerboy"));

        assert_eq!(collection.get_album(0).unwrap().get_title(),
                   "Flowerboy");
    }

    #[test]
    fn Collection_add() {
        let mut collection = Collection::new();
        collection.add(Album::new("Flowerboy"));

        assert_eq!(collection.albums.pop().unwrap().get_title(),
                   "Flowerboy");
    }
}