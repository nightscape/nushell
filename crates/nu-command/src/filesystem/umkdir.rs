use nu_engine::command_prelude::*;

use std::path::PathBuf;
use uu_mkdir::mkdir;
#[cfg(not(windows))]
use uucore::mode;

#[derive(Clone)]
pub struct UMkdir;

const IS_RECURSIVE: bool = true;
const DEFAULT_MODE: u32 = 0o777;

#[cfg(not(windows))]
fn get_mode() -> u32 {
    !mode::get_umask() & DEFAULT_MODE
}

#[cfg(windows)]
fn get_mode() -> u32 {
    DEFAULT_MODE
}

impl Command for UMkdir {
    fn name(&self) -> &str {
        "mkdir"
    }

    fn usage(&self) -> &str {
        "Create directories, with intermediary directories if required using uutils/coreutils mkdir."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["directory", "folder", "create", "make_dirs", "coreutils"]
    }

    fn signature(&self) -> Signature {
        Signature::build("mkdir")
            .input_output_types(vec![(Type::Nothing, Type::Nothing)])
            .rest(
                "rest",
                SyntaxShape::Directory,
                "The name(s) of the path(s) to create.",
            )
            .switch(
                "verbose",
                "print a message for each created directory.",
                Some('v'),
            )
            .category(Category::FileSystem)
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let mut directories = call
            .rest::<String>(engine_state, stack, 0)?
            .into_iter()
            .map(PathBuf::from)
            .peekable();

        let is_verbose = call.has_flag(engine_state, stack, "verbose")?;

        if directories.peek().is_none() {
            return Err(ShellError::MissingParameter {
                param_name: "requires directory paths".to_string(),
                span: call.head,
            });
        }

        for dir in directories {
            if let Err(error) = mkdir(&dir, IS_RECURSIVE, get_mode(), is_verbose) {
                return Err(ShellError::GenericError {
                    error: format!("{}", error),
                    msg: format!("{}", error),
                    span: None,
                    help: None,
                    inner: vec![],
                });
            }
        }

        Ok(PipelineData::empty())
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Make a directory named foo",
                example: "mkdir foo",
                result: None,
            },
            Example {
                description: "Make multiple directories and show the paths created",
                example: "mkdir -v foo/bar foo2",
                result: None,
            },
        ]
    }
}
