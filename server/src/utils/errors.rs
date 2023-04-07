use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeneralErrors {
    #[error("Can't create file")]
    CreateFileError,
    #[error("Can't delete file")]
    DeleteFileError,
    #[error("Can't read fro, file")]
    ReadFileError,
    #[error("Can't write to file")]
    WriteFileError,
    #[error("Can't parse file")]
    DeserializationError,
    #[error("Can't serialize file")]
    SerializationError,
}
