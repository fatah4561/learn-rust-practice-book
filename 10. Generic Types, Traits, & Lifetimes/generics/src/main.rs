// generic in function
// (will error due to we can't compare T since T can be anything ex: file which is uncomparable)
// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0]; 

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     return largest;
// }

struct Point<T> {
    x: T,
    y: T,
}

struct DifferPoint<T, U> {
    x: T,
    y: U,
}

// in method definition
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// constraint
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// other type
impl<X1, Y1> DifferPoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: DifferPoint<X2, Y2>) -> DifferPoint<X1, Y2> {
        DifferPoint { x: self.x, y: other.y }
    }
}

fn main() {
    let int_point = Point { x: 10, y: 5 };
    let float_point = Point {
        x: 10.5f32,
        y: 5.3f32,
    };
    // let wont_compile = Point{x: 10, y: 5.5};
    let now_compile = DifferPoint { x: 10, y: 5.5 };

    println!("{}", int_point.x());
    println!("distance {}", float_point.distance_from_origin());

    let mix = now_compile.mixup(DifferPoint { x: 11, y: 7.5 });
    println!("mix x {}, mix y {}", mix.x, mix.y);
}
