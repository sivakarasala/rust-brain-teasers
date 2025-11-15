enum Language {English, French}

// static dispatch

// const fn hello(language: Language) -> &'static str {
//     match language {
//         Language::English => "Hello World",
//         Language::French => "Bonjour le monde",
//     }
// }




fn main() {
    // #[cfg(feature="english")]
    // let hello = || println!("Hello World");
    // #[cfg(feature="french")]
    // let hello = || println!("Bonjour le monde");
    // hello();
    // static dispatch
    // println!("{}", hello(Language::English));
    // dynamic dispatch
    
    let language = Language::English;
    let hello = match language {
        Language::English => || println!("Hello World"),
        Language::French => || println!("Bonjour le monde"),
    };
    hello();
}
