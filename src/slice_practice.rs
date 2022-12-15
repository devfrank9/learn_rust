// 슬라이스는 소유권을 갖지 않는 또다른 데이터 타입이다.

// 스트링을 입력받아 첫번째 단어를 반환하는 함수를 작성하라.
// 만일 함수가 공백문자를 찾지 못하면,
// 전체 스트링이 한단어 이고
// 이 때는 전체 스트링이 반환된다.

fn first_word(s: &str) -> usize {
    let bytes = s.as_bytes(); // 입력된 String을 byte 배열로 변환한다.

    for (i, &item) in bytes.iter().enumerate() {
        // iter()는 배열의 요소를 순회하는 반복자를 반환한다.
        // enumerate()는 반복자를 반환하는데, 이 반복자는 요소의 인덱스와 요소를 튜플로 반환한다.
        // i는 인덱스를 의미한다.
        // &item은 bytes 배열의 요소를 참조한다.
        if item == b' ' {
            // 바이트 리터럴 문법으로 공백을 찾는다.
            return i; // 공백을 찾으면 인덱스를 반환한다.
        }
    }

    s.len() // 공백을 찾지 못하면 전체 스트링의 길이를 반환한다.
}

// 위 함수를 활용해보자
fn slice_main1() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // word는 5를 갖는다.
    s.clear(); // 이제 s는 ""이다.

    // word는 여전히 5를 갖는다. 하지만 5라는 값을 의미있게 쓸 수 있는 스트링은 이제 없다.
    // word는 이제 유효하지 않다.
}
// 위 프로그램은 아무런 오류가 없다.
// s.clear()를 호출한 뒤 word를 사용해도 컴파일 된다.
// word는 s의 상태와 전혀 연결되지 않아 word는 여전히 5를 갖는다.

// 따라서, 첫번째 단어를 추출하고자 s와 값 5를 사용할수 있지만, word에 5를 저장한 뒤
// s의 내용물이 변경되서 이러한 사용은 버그가 된다.

// 이러한 버그를 갖고 두번째 글자를 추출하는 함수,
fn second_word(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i + 1;
        }
    }

    s.len()
}
// 를 또 사용하게 되면 버그를 점점 찾기 어려워진다.

// 이러한 문제를 해결하기 위해 슬라이스를 사용한다.

// 스트링 슬라이스

fn practice_str_slice() {
    let s = String::from("hello world");

    let hello = &s[0..5]; // hello
    let world = &s[6..11]; // world
}

// 스트링 슬라이스는 스트링의 일부분을 참조하는 것이다.
// 대괄호내에 [시작인덱스..끝인덱스]를 사용한다.
// 슬라이스 데이터 구조는 시작 위치와 슬라이스의 길이를 저장한다.
// 길이 값은 끝 인덱스에서 시작 인덱스를 뺀 값이다.
// 따라서 let world = &s[6..11]; 경우 world는 s의 6번째 바이트를 가리키는 포인터와 길이값 5를 갖는 슬라이스가 된다.

// 스트링 슬라이스 응용
fn practice_str_slice2() {
    let s = String::from("hello");

    let slice = &s[0..2]; // hello
    let slice = &s[..2]; // hello

    let len = s.len();

    let slice = &s[3..len]; // lo
    let slice = &s[3..]; // lo

    let slice = &s[0..len]; // hello
    let slice = &s[..]; // hello
}

// 이제 다시 맨 위의 first_word 함수를 슬라이스를 사용해 다시 작성해보자
fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// 더 유용하게 스트링 슬라이스 쓰기
fn practice_str_slice3() {
    let my_string = String::from("hello world");

    // first_word가 'String'의 슬라이스로 동작
    let word = first_word2(&my_string[..]);
    let my_string_literal = "hello world";

    // first_word가 '스트링 리터럴'의 슬라이스로 동작
    let word = first_word2(&my_string_literal[..]);

    // 스트링 리터럴은 또한 스트링 슬라이스이기 때문에
    let word = first_word2(my_string_literal); // 이상 없음
}
