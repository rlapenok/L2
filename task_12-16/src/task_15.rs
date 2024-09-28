fn main() {
    let s1 = "hello";

    let s2 = String::from("hello");

    let s3 = s2.as_str();
    //5 символов=> 5 байт
    let size_of_s1 = std::mem::size_of_val(s1);

    //Pointer=>8 byte, Len=>8 byte, Cap=>8 byte
    let size_of_s2 = std::mem::size_of_val(&s2);

    //Pointer=>8 byte, Len=>8 byte
    let size_of_s3 = std::mem::size_of_val(&s3);

    //5
    println!("{:?}", size_of_s1);
    //24
    println!("{:?}", size_of_s2);
    //16
    println!("{:?}", size_of_s3);
}
