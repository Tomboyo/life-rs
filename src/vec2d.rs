use std::iter::FromIterator;

use num_traits::ToPrimitive;

#[derive(Debug)]
pub struct Vec2D<T> {
    pub width: u32,
    pub height: u32,
    data: Vec<T>,
}

pub struct Iter<'a, T> {
    width: u32,
    iter: std::slice::Iter<'a, T>,
}

pub struct Enumerator<'a, T> {
    width: u32,
    iter: std::iter::Enumerate<std::slice::Iter<'a, T>>,
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

    pub fn get(&self, x: i64, y: i64) -> Option<&T> {
        if (0..self.width as i64).contains(&x)
        && (0..self.height as i64).contains(&y) {
            let index = (self.width as i64 * y) + x;
            self.data.get(index.to_usize()?)
        } else {
            None
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            width: self.width,
            iter: self.data.iter(),
        }
    }

    // Used to change the width and height of an existing Vec2D. The intended
    // use-case is to assign the dimensions of a Vec2D after creating one via
    // iter.collect(), which by default creates a Vec2D with only one dimension.
    pub fn repartition(
        &mut self,
        width: u32,
        height: u32
    ) -> Result<(), String> {
        if width * height == self.data.len() as u32 {
            self.width = width;
            self.height = height;
            Ok(())
        } else {
            Err(format!(
                "Cannot repartition: new capacity {} differs from current
                capacity {}", width * height, self.data.len()))
        }
    }
}

impl<T> FromIterator<T> for Vec2D<T> {
    fn from_iter<I>(iter: I) -> Self
    where I: IntoIterator<Item=T>
    {
        let mut data = Vec::new();
        for element in iter {
            data.push(element);
        }
        Vec2D { width: data.len() as u32, height: 1, data: data }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, T> Iter<'a, T> {
    pub fn enumerate(self) -> Enumerator<'a, T> {
        Enumerator {
            width: self.width,
            iter: self.iter.enumerate(),
        }
    }
}

impl<'a, T> Iterator for Enumerator<'a, T> {
    type Item = ((u32, u32), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let (index, element) = self.iter.next()?;
        let y = index as u32 / self.width;
        let x = index as u32 % self.width;
        Some(((x, y), element))
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
    fn test_repartition() {
        let mut actual = Vec2D::new(3, 2, &mut |x, y| (x, y));
        println!("{:?}", actual);
        actual.repartition(2, 3).unwrap(); // reverse dimensions
        println!("{:?}", actual);

        assert_eq!(2u32, actual.width);
        assert_eq!(3u32, actual.height);
        assert_eq!(vec![
            (0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1)
        ], actual.data);
    }

    #[test]
    fn test_iter() {
        let vector = Vec2D::new(2, 2, &mut |x, y| (x, y));
        let actual: Vec<(u32, u32)> = vector.iter().map(|x| *x).collect();
        let expected = vec![(0, 0), (1, 0), (0, 1), (1, 1)];

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_iter_enumerate() {
        let vector = Vec2D::new(2, 2, &mut |x, y| (x, y));
        let actual: Vec<((u32, u32), (u32, u32))> = vector.iter().enumerate()
            .map(|(index, value)| (index, *value))
            .collect();
        let expected = vec![
            ((0, 0), (0, 0)),
            ((1, 0), (1, 0)),
            ((0, 1), (0, 1)),
            ((1, 1), (1, 1))
        ];

        assert_eq!(expected, actual);
    }
}
