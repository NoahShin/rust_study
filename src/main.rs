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
    // let threes = [3; 100];
    // let last = threes[99];
    // println!("{last}")

    // functions 함수 
    // snake 쓰는 것이 관례

  //  a_function(3,5);

   //명령문 Statement         
   // 명령문 - 무언가 일을 하고 반환값은 없음
   // 식 Expressions
   // 식 - 평가하고 나면 최종 결괏값이 있음
   // 함수는, 여러 명령문에 이어 마지막 식으로 끝남.
   // 마지막 식은 선택적
  //  let x = 3;
  //  let y = {
  //    let x = 3;
  //    5 + x
  //  };

  //  println!("y = {y}")

  // IF expression

  // let x = 5;
  // if x % 3 == 0 {
  //   println!("x는 3으로 나누어 떨어짐");
  // } else if x % 3 == 1 {
  //   println!("x를 3으로 나눈 나머지는 1")
  // } else {
  //   println!("x를 3으로 나눈 나머지는 2")
  // }

  // let 에 쓰는 if 
  // let x = 4;
  // let condition = false;

  // let y = if condition { 3 } else { 5 };
  // println!("y는 {y}입니다");
  let mut counter = 0;
  // loop {
  //   println!("loooop!");
  //   counter += 1;
  //   if counter == 3 {
  //     break;
  //   }
  // }

  // while counter < 3 {
  //   println!("rust ");
  //   counter += 1;
  // }

  // let xs = [1,2,3,4,5];
  // for x in xs {
  //   println!("{x}");
  // }

  for i in (0..5) {
    println!("{i}");
  }
}
// fn a_function(x: i32, y:i32) -> i32 {
//     let sum = x + y;
//     println!("{sum}");
//     println!("다른 함수입니다. {x}");
//     sum
// }