struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut me = User {
        email: String::from("s.shin@marathoner.kr"),
        username: String::from("sshplendid"),
        active: true,
        sign_in_count: 1,
    };

    // my email address is changed!
    me.email = String::from("sshplendid@gmail.com");

    let ori = case_initialize_fields_with_same_variable_name();    

    case_create_new_instance_from_instance(ori);
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // 매개변수와 구조체의 필드명이 같아, 특별히 명시하지 않고 초기화 가능하다.
        username,
        active: true,
        sign_in_count: 1,
    }
}

// 구조체 갱신법을 이용하여 기존 구조체 인스턴스로 새 구조체 인스턴스 생성하기
fn case_create_new_instance_from_instance(ori: User) {
    // ori의 일부 값들을 재사용하여, 인스턴스 waltz 를 생성
    let waltz = User {
        email: String::from("waltz@adorable.cat"),
        username: String::from("waltz"),
        active: ori.active,
        sign_in_count: ori.sign_in_count,
    };

    println!("waltz! {}", waltz.email);
    let waltz = User {
        username: String::from("waltz"),
        email: String::from("waltz@adorable.cat"),
        ..ori
    }; // 25 line, 32 line에서 정의한 waltz 인스턴스는 동일하다.
    println!("waltz! {}", waltz.email);
}

fn case_initialize_fields_with_same_variable_name() -> User {
    // 변수명이 필드명과 같을 때, 간단하게 필드 초기화하기
    let ori = build_user(String::from("ori@adroable.cat"), String::from("ori"));

    ori
}

fn case_other_struct() {
    // 튜플 구조체
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // 필드가 없는 유사 유닛 구조체
    struct Void();
}
