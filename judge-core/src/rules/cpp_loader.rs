use super::SeccompCtxLoader;
use libseccomp::{error::SeccompError, get_syscall_from_name, ScmpAction, ScmpFilterContext};

pub struct CppLoader {
    pub ctx: ScmpFilterContext,
}

impl SeccompCtxLoader for CppLoader {
    fn add_rules(&mut self) -> Result<(), SeccompError> {
        for syscall_name in get_white_list() {
            self.ctx.add_rule_exact(
                ScmpAction::Allow,
                get_syscall_from_name(syscall_name, None)?,
            )?;
        }
        Ok(())
    }

    fn load_ctx(&self) -> Result<(), SeccompError> {
        self.ctx.load()?;
        Ok(())
    }
}

fn get_white_list() -> Vec<&'static str> {
    let white_list = vec![
        "read",
        "fstat",
        "mmap",
        "mprotect",
        "munmap",
        "uname",
        "arch_prctl",
        "brk",
        "access",
        "exit_group",
        "close",
        "readlink",
        "sysinfo",
        "write",
        "writev",
        "lseek",
        "clock_gettime",
        "pread64",
        "execve",
        "open",
        "openat",
    ];

    white_list
}
