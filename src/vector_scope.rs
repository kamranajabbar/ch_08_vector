pub fn run() {
    
    {
        let my_vec = vec![9, 2, 11];
        println!("Vector inside the scope : {:?}", my_vec);     // Code will compile
    }

    //println!("Vector outside from the scope : {:?}", my_vec);  // "my_vec" goes out of scope and is freed here
}