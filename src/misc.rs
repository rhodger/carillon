#[derive(Clone)]
pub struct Length {
    length: u32
}

impl Length {
    pub fn from_int(x: u32) -> Length {
        let length: Length = Length {
            length: x
        };

        return length;
    }

    pub fn pretty_print(&self) -> String {
        let hours = self.length / 3600;
        let minutes = (self.length % 3600) / 60;
        let seconds = (self.length % 3600) % 60;

        return format!("{}:{}:{}", hours, minutes, seconds);
    }

    pub fn get_length(&self) -> u32 { self.length }
}