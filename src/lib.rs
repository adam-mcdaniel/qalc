mod parser;
pub mod ast;
pub mod target;

use target::Target;
use parser::ProgramParser;

pub fn process<T>(code: impl ToString, t: impl Target<T>) -> Result<T, ()> {
    match ProgramParser::new().parse(&comment::python::strip(code).unwrap()) {
        Ok(prog) => Ok(t.process(prog)),
        Err(e) => {
            eprintln!("{:#?}", e);
            Err(())
        }
    }
}
