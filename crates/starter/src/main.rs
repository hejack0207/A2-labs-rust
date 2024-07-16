
#[macro_use]
extern crate error_chain;

// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! { }
}

use errors::*;

fn main() {
    if let Err(ref e) = runwrapper() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn runwrapper() -> Result<()> {
    run().chain_err(|| "error in runwrapper")?;
    Ok(())
}

// Most functions will return the `Result` type, imported from the
// `errors` module. It is a typedef of the standard `Result` type
// for which the error type is always our own `Error`.
fn run() -> Result<()> {
    use std::fs::File;

    // This operation will fail
    File::open("contacts")
        .chain_err(|| "unable to open contacts file")?;

    Ok(())
}

#[test]
fn test1(){
    let mut i = 10;
    let mut x = &mut i;
    // let mut y = &mut x;
    let z = &x;
    // **y = 20;
}

#[test]
fn test2(){
    let mut i = 10;
    let mut x = &mut i;
    let y = &mut x;
    **y = 20;
    assert!(**y == 20);
    assert!(*x == 20);
}

#[test]
fn test3(){
    let mut i = 10;
    let mut x = &mut i;
    let y = & x;
    let yr: &mut i8 = *y;
    *yr = 20;
    assert!(**y == 20);
    assert!(*x == 20);
}
