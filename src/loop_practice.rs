// 러스트에서의 조건문
pub fn loop_practice1() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

// if문에서 조건은 반드시 boolean이어야 한다.
pub fn loop_practice2() {
    let number = 3;

    if number {
        println!("number was three");
    }
}

// 만약 0이 아닐 시에 실행하고 싶다면 다음과 같이 작성한다.
pub fn loop_practice3() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}

// else if 사용법
// 다만 조건이 상위 if 혹은 else if에 충족하면 다음 코드는 실행되지 않는다.
pub fn loop_practice4() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

// let구문 if
pub fn loop_practice5() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}

// if문의 반환값은 반드시 같은 타입이어야 한다.
// 다음 코드는 에러를 반환한다.
pub fn loop_practice6() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {}", number);
}

// 조건문이 아닌 반복문들: loop, while, for
// loop
pub fn loop_practice7() {
    loop {
        println!("again!");
    }
}
// 위 코드는 무한하게 동작하며 break를 사용하여 빠져나올 수 있다.

// while
// 조건이 true일때 동작하고, false일때 빠져나온다.
pub fn loop_practice8() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}
// while with array
// 하지만 이 방식은 에러가 발생하기 쉽다.
// 정확한 길이의 index를 사용하지 않으면 패닉을 발생한다.
// 또한 느리다. 왜냐하면 컴파일러가 실행 간에 반복물을 통해 반복될 때마다 요소에 대한 조건 검사를 수행하는
// 런타임 코드를 추가하기 때문이다.
pub fn loop_practice9() {
    let a = [10, 20, 30, 40, 50];

    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
}
// for with array
// for로 순회하면서 index지정에 신경쓰지 않아도 된다.
pub fn loop_practice10() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

// for with range
// range는 한 숫자에서 다른 숫자 전까지 모든 숫자를 차례로 생성한다.
// range라이브러리의 rev메소드는 숫자를 역순으로 생성한다.
pub fn loop_practice11() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
