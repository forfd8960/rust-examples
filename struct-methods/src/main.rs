#[derive(Debug)]
enum SpokenLang {
    Chinese,
    Russian,
    English,
    Japanese,
    German,
}

#[derive(Debug)]
enum ProgramLang {
    Rust,
    Golang,
    Python,
    Java,
}

#[derive(Debug)]
struct Programmer {
    name: String,
    age: u8,
    spoken_lang: SpokenLang,
    programming: ProgramLang,
    project_num: u32,
}

// tuple struct
#[derive(Debug)]
struct DateTime(u8, u8, u8);

#[derive(Debug)]
struct NodataStruct;

impl Programmer {
    // Associate function
    fn new(name: String, age: u8, spoken_lang: SpokenLang, programming: ProgramLang) -> Programmer {
        Programmer {
            name: name,
            age: age,
            spoken_lang: spoken_lang,
            programming: programming,
            project_num: 0 as u32,
        }
    }

    // Method
    fn programming(&self, project_name: String) {
        println!("start programming with: {}", self.name);
        println!(
            "and I am doing project: {} with {:?}",
            project_name, self.programming
        );
    }

    fn do_project(&mut self, project: String) {
        println!("start programming with: {}, {}", self.name, project);
        self.project_num += 1;
        println!("project num: {}", self.project_num,);
    }
}

fn main() {
    let programmer = Programmer {
        name: String::from("Bob"),
        age: 21,
        spoken_lang: SpokenLang::English,
        programming: ProgramLang::Rust,
        project_num: 0,
    };
    programmer.programming("rust exampels".to_string());

    let mut programmer1 = Programmer::new(
        String::from("Alice"),
        28,
        SpokenLang::English,
        ProgramLang::Golang,
    );

    programmer1.programming("text editor".to_string());

    let p = &mut programmer1;
    p.do_project("game".to_string());
    p.do_project("db".to_string());
}
