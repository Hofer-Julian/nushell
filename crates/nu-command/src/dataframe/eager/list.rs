use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    Category, Example, IntoPipelineData, PipelineData, ShellError, Signature, Value,
};

use crate::dataframe::values::NuDataFrame;

#[derive(Clone)]
pub struct ListDF;

impl Command for ListDF {
    fn name(&self) -> &str {
        "dfr list"
    }

    fn usage(&self) -> &str {
        "Lists stored dataframes"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name()).category(Category::Custom("dataframe".into()))
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Creates a new dataframe and shows it in the dataframe list",
            example: r#"let test = ([[a b];[1 2] [3 4]] | dfr to-df);
    dfr list"#,
            result: None,
        }]
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let vals = engine_state
            .scope
            .iter()
            .flat_map(|frame| {
                frame
                    .vars
                    .iter()
                    .filter_map(|var| {
                        let value = stack.get_var(*var.1, call.head);
                        match value {
                            Ok(value) => {
                                let name = String::from_utf8_lossy(var.0).to_string();
                                Some((name, value))
                            }
                            Err(_) => None,
                        }
                    })
                    .collect::<Vec<(String, Value)>>()
            })
            .filter_map(|(name, value)| match NuDataFrame::try_from_value(value) {
                Ok(df) => Some((name, df)),
                Err(_) => None,
            })
            .map(|(name, df)| {
                let name = Value::String {
                    val: name,
                    span: call.head,
                };

                let columns = Value::Int {
                    val: df.as_ref().width() as i64,
                    span: call.head,
                };

                let rows = Value::Int {
                    val: df.as_ref().height() as i64,
                    span: call.head,
                };

                let cols = vec![
                    "name".to_string(),
                    "columns".to_string(),
                    "rows".to_string(),
                ];
                let vals = vec![name, columns, rows];

                Value::Record {
                    cols,
                    vals,
                    span: call.head,
                }
            })
            .collect::<Vec<Value>>();

        let list = Value::List {
            vals,
            span: call.head,
        };

        Ok(list.into_pipeline_data())
    }
}
