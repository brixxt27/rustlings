// A basket of fruits in the form of a hash map needs to be defined. The key
// represents the name of the fruit and the value represents how many of that
// particular fruit is in the basket. You have to put at least 3 different
// types of fruits (e.g. apple, banana, mango) in the basket and the total count
// of all the fruits should be at least 5.
// <key: Fruit's name, value: The number of fruits in the basket>

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // TODO: Declare the hash map.
    // let mut basket =
    let mut basket = HashMap::new();

    // Two bananas are already given for you :)
    basket.insert(String::from("Banana"), 2);

    // TODO: Put more fruits in your basket.
    basket.insert(String::from("Melon"), 5);
    basket.insert(String::from("Lemon"), 3);

    basket
}

fn main() {
    // You can optionally experiment here.
    let basket = fruit_basket();
    
    println!("=== Option<T> 사용 예시 ===");
    
    // 1. HashMap의 get()은 Option<&V>를 반환
    let melon_count = basket.get("Melon");
    println!("Melon count (Option): {melon_count:?}", );
    
    // 2. match로 Option 처리
    match basket.get("Apple") {
        Some(count) => println!("Apple 개수: {count}"),
        None => println!("Apple이 바구니에 없습니다."),
    }
    
    // 3. if let으로 Option 처리
    if let Some(count) = basket.get("Banana") {
        println!("Banana 개수: {count}", );
    }
    
    // 4. unwrap_or()로 기본값 사용
    let orange_count = basket.get("Orange").unwrap_or(&0);
    println!("Orange 개수 (기본값 0): {orange_count}",);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
