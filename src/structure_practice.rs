// 구조체는 튜플과 유사하게 구성요소들은 각자 다른 타입을 지닐 수 있다.
// 구조체는 튜플과 다르게 각 구성요소에 이름을 붙일 수 있다.

// 구조체를 정의할땐 struct 키워드를 사용한다.
// 이후 중괄호 내에 필드(field)라는 각 구성 요소들의 타입과 접근 가능한 이름을 정의한다.

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 위 처럼 정의한 구조체를 사용하려면,
// 필드의 값을 명세한 인스턴스(instance)를 생성해야 한다.
// 인스턴스를 생성할땐 구조체 이름을 사용하고 필드의 이름과 값을 콜론(:)으로 구분하여 명세한다.
// 필드의 순서는 상관없다.

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // 구조체에서 특정 값을 읽으려면 점(.) 표기법을 사용한다.
    user1.email = String::from("anotheremail@example.com");
}

// 인스턴스는 반드시 변경 가능(mutable)이어야 한다.
// Rust에서는 특정 필드만 변경할 수 있도록 허용하지 않는다.
// 다른 표현식과 마찬가지로 함수 본문 마지막에 새 인스턴스 구조체를 표현식으로 생성하여
// 새 인스턴스로 바로 반환 할 수 있다.
fn buile_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// 위 방법보다 더 간단하게
// 필드명과 변수명이 같다면 필드명을 생략할 수 있다.
fn buile_user2(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// 구조체 갱신법을 이용해서 기존 구조체 인스턴스로 새 구조체 인스턴스 생성하기
// email과 username 필드를 제외한 나머지 필드는 user1의 값으로 초기화된다.
fn main2() {
    main();
    let user2 = User {
        email: String::from(""),
        username: String::from(""),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
}

// 더 간단하게 구조체 갱신법(struct update syntax)을 사용한다.
// 구조체 갱신법은 구조체의 필드명과 변수명이 같다면 생략할 수 있다.
fn main3() {
    main();
    let user3 = User {
        email: String::from(""),
        username: String::from(""),
        ..user1
    };
}

// 이름 없고 필드마다 타입은 다르게 정의 가능한 튜플 구조체
// 튜플 구조체는 필드명이 없고, 필드마다 타입은 다를 수 있다.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main4() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
