fn main() {
    // You can optionally experiment here.
    let a = [1, 2, 3, 4, 5];
    println!("a string: {:?}", &a[1..3]);
    println!("a string: {:?}", [2, 3]); // 첫 번째 요소의 주소와 배열 길이를 저장한다.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        // let nice_slice = ???
        let nice_slice = &a[1..4];
        assert_eq!([2, 3, 4], nice_slice);

        let nice_slice = &a[1..=3];
        assert_eq!([2, 3, 4], nice_slice);
    }
}
