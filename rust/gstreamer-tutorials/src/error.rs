use gstreamer::glib::BoolError;
use gstreamer::{glib, StateChangeError};

#[derive(Debug)]
pub enum Error {
    Glib(glib::Error),
    StateChange(StateChangeError),
    Bool(BoolError),
    Literal(&'static str),
}

impl From<glib::Error> for Error {
    fn from(err: glib::Error) -> Self {
        Self::Glib(err)
    }
}

impl From<StateChangeError> for Error {
    fn from(err: StateChangeError) -> Self {
        Self::StateChange(err)
    }
}

impl From<glib::BoolError> for Error {
    fn from(err: BoolError) -> Self {
        Self::Bool(err)
    }
}

impl From<&'static str> for Error {
    fn from(err: &'static str) -> Self {
        Self::Literal(err)
    }
}
