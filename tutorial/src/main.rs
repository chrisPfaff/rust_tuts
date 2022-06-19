fn main() {
    let mut _x = "5";
    println!("x is: {}", _x);
    fn chris() {
      let mut _name = String::from("Chris, ");
      println!("name is: {}", _name);
      _name.push_str("the Rustacean");
      println!("name is: {}", _name);
    }
    chris();
    //constants should be named in uppcase with underscores
    const PI_NUMBER: f64 = 3.141592653589793;
    println!("PI_NUMBER is: {}", PI_NUMBER);

    //data types (scaler, compound)
    //scaler: i32, u32, f32, f64, i64, i128, char, bool
    let _true_or_false: bool = true;
    let _letter: char = 'a';
    //compund: tuple, array, slice, string, vec, hashmap
    //tuple: fixed length, fixed type, fixed size
    let _tuple_one: (i32, f64, String) = (500, 6.4, String::from("hello"));
    let mut _tuple_two: (i32, f64, u8) = (500, 6.4, 1);
    let _tuple_three: (i32, f64, char) = (500, 6.4, 's');

    println!("tuple_one is: {:?}", _tuple_one);
    println!("tuple_two is: {}", _tuple_two.2);
    _tuple_two.2 = 2;
    println!("tuple_two is mutated: {:?}", _tuple_two);
}
