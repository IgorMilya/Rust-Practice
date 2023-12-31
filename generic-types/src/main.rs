// With Functions
// fn largest<T: PartialOrd + Copy>(list: &Vec<T>) -> &T {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }

// With Structures
// struct Point<T, U> {
//     x: T,
//     y: U,
// }

///////////////////////////
// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }
//
// impl Point<f64> {
//     fn y(&self) -> f64 {
//         self.y
//     }
// }

///////////////////////////
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // With Functions
    // let number_list = vec![34, 50, 25, 100, 65];
    //
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);
    //
    // let char_list = vec!['y', 'm', 'a', 'q'];
    //
    // let result = largest(&char_list);
    // println!("The largest char is {}", result);

    // With Structures
    // let p1 = Point { x: 5, y: 10 };
    // let float = Point { x: 1.0, y: 4.0 };
    // let wont_work = Point { x: 5, y: 4.0 };

    //////////////////////////////////////////////////////
    // let p = Point { x: 5, y: 10 };

    //////////////////////////////////////////////////////
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}