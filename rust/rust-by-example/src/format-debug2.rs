// `Structure` 구조체를 위한 `fmt::Debug` 구현을 얻는다.
// `Structure` 구조체는 단일 32비트 정수형 데이터를 포함하고 있는 구조체이다.
#[derive(Debug)]
struct Structure(i32);

// `Structure` 를 `Deep` 구조체 안에 넣는다.
// 그리고 역시 출력 가능하게 만든다.
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
  name: &'a str,
  age: u8  
}

fn main() {
    // `{:?}`와 함께 출력하는 것은 `{}`와 함께 출력하는 것과 유사하다.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.", "Slater", "Christian", actor="actor's");

    // `Structure`는 이제 출력 가능하다!
    println!("Now {:?} will print!", Structure(3));

    // `derive`의 문제는 결과를 둘러볼 방법이 없다는 것이다. 만약 구조체 안의 '7'만 출력하길 원한다면??
    println!("Now {:?} will print!", Deep(Structure(7)));

    // 그래서 `fmt::debug`는 약간의 우아함을 희생해서 이를 출력 가능하게 만든다. Rust는 `{:#?}`를 통해 "pretty printing"을 제공한다.
    println!("It is {:#?}", Structure(7));


    let name = "Peter";
    let age = 27;
    let peter = Person {name, age};

    // Pretty printing
    println!("{:#?}", peter);
}
