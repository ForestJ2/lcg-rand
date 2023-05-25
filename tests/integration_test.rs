#[test]
fn test_range() {
    let mut r = lcg_rand::rand::LCG::new();
    let mut outside: bool = false;

    for _ in 0..100 {
        let v: u8 = r.range(1, 3) as u8;

        if v < 1 || v > 3 {
            outside = true;
            break;
        }
    }

    assert_eq!(outside, false);
}

#[test]
fn test_choose() {
    let v: Vec<u8> = Vec::from("Hello world!");
    let mut r = lcg_rand::rand::LCG::new();

    let mut random  = lcg_rand::rand::LCG::new();
    let arr: [&str; 2] = ["Hello World!", "hello world"];

    println!("{}", random.choose(&arr));

    assert_eq!(v.contains( r.choose(&v) ), true);

}