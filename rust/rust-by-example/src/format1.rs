fn main() {
    /*
    Formatated print
    `format!`: 포맷이 있는 텍스트를 스트링 형태로 리턴
    `print!`: `format!`과 동일하지만 텍스트를 콘솔 출력(`io::stdout`)
    `println!`: `print!`에 개행문자 추가
    `eprint!`: `format!`과 같으나 텍스트를 `표준에러`로 출력(`io::stderr`)
    `eprintln!`: `eprint!`에 개행문자 추가
    */

    let days_of_month = format!("{} days", 31);
    print!("this month has {}.", days_of_month);
    println!("this month has {}.", days_of_month);
    eprint!("Is this error?");
    eprintln!("Is this error?");
}
