use anyhow::Result;
use std::{io::Write, marker::PhantomData};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::utils::{
    compile::ScarbProjectCompiler, generate::ScarbProjectGenerator, parse::SierraParser,
    run::SierraRunner,
};

pub struct LoggerGenerator<T, U>(pub T, PhantomData<U>)
where
    T: ScarbProjectGenerator<U>;

pub struct LoggerCompiler<T, U>(pub T, PhantomData<U>)
where
    T: ScarbProjectCompiler<U>;

pub struct LoggerParser<T, U>(pub T, PhantomData<U>)
where
    T: SierraParser<U>;

pub struct LoggerRunner<T, U>(pub T, String, PhantomData<U>)
where
    T: SierraRunner<U>;

impl<T, U> LoggerGenerator<T, U>
where
    T: ScarbProjectGenerator<U>,
{
    pub fn new(generator: T) -> Self {
        LoggerGenerator(generator, PhantomData)
    }
}

impl<T, U> LoggerCompiler<T, U>
where
    T: ScarbProjectCompiler<U>,
{
    pub fn new(compiler: T) -> Self {
        LoggerCompiler(compiler, PhantomData)
    }
}

impl<T, U> LoggerParser<T, U>
where
    T: SierraParser<U>,
{
    pub fn new(parser: T) -> Self {
        LoggerParser(parser, PhantomData)
    }
}

impl<T, U> LoggerRunner<T, U>
where
    T: SierraRunner<U>,
{
    #[allow(dead_code)]
    pub fn new(runner: T) -> Self {
        LoggerRunner::with_name(runner, "")
    }

    pub fn with_name(runner: T, name: impl Into<String>) -> Self {
        LoggerRunner(runner, name.into(), PhantomData)
    }

    pub fn new_vec(runners: Vec<T>) -> Vec<Self> {
        runners
            .into_iter()
            .enumerate()
            .map(|(i, r)| LoggerRunner::with_name(r, format!("Runner {}", i + 1)))
            .collect()
    }
}

impl<T, U> ScarbProjectGenerator<U> for LoggerGenerator<T, U>
where
    T: ScarbProjectGenerator<U>,
{
    fn generate(self) -> Result<U> {
        print_blue("Generating cairo code...\n");
        self.0.generate()
    }
}

impl<T, U> ScarbProjectCompiler<U> for LoggerCompiler<T, U>
where
    T: ScarbProjectCompiler<U>,
{
    fn compile(self) -> Result<U> {
        print_blue("Compiling cairo to sierra...\n");
        self.0.compile()
    }
}

impl<T, U> SierraParser<U> for LoggerParser<T, U>
where
    T: SierraParser<U>,
{
    fn parse(self) -> Result<U> {
        print_blue("Parsing sierra code...\n");
        self.0.parse()
    }
}

fn print_blue(message: impl Into<String>) {
    print_color(message, Color::Blue)
}

fn print_color(message: impl Into<String>, color: Color) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut binding = ColorSpec::new();
    binding.set_fg(Some(color)).set_bold(true);
    stdout.set_color(&binding).unwrap();
    write!(stdout, "{}", message.into()).unwrap();
    stdout.reset().unwrap();
}
