/**
 * Implementation of the "Van Emde Boas tree" data structure.
 * https://en.wikipedia.org/wiki/Van_Emde_Boas_tree
 *
 * No gurantee for correctness or efficiency.
 */

// Source: https://stackoverflow.com/a/28666474/5455712
macro_rules! make_array {
    ($n:expr, $constructor:expr) => {{
        let mut items: [_; $n] = std::mem::MaybeUninit::uninit().assume_init();
        for (i, place) in items.iter_mut().enumerate() {
            std::ptr::write(place, $constructor(i));
        }
        items
    }}
}

#[derive(Debug)]
pub struct VebBase {
    min: i16,
    max: i16,
    children: [Option<Box<VebBase>>; 256],
    aux: Option<Box<VebBase>>,
}

impl VebBase {
    pub fn new() -> Self {
        VebBase {
            min: 256,
            max: -1,
            children: unsafe { make_array!(256, |_| None) },
            aux: None,
        }
    }

    pub fn lookup(&self, key: i16) -> Option<bool> {
        if self.max < self.min {
            return None;
        } else if self.min == key {
            return Some(true);
        } else if key <= self.max {
            let index = f64::floor((key as f64) / 256.0);
            let lo = key % 256;

            let result = match &self.children[index as usize] {
                None => None,
                Some(child) => {
                    if lo <= child.max {
                        return child.lookup(lo);
                    }

                    None
                }
            };

            return result;
        }

        None
    }

    pub fn insert(&mut self, mut key: i16) {
        if self.min > self.max {
            self.min = key;
            self.max = key;
            return;
        }

        if key < self.min {
            // swap(key, self.min)
            let tmp = self.min;
            self.min = key;
            key = tmp;
        }

        if key > self.max {
            self.max = key;
        }

        let index = f64::floor((key as f64) / 256.0) as i16;
        let lo = key % 256;

        let child = match &mut self.children[index as usize] {
            Some(child) => child,
            None => {
                self.children[index as usize] = Some(Box::new(Self::new()));

                match &mut self.children[index as usize] {
                    None => panic!("Failed to insert child."),
                    Some(child) => child,
                }
            },
        };

        child.insert(lo);

        if child.min == child.max {
            match &mut self.aux {
                Some(aux_child) => {
                    aux_child.insert(index);
                },
                None => {
                    let mut aux = Box::new(Self::new());
                    aux.insert(index);
                    self.aux = Some(aux);
                }
            };
        }
    }

    pub fn delete(&mut self, mut key: i16) {
        if self.min == self.max && self.min == key {
            self.min = 256;
            self.max = -1;
            return;
        }

        if key == self.min {
            let aux = match &self.aux {
                Some(aux) => aux,
                None => panic!("Missing aux."),
            };

            let high = aux.min * 256;
            let j = aux.min;

            let child_min = match &self.children[j as usize] {
                Some(child) => child.min,
                None => panic!("Expected child at index {}.", j),
            };

            key = high + child_min;
            self.min = key;
        }

        let index = f64::floor((key as f64) / 256.0) as i16;
        let lo = key % 256;

        match &mut self.children[index as usize] {
            Some(child) => {
                child.delete(lo);
                if child.min > child.max {
                    match &mut self.aux {
                        Some(aux) => aux.delete(index),
                        None => panic!("Expected aux for {}.", index),
                    }
                }
            },
            None => panic!("Expected child at index {}.", lo),
        }

        if key == self.max {
            match &self.aux {
                Some(aux) if aux.max >= self.min => {
                    let high = aux.max * 256;
                    let j = aux.max;

                    let child_max = match &self.children[j as usize] {
                        Some(child) => child.max,
                        None => panic!("Expected child at index {}.", j),
                    };

                    self.max = high + child_max;
                },
                _ => {
                    self.max = self.min;
                }

            }
        }
    }

    pub fn find_next(&self, key: i16) -> i16 {
        if key < self.min {
            return self.min;
        }

        if key >= self.max {
            return i16::MAX;
        }

        let index = f64::floor((key as f64) / 256.0) as i16;
        let lo = key % 256;

        let result = match self.children[index as usize] {
            Some(ref child) if lo < child.max => {
                Some((256 * index) + child.find_next(lo))
            },
            _ => None,
        };

        match result {
            Some(value) => value,
            None => {
                let j = match &self.aux {
                    Some(aux) => {
                        aux.find_next(index)
                    },
                    None => panic!("Expected aux to exist.")
                };

                match &self.children[j as usize] {
                    Some(child) => {
                        (256 * j) + child.min
                    },
                    None => panic!("Expected entry at index {}.", j)
                }
            }
        }
    }

    pub fn find_previous(&self, key: i16) -> i16 {
        if self.max != -1 && key > self.max {
            return self.max;
        }

        if key < self.min {
            return -1;
        }

        let index = f64::floor((key as f64) / 256.0) as i16;
        let lo = key % 256;

        let result = match self.children[index as usize] {
            Some(ref child) if lo > child.max => { // Changed
                Some((256 * index) + child.find_previous(lo))
            },
            _ => None,
        };

        match result {
            Some(value) => value,
            None => {
                let j = match &self.aux {
                    Some(aux) => {
                        aux.find_previous(index) // Changed
                    },
                    None => -1
                };

                if j == -1 {
                    if key > self.min {
                        return self.min;
                    }

                    return -1;
                }

                match &self.children[j as usize] {
                    Some(child) => {
                        (256 * j) + child.max // Changed
                    },
                    None => panic!("Expected entry at index {}.", j)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn insert_one_element() {
        let mut tree = VebBase::new();
        assert_eq!(tree.lookup(4), None);
        tree.insert(4);
        assert_eq!(tree.lookup(4), Some(true));
    }

    #[test]
    fn remove_one_element() {
        let mut tree = VebBase::new();
        assert_eq!(tree.lookup(4), None);
        tree.insert(4);
        assert_eq!(tree.lookup(4), Some(true));
        tree.delete(4);
        assert_eq!(tree.lookup(4), None);
    }

    #[test]
    fn find_next_element() {
        let mut tree = VebBase::new();
        tree.insert(12);
        tree.insert(1024);
        assert_eq!(tree.find_next(4), 12);
        assert_eq!(tree.find_next(34), 1024);
        assert_eq!(tree.find_next(666), 1024);
        assert_eq!(tree.find_next(1024), i16::MAX);

        tree.delete(1024);
        assert_eq!(tree.find_next(34), i16::MAX);
        assert_eq!(tree.find_next(90), i16::MAX);
    }

    #[test]
    fn find_previous_element() {
        let mut tree = VebBase::new();
        tree.insert(12);
        tree.insert(1024);
        assert_eq!(tree.find_previous(4), -1);
        assert_eq!(tree.find_previous(34), 12);
        assert_eq!(tree.find_previous(666), 12);
        assert_eq!(tree.find_previous(1024), 12);
        assert_eq!(tree.find_previous(1025), 1024); // Check

        tree.delete(1024);
        assert_eq!(tree.find_previous(34), 12);
        assert_eq!(tree.find_previous(1025), 12);

        tree.insert(6001);
        assert_eq!(tree.find_previous(i16::MAX), 6001);

        assert_eq!(tree.find_previous(i16::MIN), -1);
    }
}
