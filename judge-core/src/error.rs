use libseccomp::error::SeccompError;
use nix::errno::Errno;
use std::ffi::NulError;
use std::io;

#[derive(Debug)]
pub enum JudgeCoreError {
    NixErrno(Errno),
    SeccompError(SeccompError),
    FFINulError(NulError),
    IOError(io::Error),
}

impl From<Errno> for JudgeCoreError {
    fn from(error: Errno) -> JudgeCoreError {
        JudgeCoreError::NixErrno(error)
    }
}

impl From<SeccompError> for JudgeCoreError {
    fn from(error: SeccompError) -> JudgeCoreError {
        JudgeCoreError::SeccompError(error)
    }
}

impl From<NulError> for JudgeCoreError {
    fn from(error: NulError) -> JudgeCoreError {
        JudgeCoreError::FFINulError(error)
    }
}

impl From<io::Error> for JudgeCoreError {
    fn from(error: io::Error) -> JudgeCoreError {
        JudgeCoreError::IOError(error)
    }
}
