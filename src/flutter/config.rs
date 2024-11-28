use crate::utils::create_file;
use serde::{Deserialize, Serialize};

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
    pub(crate) dir_name: Option<String>,
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
    pub(crate) dir_name: Option<String>,
    pub(crate) shell_index_enum_names: Vec<String>,
    pub(crate) shells: Vec<RouteType>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct RoutePath {
    pub(crate) name: String,
    pub(crate) dir_name: Option<String>,
    pub(crate) uri: Option<String>,
    pub(crate) path_reg_exp: Option<String>,
    pub(crate) children: Option<Vec<RouteType>>,
}

pub(crate) fn generate_sample_config(
    file_name: &str,
    overwrite_conflict: bool,
    skip_conflict: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config {
        use_case_config: UseCaseConfig {
            use_cases: vec![
                UseCase {
                    name: String::from("Sample"),
                    dir_name: Some(String::from("auth")),
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
                    dir_name: Some(String::from("main")),
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
                UseCase {
                    name: String::from("Sample3"),
                    dir_name: None,
                    return_type: String::from("SampleReturnType3"),
                    is_future_call: false,
                    exceptions: vec![
                        UseCaseExceptionConfig {
                            name: String::from("Sample3"),
                        },
                        UseCaseExceptionConfig {
                            name: String::from("Sample3"),
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
                    dir_name: Some(String::from("auth")),
                    children: Some(vec![
                        RouteType::RoutePath(RoutePath {
                            name: String::from("SignUp"),
                            uri: Some(String::from("/sign-up")),
                            path_reg_exp: Some(String::from("^/sign-up$")),
                            dir_name: Some(String::from("auth")),
                            children: None,
                        }),
                        RouteType::RoutePath(RoutePath {
                            name: String::from("ForgotPassword"),
                            uri: Some(String::from("/forgot-password")),
                            path_reg_exp: Some(String::from("^/forgot-password$")),
                            dir_name: Some(String::from("auth")),
                            children: Some(vec![RouteType::RoutePath(RoutePath {
                                name: String::from("ResetPassword"),
                                uri: Some(String::from("/reset-password")),
                                dir_name: Some(String::from("auth")),
                                path_reg_exp: Some(String::from("^/reset-password$")),
                                children: None,
                            })]),
                        }),
                    ]),
                }),
                RouteType::RoutePath(RoutePath {
                    name: String::from("VerifyAccount"),
                    dir_name: Some(String::from("auth")),
                    uri: Some(String::from("/verify-account")),
                    path_reg_exp: Some(String::from("^/verify-account$")),
                    children: None,
                }),
                RouteType::RoutePath(RoutePath {
                    name: String::from("FetchLoading"),
                    uri: None,
                    path_reg_exp: None,
                    children: None,
                    dir_name: Some(String::from("auth")),
                }),
                RouteType::RoutePath(RoutePath {
                    name: String::from("SessionTimeout"),
                    uri: None,
                    path_reg_exp: None,
                    children: None,
                    dir_name: Some(String::from("auth")),
                }),
                RouteType::ShellRoute(ShellRoutePath {
                    name: String::from("Main"),
                    dir_name: Some(String::from("main")),
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
                            dir_name: Some(String::from("main")),
                        }),
                        RouteType::RoutePath(RoutePath {
                            name: String::from("Sample2"),
                            uri: Some(String::from("/sample2-url")),
                            path_reg_exp: Some(String::from("^/sample2-url$")),
                            children: None,
                            dir_name: Some(String::from("main")),
                        }),
                        RouteType::RoutePath(RoutePath {
                            name: String::from("Sample3"),
                            uri: Some(String::from("/sample3-url")),
                            path_reg_exp: Some(String::from("^/sample3-url$")),
                            children: None,
                            dir_name: Some(String::from("main")),
                        }),
                    ],
                }),
            ],
            default_route_path_name: String::from("SignIn"),
        },
    };
    let config_str = serde_yaml::to_string(&config).unwrap();
    create_file(
        file_name,
        config_str.as_bytes(),
        overwrite_conflict,
        skip_conflict,
    )?;
    Ok(())
}
