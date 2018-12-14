fn main() {
    create_iterator();
}

fn create_iterator() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

// 환경을 캡쳐하는 클로저

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

// Iterator 트레잇
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstartion() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);

        //println!("{:?}", v1_iter); // moved value
    }

    #[test]
    fn map_example() {
        let v1: Vec<i32> = vec![1, 2, 3];

        let result: Vec<_> = v1.iter().map(|x| x + 1).collect();

        println!("{:?}", result);

        assert_eq!(result, vec![2, 3, 4]);
    }

    use super::*;

    #[test]
    fn filtered_shoes_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("Sneaker"),
            },
            Shoe {
                size: 19,
                style: String::from("Sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("Crocs"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("Sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("Crocs"),
                },
            ]
        );
    }

    #[test]
    fn counter_iterator_next() {
        let mut counter = Counter::new();

        assert_eq!(counter.next() , Some(1));
        assert_eq!(counter.next() , Some(2));
        assert_eq!(counter.next() , Some(3));
        assert_eq!(counter.next() , Some(4));
        assert_eq!(counter.next() , Some(5));
        assert_eq!(counter.next() , None);
    }
}
