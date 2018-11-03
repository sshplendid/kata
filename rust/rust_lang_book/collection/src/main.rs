fn main() {
    case_create_new_vector_1();
    case_create_new_vector_2();
    case_update_vector();
    case_drop_vector();
    case_read_elements_of_vectors();
    case_ownership_in_vector();
    case_iterate_vector();
    case_use_enum_to_store_multiple_types();
    println!("Hello, world!");
}

fn case_create_new_vector_1() {
    let v: Vec<i32> = Vec::new(); // 불변의 벡터, 타입을 명시해야 함.
                                  // v.push(5); // ! error

    println!("v => {:?}", v);
}

fn case_create_new_vector_2() {
    let v = vec![1, 2, 3]; // 초기값으로부터 타입을 추론한다.

    println!("v => {:?}", v);
}

fn case_update_vector() {
    let mut v = Vec::new(); // 아래 코드에서 데이터로부터 타입을 추론하므로 타입을 명시할 필요가 없다.
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("v => {:?}", v);
}

fn case_drop_vector() {
    {
        let v: Vec<&str> = vec!["a", "b", "c", "d"];
        // do something with v
        println!("v => {:?}", v);
    }
    // v is dropped.
}

fn case_read_elements_of_vectors() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[3]; // 벡터 길이를 초과하는 요소를 조회하면 panic!
    println!("third => {}", third);

    let third: Option<&i32> = v.get(8); // There's no element!
    match third {
        Some(n) => {
            println!("third => {}", n);
        }
        _ => {
            println!("There's no element!");
        }
    }
    if let None = third {
        println!("I told you.");
    }
}

fn case_ownership_in_vector() {
    let mut v = vec![1, 2, 3, 4, 5];
    {
        let first = &v[0];
    }
    v.push(6);
}

fn case_iterate_vector() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("iteration {}", i);
    }

    {
        // 벡터의 값을 변경하기ㅇ
        let mut v = vec![1, 2, 3];
        for i in &mut v {
            *i += 10;
            println!("mut iteration {}", i);
        }
    }
}

fn case_use_enum_to_store_multiple_types() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
