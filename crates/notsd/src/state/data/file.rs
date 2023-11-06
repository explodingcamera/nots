use color_eyre::eyre::Result;
use opendal::Operator as Op;

#[derive(Clone)]
pub struct Fs(pub Op);
