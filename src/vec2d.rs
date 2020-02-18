use num_traits::ToPrimitive;

#[derive(Debug)]
pub struct Vec2D<T> {
    pub width: u32,
    pub height: u32,
    data: Vec<T>,
}

impl<T> Vec2D<T> {
    pub fn new<F>(width: u32, height: u32, initializer: &mut F) -> Self
    where F: FnMut(u32, u32) -> T
    {
        let capacity = width * height;
        let data = (0..capacity)
            .map(|index| {
                let y = index / width;
                let x = index % width;
                initializer(x, y)
            })
            .collect();

        Self { width, height, data }
    }

    pub fn from_vec(
        width: u32,
        height: u32,
        data: Vec<T>
    ) -> Result<Self, String> {
        if width as usize * height as usize == data.len() {
            Ok(Self { width, height, data })
        } else {
            Err("Invalid dimensions: Width * height must equal data.len"
                .to_string())
        }
    }

    pub fn get(&self, x: i64, y: i64) -> Option<&T> {
        if (0..self.width as i64).contains(&x)
        && (0..self.height as i64).contains(&y) {
            let index = (self.width as i64 * y) + x;
            self.data.get(index.to_usize()?)
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = ((u32, u32), &T)> {
        self.data.iter().enumerate()
            .map(move |(index, t)| {
                let y = index.to_u32().unwrap() / self.width;
                let x = index.to_u32().unwrap() % self.width;
                ((x, y), t)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let actual = Vec2D::new(3, 2, &mut |x, y| (x, y));
        assert_eq!(3u32, actual.width);
        assert_eq!(2u32, actual.height);
        assert_eq!(vec![
            (0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1)
        ], actual.data);
    }

    #[test]
    fn test_from_vec() {
        let actual = Vec2D::from_vec(2, 3, vec![0, 1, 2, 3, 4, 5]).unwrap();
        assert_eq!(2u32, actual.width);
        assert_eq!(3u32, actual.height);
        assert_eq!(vec![0, 1, 2, 3, 4, 5], actual.data);
    }

    #[test]
    fn test_from_vec_incompatible_dimensions() {
        assert!(Vec2D::from_vec(3, 3, vec![1]).is_err());
    }

    #[test]
    fn test_get() {
        let vec = Vec2D::new(2, 2, &mut |x, y| (x, y));
        assert_eq!(&(1, 1), vec.get(1, 1).unwrap());
    }

    #[test]
    fn test_get_x_bounds() {
        let vec = Vec2D::new(2, 2, &mut |x, y| (x, y));
        assert_eq!(None, vec.get(-1, 0), "(-1, 0) is outside bounds");
        assert_eq!(None, vec.get(2, 0), "(2, 0) is outside bounds");
    }

    #[test]
    fn test_get_y_bounds() {
        let vec = Vec2D::new(2, 2, &mut |x, y| (x, y));
        assert_eq!(None, vec.get(0, -1), "(0, -1) is outside bounds");
        assert_eq!(None, vec.get(0, 2), "(0, 2) is outside bounds");
    }

    #[test]
    fn test_iter() {
        let vec2d = Vec2D::new(2, 3, &mut |x, y| (x, y));
        let actual: Vec<((u32, u32), &(u32, u32))> = vec2d.iter().collect();
        assert_eq!(vec![
            ((0, 0), &(0, 0)),
            ((1, 0), &(1, 0)),
            ((0, 1), &(0, 1)),
            ((1, 1), &(1, 1)),
            ((0, 2), &(0, 2)),
            ((1, 2), &(1, 2)),
        ], actual);
    }
}
