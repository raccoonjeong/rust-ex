fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}
// https://doc.rust-kr.org/ch03-02-data-types.html 표 3-1: 러스트의 정수형 타입들