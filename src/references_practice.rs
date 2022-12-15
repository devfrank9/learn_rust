fn ref_main1() {
    let s1 = String::from("hello");
    let len = calculate_length1(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

// 실제 값 대신 참조자를 파라미터를 갖는 함수는
// 소유권을 갖지 않기 때문에 소유권을 되돌려 주기 위해
// 반환값을 사용할 필요가 없다.
// 함수의 파라미터로 참조자를 만드는 것은 빌림
fn calculate_length1(s: &String) -> usize {
    // s는 String에 대한 참조자
    s.len()
} // 여기서 s는 범위 밖으로 벗어나고, 따라서 아무런 일도 일어나지 않습니다.

// 참조자는 기본적으로 불변이다.
// 다음은 오류가 발생한다.
fn ref_main2() {
    let mut s = String::from("hello");
    change(&mut s);
}
fn calculate_length2(some_string: &String) -> usize {
    some_string.push_str(", world"); // 오류!
}

// 가변 참조자
fn ref_main3() {
    let mut s = String::from("hello");
    change(&mut s);
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 하지만 러스트는 가변 참조자를 하나만 허용한다.
// 다음은 오류가 발생한다.
fn ref_main4() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);
}

// 이러한 불편함이 있는데 불구하고 장점이 있는데,
// 데이터레이스 (data race)를 방지한다는 점이다.
// 데이터 레이스의 세가지 예를 들어보자.
// 1. 두개이상의 포인터가 동시에 같은 데이터에 접근한다.
// 2. 그 중 하나 이상의 포인터가 데이터를 쓴다.
// 3. 데이터에 접근하는데 동기화를 하는 어떤 매커니즘도 없다.

// 가변참조자와 불변참조자를 혼용시에도 에러가 발생한다.
fn ref_main5() {
    let mut s = String::from("hello");
    let r1 = &s; // 불변 참조자
    let r2 = &s; // 불변 참조자
    let r3 = &mut s; // 가변 참조자
    println!("{}, {}, and {}", r1, r2, r3);
}

// 댕글링 참조자
// 러스트는 댕글링 참조자를 허용하지 않는다.
// 댕글링 포인터는 어떤 메모리를 가리키는 포인터를 보존하는동안,
// 그 메모리를 해제함으로써 다른 개체에게 사용하도록 준지도 모른 메모리를 참조하는 포인터다.
fn ref_main6() {
    let reference_to_nothing = dangle();
}
fn dangle() -> &String {
    // dangle은 String에 대한 참조자를 반환합니다.
    let s = String::from("hello"); // s는 새로운 String입니다.
    &s // s의 참조자를 반환합니다.
} // 여기서 s는 범위 밖으로 벗어나고, 따라서 메모리가 해제됩니다.

// 참조자의 규칙 정리
// 1. 하나의 가변 참조자
// 2. 여러개의 불변 참조자
// 3. 불변 참조자와 가변 참조자는 동시에 존재할 수 없다.
// 4. 참조자는 항상 유효해야 한다.
