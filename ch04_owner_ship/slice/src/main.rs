fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word nhận giá trị 5

    s.clear(); // Xóa sạch chuỗi s! Bây giờ s = ""
    println!("{word}");
    // word vẫn mang giá trị 5!
    // Nhưng số 5 này giờ vô nghĩa vì chuỗi `s` đã trống rỗng.
    // Nếu dùng `word` để truy cập vào `s`, chương trình sẽ lỗi (panic).
}