use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let v: Vec<i32> = Vec::new();

    let v2 = vec![1, 2, 3];

    let mut v_mut = Vec::new();

    v_mut.push(1);
    v_mut.push(2);

    v_mut.push(3);

    let third: &i32 = &v2[2];

    println!("third element: {}", third);

    match v2.get(3) {
        Some(third) => println!("third element: {}", third),
        None => println!("third element is not exists"),
    };

    // match if let Some(ele) = v.get(1) {
    //     Some(ele)
    // } else {
    //     None
    // } {
    //     Some(ele) => println!("fourth element: {}", ele),
    //     None => println!("fourth element is not exists"),
    // }

    if let Some(ele) = v2.get(2) {
        println!("fourth element: {}", ele);
    }

    let v_bool: bool = match match match true {
        true => false,
        false => true,
    } {
        false => true,
        true => false,
    } {
        true => false,
        false => true,
    };

    // let vv = match match match Some(1) {
    //     None => Some(None),
    //     Some(v) => Some(Some(v)),
    // } {
    //     None => Some(None),
    //     Some(v) => Some(Some(v)),
    // } {
    //     None => Some(None),
    //     Some(v) => Some(Some(v)),
    // };

    let v1ele = &v_mut[0];

    v_mut.push(1);

    // println!("{}", v1ele);

    iter();

    string_lab();

    hash()
}

fn hash() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    let teams: Vec<String> = vec![
        String::from("blue"),
        String::from("yellow"),
        String::from("red"),
        String::from("green"),
        String::from("black"),
    ];
    let initial = vec![10, 50, 30, 70, 20];

    let scores2: HashMap<_, _> = teams.iter().zip(initial.iter()).collect();

    println!("{:#?}", scores2);

    let field_name = String::from("Favorite color");
    let field_value = String::from("blue");

    let mut map = HashMap::new();

    map.insert(field_name, field_value);

    let team_name = String::from("blue");

    let score = scores.get(&team_name).expect("djfladsl");

    println!("{:?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.entry(String::from("pink")).or_insert(60);
    scores.entry(String::from("pink")).or_insert(70);

    println!("{:?}", scores);

    word_count()
}

fn word_count() {
    let text = "Sunt brabeutaes manifestum bi-color, talis habitioes. To some, a scholar is a suffering for studying.
    Biscuit eaters whine with yellow fever!If you do or contact with a mysterious surrender, music develops you.
Prarere callide ducunt ad castus amicitia. Tatas ortum in fidelis aboa! Ubi est clemens navis? Equisos prarere! Sunt decores demitto rusticus, placidus cannabises.
A fraternal form of sex is the politics. Everything we do is connected with death: purpose, tantra, sainthood, advice. To some, a cow is an issue for fearing. Order is not the boundless silence of the karma., To the rich oysters add chili, bok choy, soy sauce and old chicken.
Wobble without procedure, and we won’t transfer a space suit. Ho-ho-ho! adventure of fortune. A falsis, vigil bassus navis.";

    let mut map: HashMap<&str, u32> = HashMap::new();

    for word in text.split_whitespace() {
        let mut count = map.entry(word).or_insert(0);
        *count += 1
    }

    println!("{:#?}", map);
}

fn string_lab() {
    let mut string = String::new();

    let data: &str = "initial value";

    let s_from_data = data.to_string();

    let s_from_data2 = String::from(data);

    let hello = String::from("한글로 이루어진 문자열 입니다");

    let mut foo = String::from("foo");

    foo.push_str("bar");

    foo.push('!');

    {
        let hello = String::from("hello, ");
        let world = String::from("world!");

        // let helloworld = hello + &world;

        let helloworld = format!("{}{}", hello, world);

        println!("{}", helloworld);

        println!("{}", hello);
        println!("{}", world);
    }

    {
        let str: &str = "abiriant";

        let string: String = "abiria".to_string();

        // println!("{}", str[0usize]);
        // println!("{}", string[0]);

        let korean_hello = "안녕하세요".to_string();

        let len = korean_hello.len();

        println!("{}", len);

        // let zero_index = &korean_hello[0];

        // println!("{}", zero_index);

        let an = &korean_hello[0..3];

        println!("{}", an);

        // let an = &korean_hello[0..4];

        let chars = korean_hello.chars();

        for ch in chars {
            println!("{}", ch);
        }

        for byte in korean_hello.bytes() {
            println!("{}", byte);
        }
    }
}

fn iter() {
    let v = vec![1, 1, 2, 3, 5, 8];

    for i in &v {
        println!("{}", i);
    }

    // for i in v {
    //     println!("{}", i);
    // }

    let mut v_m: Vec<i32> = vec![1, 1, 2, 3, 4, 5, 6];

    for i in &mut v_m {
        *i += 50;
    }

    for i in &v_m {
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(123),
        SpreadsheetCell::Float(14.2321),
        SpreadsheetCell::Text(String::from("sjdladdsklk")),
    ];
}
