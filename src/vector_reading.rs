pub fn run() {
    let my_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let third  = my_vec[2];

    println!("Vector all elements : {:?}", my_vec);
    println!("Vector 3rd element  : {:?}", third);
    println!("Vector all elements : {:?}", my_vec);

    // If we access the element that is out of index, below code returns "None" without panicking.
    match my_vec.get(2) {
        Some(third) => println!("The third element of vector is {}", third),
        None        => println!("There is no 9th element in vector."),
    }

    //let does_not_exist  = my_vec[50]; // If we access the element that is out of index, program will be panic.
    //println!("Vector with not existing element : {:?}", does_not_exist);

    let mut my_new_vector = vec![11, 22, 33, 44, 55];
    let first = my_new_vector[0];
    my_new_vector.push(66); 
    println!("The first element is: {}", first);
    println!("Vector after push is: {:?}", my_new_vector);
}