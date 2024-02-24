# LearnRUST

# Current recommended resources #

1. https://doc.rust-lang.org/book/ Main guide/tutorial for RUST
2. https://doc.rust-lang.org/stable/ overview documentation of all other helpful links for RUST 
3. https://github.com/rust-lang/rustlings/ An interactive guide that runs in your console, should be used together with The Book above
4. https://doc.rust-lang.org/rust-by-example/index.html Learn by example, rather than just reading
5. (a book for those inclined) https://www.amazon.com/dp/B08WZ2D7WC?tag=geekflare-20&linkCode=ogi&th=1&psc=1
6. https://www.rustadventure.dev/ (haven't tried this yet, but keeping it as an option for later)

# Some helpful RUST commands

`rustc` - this command will compile your main.rs source file, this is used for more simple programs. Example: "rustc main.rs" this will compile it, then you would
        type ./main to run the application.

`rustup update` - this will update your rust install, if you have installed on Linux from your distro's maintained repo, not necessary to run.

`cargo update` - This command will update dependencies in the `Cargo.lock` file to the latest version.

`cargo new projectname` - create a new project

`cargo run` - Using "cargo run" will first compile and then run the executable, "cargo run" is more convenient than having to remember to run "cargo build" and then use the whole path to the binary, so most developers use "cargo run". Also if you have not made any changes to your source, it will not compile it again, it will simply run the executable again. 

`cargo build`

`cargo check` - "cargo check" is much faster than "cargo build" because it skips the step of producing an executable. If you’re continually checking your work while writing the code, using cargo check will speed up the process of letting you know if your project is still compiling!

`cargo build --release` - When your project is finally ready for release, you can use "cargo build --release" to compile it with optimizations. This command will create an executable in target/release instead of target/debug. The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile. 

`cargo doc --open` - When you need to know which traits to use and which methods and functions to call from a crate, each crate has documentation with instructions for using it. When you run this command it will build documentation provided by all your dependecies locally and open it in your browser.

# Some helpful git commands

To work on any existing github projects, you can use the following commands to check out the code using Git, change to that project’s directory, and build: 

$ `git clone example.org/someproject`  
$ `cd someproject`  
$ `cargo build`  

# More information about Rust 

The Cargo Book
https://doc.rust-lang.org/cargo/

The Rust Standard Library
https://doc.rust-lang.org/std/prelude/index.html
