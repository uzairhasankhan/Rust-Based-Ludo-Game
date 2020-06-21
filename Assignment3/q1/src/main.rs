#[derive(Debug)]
struct Student {
    name:       String,
    grade:      u8,
    age:        u8,
    percentage: u8,
}

impl Student {
    fn student_data (name:String, grade:u8,age:u8,percentage:u8) -> Student {
        Student {
            name:        name,
            grade:       grade,
            age:         age,
            percentage: percentage,
        }
    }

    fn percentage (percent: u8) {
        println!("\nThe percentage of student is {}",percent);

    }
}
fn main() {
    let student_1 = Student :: student_data (String :: from ("Abeeha Khan"),1,4,90);
    println!("{:#?}",student_1);

    let student_1_percent = Student :: percentage (student_1.percentage);

}
