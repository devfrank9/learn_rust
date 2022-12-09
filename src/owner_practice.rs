// 스택은 값을 받아들인 순서대로 값을 저장하고 반대 방향으로 값을 지운다.
// 데이터 추가는 push, 데이터 제거는 pop이라고 한다.

// 반면 힙은 데이터를 저장할때 먼저 저장할 공간이 있는지 확인한다.
// 운영체제는 힙을 빈 지점에 할당하고 해당 지점의 포인터를 돌려준다.
// 스택에 포인터를 푸싱하는 것은 할당에 해당되지 않는다.
// 이처럼 힙은 포인터를 통해 데이터를 찾아 스택보다 느리다.

// 러스트 소유권 규칙
// 러스트의 각각의값은 해당값의 오너(owner)라고 불리우는 변수를 갖고 있다.
// 한번에 딱 하나의 오너만 존재할 수 있다.
// 오너가 스코프 밖으로 벗어나는 때, 값은 버려진다.(dropped)

// 러스트는 컴파일러가 오너가 스코프 밖으로 벗어나는 순간을 알 수 있도록
// 스코프를 통해 값을 관리한다.
pub fn owner_practice1() {
    // s는 아직 유효하지 않다.
    let s = "hello"; // s가 유효한 범위가 시작된다.
                     // s는 유효하다.
} // 여기서 s는 더 이상 유효하지 않다.

// String과 string 리터럴 차이
// String은 힙에 데이터를 저장하고 string 리터럴은 스택에 저장된다.
// String은 가변이고 string 리터럴은 불변이다.

pub fn owner_practice2() {
    let s = String::from("hello"); // s가 유효한 범위가 시작된다.
                                   // s는 유효하다.
} // 여기서 s는 더 이상 유효하지 않다.

// 변수와 데이터가 상호작용하는 것 : 이동(move)
pub fn owner_practice3() {
    let x = 5;
    let y = x; // x의 값이 y로 이동한다.

    // 위와 다르게 아래 예시는
    // s1의 값이 s2로 이동하는 것이 아니라
    // s1의 데이터에 대한 포인터,길이값,용량값이 s2로 이동한다.
    let s1 = String::from("hello");
    let s2 = s1; // s1의 값이 s2로 이동한다.
}

// 위의 s1 s2의 러스트 메모리 손상 방지 예제
// s1의 데이터에 대한 포인터,길이값,용량값이 s2로 이동한다.
// s1은 더 이상 유효하지 않다.
pub fn owner_practice4() {
    let s1 = String::from("hello");
    let s2 = s1; // s1의 값이 s2로 이동한다.
    println!("{}, world!", s1); // s1은 더 이상 유효하지 않다. , error!
}

// 변수와 데이터가 상호작용하는 것 : 클론(clone)
// 러스트는 힙에 있는 데이터를 복사하기 위해 clone 메소드를 제공한다.
pub fn owner_practice5() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // s1의 값이 s2로 복사된다.
    println!("s1 = {}, s2 = {}", s1, s2);
}

// 하지만 정수형은 clone메서드를 사용하지 않아도 복사된다.
pub fn owner_practice6() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}

// 이처럼 copy 트레잇을 가진 타입은 스택에 저장되기 때문에
// clone 메서드를 사용하지 않아도 복사된다.
// 이런 타입은 정수형, 부울형, 부동소수점형, 문자형, 튜플이다.

// 소유권과 함수
pub fn owner_main1() {
    let s = String::from("hello"); // s가 유효한 범위가 시작된다.
    takes_ownership(s); // s의 값이 함수 안으로 이동한다.
                        // s는 더 이상 유효하지 않다.
    let x = 5; // x가 유효한 범위가 시작된다.
    makes_copy(x); // x가 함수 안으로 이동한다.
                   // x는 여전히 유효하다.
} // 여기서 x는 더 이상 유효하지 않다. 그리고 s도 더 이상 유효하지 않다.

pub fn takes_ownership(some_string: String) {
    // some_string이 유효한 범위가 시작된다.
    println!("{}", some_string);
} // 여기서 some_string은 더 이상 유효하지 않다.

pub fn makes_copy(some_integer: i32) {
    // some_integer가 유효한 범위가 시작된다.
    println!("{}", some_integer);
} // 여기서 some_integer은 더 이상 유효하지 않다.

// 반환값과 스코프
pub fn owner_main2() {
    let s1 = gives_ownership(); // gives_ownership은 반환값을 s1에 바인딩한다.
    let s2 = String::from("hello"); // s2가 유효한 범위가 시작된다.
    let s3 = takes_and_gives_back(s2); // s2는 takes_and_gives_back에 이동되고, 이 함수의 반환값은 s3에 바인딩된다.
} // 여기서 s3은 더 이상 유효하지 않다. s2도 더 이상 유효하지 않다.

pub fn gives_ownership() -> String {
    // gives_ownership은 반환값을 호출한 함수로 이동시킨다.
    let some_string = String::from("hello"); // some_string이 유효한 범위가 시작된다.
    some_string // some_string이 반환되고, 호출한 함수로 이동한다.
}

pub fn takes_and_gives_back(a_string: String) -> String {
    // a_string이 유효한 범위가 시작된다.
    a_string // a_string이 반환되고, 호출한 함수로 이동한다.
}

// 튜플을 이용한 여러 값을 반환하기
pub fn owner_main3() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

pub fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len 메서드는 문자열의 길이를 반환한다.
    (s, length)
}
