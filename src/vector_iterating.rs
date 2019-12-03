pub fn run(){
    // 1st way
    let my_vec1 = vec![11,22,33,44,55,66];

    let mut c = 0;

    for i in &my_vec1 {
        println!("Vector {} index element : {}", c, i);
        c += 1;
    }

    // 2nd way
    let mut my_vec2 = vec![11,22,33,44,55,66];
    for c in &mut my_vec2 {
        println!("Vector {}", c);
        *c += 0;
    }
}