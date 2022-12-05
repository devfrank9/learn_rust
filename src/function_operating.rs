// 러스트는 snake_case를 사용한다.
// 소문자를 사용하며 밑줄 표시로 구분한다.
// 선언은 fn으로 시작하여 함수 이름 뒤에 괄호 형식을 두고,
// 중괄호는 함수의 시작과 종료 지점을 알려주게 됩니다.
pub fn sub_main() {
    println!("Hello, world!");
    another_function();
}

// 함수를 호출할때
// 러스트는 함수의 위치를 신경쓰지 않는다. 어디든 정의 되어 있으면 된다.
pub fn another_function() {
    println!("Another function.");
}

// 함수의 매개변수, 전달인자
pub fn sub_main2() {
    // 전달인자
    another_function2(5);
}
// 매개변수
// 매개변수를 선언할때 반드시 타입을 명시해야 한다.
pub fn another_function2(x: i32) {
    println!("The value of x is: {}", x);
}
// 여러개의 매개변수
pub fn another_function3(x: i32, y: i64) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// 러스트는 반환값이 없으면 bind하지 못한다.
// 따라서 let y = 6은 반환값이 없으므로 let x에 bind 할 수 없다.
pub fn another_function4() {
    let x = (let y = 6);
}

// 함수표현식
pub fn sub_main3() {
    let x = 5;
    // y에 할당한 블록 {}은 표현식 부 이다.
    let y = {
        let x = 3;
        // 세미콜론울 붙히지 않으면 표현식이 되며 반환값이 된다.
        x + 1
        // 세미콜론을 붙히면 구문이 되고, 반환값이 아니다.
        // x + 1;
    };
    println!("The value of y is: {}", y);
}

// 반환값을 갖는 함수
pub fn five() -> i32 {
    5
}

pub fn sub_main4() {
    let x = five();
    println!("The value of x is: {}", x);
}

// 반환값을 갖는 함수(2)
pub fn sub_main5() {
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

pub fn plus_one(x: i32) -> i32 {
    x + 1
}

// 이렇게 하면 에러!
pub fn plus_one2(x: i32) -> i32 {
    x + 1;
}