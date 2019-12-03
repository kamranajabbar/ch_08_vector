pub fn run(){
    let mut my_vector = Vec::new();

    println!("Vector before pushing any value : {:?}", my_vector);
    my_vector.push(10);
    my_vector.push(20);
    my_vector.push(30);
    println!("Vector after pushing some values : {:?}", my_vector);
}