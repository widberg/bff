use std::collections::BTreeSet;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::{fmt, fs};

use encoding_rs::WINDOWS_1252;

// TODO: This should all probably work better with the other tsc formats.

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Script {
    pub commands: Vec<Command>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommandArgument {
    pub string: String,
    pub number: Option<f32>,
}

impl CommandArgument {
    #[must_use]
    pub fn new(string: impl Into<String>) -> Self {
        let string = string.into();
        let numeric_string = string.strip_suffix('f').unwrap_or(&string);
        let number = match normalize(numeric_string).as_str() {
            "TRUE" | "ON" => Some(1.0),
            "FALSE" | "OFF" => Some(0.0),
            _ => numeric_string.parse::<f32>().ok(),
        };
        Self { string, number }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Command {
    pub command_name: String,
    pub arguments: Vec<CommandArgument>,
    pub line: usize,
}

#[derive(Clone, Debug)]
struct Condition {
    first: String,
    rest: Vec<(LogicalOperator, String)>,
}

impl Condition {
    fn parse(arguments: &[CommandArgument], single_name: bool) -> Result<Self, String> {
        let Some(first) = arguments.first() else {
            return Err("conditional directive requires an expression".to_owned());
        };

        if single_name && arguments.len() != 1 {
            return Err("conditional directive requires exactly one variable name".to_owned());
        }
        if arguments.len().is_multiple_of(2) {
            return Err("conditional expression must alternate names and operators".to_owned());
        }

        let mut rest = Vec::new();
        for [operator, name] in arguments[1..].as_chunks::<2>().0 {
            let operator = match operator.string.as_str() {
                "&&" => LogicalOperator::And,
                "||" => LogicalOperator::Or,
                _ => {
                    return Err(format!(
                        "unknown conditional operator {:?}",
                        operator.string
                    ));
                }
            };
            rest.push((operator, name.string.clone()));
        }

        Ok(Self {
            first: first.string.clone(),
            rest,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LogicalOperator {
    And,
    Or,
}

pub trait ScriptParser {
    type Script;
    type Error;

    fn parse(&self, input: &str) -> Result<Self::Script, Self::Error>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AsoboParser;

impl ScriptParser for AsoboParser {
    type Script = Script;
    type Error = ParseError;

    fn parse(&self, input: &str) -> Result<Script, ParseError> {
        let uncommented = strip_comments(input);
        let mut commands = Vec::new();

        for (line_index, line) in uncommented.lines().enumerate() {
            let line_number = line_index + 1;
            let arguments = tokenize(line, line_number)?;
            let Some((name, arguments)) = arguments.split_first() else {
                continue;
            };

            commands.push(Command {
                command_name: name.string.clone(),
                arguments: arguments.to_vec(),
                line: line_number,
            });
        }

        Ok(Script { commands })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "line {}: {}", self.line, self.message)
    }
}

impl Error for ParseError {}

pub trait ScriptExecutor<S> {
    type Error;

    fn execute(&mut self, script: &S) -> Result<(), Self::Error>;
}

pub trait ScriptLoader {
    fn load(&self, path: &Path) -> std::io::Result<String>;
}

#[derive(Clone, Debug)]
pub struct FileSystemScriptLoader {
    root: PathBuf,
}

impl FileSystemScriptLoader {
    #[must_use]
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }
}

impl ScriptLoader for FileSystemScriptLoader {
    fn load(&self, path: &Path) -> std::io::Result<String> {
        let bytes = fs::read(self.root.join(path))?;
        Ok(WINDOWS_1252.decode(&bytes).0.into_owned())
    }
}

pub type CommandCallback<P, L, U> =
    Rc<dyn Fn(&mut AsoboExecutor<P, L, U>, &Command) -> Result<(), ExecutionError>>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandKey {
    pub full_name: String,
    pub short_name: String,
}

impl CommandKey {
    #[must_use]
    pub fn new(command_name: impl AsRef<str>) -> Self {
        let command_name = command_name.as_ref();
        Self {
            full_name: normalize(command_name),
            short_name: command_name
                .chars()
                .filter(|character| character.is_ascii_uppercase() || character.is_ascii_digit())
                .take(15)
                .collect(),
        }
    }

    fn matches(&self, command_name: &str) -> bool {
        command_name == self.full_name
            || (!self.short_name.is_empty() && command_name == self.short_name)
    }
}

#[derive(Debug)]
pub enum ExecutionError {
    Parse {
        path: PathBuf,
        source: ParseError,
    },
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    MissingSourcePath {
        command: String,
        line: usize,
    },
    InvalidDirective {
        command: String,
        line: usize,
        message: String,
    },
    UnmatchedElse {
        line: usize,
    },
    DuplicateElse {
        line: usize,
    },
    UnmatchedEndIf {
        line: usize,
    },
    UnterminatedIf,
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse { path, source } => {
                write!(formatter, "failed to parse {}: {source}", path.display())
            }
            Self::Io { path, source } => {
                write!(formatter, "failed to load {}: {source}", path.display())
            }
            Self::MissingSourcePath { command, line } => {
                write!(formatter, "line {line}: {command} requires a script path")
            }
            Self::InvalidDirective {
                command,
                line,
                message,
            } => write!(formatter, "line {line}: invalid {command}: {message}"),
            Self::UnmatchedElse { line } => write!(formatter, "line {line}: #else without #if"),
            Self::DuplicateElse { line } => write!(formatter, "line {line}: duplicate #else"),
            Self::UnmatchedEndIf { line } => write!(formatter, "line {line}: #endif without #if"),
            Self::UnterminatedIf => formatter.write_str("script ended before #endif"),
        }
    }
}

impl Error for ExecutionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Parse { source, .. } => Some(source),
            Self::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
enum ExecutionFrame {
    CommandLine,
    Script {
        path: PathBuf,
        arguments: Vec<String>,
    },
}

#[derive(Clone, Copy, Debug)]
struct ConditionalFrame {
    parent_active: bool,
    condition: bool,
    active: bool,
    saw_else: bool,
}

pub struct AsoboExecutor<P = AsoboParser, L = FileSystemScriptLoader, U = ()> {
    parser: P,
    loader: L,
    variables: BTreeSet<String>,
    user_data: U,
    handlers: Vec<(CommandKey, CommandCallback<P, L, U>)>,
    default_callback: Option<CommandCallback<P, L, U>>,
}

impl<L> AsoboExecutor<AsoboParser, L, ()>
where
    L: ScriptLoader,
{
    #[must_use]
    pub fn new(loader: L) -> Self {
        Self::with_parser(AsoboParser, loader)
    }
}

impl<L, U> AsoboExecutor<AsoboParser, L, U>
where
    L: ScriptLoader,
{
    #[must_use]
    pub fn with_user_data(loader: L, user_data: U) -> Self {
        Self::with_parser_and_user_data(AsoboParser, loader, user_data)
    }
}

impl<P, L, U> AsoboExecutor<P, L, U> {
    pub fn set_variable(&mut self, name: impl AsRef<str>) {
        self.variables.insert(normalize(name.as_ref()));
    }

    pub fn unset_variable(&mut self, name: impl AsRef<str>) {
        self.variables.remove(&normalize(name.as_ref()));
    }

    #[must_use]
    pub fn has_variable(&self, name: impl AsRef<str>) -> bool {
        self.variables.contains(&normalize(name.as_ref()))
    }

    pub fn variables(&self) -> impl Iterator<Item = &str> {
        self.variables.iter().map(String::as_str)
    }

    #[must_use]
    pub const fn user_data(&self) -> &U {
        &self.user_data
    }

    #[must_use]
    pub const fn user_data_mut(&mut self) -> &mut U {
        &mut self.user_data
    }

    #[must_use]
    pub fn into_user_data(self) -> U {
        self.user_data
    }

    pub fn on_command<F>(&mut self, command_name: impl AsRef<str>, callback: F)
    where
        F: Fn(&mut Self, &Command) -> Result<(), ExecutionError> + 'static,
    {
        self.handlers
            .push((CommandKey::new(command_name), Rc::new(callback)));
    }

    pub fn remove_command(&mut self, command_name: impl AsRef<str>) -> bool {
        let command_name = normalize(command_name);
        let Some(index) = self
            .handlers
            .iter()
            .rposition(|(key, _)| key.matches(&command_name))
        else {
            return false;
        };
        self.handlers.remove(index);
        true
    }

    pub fn on_default<F>(&mut self, callback: F)
    where
        F: Fn(&mut Self, &Command) -> Result<(), ExecutionError> + 'static,
    {
        self.default_callback = Some(Rc::new(callback));
    }

    pub fn remove_default(&mut self) -> bool {
        self.default_callback.take().is_some()
    }
}

impl<P, L> AsoboExecutor<P, L, ()>
where
    P: ScriptParser<Script = Script, Error = ParseError>,
    L: ScriptLoader,
{
    #[must_use]
    pub fn with_parser(parser: P, loader: L) -> Self {
        Self::with_parser_and_user_data(parser, loader, ())
    }
}

impl<P, L, U> AsoboExecutor<P, L, U>
where
    P: ScriptParser<Script = Script, Error = ParseError>,
    L: ScriptLoader,
{
    #[must_use]
    pub fn with_parser_and_user_data(parser: P, loader: L, user_data: U) -> Self {
        let mut executor = Self {
            parser,
            loader,
            variables: BTreeSet::new(),
            user_data,
            handlers: Vec::new(),
            default_callback: None,
        };
        executor.register_default_handlers();
        executor
    }

    pub fn execute_file(&mut self, path: impl Into<PathBuf>) -> Result<(), ExecutionError> {
        self.execute_loaded(path.into(), Vec::new())
    }

    pub fn execute_script_text(
        &mut self,
        name: impl Into<PathBuf>,
        text: &str,
        arguments: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<(), ExecutionError> {
        let path = name.into();
        let script = self
            .parser
            .parse(text)
            .map_err(|source| ExecutionError::Parse {
                path: path.clone(),
                source,
            })?;
        let frame = ExecutionFrame::Script {
            path,
            arguments: arguments.into_iter().map(Into::into).collect(),
        };
        self.execute_script(&script, &frame)
    }

    pub fn execute_command_line(&mut self, line: &str) -> Result<(), ExecutionError> {
        let script = self
            .parser
            .parse(line)
            .map_err(|source| ExecutionError::Parse {
                path: PathBuf::from("<command line>"),
                source,
            })?;
        self.execute_script(&script, &ExecutionFrame::CommandLine)
    }

    fn execute_loaded(
        &mut self,
        path: PathBuf,
        arguments: Vec<String>,
    ) -> Result<(), ExecutionError> {
        let text = self
            .loader
            .load(&path)
            .map_err(|source| ExecutionError::Io {
                path: path.clone(),
                source,
            })?;
        let script = self
            .parser
            .parse(&text)
            .map_err(|source| ExecutionError::Parse {
                path: path.clone(),
                source,
            })?;
        self.execute_script(&script, &ExecutionFrame::Script { path, arguments })
    }

    fn execute_script(
        &mut self,
        script: &Script,
        frame: &ExecutionFrame,
    ) -> Result<(), ExecutionError> {
        let mut conditionals = Vec::new();

        for command in &script.commands {
            let command = expand_command(command, frame);
            if is_asobo_directive(&command.command_name) {
                self.execute_directive(&command, &mut conditionals)?;
            } else if is_active(&conditionals) {
                self.execute_command(command)?;
            }
        }

        if conditionals.is_empty() {
            Ok(())
        } else {
            Err(ExecutionError::UnterminatedIf)
        }
    }

    fn execute_directive(
        &mut self,
        command: &Command,
        conditionals: &mut Vec<ConditionalFrame>,
    ) -> Result<(), ExecutionError> {
        match normalize(&command.command_name).as_str() {
            "#SET" if is_active(conditionals) => {
                let name = exactly_one_argument(command)?;
                self.set_variable(&name.string);
            }
            "#UNSET" if is_active(conditionals) => {
                let name = exactly_one_argument(command)?;
                self.unset_variable(&name.string);
            }
            "#IF" | "#IFDEF" | "#IFNOT" | "#IFNDEF" => {
                let parent_active = is_active(conditionals);
                let mut value = self.evaluate(
                    &Condition::parse(
                        &command.arguments,
                        matches!(
                            normalize(&command.command_name).as_str(),
                            "#IFDEF" | "#IFNOT" | "#IFNDEF"
                        ),
                    )
                    .map_err(|message| invalid_directive(command, message))?,
                );
                if matches!(
                    normalize(&command.command_name).as_str(),
                    "#IFNOT" | "#IFNDEF"
                ) {
                    value = !value;
                }
                conditionals.push(ConditionalFrame {
                    parent_active,
                    condition: value,
                    active: parent_active && value,
                    saw_else: false,
                });
            }
            "#ELSE" => {
                require_no_arguments(command)?;
                let Some(conditional) = conditionals.last_mut() else {
                    return Err(ExecutionError::UnmatchedElse { line: command.line });
                };
                if conditional.saw_else {
                    return Err(ExecutionError::DuplicateElse { line: command.line });
                }
                conditional.saw_else = true;
                conditional.active = conditional.parent_active && !conditional.condition;
            }
            "#ENDIF" => {
                require_no_arguments(command)?;
                if conditionals.pop().is_none() {
                    return Err(ExecutionError::UnmatchedEndIf { line: command.line });
                }
            }
            "#DEFINE" if is_active(conditionals) => {
                let Some((command_name, arguments)) = command.arguments.split_first() else {
                    return Err(invalid_directive(command, "requires a command"));
                };
                self.execute_command(Command {
                    command_name: command_name.string.clone(),
                    arguments: arguments.to_vec(),
                    line: command.line,
                })?;
            }
            "#SET" | "#UNSET" | "#DEFINE" => {}
            _ => unreachable!("only Asobo directives are sent to execute_directive"),
        }

        Ok(())
    }

    fn evaluate(&self, condition: &Condition) -> bool {
        let mut value = self.has_variable(&condition.first);
        for (operator, name) in &condition.rest {
            let next = self.has_variable(name);
            value = match operator {
                LogicalOperator::And => value && next,
                LogicalOperator::Or => value || next,
            };
        }
        value
    }

    fn execute_command(&mut self, command: Command) -> Result<(), ExecutionError> {
        let name = normalize(&command.command_name);
        if let Some(handler) = self
            .handlers
            .iter()
            .rev()
            .find(|(key, _)| key.matches(&name))
            .map(|(_, handler)| Rc::clone(handler))
        {
            return handler(self, &command);
        }

        if let Some(callback) = self.default_callback.clone() {
            callback(self, &command)?;
        }

        Ok(())
    }

    fn register_default_handlers(&mut self) {
        self.on_command("SouRCe", |executor, command| {
            executor.execute_source_command(command)
        });
        self.on_command("BSouRCe", |executor, command| {
            executor.execute_source_command(command)
        });
    }

    fn execute_source_command(&mut self, command: &Command) -> Result<(), ExecutionError> {
        let Some(path) = command.arguments.first() else {
            return Err(ExecutionError::MissingSourcePath {
                command: command.command_name.clone(),
                line: command.line,
            });
        };
        let arguments = command.arguments[1..]
            .iter()
            .map(|argument| argument.string.clone())
            .collect();
        self.execute_loaded(PathBuf::from(path.string.replace('\\', "/")), arguments)
    }
}

impl<P, L, U> ScriptExecutor<Script> for AsoboExecutor<P, L, U>
where
    P: ScriptParser<Script = Script, Error = ParseError>,
    L: ScriptLoader,
{
    type Error = ExecutionError;

    fn execute(&mut self, script: &Script) -> Result<(), Self::Error> {
        self.execute_script(script, &ExecutionFrame::CommandLine)
    }
}

fn is_asobo_directive(command_name: &str) -> bool {
    matches!(
        normalize(command_name).as_str(),
        "#SET"
            | "#UNSET"
            | "#IF"
            | "#IFDEF"
            | "#IFNOT"
            | "#IFNDEF"
            | "#ELSE"
            | "#ENDIF"
            | "#DEFINE"
    )
}

fn invalid_directive(command: &Command, message: impl Into<String>) -> ExecutionError {
    ExecutionError::InvalidDirective {
        command: command.command_name.clone(),
        line: command.line,
        message: message.into(),
    }
}

fn exactly_one_argument(command: &Command) -> Result<&CommandArgument, ExecutionError> {
    if command.arguments.len() == 1 {
        Ok(&command.arguments[0])
    } else {
        Err(invalid_directive(command, "requires exactly one argument"))
    }
}

fn require_no_arguments(command: &Command) -> Result<(), ExecutionError> {
    if command.arguments.is_empty() {
        Ok(())
    } else {
        Err(invalid_directive(command, "does not accept arguments"))
    }
}

fn strip_comments(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut characters = input.chars().peekable();
    let mut in_block_comment = false;
    let mut in_quote = false;

    while let Some(character) = characters.next() {
        if in_block_comment {
            if character == '*' && characters.peek() == Some(&'/') {
                characters.next();
                in_block_comment = false;
            } else if character == '\n' {
                output.push('\n');
            }
            continue;
        }

        if in_quote {
            output.push(character);
            if character == '\\' && characters.peek() == Some(&'"') {
                output.push(characters.next().expect("peeked quote must exist"));
            } else if character == '"' {
                in_quote = false;
            }
            continue;
        }

        match character {
            '"' => {
                in_quote = true;
                output.push(character);
            }
            '/' if characters.peek() == Some(&'*') => {
                characters.next();
                in_block_comment = true;
            }
            '/' if characters.peek() == Some(&'/') => {
                characters.next();
                for next in characters.by_ref() {
                    if next == '\n' {
                        output.push('\n');
                        break;
                    }
                }
            }
            _ => output.push(character),
        }
    }

    output
}

fn tokenize(line: &str, line_number: usize) -> Result<Vec<CommandArgument>, ParseError> {
    let mut arguments = Vec::new();
    let mut current = String::new();
    let mut characters = line.chars().peekable();
    let mut in_quote = false;
    let mut token_started = false;

    while let Some(character) = characters.next() {
        match character {
            '"' => {
                in_quote = !in_quote;
                token_started = true;
            }
            '\\' if in_quote && characters.peek() == Some(&'"') => {
                current.push(characters.next().expect("peeked quote must exist"));
                token_started = true;
            }
            ' ' | '\t' if !in_quote => {
                if token_started {
                    arguments.push(CommandArgument::new(std::mem::take(&mut current)));
                    token_started = false;
                }
            }
            _ => {
                current.push(character);
                token_started = true;
            }
        }
    }

    if in_quote {
        return Err(ParseError {
            line: line_number,
            message: "unterminated quoted string".to_owned(),
        });
    }
    if token_started {
        arguments.push(CommandArgument::new(current));
    }
    Ok(arguments)
}

fn normalize(value: impl AsRef<str>) -> String {
    value.as_ref().to_ascii_uppercase()
}

fn is_active(conditionals: &[ConditionalFrame]) -> bool {
    conditionals
        .last()
        .is_none_or(|conditional| conditional.active)
}

fn expand_command(command: &Command, frame: &ExecutionFrame) -> Command {
    Command {
        command_name: expand(&command.command_name, frame),
        arguments: command
            .arguments
            .iter()
            .map(|argument| CommandArgument::new(expand(&argument.string, frame)))
            .collect(),
        line: command.line,
    }
}

fn expand(value: &str, frame: &ExecutionFrame) -> String {
    let ExecutionFrame::Script { path, arguments } = frame else {
        return value.to_owned();
    };

    let mut output = String::with_capacity(value.len());
    let mut characters = value.chars().peekable();

    while let Some(character) = characters.next() {
        if character != '%' {
            output.push(character);
            continue;
        }

        let mut index = String::new();
        while let Some(next) = characters.peek() {
            if next.is_ascii_digit() {
                index.push(*next);
                characters.next();
            } else {
                break;
            }
        }
        if index.is_empty() {
            output.push('%');
            continue;
        }

        match index.parse::<usize>() {
            Ok(0) => output.push_str(
                path.file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or_default(),
            ),
            Ok(index) => {
                if let Some(argument) = arguments.get(index - 1) {
                    output.push_str(argument);
                }
            }
            Err(_) => output.push('%'),
        }
    }

    output
}
