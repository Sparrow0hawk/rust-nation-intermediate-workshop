use std::fmt::Display;

trait MyIterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // // Replace ? with correct Generic type parameters
    fn my_filter<P>(self, predicate: P) -> MyFilter<Self, P>
    where
        Self: Sized,
        P: Fn(&Self::Item) -> bool,
    {
        MyFilter {
            iterator: self,
            predicate,
        }
    }

    // fn my_map(self, mapper: f) -> MyMap {
    //     MyMap::new(self, f)
    // }

    // fn my_sum(mut self) -> i32 {
    //     todo!()
    // }
}

impl<T> MyIterator for Vec<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            None
        } else {
            Some(self.remove(0))
        }
    }
}

struct MyFilter<I, P> {
    iterator: I,
    predicate: P,
}

struct MyMap<I, M> {
    iterator: I,
    mapper: M,
}

impl<I: MyIterator, P> MyIterator for MyFilter<I, P>
where
    I: MyIterator,
    P: Fn(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(value) = self.iterator.next() {
            if (self.predicate)(&value) {
                return Some(value);
            }
        }
        None
    }
}

fn print_iterator<T: Display>(mut iterator: impl MyIterator<Item = T>) {
    while let Some(i) = iterator.next() {
        print!("{},", i);
    }
    println!();
}

fn main() {
    let enumeration = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    print_iterator(enumeration.clone());

    let filtered = enumeration.clone().my_filter(|&item| item % 2 == 0);
    print_iterator(filtered);

    // let mapped = enumeration.clone().my_map(|item| format!("Value: {}", item));
    // print_iterator(mapped);

    // let total = enumeration.clone().my_sum();
    // println!("Total: {}", total);

    // let filtered_mapped_total = enumeration.clone()
    //     .my_filter(|&item| item % 2 == 0)
    //     .my_map(|item| item * 2)
    //     .my_sum();
    // println!("Filtered Mapped total is: {}", filtered_mapped_total);
}
