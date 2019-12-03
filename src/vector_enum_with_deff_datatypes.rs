pub fn run(){
    #[derive(Debug)]
    enum EmpDetailsSheet {
        Name(String),
        Age(u8),
        Height(f64),
    }

    let emp_data = vec! [
        EmpDetailsSheet::Name(String::from("Kamran Jabbar")),
        EmpDetailsSheet::Age(30),
        EmpDetailsSheet::Height(60.07),
    ];

    println!("Employee Vector : {:?}", emp_data);
}