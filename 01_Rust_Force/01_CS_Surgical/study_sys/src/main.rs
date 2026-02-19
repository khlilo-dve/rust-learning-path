struct Student {
    name: String,
    score: u64,
}

impl Student {
    fn new(name: String, score: u64) -> Student {
        Student { name, score }
    }
}

struct Classroom {
    students: Vec<Student>,
}

impl Classroom {
    fn new() -> Classroom {
        Classroom {
            students: Vec::with_capacity(50),
        }
    }
    fn enroll(&mut self, s: Student) {
        self.students.push(s);
    }
    fn print_all(&self) {
        println!("\n班级名单");
        for tx in &self.students {
            println!("姓名：{}，分数：{}", tx.name, tx.score);
        }
    }
    fn average_score(&self) -> u64 {
        let mut total_score = 0;
        for x in &self.students {
            total_score = total_score + x.score;
        }
        let count = self.students.len();
        if count == 0 {
            return 0;
        }
        total_score / (count as u64)
    }
}

fn main() {
    let mut my_class = Classroom::new();
    let s1 = Student::new(String::from("khlilo"), 100);
    let s2 = Student::new(String::from("bob"), 20);
    my_class.enroll(s1);
    my_class.enroll(s2);
    my_class.print_all();
    let avg = my_class.average_score();
    println!("班级平均分：{}", avg);
}
