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
  // let mut counter = 0;
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

  // for i in (0..5) {
  //   println!("{i}");
  // }

  // 소유권 !!!!!!! 
  // 메모리 할당과 해제

  // 러스트에서 모든 값은 소유자가 있다
  // 한 시점에 딱 하나의 소유자만 있을 수 있다.
  // 소유자의 범위가 끝나면, 값도 제거된다.
    
  // {
  //   let s1 = String::from("hello~"); // Heap 안에 있음
  //   let s2 = s1; // s1의 소유권이 s2로 넘어감  s1.clone() 하면 s1 도 소유권이 남아있고, s2는 s1의 복제값을 가져감
  //   // 그대신 힙메모리에 같은 데이터를 2개를 가지고 있겠찌

  //   println!("s2 = {s2}");
  //   // println!("s1 = {s1}"); 놀랍다 증말

  //   let x= 3;
  //   let y = x; // 기본 데이터 타입은 카피 알아서함... 클론 안해도 러스트가 자동으로 해줌 소유권 개념이 없다고 봐도댐 (String 튜플 제외))
  // }

  // 함수 호출 시 소유권 이동
  let s = String::from("hello"); // 힙에 저장됨
  // string_length(s); // 이 때 s의 소유권이 저 함수 안으로 넘어가 버림
  // println!("s = {}", s);// 그래서 이게ㅐ 안됨. 에러남..s 는 더 이상 main 함수에서 못 씀
  // 만약 기본 데이터 타입이면 그냥 복사 일어남

  // 근데 
  let aa = string_length(s); // 이렇게 하면 aa 로 소유권이 넘어감;
  // println!("s = {}", s);// 그래서 이게ㅐ 안됨. 에러남..s 는 더 이상 main 함수에서 못 씀

}

fn string_length(s: String) -> String {
  println!("string len is {}", s.len());
  s
}
