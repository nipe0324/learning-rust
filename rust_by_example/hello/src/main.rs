use std::fmt;

fn main() {
    formatted_print();
    debug_trait();
    display_trait();
    display_trait_with_list();
    format_macro();
}

// pringing macros
// format!: write formatted text to String
// print!: same as format! but the text is printed to the console (io::stdout).
// println!: same as print! but a newline is appended.
// eprint!: same as print! but the text is printed to the standard error (io::stderr).
// eprintln!: same as eprint! but a newline is appended.
//
// format traits
// fmt::Debug: Uses the {:?} marker. Format text for debugging purposes.
// fmt::Display: Uses the {} marker. Format text in a more elegant, user friendly fashion.
fn formatted_print() {
    // {} will be replaced with any arguments.
    println!("{} days", 31);
    // 31 days

    // Positional arguments can be used
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // Alice, this is Bob. Bob, this is Alice

    // Named arguments can be used
    println!(
        "{subject} {verb} {object}",
        subject = "the quick brown fox",
        verb = "jumps over",
        object = "the lazy dog"
    );
    // the quick brown fox jumps over the lazy dog

    // Special formatting can be specified after a `:`
    println!("Base 10:               {}", 69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C

    // right-justify text with a specified width
    println!("{number:>5}", number = 1); // "    1"

    // pad numbers with extra zeroes
    println!("{number:0<5}", number = 1); // "10000"

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"

    struct Structure(i32);

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line
}

// The fmt::Debug trait makes this very straightforward.
// All types can derive (automatically create) the fmt::Debug implementation.
// This is not true for fmt::Display which must be manually implemented.
// "pretty pringing" with {:#?}
fn debug_trait() {
    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);

    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    // 12 months in a year.
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );
    // "Christian" "Slater" is the "actor's" name.

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));
    // Now Structure(3) will print!

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));
    // Now Deep(Structure(7)) will print!

    // Prity printing
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let peter = Person {
        name: "Peter",
        age: 27,
    };
    println!("{:#?}", peter);
    // Person {
    //     name: "Peter",
    //     age: 27,
    // }
}

// Manually implementing fmt::Display, which uses the {} print marker
fn display_trait() {
    struct Structure(i32);

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    println!("{}", Structure(3));
    // 3

    // A structure holding two numbers.
    #[derive(Debug)]
    struct MinMax(i64, i64);

    // Implement `Display` for `MinMax`.
    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Use `self.number` to refer to each positional data point.
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    let minmax = MinMax(0, 14);
    println!("Display minmax: {}", minmax); // Display minmax: (0, 14)
    println!("Debug minmax: {:?}", minmax); // Debug minmax: MinMax(0, 14)

    #[derive(Debug)]
    struct Complex {
        real: f64,
        imag: f64,
    }

    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }

    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("Display complex: {}", complex); // Display complex: 3.3 + 7.2i
    println!("Debug complex: {:?}", complex); // Debug complex: Complex { real: 3.3, imag: 7.2 }
}

// Listに対するDisplayの実装はトリッキー
fn display_trait_with_list() {
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;

            write!(f, "[")?;

            for (count, v) in vec.iter().enumerate() {
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}: {}", count, v)?;
            }

            write!(f, "]")
        }
    }

    let v = List(vec![1, 2, 3]);
    println!("{}", v); // [0: 1, 1: 2, 2: 3]
}

// format!("{}", foo) -> "3735928559"
// format!("0x{:X}", foo) -> "0xDEADBEEF"
// format!("0o{:o}", foo) -> "0o33653337357"
fn format_macro() {
    struct City {
        name: &'static str,
        lat: f32,
        lon: f32,
    }

    impl fmt::Display for City {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

            write!(
                f,
                "{}: {:.3}°{} {:.3}°{}",
                self.name,
                self.lat.abs(),
                lat_c,
                self.lon.abs(),
                lon_c
            )
        }
    }

    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ]
    .iter()
    {
        println!("{}", *city);
    }
    // Dublin: 53.348°N 6.260°W
    // Oslo: 59.950°N 10.750°E
    // Vancouver: 49.250°N 123.100°

    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        println!("{:?}", *color);
    }
    // Color { red: 128, green: 255, blue: 90 }
    // Color { red: 0, green: 3, blue: 254 }
    // Color { red: 0, green: 0, blue: 0 }
}
