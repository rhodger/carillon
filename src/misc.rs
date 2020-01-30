pub struct Length {
    length: u32
}

impl Length {
    pub fn from_int(x: u32) -> Length {
        Length {
            length: x
        }
    }

    pub fn pretty_print(&self) -> String {
        let hours = self.length / 3600;
        let minutes = (self.length % 3600) / 60;
        let seconds = (self.length % 3600) % 60;

        return format!("{}:{}:{}", hours, minutes, seconds);
    }

    pub fn get_length(&self) -> u32 { self.length }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_from_int() {
        let length = Length::from_int(120);

        assert_eq!(length.length, 120);
    }

    #[test]
    fn length_pretty_print() {
        let length = Length::from_int(120);

        assert_eq!(length.pretty_print(), "0:2:0".to_string());
    }

    #[test]
    fn length_get_length() {
        let length = Length::from_int(120);

        assert_eq!(length.get_length(), 120);
    }
}