enum Grade {
    Bachelor,
    Master,
    PhD,
}

enum Major {
    ComputerScience,
    ElectricalEngineering,
}

struct Student {
    name: String,
    grade: Grade,
    major: Major,
}

impl Student {
    fn new(name: String, grade: Grade, major: Major) -> Self {
        Self {
            name: name,
            grade: grade,
            major: major,
        }
    }

    fn introduce_yourself(&self) {
        let grade_response = match self.grade {
            Grade::Master => "Master",
            Grade::Bachelor => "Bachelor",
            Grade::PhD => "PhD",
        };

        let major_response = match self.major {
            Major::ComputerScience => "Computer Science",
            Major::ElectricalEngineering => "Electrical Engineering",
        };

        println!("Hi, my name is {:?}, I am a {:?} of {:?}", self.name, grade_response, major_response);
    }
}

fn main() {
    let grade_student = Student::new("Devi".to_string(), Grade::Bachelor, Major::ComputerScience);
    
    grade_student.introduce_yourself();
}