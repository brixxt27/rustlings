fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???
    
    // 방법 1: 기본 배열 리터럴 (현재 방법)
    // let a = ["I will be the best, someday!"; 100];
    
    // 방법 2: Vec 사용
    // let a: Vec<&str> = vec!["I will be the best, someday!"; 100];
    let a: Vec<&str> = vec!["I will be the best, someday!"; 100];
    
    // 방법 3: Range와 map 사용
    // let a: Vec<String> = (0..100).map(|_| "I will be the best, someday!".to_string()).collect();
    
    // 방법 4: repeat iterator 사용
    // let a: Vec<&str> = std::iter::repeat("I will be the best, someday!").take(100).collect();

    // "someday" 단어 개수 세기
    let mut someday_count = 0;
    for element in &a {
        if element.contains("someday") {
            someday_count += 1;
        }
    }
    println!("The word 'someday' appears {someday_count} times in the array");

    if a.len() >= 100 {
        // 다양한 출력 방식 비교
        println!("=== 다양한 포맷팅 방식 비교 ===");
        
        // 1. Debug 포맷팅 (현재 방식)
        println!("Debug format {:?}: {:?}", "첫 3개 요소", &a[0..3]);
        
        // 플레이스홀더 예시들
        println!("=== 플레이스홀더 종류 ===");
        
        let number = 42;
        let name = "Rust";
        
        // 1. 기본 플레이스홀더
        println!("기본: {number}");
        
        // 2. 위치 지정 플레이스홀더
        println!("위치지정: {0} and {1}", number, name);
        
        // 3. 명명된 플레이스홀더 (파라미터 이름: num, lang)
        println!("명명된: {num} in {lang}", num = number, lang = name);
        
        // 4. 포맷 지정자가 있는 플레이스홀더
        println!("16진수: {num:x}", num = number);  // x = 16진수 포맷
        println!("소수점: {pi:.2}", pi = std::f64::consts::PI); // 내장 PI 상수 사용
        
        // 5. Debug 플레이스홀더
        println!("Debug: {data:?}", data = &a[0..2]);

        // 3. Pretty Debug 포맷팅 (예쁘게 정렬)
        println!("Pretty Debug format: {:#?}", &a[0..3]);
        
        // 4. 일반 Display 포맷팅 (문자열의 경우)
        println!("Display format: {}", a[0]);
        
        // println!("Array: {a:?}");
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }

    
}
