// 러스트의 변수는 기본적으로 불변성을 갖는다
pub fn variable_practice1() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

// 변수를 다음과 같이 선언하면 불변성을 가지지 않는다
pub fn variable_practice2() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

// 상수는 절대적인 불변성을 갖는다. mut 키워드를 사용할 수 없다
// 상수는 선언된 영역내 프로그램이 실행되는 동안 항상 유효하다.
// 그러므로 앱 도메인 전체 또는 프로그램의 여러 부분에서 사용할 수 있다.
pub fn constant_practice1() {
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
}

// 같은 변수명으로 다음과 같이 shadowing을 할 수 있다.
// mut과는 다르게 값의 유형을 변경할 수 있다.
pub fn shadowing_practice1() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // 또한 다음과 같은 차이가 있다.
    let spaces = "   ";
    let spaces = spaces.len();

    let mut spaces = "   ";
    spaces = spaces.len(); // 문자열에서 숫자로 변경하려고 하면 컴파일 에러가 발생한다.
}