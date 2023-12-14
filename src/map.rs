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

impl Display for Map<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            let idx = y * self.width;
            write!(
                f,
                "{}\n",
                self.data[idx..idx + self.width].iter().collect::<String>()
            )?;
        }

        Ok(())
    }
}

impl<T> Map<T> {
    pub fn from_str(str: &str) -> Map<T>
    where
        T: From<char>,
    {
        let width = str.lines().next().unwrap().len();
        let str = str.replace("\r\n", "").replace("\n", "");
        let height = str.len() / width;
        let data = str
            .chars()
            .into_iter()
            .map(|c| T::from(c))
            .collect::<Vec<T>>();

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

    fn index(&self, x: i32, y: i32) -> i32 {
        (y * self.width as i32) + x
    }

    pub fn get_xy(&self, x: i32, y: i32) -> Option<&T> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return None;
        }

        let index = self.index(x, y);

        if index >= 0 && index < self.data.len() as i32 {
            Some(&self.data[index as usize])
        } else {
            None
        }
    }
}
