use std::fmt::Display;

#[derive(Debug)]
pub struct Map<T> {
    data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> IntoIterator for Map<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T: Display> Display for Map<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                let val = self.get_xy(x, y).unwrap();
                val.fmt(f)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl Map<char> {
    pub fn as_key(&self) -> String {
        self.data.iter().collect::<String>()
    }

    pub fn replace(&mut self, data: &str) {
        self.data = data.chars().collect::<Vec<char>>();
    }
}

#[allow(dead_code)]
impl<T> Map<T> {
    pub fn from_str(str: &str) -> Map<T>
    where
        T: From<char>,
    {
        Map::from_str_map(str, T::from)
    }

    pub fn from_str_map<F>(str: &str, f: F) -> Map<T>
    where
        F: FnMut(char) -> T,
    {
        let width = str.lines().next().unwrap().len();
        let str = str.replace("\r\n", "").replace("\n", "");
        let height = str.len() / width;
        let data = str.chars().into_iter().map(f).collect::<Vec<T>>();

        Map {
            width,
            height,
            data,
        }
    }

    pub fn from_wh(width: usize, height: usize) -> Map<T>
    where
        T: Default,
    {
        let mut data = Vec::new();
        data.resize_with(width * height, T::default);

        Map {
            width,
            height,
            data,
        }
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }

    pub fn get_loc(&self, idx: usize) -> (i32, i32) {
        let idx = idx as i32;
        let x = idx % self.width as i32;
        let y = idx / self.width as i32;
        (x, y)
    }

    fn index(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width) + x as usize
    }

    pub fn get_xy(&self, x: i32, y: i32) -> Option<&T> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return None;
        }

        let index = self.index(x, y);

        if index < self.data.len() {
            Some(&self.data[index as usize])
        } else {
            None
        }
    }

    pub fn set_xy(&mut self, x: i32, y: i32, val: T) -> bool {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return false;
        }

        let index = self.index(x, y);

        if index < self.data.len() {
            self.data[index as usize] = val;
            true
        } else {
            false
        }
    }

    pub fn swap(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        let idx1 = self.index(x1, y1);
        let idx2 = self.index(x2, y2);
        self.data.swap(idx1, idx2);
    }
}
