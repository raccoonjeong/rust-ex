fn main() {
    let x = 3;
    println!("x의 값은 {x}입니다.");
    // 변수 쉐도잉
    let x = x + 1;
    println!("x의 값은 {x}입니다.");
    {
        let x = x * 2;
        println!("안쪽 범위에서 x의 값은 {x}입니다.");
    }
    println!("x의 값은 {x}입니다.");
}

const PI: f32 = 3.141592;

fn main3() {
    println!("PI상수값은 {PI}입니다.");
}

fn main2() {
    let mut x = 3;
    println!("x의 값은 {x}입니다.");
    x = 7;
    println!("x의 값은 {x}입니다.");
}

fn main1() {
    println!("Hello, world!");
}
