#[derive(Debug)]

struct Student {
    name: String,
    score: u64,
}

impl Student {
    fn new(name: impl Into<String>, score: u64) -> Self {
        Student {
            name: name.into(),
            score,
        }
    }
}
#[derive(Debug, Default)]
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
        for student in &self.students {
            println!("姓名：{}，分数：{}", student.name, student.score);
        }
    }
    fn average_score(&self) -> Option<f64> {
        if self.students.is_empty() {
            return None;
        }
        let total_score: u64 = self.students.iter().map(|s| s.score).sum();
        Some(total_score as f64 / self.students.len() as f64)
    }
}

fn main() {
    let mut my_class = Classroom::new();
    let s1 = Student::new("khlilo", 100);
    let s2 = Student::new("bob", 20);
    my_class.enroll(s1);
    my_class.enroll(s2);
    my_class.print_all();
    match my_class.average_score() {
        Some(avg) => println!("班级平均分：{:.2}", avg),
        None => println!("班级为空，暂无平均分数据。"),
    }
}
