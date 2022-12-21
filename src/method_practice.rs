// 메소드는 구조체의 내용안에 정의되는 함수이다.
// 메소드 정의하기

// 구조체 내용 안에 함수를 정의하기 위해,
// impl(implementation) 블록을 사용한다.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 메소드 정의하기
    // self는 구조체의 인스턴스를 가리킨다.
    // 또한 &self를 씀으로써 구조체의 인스턴스를 불변으로 참조한다.
    // self의 소유권을 가져갈수도, 변경 불가능하게 빌릴 수도, 다른 파라미터와 비슷하게 변경 가능하게 빌릴 수도 있다.
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// 메소드 문법을 이용하여 호출한다.
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

// 많은 파라미터를 가진 메소드

fn main2() {
    let rect1 = Rectangle {
        width: 50,
        height: 30,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.can_hold(&rect2)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.can_hold(&rect3)
    );
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// 연관함수
// self 파라미터를 갖지 않는 함수도 impl 내에 정의하는 것이 허용된다.
// 이 함수가 해당 구조체와 연관되어 있기 때문이다.
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
