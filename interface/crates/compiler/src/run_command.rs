use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use tempfile::NamedTempFile;

use crate::flags::ExtraCompilerFlags;

use super::Error;

fn format_additional_libraries<'a, P: AsRef<Path> + 'a + ?Sized>(path: &'a P) -> [&'a OsStr; 2] {
    [OsStr::new("-I"), path.as_ref().as_os_str()]
}
fn format_macro_definitions<'a>((def, value): (&'a String, &'a Option<String>)) -> String {
    if let Some(value) = value {
        format!("-D {def}={value}")
    } else {
        format!("-D {def}")
    }
}

pub(super) fn run_gpp_on_into(
    input_file: &Path,
    compiled_binary: &Path,
    compiler_flags: &ExtraCompilerFlags,
) -> Result<Arc<str>, Error> {
    let output = Command::new("g++")
        .args(
            compiler_flags
                .iter_includes()
                .flat_map(format_additional_libraries),
        )
        .args(
            compiler_flags
                .iter_definitions()
                .map(format_macro_definitions),
        )
        .arg("-o")
        .arg(compiled_binary)
        .arg(input_file)
        .output()
        .map_err(Error::CompilerInvocation)?;
    Ok(String::from_utf8(output.stderr)?.into())
}

pub fn run_gpp(
    input_file_content: &str,
    compiler_flags: &ExtraCompilerFlags,
) -> Result<Arc<str>, Error> {
    let mut input_file = NamedTempFile::with_suffix(".cpp").map_err(Error::TempfileGeneration)?;
    let compiled_binary = NamedTempFile::new().map_err(Error::TempfileGeneration)?;

    input_file
        .write_fmt(format_args!("{input_file_content}"))
        .map_err(Error::TempfileGeneration)?;

    run_gpp_on_into(input_file.path(), compiled_binary.path(), compiler_flags)
}
