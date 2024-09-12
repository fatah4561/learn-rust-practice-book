#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// method is function that bind to struct (or enum, and trait)
impl Rectangle {
    // everything in this block will be associated with Rectangle, hence it is called associated function
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        // can have same name with the field
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle { 
            width: self.width.max(other.width), // this max is NOT rectangle struct but i32 method
            height: self.height.max(other.height),
        }
    }
}

// multiple impl block is allowed
impl Rectangle {
    // associated functions that doesn't have self as first param is NOT a method
    // ex: String::from
    // mostly used as constructor
    fn square(size: u32) -> Rectangle {
        // easier to call than having to create instance with 2 parameter
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width = 10;
    let height = 10;

    println!("{}x{}", width, height);

    println!("the area is {}", area(width, height));

    let dimension: (u32, u32) = (width, height);
    println!("the area_tuple is {}", area_tuple(dimension));

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("the are_struct is {}", area_struct(&rect1));
    println!("the are_struct is {}", area_struct(&rect1));

    // derived debug for struct
    println!("{:?}", rect1);
    println!("{rect1:?}");
    println!("{:#?}", rect1); // pretty print

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(10 * scale), // print and return, using debug but will print to stderr not stdout
        height: 40,
    };

    dbg!(&rect2); // don't take ownership of rect2

    println!("area by method {}", rect2.area());
    println!("width by method {}", rect2.width());
    println!("width {}", rect2.width);

    // -- method with multi param
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("sq {:?}", sq);

    // method call are actually syntactic sugar ex:
    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect2? {}", Rectangle::can_hold(&rect1, &rect2));

    // rust will automatically reference and dereference 
    // as need to match the self type ex:
    let r = &mut Box::new(Rectangle { 
        width: 1,
        height: 2
    });
    let area1 = r.area();
    let area2 = Rectangle::area(&**r);
    assert_eq!(area1, area2);

    // without mut the method set_width is 
    // not possible
    let rect = Rectangle { // R & O Permission no W
        width: 1,
        height: 2
    };

    // rect.set_width(3);


    // Added the mut keyword to the let-binding
    let mut rect = Rectangle {
        width: 0,
        height: 0
    };
    rect.set_width(1);     // this is now ok
    
    let rect_ref = &rect;
    // rect_ref.set_width(2); // but this is still not ok

    // -- calling method with self will move the instance
    // except if the struct implement copy
    let other_rect = Rectangle {
        width: 1,
        height: 2
    };
    let max_rect = rect.max(other_rect);
    println!("{:?}, ", max_rect);

}

fn area(width: u32, height: u32) -> u32 {
    // refactor the param is 2 but actually related so we use struct
    width * height
}

fn area_tuple(dimension: (u32, u32)) -> u32 {
    // refactor attempt 1 use tuple a bit clearer
    dimension.0 * dimension.1 // but still unclear because this doesn't have name and need to use index
}

// fn area_struct(rect: Rectangle) ->u32 { // refactor struct clearer (this will move it)
fn area_struct(rect: &Rectangle) -> u32 {
    // refactor struct clearer (thi will borrow it so rect1 is still usable)
    rect.width * rect.height
}
