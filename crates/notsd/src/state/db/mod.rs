use color_eyre::eyre::Result;
use opendal::Operator;

mod file;
pub use file::Fs;

pub fn fs_operator(path: &str) -> Result<opendal::Operator> {
    let mut builder = opendal::services::Fs::default();
    builder.root(path);

    let op: Operator = Operator::new(builder)?.finish();
    Ok(op)
}
