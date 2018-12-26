use std::io;

fn main() {

    // 패턴 매칭을 ㅅ ㅏ용하기
    let mut buffer = String::new();

    match io::stdin().read_line(& mut buffer) {
        Ok(count) => {
            println!("#################");
            println!("{}", buffer);
            println!("  {} bytes read using pattern maching", count);
            println!("#################");
        },
        Err(error) => println!("error: {}", error),
    }


    // unwrap 사용
    let mut buffer = String::new();

    let count: usize = io::stdin().read_line(& mut buffer).unwrap();

            println!("#################");
            println!("{}", buffer);
            println!("  {} bytes read using unwrap", count);
            println!("#################");
}
