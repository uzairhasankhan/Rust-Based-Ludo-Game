#[derive(Debug)]
struct Student {

    name : String,
    age  : u8,
    class: u8,
    pass : bool,

}

fn main() {
    let mut student_1 = build_student(String :: from("Abeeha Khan"),3,1,true);
    println!("The data of {:#?} is \n{:#?}",student_1.name,student_1);

    student_1.class = 2;
    println!("The data of {:#?} is \n{:#?}",student_1.name,student_1);

}

fn build_student (name : String, age : u8, class : u8, pass :bool) -> Student {
    Student{
        name,
        age,
        class,
        pass,
    }
}
