use crate::utils::create_file;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct Config {
    pub(crate) route_path_config: NavigationConfig,
    pub(crate) use_case_config: ApplicationCaseConfig,
    pub(crate) riverpod_config: RiverpodConfig,
    pub(crate) domain_config: DomainConfig,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct DomainConfig {
    pub(crate) entities: Option<Vec<DartClass>>,
    pub(crate) repositories: Option<Vec<RepositoryConfig>>,
    pub(crate) services: Option<Vec<ServiceConfig>>,
    pub(crate) common_exceptions: Option<Vec<ExceptionConfig>>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct RepositoryConfig {
    pub(crate) name: String,
    pub(crate) exceptions: Option<Vec<ExceptionConfig>>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct ServiceConfig {
    pub(crate) name: String,
    pub(crate) exceptions: Option<Vec<ExceptionConfig>>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct ExceptionConfig {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) fields: Option<Vec<DartClassField>>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct ApplicationCaseConfig {
    pub(crate) use_cases: Vec<UseCaseConfig>,
}

#[derive(Deserialize, Serialize)]
pub(crate) enum UseCaseType {
    Command,
    Query,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct UseCaseConfig {
    pub(crate) name: String,
    pub(crate) use_case_type: UseCaseType,
    pub(crate) return_type: String,
    pub(crate) is_future_call: bool,
    pub(crate) dir_name: Option<String>,
    pub(crate) exceptions: Vec<ExceptionConfig>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct NavigationConfig {
    pub(crate) default_route_path: RouteConfigType,
    pub(crate) route_paths: Vec<RouteConfigType>,
}

#[derive(Deserialize, Serialize)]
pub(crate) enum RouteConfigType {
    RoutePath(RoutePathConfig),
    ShellRoute(ShellRoutePathConfig),
}

#[derive(Deserialize, Serialize)]
pub(crate) struct ShellRoutePathConfig {
    pub(crate) name: String,
    pub(crate) dir_name: Option<String>,
    pub(crate) shell_index_enum_names: Vec<String>,
    pub(crate) shells: Vec<RouteConfigType>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct RoutePathConfig {
    pub(crate) name: String,
    pub(crate) dir_name: Option<String>,
    pub(crate) uri: Option<String>,
    pub(crate) path_reg_exp: Option<String>,
    pub(crate) children: Option<Vec<RouteConfigType>>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct RiverpodConfig {
    pub(crate) use_riverpod_generator: bool,
    pub(crate) providers: Option<Vec<ProviderConfig>>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct ProviderConfig {
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
    NewClass(DartClass),
    RefClass(DartClassRef),
}

#[derive(Deserialize, Serialize)]
pub(crate) struct DartClassRef {
    pub(crate) name: String,
    pub(crate) path: String,
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
    pub(crate) is_final: bool,
}

#[derive(Deserialize, Serialize)]
pub(crate) enum DartAvailableFieldType {
    String,
    Int,
    Dynamic,
    Bool,
    Double,
}

impl std::fmt::Display for DartAvailableFieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DartAvailableFieldType::String => "String",
            DartAvailableFieldType::Int => "int",
            DartAvailableFieldType::Dynamic => "dynamic",
            DartAvailableFieldType::Bool => "bool",
            DartAvailableFieldType::Double => "double",
        };
        write!(f, "{}", s)
    }
}

pub(crate) fn generate_sample_config(
    file_name: &str,
    overwrite_conflict: bool,
    skip_conflict: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config {
        use_case_config: ApplicationCaseConfig {
            use_cases: vec![
                UseCaseConfig {
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
                                is_final: true,
                            }]),
                        },
                        ExceptionConfig {
                            name: String::from("Sample2"),
                            description: None,
                            fields: Some(vec![DartClassField {
                                name: String::from("message"),
                                dart_type: DartType::Literal(DartAvailableFieldType::String),
                                nullable: true,
                                is_final: true,
                            }]),
                        },
                    ],
                    use_case_type: UseCaseType::Query,
                },
                UseCaseConfig {
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
                                is_final: true,
                            }]),
                        },
                        ExceptionConfig {
                            name: String::from("Sample2"),
                            description: None,
                            fields: Some(vec![DartClassField {
                                name: String::from("message"),
                                dart_type: DartType::Literal(DartAvailableFieldType::String),
                                nullable: true,
                                is_final: true,
                            }]),
                        },
                    ],
                    use_case_type: UseCaseType::Command,
                },
                UseCaseConfig {
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
                            is_final: true,
                        }]),
                    }],
                    use_case_type: UseCaseType::Command,
                },
            ],
        },
        route_path_config: NavigationConfig {
            route_paths: vec![
                RouteConfigType::RoutePath(RoutePathConfig {
                    name: String::from("SignIn"),
                    uri: Some(String::from("/sign-in")),
                    path_reg_exp: Some(String::from("^/sign-in$")),
                    dir_name: Some(String::from("auth")),
                    children: Some(vec![
                        RouteConfigType::RoutePath(RoutePathConfig {
                            name: String::from("SignUp"),
                            uri: Some(String::from("/sign-up")),
                            path_reg_exp: Some(String::from("^/sign-up$")),
                            dir_name: Some(String::from("auth")),
                            children: None,
                        }),
                        RouteConfigType::RoutePath(RoutePathConfig {
                            name: String::from("ForgotPassword"),
                            uri: Some(String::from("/forgot-password")),
                            path_reg_exp: Some(String::from("^/forgot-password$")),
                            dir_name: Some(String::from("auth")),
                            children: Some(vec![RouteConfigType::RoutePath(RoutePathConfig {
                                name: String::from("ResetPassword"),
                                uri: Some(String::from("/reset-password")),
                                dir_name: Some(String::from("auth")),
                                path_reg_exp: Some(String::from("^/reset-password$")),
                                children: None,
                            })]),
                        }),
                    ]),
                }),
                RouteConfigType::RoutePath(RoutePathConfig {
                    name: String::from("VerifyAccount"),
                    dir_name: Some(String::from("auth")),
                    uri: Some(String::from("/verify-account")),
                    path_reg_exp: Some(String::from("^/verify-account$")),
                    children: None,
                }),
                RouteConfigType::RoutePath(RoutePathConfig {
                    name: String::from("FetchLoading"),
                    uri: None,
                    path_reg_exp: None,
                    children: None,
                    dir_name: Some(String::from("auth")),
                }),
                RouteConfigType::RoutePath(RoutePathConfig {
                    name: String::from("SessionTimeout"),
                    uri: None,
                    path_reg_exp: None,
                    children: None,
                    dir_name: Some(String::from("auth")),
                }),
                RouteConfigType::ShellRoute(ShellRoutePathConfig {
                    name: String::from("Main"),
                    dir_name: Some(String::from("main")),
                    shell_index_enum_names: vec![
                        String::from("SampleShell1"),
                        String::from("SampleShell2"),
                        String::from("SampleShell3"),
                    ],
                    shells: vec![
                        RouteConfigType::RoutePath(RoutePathConfig {
                            name: String::from("Sample1"),
                            uri: Some(String::from("/sample1-url")),
                            path_reg_exp: Some(String::from("^/sample1-url$")),
                            children: None,
                            dir_name: Some(String::from("main")),
                        }),
                        RouteConfigType::RoutePath(RoutePathConfig {
                            name: String::from("Sample2"),
                            uri: Some(String::from("/sample2-url")),
                            path_reg_exp: Some(String::from("^/sample2-url$")),
                            children: None,
                            dir_name: Some(String::from("main")),
                        }),
                        RouteConfigType::RoutePath(RoutePathConfig {
                            name: String::from("Sample3"),
                            uri: Some(String::from("/sample3-url")),
                            path_reg_exp: Some(String::from("^/sample3-url$")),
                            children: None,
                            dir_name: Some(String::from("main")),
                        }),
                    ],
                }),
            ],
            default_route_path: RouteConfigType::RoutePath(RoutePathConfig {
                name: String::from("SignIn"),
                uri: Some(String::from("/sign-in")),
                path_reg_exp: Some(String::from("^/sign-in$")),
                dir_name: Some(String::from("auth")),
                children: None,
            }),
        },
        provider_config: RiverpodConfig {
            providers: Some(vec![
                ProviderConfig {
                    name: String::from("Sample"),
                    dir_name: Some(String::from("auth")),
                    state: DartType::Literal(DartAvailableFieldType::String),
                    auto_dispose: true,
                    family_type: None,
                    provider_type: ProviderType::Provider,
                },
                ProviderConfig {
                    name: String::from("Sample2"),
                    dir_name: Some(String::from("main")),
                    state: DartType::NewClass(DartClass {
                        name: String::from("ClassName"),
                        fields: Some(vec![DartClassField {
                            name: String::from("sampleField"),
                            dart_type: DartType::Literal(DartAvailableFieldType::Int),
                            nullable: false,
                            is_final: true,
                        }]),
                    }),
                    auto_dispose: true,
                    family_type: None,
                    provider_type: ProviderType::Provider,
                },
            ]),
            use_riverpod_generator: false,
        },
        domain_config: DomainConfig {
            entities: Some(vec![DartClass {
                name: String::from("SampleClass"),
                fields: Some(vec![DartClassField {
                    name: String::from("sampleField"),
                    dart_type: DartType::Literal(DartAvailableFieldType::Int),
                    nullable: false,
                    is_final: true,
                }]),
            }]),
            repositories: Some(vec![RepositoryConfig {
                name: String::from("smpleRepository"),
                exceptions: Some(vec![ExceptionConfig {
                    name: String::from("sampleException"),
                    description: Some(String::from("これはサンプルの説明")),
                    fields: Some(vec![DartClassField {
                        name: String::from("field1"),
                        dart_type: DartType::Literal(DartAvailableFieldType::Int),
                        nullable: false,
                        is_final: true,
                    }]),
                }]),
            }]),
            services: Some(vec![ServiceConfig {
                name: String::from("smpleRepository"),
                exceptions: Some(vec![ExceptionConfig {
                    name: String::from("sampleException"),
                    description: Some(String::from("これはサンプルの説明")),
                    fields: Some(vec![DartClassField {
                        name: String::from("field1"),
                        dart_type: DartType::Literal(DartAvailableFieldType::Int),
                        nullable: false,
                        is_final: true,
                    }]),
                }]),
            }]),
            common_exceptions: Some(vec![ExceptionConfig {
                name: String::from("sampleException"),
                description: Some(String::from("これはサンプルの説明")),
                fields: Some(vec![DartClassField {
                    name: String::from("field1"),
                    dart_type: DartType::Literal(DartAvailableFieldType::Int),
                    nullable: false,
                    is_final: true,
                }]),
            }]),
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
