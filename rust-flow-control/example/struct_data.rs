#[derive(Debug)]
enum SpokenLang {
    Chinese,
    Russian,
    English,
    Japanese,
    German
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
}

// tuple struct
#[derive(Debug)]
struct DateTime(u8, u8, u8);

#[derive(Debug)]
struct NodataStruct;


fn main() {
    let a_programmer = Programmer{
        name: String::from("xiao ming"),
        age: 21,
        spoken_lang: SpokenLang::Chinese,
        programming: ProgramLang::Rust,
    };
    println!("programmer: {:?}", a_programmer);

    let date_time = DateTime(11, 6, 30);
    println!("date_time: {:?}", date_time);

    let nodata_struct = NodataStruct;
    println!("nodata_struct: {:?}", nodata_struct);
}