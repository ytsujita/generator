use crate::utils::create_file;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct Config {
    pub(crate) route_path_config: RoutePathConfig,
    pub(crate) use_case_config: UseCaseConfig,
    pub(crate) provider_config: ProviderConfig,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct DomainConfig {
    pub(crate) entities: Option<Vec<DartType>>,
    pub(crate) repositories: Option<Vec<Repository>>,
    pub(crate) services: Option<Vec<Service>>,
    pub(crate) common_exceptions: Option<Vec<ExceptionConfig>>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Repository {
    pub(crate) name: String,
    pub(crate) exceptions: Option<Vec<ExceptionConfig>>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Service {
    pub(crate) name: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct ExceptionConfig {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) fields: Option<Vec<DartClassField>>,
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
pub(crate) struct UseCase {
    pub(crate) name: String,
    pub(crate) use_case_type: UseCaseType,
    pub(crate) return_type: String,
    pub(crate) is_future_call: bool,
    pub(crate) dir_name: Option<String>,
    pub(crate) exceptions: Vec<ExceptionConfig>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct RoutePathConfig {
    pub(crate) default_route_path_name: RouteType,
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

#[derive(Deserialize, Serialize)]
pub(crate) struct ProviderConfig {
    pub(crate) use_riverpod_generator: bool,
    pub(crate) providers: Option<Vec<Provider>>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct Provider {
    pub(crate) name: String,
    pub(crate) dir_name: Option<String>,
    pub(crate) state: DartType,
    pub(crate) auto_dispose: bool,
    pub(crate) family_type: Option<DartType>,
    pub(crate) provider_type: ProviderType,
}

#[derive(Deserialize, Serialize)]
pub(crate) enum ProviderType {
    Provider,
    NotifierProvider,
    FutureProvider,
    StreamProvider,
    AsyncNotifierProvider,
    StreamNotifierProvider,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub(crate) enum DartType {
    Literal(DartAvailableFieldType),
    Class(DartClass),
}

#[derive(Deserialize, Serialize)]
pub(crate) struct DartClass {
    pub(crate) name: String,
    pub(crate) fields: Option<Vec<DartClassField>>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct DartClassField {
    pub(crate) name: String,
    pub(crate) dart_type: DartType,
    pub(crate) nullable: bool,
}

#[derive(Deserialize, Serialize)]
pub(crate) enum DartAvailableFieldType {
    String,
    Int,
    Dynamic,
    Bool,
    Double,
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
                        ExceptionConfig {
                            name: String::from("Sample"),
                            description: None,
                            fields: Some(vec![DartClassField {
                                name: String::from("message"),
                                dart_type: DartType::Literal(DartAvailableFieldType::String),
                                nullable: true,
                            }]),
                        },
                        ExceptionConfig {
                            name: String::from("Sample2"),
                            description: None,
                            fields: Some(vec![DartClassField {
                                name: String::from("message"),
                                dart_type: DartType::Literal(DartAvailableFieldType::String),
                                nullable: true,
                            }]),
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
                        ExceptionConfig {
                            name: String::from("Sample"),
                            description: None,
                            fields: Some(vec![DartClassField {
                                name: String::from("message"),
                                dart_type: DartType::Literal(DartAvailableFieldType::String),
                                nullable: true,
                            }]),
                        },
                        ExceptionConfig {
                            name: String::from("Sample2"),
                            description: None,
                            fields: Some(vec![DartClassField {
                                name: String::from("message"),
                                dart_type: DartType::Literal(DartAvailableFieldType::String),
                                nullable: true,
                            }]),
                        },
                    ],
                    use_case_type: UseCaseType::Command,
                },
                UseCase {
                    name: String::from("Sample3"),
                    dir_name: None,
                    return_type: String::from("SampleReturnType3"),
                    is_future_call: false,
                    exceptions: vec![ExceptionConfig {
                        name: String::from("Sample"),
                        description: None,
                        fields: Some(vec![DartClassField {
                            name: String::from("message"),
                            dart_type: DartType::Literal(DartAvailableFieldType::String),
                            nullable: true,
                        }]),
                    }],
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
            default_route_path_name: RouteType::RoutePath(RoutePath {
                name: String::from("SignIn"),
                uri: Some(String::from("/sign-in")),
                path_reg_exp: Some(String::from("^/sign-in$")),
                dir_name: Some(String::from("auth")),
                children: None,
            }),
        },
        provider_config: ProviderConfig {
            providers: Some(vec![
                Provider {
                    name: String::from("Sample"),
                    dir_name: Some(String::from("auth")),
                    state: DartType::Literal(DartAvailableFieldType::String),
                    auto_dispose: true,
                    family_type: None,
                    provider_type: ProviderType::Provider,
                },
                Provider {
                    name: String::from("Sample2"),
                    dir_name: Some(String::from("main")),
                    state: DartType::Class(DartClass {
                        name: String::from("ClassName"),
                        fields: Some(vec![DartClassField {
                            name: String::from("sampleField"),
                            dart_type: DartType::Literal(DartAvailableFieldType::Int),
                            nullable: false,
                        }]),
                    }),
                    auto_dispose: true,
                    family_type: None,
                    provider_type: ProviderType::Provider,
                },
            ]),
            use_riverpod_generator: false,
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
