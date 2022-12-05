// 러스트 타입 - 정수형, 부동소수점 숫자, boolean, 문자, 네가지 스칼라 타입

// 러스트는 타입이 고정된 언어다.
// 컴파일시에 타입을 추측해서 변환시켜 주지만,
// string을 parse해서 숫자로 변환하는 것처럼 타입 선택 폭이 넓은 경우 반드시 타입 명시를 해야한다.
pub fn data_type_practice1() {
    let guess = "42".parse().expect("Not a number!"); // error
    let guess: u32 = "42".parse().expect("Not a number!"); // ok
}

// 스칼라 타입 - 하나의 값으로 표현
// signed = 부호가 있는 정수, unsigned = 부호가 없는 정수
// signed는 -2^n ~ 2^n-1, unsigned는 0 ~ 2^n-1
// i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize

// 부동소수점 타입
// f32, f64
// 기본타입은 f64, 최신 CPU에서 f64와 f32의 속도 차이는 없는데 더 정밀한 표현이 가능해서
pub fn data_type_practice2() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}

// 수학적 연산
pub fn data_type_practice3() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
}

// boolean 타입
pub fn data_type_practice4() {
    let t = true;
    let f: bool = false; // with explicit type annotation
}

// 문자 타입
// 작은따음표는 char
// 큰따음표는 string
pub fn data_type_practice5() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}

// 복합 타입
pub fn data_type_practice6() {
    // 튜플
    // 튜플은 여러 타입을 가질 수 있다.
    // 튜플은 한번 선언되면 길이가 고정되어 변경할 수 없다.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // 배열
    // 배열은 한번 선언되면 길이가 고정되어 변경할 수 없다.
    // 배열은 모든 요소가 같은 타입이어야 한다.
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
    let first = a[0];
    let second = a[1];
}