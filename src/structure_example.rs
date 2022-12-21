// 구조체를 이용한 예제 프로그램

// 먼저 단순한 사각형 넓이 계산 프로그램을 작성.
fn main() {
    let length1 = 50;
    let width1 = 30;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(length1, width1)
    );
}

fn area(length: u32, width: u32) -> u32 {
    length * width
}

// 위 코드를 튜플로 리팩터링
// 비교적 간단한 프로그램이지만, 튜플을 사용하면 의미를 알기 어려운 코드가 됨.
// 왜냐하면 튜플은 길이가 정해져 있고,
// 요소에 대한 이름이 없어 튜플 내의 값을 인덱스로 접근해야 하기 때문
fn main2() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// 이제 본격적으로 구조체 도입해보기.
// 구조체는 튜플과 달리 각 요소에 이름을 붙일 수 있음.
// 또한 구조체의 소유권이 아닌 빌리기를 원하기 때문에,
// main3에서 호출할때와 area3의 배개변수에 &을 붙여 사용
struct Rectangle {
    length: u32,
    width: u32,
}

fn main3() {
    let rect1 = Rectangle {
        length: 50,
        width: 30,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect1)
    );
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}

// 파생트레잇(derived traits)를 이용해 구조체 출력하기
struct Rectangle2 {
    length: u32,
    width: u32,
}

// 이 코드는 에러가 발생한다.
// println! 매크로는 출력하려는 값에 대해 Display 트레잇을 구현해야 한다.
// 왜냐하면 구조체는 Display 트레잇을 구현하지 않았기 때문.
fn main4() {
    let rect2 = Rectangle2 {
        length: 50,
        width: 30,
    };

    println!("rect1 is {}", rect2);
}

// {} 내에 :?를 추가하면 Debug 트레잇을 사용해 출력
// 하지만 여전히 코드에서 에러가 발생한다.
// 왜냐하면 이번에는 구조체에게 해당 기능을 사용하도록 명시적인 사전동의를 해야한다.

#[derive(Debug)]
struct Rectangle3 {
    length: u32,
    width: u32,
}

fn main5() {
    let rect3 = Rectangle3 {
        length: 50,
        width: 30,
    };

    println!("rect1 is {:?}", rect3);
}

// 마지막으로 {:?} 대신 {:#?}를 사용하면
// 구조체의 내용을 보기 좋게 출력할 수 있다.
