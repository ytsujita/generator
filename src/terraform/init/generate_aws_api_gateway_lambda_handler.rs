// use askama::Template;
//
// mod filters {
//     use change_case::{camel_case, pascal_case, snake_case};
//
//     pub fn pascal<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
//         Ok(pascal_case(&(s.to_string())))
//     }
//
//     pub fn camel<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
//         Ok(camel_case(&(s.to_string())))
//     }
//
//     pub fn snake<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
//         Ok(snake_case(&(s.to_string())))
//     }
// }
//
// #[derive(Template)]
// #[template(path = "terraform/lambda/src/main.rs", escape = "none")]
// pub(super) struct ApiGatewayLambdaHandlerMainTemplate {}
//
// #[derive(Template)]
// #[template(path = "terraform/lambda/src/handler.rs", escape = "none")]
// pub(super) struct ApiGatewayLambdaHandlerHandlerTemplate {
//     use_case_name: String,
// }
//
// #[derive(Template)]
// #[template(path = "terraform/lambda/src/use_case.rs", escape = "none")]
// pub(super) struct ApiGatewayLambdaHandlerUseCaseTemplate {
//     use_case_names: Vec<String>,
// }
//
// #[derive(Template)]
// #[template(path = "terraform/lambda/src/use_case/use_case.rs", escape = "none")]
// pub(super) struct ApiGatewayLambdaHandlerUseCaseDetailTemplate {}
//
// #[derive(Template)]
// #[template(path = "terraform/lambda/src/dynamodb.rs", escape = "none")]
// pub(super) struct ApiGatewayLambdaHandlerDynamoDBTemplate {
//     table_names: Vec<String>,
// }
//
// #[derive(Template)]
// #[template(path = "terraform/lambda/src/dynamodb/accessor.rs", escape = "none")]
// pub(super) struct ApiGatewayLambdaHandlerDynamoDBAccessorTemplate {}
//
// fn generate_lambda_function_sources(
//     dst: &Path,
//     overwrite_all: bool,
//     ignore_conflict_config_file: bool,
// ) -> Result<(), std::io::Error> {
//     if !dst.exists() {
//         fs::create_dir(dst)?;
//     }
//     let glob = "**/*";
//     for file in SRC_DIR.find(glob).unwrap() {
//         let dst_path = dst.join(file.path());
//         match file {
//             include_dir::DirEntry::Dir(d) => {
//                 fs::create_dir_all(d.path().as_os_str().to_str().unwrap()).unwrap();
//             }
//             include_dir::DirEntry::File(f) => {
//                 let file_path = f.path().as_os_str().to_str().unwrap();
//                 let file_buf: PathBuf = PathBuf::from_str(file_path).unwrap();
//                 if let Some(parent) = file_buf.parent() {
//                     if !parent.exists() {
//                         fs::create_dir_all(parent)?;
//                     }
//                 }
//                 let _ = create_file(
//                     dst_path.as_os_str().to_str().unwrap(),
//                     f.contents(),
//                     overwrite_all,
//                     ignore_conflict_config_file,
//                 );
//             }
//         }
//     }
//     Ok(())
// }
