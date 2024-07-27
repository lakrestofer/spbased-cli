use anyhow::{anyhow, Result};
use std::{
    ffi::{OsStr, OsString},
    io::ErrorKind,
    path::PathBuf,
};

use normalize_path::NormalizePath;

use inquire::{
    autocompletion::{Autocomplete, Replacement},
    ui::RenderConfig,
    CustomUserError, Text,
};

pub struct PathPromt<'a> {
    text_promt: Text<'a>,
}

impl<'a> PathPromt<'a> {
    pub fn new(message: &'static str) -> Self {
        let text_promt = Text {
            message,
            initial_value: Some("."),
            default: None,
            placeholder: None,
            help_message: None,
            formatter: &|s| format!("{s}"),
            autocompleter: Some(Box::new(FilePathCompleter::new(3))),
            validators: vec![],
            page_size: 3,
            render_config: RenderConfig::default(),
        };

        Self { text_promt }
    }

    pub fn prompt_skippable(self) -> Result<Option<String>> {
        self.text_promt
            .prompt_skippable()
            .map_err(|e| anyhow!("Could not retrieve path: {}", e))
    }
}

#[derive(Clone, Default)]
pub struct FilePathCompleter {
    input: String,
    paths: Vec<String>,
    n_completions: usize,
}
impl FilePathCompleter {
    pub fn new(n_completions: usize) -> Self {
        Self {
            n_completions,
            ..Default::default()
        }
    }
}

impl FilePathCompleter {
    fn update_input(&mut self, input: &str) -> Result<(), CustomUserError> {
        if input == self.input {
            return Ok(());
        }
        self.input = input.to_owned();
        self.paths.clear();

        let current_dir = std::path::PathBuf::from(".");
        let empty = std::path::PathBuf::from("");

        let input_path = std::path::PathBuf::from(input).normalize();

        // from where do we search for completions
        let (scan_dir, file_name): (PathBuf, OsString) = {
            if input_path.is_dir() {
                // the provided directory
                (input_path, OsString::from(""))
            } else {
                // the parent directory or current directory is there is none
                let parent = input_path
                    .parent()
                    .map(|p| p.to_owned())
                    .filter(|p| p != &empty)
                    .unwrap_or(current_dir);
                let file_name = input_path
                    .file_name()
                    .map(ToOwned::to_owned)
                    .unwrap_or(OsString::from(""));
                (parent, file_name)
            }
        };

        // we retrieve all the directories in scan_dir
        // that begin with input_path
        if let Ok(dirs) = scan_dir.read_dir() {
            let entries: Vec<String> = dirs
                .filter_map(|e| e.ok()) // the entries that we are allowed to read
                .filter(|e| e.path().is_dir()) // the entries that are directories
                .filter(|e| e.file_name().ge(&file_name))
                // .filter_map(|e| std::fs::canonicalize(e.path()).ok())
                .filter_map(|e| e.path().to_str().map(ToOwned::to_owned))
                .take(self.n_completions)
                .collect();

            self.paths = entries;
        }

        Ok(())
    }
}

impl Autocomplete for FilePathCompleter {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        self.update_input(input)?;

        Ok(self.paths.clone())
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Replacement, CustomUserError> {
        self.update_input(input)?;

        Ok(match highlighted_suggestion {
            Some(suggestion) => Replacement::Some(suggestion),
            None => Replacement::None,
        })
    }
}
