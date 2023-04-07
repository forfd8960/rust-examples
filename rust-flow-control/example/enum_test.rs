
#[derive(Debug)]
enum ProgramLang {
    Rust,
    Golang,
    Python,
    Java,
}

#[derive(Debug)]
enum Clock {
    Sundial(u8),
    Digital(u8, u8),
    Analog(u8, u8, u8),
}

fn tell_time(clock: Clock) {
    match clock {
        Clock::Sundial(hours) => {
            println!("sundial hours: {}", hours);
        },
        Clock::Digital(hours, minutes) => {
            println!("digital hours: {}, minutes: {}", hours, minutes);
        },
        Clock::Analog(hours, minutes, seconds) => {
            println!("analog hours: {}, minutes: {}, seconds: {}", hours, minutes, seconds);
        }
    }
}

fn main () {
    let rust_lang = ProgramLang::Rust;
    println!("rust lang: {:?}", rust_lang);
    check_lang(ProgramLang::Rust);
    check_lang(ProgramLang::Golang);
    check_lang(ProgramLang::Python);
    check_lang(ProgramLang::Java);

    println!("clock enum");
    tell_time(Clock::Analog(10, 50, 50));
    tell_time(Clock::Digital(10, 50));
    tell_time(Clock::Sundial(10));
}

fn check_lang(lang: ProgramLang) {
    println!("check lang: {:?}", lang);

    match lang {
        ProgramLang::Rust => println!("Rust"),
        ProgramLang::Golang => println!("Go"),
        ProgramLang::Python => println!("Python"),
        ProgramLang::Java => println!("Java"),
    }
}