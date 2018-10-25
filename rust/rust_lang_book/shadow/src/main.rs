fn main() {
    let x = 5;

    let x = x + 1; // 동일한 변수명으로 새로 할당이 가능하다. 기존의 5를 가진 x는 `shadow`되었다고 표현한다.

    let x = x * 2; // 6을 가진 x는 shadow되었다.

    println!("The value of x is {}", x);


    let spaces = "    ";
    println!("spaces >{}<", spaces);

    let spaces = spaces.len(); // 값의 유형을 변경할 수 있으면서도 동일한 이름을 사용할 수 있다.
    println!("It has {} spaces.", spaces);
}
