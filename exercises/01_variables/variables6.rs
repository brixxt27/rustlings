// TODO: Change the line below to fix the compiler error.
const NUMBER: i32 = 3; // const는 let과 달리 타입을 무조건 명시해야 하나? let도 mut 키워드를 사용하지 않으면 사실 상 상수인데, 왜 const가 따로 있는 거지? const는 shadowing이 되지 않나?

fn main() {
    println!("Number: {NUMBER}");
}
