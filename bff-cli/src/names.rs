use std::io;

use bff::names::Names;

pub fn names() -> Result<(), Box<dyn std::error::Error>> {
    let mut names = Names::default();
    names.read(io::stdin().lock())?;

    println!("{:#?}", names);

    names.write(&mut io::stdout().lock())?;

    Ok(())
}
