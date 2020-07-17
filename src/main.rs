fn main() {
    let array: [i32; 3] = [5, 7, 2];

    println!("{}", array[1]);

    for num in array.iter() {
        println!("{}", num)
    }
}
