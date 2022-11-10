/// 책으로 쓰는 타입 - documentation
struct Book;
/*  */
// 이것이 코멘트

// 1. Integers =================================================================

// + plus sign
// - minus sign
// signed integer =  i8, i16, i32, i64, i128, and isize
// unSigned integer (음수가 될수없다) = u8, u16, u32, u64, u128, and usize
// 뒤에 숫자는 bit 수를 의미한다.
// ex) i32 = 32bit signed integer = 4byte signed integer

// computer architecture에 따른것.
// isize -> 32bit -> i32
// isize -> 64bit -> i64
// let my_num = 5; // 만약 아무런 타입 지정을 안하면 i32로 지정된다.

// 만약, 다른 타입을 서로 비교하려고 하면 에러가 난다.
/*
let my_num: u8 = 100; // 255
let my_other_num = 50; // i32
let third_num = my_num + my_other_num; // error
u8 + i32 을 했으므로 타입 에러가 난다.
*/



// 2. Chars(1) ===================================================================

// ""는 string 타입이다. ex) "hello" -> string, 'hello' -> not string
// '' 는 char 타입이다. ex) let my_char = 'a'; -> char 이렇게 하나의 글자만 가능하다.
// 또한 다음과 같이 이모지 ,유니코드 , 공백을 char로 사용할 수 있다.
// let space = ' '; -> char
// other_language = '🦀'; -> char
// char 타입은 4byte이다. (32bit)

// casting = simple type change using 'as'
// let my_num: u16 = 100;
// let second_num: u8 = my_num
// third_num = my_num + second+num as u16; // 이런식으로 바로 타입 변환시켜 사용가능
// casting은 ASCII 의 적은 확장범위(255자)를 확장해주기 위해 사용된다.
// let my_num = 'a' as u8; // 97
// println!("{}", my_num); // 97
// 'a' char는 97 u8로 casting된다.

fn main() { 
    println!("Hello, world!");
}
