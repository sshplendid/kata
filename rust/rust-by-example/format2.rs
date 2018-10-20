fn main() {
    println!("{0}, Thgis is {1}. {1}, this is {0}", "Alice", "Bob"); // Alice, Thgis is Bob. Bob, this is Alice

    println!("{subject} {verb} {object}",
            object="the lazy dog",
            subject="the quick brown fox",
            verb="jump over"); // the quick brown fox jump over the lazy dog

    // 특별한 형식은 `:` 이후에 정할 수 있다.
    println!("{} of {:b} people know binary, the other half doesn't.", 1, 2); // 1 of 10 people know binary, the other half doesn't.

    // 길이를 정해서 우측정렬 포맷을 설정할 수 있다.
    println!("{number:>width$}", number=1, width=6);        //      1
    println!("{number:>width$}", number=31, width=6);       //     31
    println!("{number:>width$}", number=666666, width=6);   // 666666
    println!("{number:>width$}", number=7654321, width=6);  // 7654321
    
    
    
    
    
    // 패딩을 공백에서 다른 문자로 변경할 수 있다.
    println!("{number:>0width$}", number=1, width=6);       // 000001
    println!("{number:>0width$}", number=31, width=6);      // 000031
    println!("{number:>0width$}", number=666666, width=6);  // 666666
    println!("{number:>0width$}", number=7654321, width=6); // 7654321
    

    println!("My name is {0}, {1} {0}.", "Bond", "James");

    #[allow(dead_code)]
    struct Structure(i32);

    // 위와 같은 커스텀 타입은 더욱 복잡함을 필요로한다.
    // println!("This struct `{}` won't print...", Structure(3)); // FIXME!
}
