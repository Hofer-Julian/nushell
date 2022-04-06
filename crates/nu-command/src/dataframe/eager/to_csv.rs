use std::{fs::File, path::PathBuf};

use nu_engine::CallExt;
use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    Category, Example, PipelineData, ShellError, Signature, Spanned, SyntaxShape, Value,
};
use polars::prelude::{CsvWriter, SerWriter};

use super::super::values::NuDataFrame;

#[derive(Clone)]
pub struct ToCSV;

impl Command for ToCSV {
    fn name(&self) -> &str {
        "dfr to-csv"
    }

    fn usage(&self) -> &str {
        "Saves dataframe to csv file"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("file", SyntaxShape::Filepath, "file path to save dataframe")
            .named(
                "delimiter",
                SyntaxShape::String,
                "file delimiter character",
                Some('d'),
            )
            .switch("no-header", "Indicates if file doesn't have header", None)
            .category(Category::Custom("dataframe".into()))
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Saves dataframe to csv file",
                example: "[[a b]; [1 2] [3 4]] | dfr to-df | dfr to-csv test.csv",
                result: None,
            },
            Example {
                description: "Saves dataframe to csv file using other delimiter",
                example: "[[a b]; [1 2] [3 4]] | dfr to-df | dfr to-csv test.csv -d '|'",
                result: None,
            },
        ]
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        command(engine_state, stack, call, input)
    }
}

fn command(
    engine_state: &EngineState,
    stack: &mut Stack,
    call: &Call,
    input: PipelineData,
) -> Result<PipelineData, ShellError> {
    let file_name: Spanned<PathBuf> = call.req(engine_state, stack, 0)?;
    let delimiter: Option<Spanned<String>> = call.get_flag(engine_state, stack, "delimiter")?;
    let no_header: bool = call.has_flag("no-header");

    let mut df = NuDataFrame::try_from_pipeline(input, call.head)?;

    let mut file = File::create(&file_name.item).map_err(|e| {
        ShellError::SpannedLabeledError(
            "Error with file name".into(),
            e.to_string(),
            file_name.span,
        )
    })?;

    let writer = CsvWriter::new(&mut file);

    let writer = if no_header {
        writer.has_header(false)
    } else {
        writer.has_header(true)
    };

    let writer = match delimiter {
        None => writer,
        Some(d) => {
            if d.item.len() != 1 {
                return Err(ShellError::SpannedLabeledError(
                    "Incorrect delimiter".into(),
                    "Delimiter has to be one char".into(),
                    d.span,
                ));
            } else {
                let delimiter = match d.item.chars().next() {
                    Some(d) => d as u8,
                    None => unreachable!(),
                };

                writer.with_delimiter(delimiter)
            }
        }
    };

    writer.finish(df.as_mut()).map_err(|e| {
        ShellError::SpannedLabeledError(
            "Error writing to file".into(),
            e.to_string(),
            file_name.span,
        )
    })?;

    let file_value = Value::String {
        val: format!("saved {:?}", &file_name.item),
        span: file_name.span,
    };

    Ok(PipelineData::Value(
        Value::List {
            vals: vec![file_value],
            span: call.head,
        },
        None,
    ))
}