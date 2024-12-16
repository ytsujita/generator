use askama::Template;

use crate::{aws::AwsCommandError, utils::create_file};

#[derive(Template)]
#[template(path = "aws/api_gateway_handler/main.rs", escape = "none")]
struct ApiGatewayHandlerMain {}

pub(crate) fn generate_api_gateway_lambda_handler(
    overwrite_conflict_file: bool,
    skip_conflict_file: bool,
) -> Result<(), AwsCommandError> {
    let main = ApiGatewayHandlerMain {};
    let main_render_result = main.render().unwrap();
    let file_name = "main.rs";
    create_file(
        file_name,
        main_render_result.as_bytes(),
        overwrite_conflict_file,
        skip_conflict_file,
    )?;
    Ok(())
}
