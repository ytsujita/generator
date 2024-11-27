use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::utils::input_yes;

#[derive(Deserialize, Serialize)]
pub(crate) struct Config {
    pub(crate) route_path_config: RoutePathConfig,
    pub(crate) use_case_config: UseCaseConfig,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct UseCaseConfig {
    pub(crate) use_cases: Vec<UseCase>,
}

#[derive(Deserialize, Serialize)]
pub(crate) enum UseCaseType {
    Command,
    Query,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct UseCaseExceptionConfig {
    pub(crate) name: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct UseCase {
    pub(crate) name: String,
    pub(crate) use_case_type: UseCaseType,
    pub(crate) return_type: String,
    pub(crate) is_future_call: bool,
    pub(crate) exceptions: Vec<UseCaseExceptionConfig>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct RoutePathConfig {
    pub(crate) default_route_path_name: String,
    pub(crate) route_paths: Vec<RouteType>,
}

#[derive(Deserialize, Serialize)]
pub(crate) enum RouteType {
    RoutePath(RoutePath),
    ShellRoute(ShellRoutePath),
}

#[derive(Deserialize, Serialize)]
pub(crate) struct ShellRoutePath {
    pub(crate) name: String,
    pub(crate) shell_index_enum_names: Vec<String>,
    pub(crate) shells: Vec<RouteType>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct RoutePath {
    pub(crate) name: String,
    pub(crate) uri: Option<String>,
    pub(crate) path_reg_exp: Option<String>,
    pub(crate) children: Option<Vec<RouteType>>,
}

pub(crate) fn generate_sample_config(
    file_name: &str,
    delete_all_conflict_file: bool,
    ignore_all_conflict_file: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let path_buf = PathBuf::from(file_name);
    if path_buf.exists() {
        if ignore_all_conflict_file {
            return Ok(());
        }
        if !delete_all_conflict_file
            && !input_yes(&format!(
                "{} file is exist. Do you want to overwrite it?",
                file_name,
            ))
        {
            return Ok(());
        }
    }
    let config = Config {
        use_case_config: UseCaseConfig {
            use_cases: vec![
                UseCase {
                    name: String::from("Sample"),
                    return_type: String::from("SampleReturnType"),
                    is_future_call: true,
                    exceptions: vec![
                        UseCaseExceptionConfig {
                            name: String::from("Sample"),
                        },
                        UseCaseExceptionConfig {
                            name: String::from("Sample"),
                        },
                    ],
                    use_case_type: UseCaseType::Query,
                },
                UseCase {
                    name: String::from("Sample2"),
                    return_type: String::from("SampleReturnType2"),
                    is_future_call: false,
                    exceptions: vec![
                        UseCaseExceptionConfig {
                            name: String::from("Sample"),
                        },
                        UseCaseExceptionConfig {
                            name: String::from("Sample"),
                        },
                    ],
                    use_case_type: UseCaseType::Command,
                },
            ],
        },
        route_path_config: RoutePathConfig {
            route_paths: vec![
                RouteType::RoutePath(RoutePath {
                    name: String::from("SignIn"),
                    uri: Some(String::from("/sign-in")),
                    path_reg_exp: Some(String::from("^/sign-in$")),
                    children: Some(vec![
                        RouteType::RoutePath(RoutePath {
                            name: String::from("SignUp"),
                            uri: Some(String::from("/sign-up")),
                            path_reg_exp: Some(String::from("^/sign-up$")),
                            children: None,
                        }),
                        RouteType::RoutePath(RoutePath {
                            name: String::from("ForgotPassword"),
                            uri: Some(String::from("/forgot-password")),
                            path_reg_exp: Some(String::from("^/forgot-password$")),
                            children: Some(vec![RouteType::RoutePath(RoutePath {
                                name: String::from("ResetPassword"),
                                uri: Some(String::from("/reset-password")),
                                path_reg_exp: Some(String::from("^/reset-password$")),
                                children: None,
                            })]),
                        }),
                    ]),
                }),
                RouteType::RoutePath(RoutePath {
                    name: String::from("VerifyAccount"),
                    uri: Some(String::from("/verify-account")),
                    path_reg_exp: Some(String::from("^/verify-account$")),
                    children: None,
                }),
                RouteType::RoutePath(RoutePath {
                    name: String::from("FetchLoading"),
                    uri: None,
                    path_reg_exp: None,
                    children: None,
                }),
                RouteType::RoutePath(RoutePath {
                    name: String::from("SessionTimeout"),
                    uri: None,
                    path_reg_exp: None,
                    children: None,
                }),
                RouteType::ShellRoute(ShellRoutePath {
                    name: String::from("Main"),
                    shell_index_enum_names: vec![
                        String::from("SampleShell1"),
                        String::from("SampleShell2"),
                        String::from("SampleShell3"),
                    ],
                    shells: vec![
                        RouteType::RoutePath(RoutePath {
                            name: String::from("Sample1"),
                            uri: Some(String::from("/sample1-url")),
                            path_reg_exp: Some(String::from("^/sample1-url$")),
                            children: None,
                        }),
                        RouteType::RoutePath(RoutePath {
                            name: String::from("Sample2"),
                            uri: Some(String::from("/sample2-url")),
                            path_reg_exp: Some(String::from("^/sample2-url$")),
                            children: None,
                        }),
                        RouteType::RoutePath(RoutePath {
                            name: String::from("Sample3"),
                            uri: Some(String::from("/sample3-url")),
                            path_reg_exp: Some(String::from("^/sample3-url$")),
                            children: None,
                        }),
                    ],
                }),
            ],
            default_route_path_name: String::from("SignIn"),
        },
    };
    let mut file = File::create(file_name)?;
    let config_str = serde_yaml::to_string(&config).unwrap();
    file.write_all(config_str.as_bytes())?;
    file.flush()?;
    Ok(())
}
