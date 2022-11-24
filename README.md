## 그냥 간단하게 아는것

책으로 쓰는 타입 - documentation
struct Book;
/\* \*/
// 이것이 코멘트

# 1. Integers

- `+` plus sign
- `-` minus sign
- signed integer = i8, i16, i32, i64, i128, and isize
- unSigned integer (음수가 될수없다) = u8, u16, u32, u64, u128, and usize
- 뒤에 숫자는 bit 수를 의미한다.
- `ex) i32 = 32bit signed integer = 4byte signed integer`

- sign size는 computer architecture에 따른것.
- isize -> 32bit -> i32
- isize -> 64bit -> i64
- let my_num = 5; // 만약 아무런 타입 지정을 안하면 i32로 지정된다.

- 만약, 다른 타입을 서로 비교하려고 하면 에러가 난다.

```
let my_num: u8 = 100; // 255
let my_other_num = 50; // i32
let third_num = my_num + my_other_num; // error
```

- u8 + i32 을 했으므로 타입 에러가 난다.

# 2. Chars(1)

- ""는 string 타입이다. ex) "hello" -> string, 'hello' -> not string
- '' 는 char 타입이다. ex) let my_char = 'a'; -> char 이렇게 하나의 글자만 가능하다.
- 또한 다음과 같이 이모지 ,유니코드 , 공백을 char로 사용할 수 있다.

```
 let space = ' '; -> char
 other_language = '🦀'; -> char
```

- char 타입은 4byte이다. (32bit)

```
 casting = simple type change using 'as'
 let my_num: u16 = 100;
 let second_num: u8 = my_num
 third_num = my_num + second+num as u16; // 이런식으로 바로 타입 변환시켜 사용가능
 casting은 ASCII 의 적은 확장범위(255자)를 확장해주기 위해 사용된다.
 let my_num = 'a' as u8; // 97
 println!("{}", my_num); // 97
```

- 'a' char는 97 u8로 casting된다.

# 3. Chars(2)

- char 타입은 4byte이다. (32bit)
- .len() 을 사용하면 byte 수를 알 수 있다.
- .chars().count() 를 사용하면 char 수를 알 수 있다.

# 4. Floats

```
 let number = 0\_**_u8; // _ 를 사용하면 숫자를 읽기 쉽게 만들 수 있다.
 let number2 = 1**6**\_2**4*\_\_i32; // * 를 사용하면 숫자를 읽기 쉽게 만들 수 있다.
 println!("{} {}", number, number2); // 0 1624
```

```
 let number = 9.; // f64
 let number2: f32 = 0.0; // f32
 let other_number = 9; // i32
 println!("{}", number as i32 + other_number); // 18
 println!("{}", number + other_number as f64); // 18.6
```

# 5. println!

- 뒤에 !가 붙으면 macro이다.
- 뒤에 !가 붙지 않으면 function이다.

- marcro는 function과 다르게 미리 정의되어있는 것이 아니라
- 컴파일러가 코드를 분석하고 실행시키는 것이다.

- function은 미리 정의되어있는 것이다.
- function은 컴파일러가 코드를 분석하고 실행시키는 것이 아니라
- 미리 정의되어있는 것을 호출하는 것이다.

```
fn give_age() -> i32 {
  42
}

fn main() {
  println!("age is {}", give_age());
}
```

