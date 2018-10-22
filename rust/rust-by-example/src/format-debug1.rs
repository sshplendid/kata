// 이 구조체는 `fmt::Display`나 `fmt::Debug`로는 출력되지 않는다.
struct UnPrintable(i32);

// println!("This struct `{}` won't print...", UnPrintable(3));

// `derive` 속성은 `fmt::Debug`를 사용해서 이 `struct`가 출력 가능하도록 만드는데 필요한 구현을 자동으로 생성한다.
#[derive(Debug)]
struct DebugPrintable(i32);

fn main() {
    let x = UnPrintable(3);
    // println!("This struct `{:?}` will print...", x);
    println!("This struct `{:?}` will print...", DebugPrintable(3));

}
