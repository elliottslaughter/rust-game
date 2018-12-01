#[derive(Debug)]
pub enum Error {
    SdlError(String),
    WindowError(sdl2::video::WindowBuildError),
    CanvasError(sdl2::IntegerOrSdlError),
}

impl From<String> for Error {
    fn from(error: String) -> Error {
        Error::SdlError(error)
    }
}

impl From<sdl2::video::WindowBuildError> for Error {
    fn from(error: sdl2::video::WindowBuildError) -> Error {
        Error::WindowError(error)
    }
}

impl From<sdl2::IntegerOrSdlError> for Error {
    fn from(error: sdl2::IntegerOrSdlError) -> Error {
        Error::CanvasError(error)
    }
}
