fn main() {
    // 소유권, Ownership
    let s = "hello";
    let s:& str = "hello"; // 3,4 line은 완전 동등한 구문이다.
    // String Literal은 문자열에 대한 포인터, 그래서 불변이다.

   println!("{}!", s);

    {
        let mut s = String::from("hello");

        s.push_str(", world!");
        println!("{}", s);
    }

    {
        // 이동(move)

        let x = 5; // 스택에 있는 값은 바로 복사(copy)를 한다. 그래서 소유권의 이동이 없다.
        let y = x;

        println!("x = {}, y = {}", x, y);



        let s1 = String::from("hello"); // s1은 문자열으 위치정보를 가진 포인터이다. 실제 데이터는 heap에 있음.
        let s2 = s1;                    // "hello"에 대한 소유권이 s1에서 s2로 이동

        // println!("{}, world!", s1); // 소유권이 없는 s1을 호출 => error
        println!("{}, world!", s2);    // s2는 소유권을 가지고 있기 때문에 정상적으로 호출됨.
    }

    {
        // 클론을 사용해서 deep copy가 가능하다.
        let s1 = String::from("hello");
        let s2 = s1.clone();
        let equal:bool = s1 == s2;

        println!("s1 = {}, s2 = {}, equality = {}", s1, s2, equal);
    }
}
