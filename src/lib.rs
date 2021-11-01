pub trait MyIterator: Sized {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    fn map<B, F>(self, f: F) -> MyMap<Self, F>
    where
        F: FnMut(Self::Item) -> B
    {
        MyMap::new(self, f)
    }

    fn collect<B>(self) -> B
    where
        B: MyFromIterator<Self::Item>,
    {
        B::from_iter(self)
    }
}

pub trait MyFromIterator<A> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: MyIterator<Item = A>;
}

impl MyFromIterator<i32> for Vec<i32> {
    fn from_iter<T>(mut iter: T) -> Vec<i32>
    where
        T: MyIterator<Item = i32>
    {
        let mut result = Vec::new();
        while let Some(item) = iter.next() {
            result.push(item);
        }
        result
    }
}

pub struct MyMap<I, F> {
    iterator: I,
    func: F,
}

impl<I, F> MyMap<I, F> {
    fn new(iterator: I, func: F) -> MyMap<I, F>{
        MyMap {
            iterator,
            func,
        }
    }
}

impl<I, F, B> MyIterator for MyMap<I, F>
where
    I: MyIterator,
    F: FnMut(I::Item) -> B,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iterator.next()?;
        Some((self.func)(item))        
    }
}

pub struct MyVecIterator<'a, T> {
    vec: &'a Vec<T>,
    current_index: usize,
}

impl<'a, T> MyIterator for MyVecIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.current_index;
        let item = self.vec.get(index);
        self.current_index += 1;
        item
    }
}

impl<'a, T> std::iter::IntoIterator for MyVecIterator<'a, T> 
where
    T: Clone,
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.to_vec().into_iter()
    }
}

pub trait MyIteratorExt<T> {
    fn my_iter<'a>(&'a self) -> MyVecIterator<'a, T>;
}

impl<T> MyIteratorExt<T> for Vec<T> {
    fn my_iter<'a>(&'a self) -> MyVecIterator<'a, T> {
        MyVecIterator {
            vec: &self,
            current_index: 0,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_works() {
        let mut vec = MyVecIterator {
            vec: &vec![1, 2, 3],
            current_index: 0,
        };
        assert_eq!(Some(&1), vec.next());
        assert_eq!(Some(&2), vec.next());
        assert_eq!(Some(&3), vec.next());
        assert_eq!(None, vec.next());
        assert_eq!(vec.vec, &vec![1, 2, 3]);

        while let Some(v) = vec.next() {
            println!("{}", v);
        }

        let mut vec: MyVecIterator<i32> = MyVecIterator {
            vec: &vec![],
            current_index: 0,
        };
        assert_eq!(None, vec.next());
        assert_eq!(vec.vec, &vec![]);

        let vec = vec![1, 2, 3];
        let mut iter = vec.my_iter();
        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&3), iter.next());
        assert_eq!(None, iter.next());

        for item in iter {
            println!("{}", item);
        }

        let vec = vec![1, 2, 3];
        let mut iter = vec.my_iter().map(|x| x * 2);
        assert_eq!(Some(2), iter.next());
        assert_eq!(Some(4), iter.next());
        assert_eq!(Some(6), iter.next());
        assert_eq!(None, iter.next());

        let vec = vec![1, 2, 3];
        let iter = vec.my_iter().map(|x| x * 2).collect::<Vec<i32>>();
        assert_eq!(vec![2, 4, 6], iter);

    }
}
