#[derive(Debug)]
pub struct Map {
    data: Vec<char>,
    pub width: usize,
    pub height: usize,
}

impl IntoIterator for Map {
    type Item = char;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl Map {
    pub fn from_str(str: &str) -> Map {
        let width = str.lines().next().unwrap().len();
        let height = str.len() / width;
        let data = str
            .replace("\r\n", "")
            .chars()
            .into_iter()
            .collect::<Vec<char>>();

        Map {
            width,
            height,
            data,
        }
    }

    fn index(&self, x: i32, y: i32) -> i32 {
        (y * self.width as i32) + x
    }

    pub fn get_xy(&self, x: i32, y: i32) -> Option<char> {
        let index = self.index(x, y);

        if index >= 0 && index < self.data.len() as i32 {
            Some(self.data[index as usize])
        } else {
            None
        }
    }
}
