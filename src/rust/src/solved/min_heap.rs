use std::cmp::PartialOrd;

struct MinHeap<T> {
    data: Vec<T>,
}

impl <T> MinHeap<T> where T: PartialOrd {
    fn new() -> Self {
        Self { data: Vec::<T>::new() }
    }

    #[warn(dead_code)]
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn append(&mut self, value: T) {
        self.data.push(value);
        self.move_up(self.data.len() - 1)
    }

    fn remove(&mut self, value: &T) {
        let index = self.find_index2(value);
        let last_index = self.data.len() - 1;
        self.data.swap(index, last_index);
        self.data.pop();
        self.move_down(index)
    }

    fn get_parent(i: usize) -> usize {
        if i > 0 {
            (i - 1) / 2
        } else {
            0
        }
    }

    fn find_index2(&self, value: &T) -> usize {
        for i in 0..self.data.len() {
            if &self.data[i] == value {
                return i
            }
        }
        panic!()
    }

    fn can_move_to_kid(&self, i: usize, size: usize) -> usize {
        if i >= size {
            return 0
        }

        let value = &self.data[i];

        let kid = i * 2 + 1;
        if kid < size && value > &self.data[kid] {
            let kid2 = i * 2 + 2;
            if kid2 < size && value > &self.data[kid2] {
                if self.data[kid2] < self.data[kid] {
                    return kid2
                }
                return kid
            }
            return kid
        }

        let kid = i * 2 + 2;
        if kid < size && value > &self.data[kid] {
            return kid
        }

        0
    }

    fn move_down(&mut self, index: usize) {
        let mut curr_index = index;

        let size = self.data.len();
        let mut kid = self.can_move_to_kid(curr_index, size);

        while kid != 0 {
            self.data.swap(kid, curr_index);
            curr_index = kid;
            kid = self.can_move_to_kid(curr_index, size)
        }
    }

    fn move_up(&mut self, index: usize) {
        let mut curr_index = index;

        let mut parent = Self::get_parent(curr_index);

        while index != 0 && self.data[parent] > self.data[curr_index] {
            self.data.swap(parent, curr_index);
            curr_index = parent;
            parent = Self::get_parent(curr_index);
        }
    }

    fn get_min(&self) -> &T {
        &self.data[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean_heap() {
        let mut heap = MinHeap::<i32>::new();
        heap.append(3);
        assert_eq!(*heap.get_min(), 3);
        heap.append(2);
        assert_eq!(*heap.get_min(), 2);
        heap.append(1);
        assert_eq!(*heap.get_min(), 1);

        heap.remove(&1);
        assert_eq!(*heap.get_min(), 2);
        heap.remove(&2);
        assert_eq!(*heap.get_min(), 3);
        heap.remove(&3);
        assert_eq!(heap.is_empty(), true);

        heap.append(4);
        assert_eq!(*heap.get_min(), 4);
        heap.append(9);
        assert_eq!(*heap.get_min(), 4);
        heap.remove(&4);
        assert_eq!(*heap.get_min(), 9);
    }

    #[test]
    fn test_mean_heap2() {
        let mut heap = MinHeap::<i32>::new();

        heap.append(-419921);
        heap.append(429676);

        assert_eq!(*heap.get_min(), -419921);

        heap.remove(&429676);

        assert_eq!(*heap.get_min(), -419921);

        heap.append(21716);
        heap.append(551843);
        heap.append(950119);
        heap.append(63171);

        assert_eq!(*heap.get_min(), -419921);

        heap.append(841804);
        heap.append(170054);
        heap.append(835419);

        heap.remove(&835419);
        heap.remove(&950119);

        assert_eq!(*heap.get_min(), -419921);

        heap.append(258308);
        heap.append(-734231);
        heap.append(569347);
        heap.append(52941);
        heap.append(777770);

        heap.remove(&-734231);

        heap.append(355316);

        assert_eq!(*heap.get_min(), -419921);

        heap.append(415025);
        heap.append(754479);

        heap.remove(&777770);

        heap.append(-744898);

        heap.remove(&551843);

        heap.append(509662);

        assert_eq!(*heap.get_min(), -744898);

        heap.append(765746);

        assert_eq!(*heap.get_min(), -744898);

        heap.append(809282);

        heap.remove(&-744898);

        assert_eq!(*heap.get_min(), -419921);
    }
}