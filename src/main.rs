fn main() {

    // a tuple can be a collection of several different data types, such as a number, a boolean, a string, etc. Because a tuple can contain multiple data types, there is no effective or safe method for iterating over them. 
   
    let my_tuple = (14, "a number", false);

    // tuples us "." notation to get elements out of them, e.g. my_tuple.0, my_tuple.x, etc.

    println!("{}, {}, {}", my_tuple.0, my_tuple.1, my_tuple.2);

    // arrays in rust are more strict than javascript or python. every element in the array must have the same type and the array cannnot change length. This makes it good for grouping similiarly structured data and thus it can be iterated on. You can still use square brackets to get elements out of them.

    let array: [i32; 3] = [5, 7, 2];

    for num in array.iter() {
        println!("{}", num)
    }
}
