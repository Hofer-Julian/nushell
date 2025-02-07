use nu_engine::CallExt;
use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{
    Category, Example, IntoInterruptiblePipelineData, PipelineData, ShellError, Signature, Span,
    SyntaxShape, Value,
};

#[derive(Clone)]
pub struct Append;

impl Command for Append {
    fn name(&self) -> &str {
        "append"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("append")
            .required("row", SyntaxShape::Any, "the row to append")
            .category(Category::Filters)
    }

    fn usage(&self) -> &str {
        "Append a row to the table."
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                example: "[0,1,2,3] | append 4",
                description: "Append one Int item",
                result: Some(Value::List {
                    vals: vec![
                        Value::test_int(0),
                        Value::test_int(1),
                        Value::test_int(2),
                        Value::test_int(3),
                        Value::test_int(4),
                    ],
                    span: Span::test_data(),
                }),
            },
            Example {
                example: "[0,1] | append [2,3,4]",
                description: "Append three Int items",
                result: Some(Value::List {
                    vals: vec![
                        Value::test_int(0),
                        Value::test_int(1),
                        Value::test_int(2),
                        Value::test_int(3),
                        Value::test_int(4),
                    ],
                    span: Span::test_data(),
                }),
            },
            Example {
                example: "[0,1] | append [2,nu,4,shell]",
                description: "Append Ints and Strings",
                result: Some(Value::List {
                    vals: vec![
                        Value::test_int(0),
                        Value::test_int(1),
                        Value::test_int(2),
                        Value::test_string("nu"),
                        Value::test_int(4),
                        Value::test_string("shell"),
                    ],
                    span: Span::test_data(),
                }),
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
        let val: Value = call.req(engine_state, stack, 0)?;
        let vec: Vec<Value> = process_value(val);
        let metadata = input.metadata();

        Ok(input
            .into_iter()
            .chain(vec)
            .into_iter()
            .into_pipeline_data(engine_state.ctrlc.clone())
            .set_metadata(metadata))
    }
}

fn process_value(val: Value) -> Vec<Value> {
    match val {
        Value::List {
            vals: input_vals,
            span: _,
        } => {
            let mut output = vec![];
            for input_val in input_vals {
                output.push(input_val);
            }
            output
        }
        _ => {
            vec![val]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        use crate::test_examples;

        test_examples(Append {})
    }
}
