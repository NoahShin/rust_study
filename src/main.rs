// use std::io;
// const PI: f32 = 3.141592;
fn main() {
    // println!("[가위, 바위, 보] 중 하나를 입력하세요!");

    // let mut decision = String::new();

    // io::stdin().read_line(&mut decision)
    //     .expect("입력 실패");

    // println!("당신의 선택은.... 바로 = {decision}");
    //////////////////////////////////////////////////////
    // let mut x = 3;
    // println!("x의 값은 {x}입니다,, ㅎㅎ");

    // x = 7;

    // println!("x의 값은 {x}입니다,, ㅎㅎ");
    ////////////////////////////////////////////
    // println!("PI 상수값은 {PI} 입니당.")

    // let x = 3;
    // println!("x의 값은 {x}입니다,, ㅎㅎ");
    // let x = x + 1;
    // println!("x의 값은 {x}입니다,, ㅎㅎ");

    // {
    //   let x = x * 2;
    //   println!("안쪽의 x의 값은 {x}입니다,, ㅎㅎ");
    // }

    // println!("x의 값은 {x}입니다,, ㅎㅎ");

    // let x  = 12.0 / 3.14;
    // println!("x의 값은 {x}입니다,, ㅎㅎ");
    // let b = true;
    // let c = '🩷';

    // 튜플

    // let t: (i32, bool, f64) = (11, true, 12.12);

    // 튜플 구조분해 
    // let (x,y,z) = t;

    // println!("y는 {y} 입니다");

    // 튜플 아이템 접근
    // let x = t.0;
    // let y = t.1;
    // let z = t.2;

    // println!("y는 {y} 입니다");

    // 특별한 튜플 unit // Java js 에서 void
    // 값이 없음 

    // ARRAY 배열 ! 
    // let aa = [1,2,3,4,5];

    // 길이 고정
    // 배열안에 값들의 data type 은 모두 같아야함 

    // 특정값으로 미리 채워서 배열 만들어 놓기 가능
    let threes = [3; 100];
    let last = threes[99];
    println!("{last}")


}
