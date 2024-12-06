use crate::utils::create_file;
use change_case::pascal_case;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct Config {
    pub(crate) route_path_config: NavigationConfig,
    pub(crate) application_config: ApplicationConfig,
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
    pub(crate) fields: Option<Vec<DartField>>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct ApplicationConfig {
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
    pub(crate) return_type: DartType,
    pub(crate) dir_name: Option<String>,
    pub(crate) args: Option<Vec<DartField>>,
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
    pub(crate) shell_index: ShellIndexType,
    pub(crate) shells: std::collections::HashMap<String, RouteConfigType>,
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) enum ShellIndexType {
    String(Vec<String>),
    Int(Vec<i64>),
    Enum(Vec<String>),
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
    Int,
    Double,
    String,
    Bool,
    Dynamic,
    List(Box<DartType>),
    Map(Box<DartType>, Box<DartType>),
    Set(Box<DartType>),
    Future(Box<DartType>),
    Stream(Box<DartType>),
    NewClass(DartClass),
    RefClass(DartClassRef),
    Void,
    Function(Box<DartFunc>),
}

#[derive(Deserialize, Serialize)]
pub(crate) struct DartFunc {
    pub(crate) return_type: DartType,
    pub(crate) args: Vec<DartArg>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct DartArg {
    pub(crate) name: String,
    pub(crate) required: bool,
    pub(crate) arg_type: DartType,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct DartClassRef {
    pub(crate) name: String,
    pub(crate) path: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct DartClass {
    pub(crate) name: String,
    pub(crate) fields: Option<Vec<DartField>>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct DartField {
    pub(crate) name: String,
    pub(crate) dart_type: DartType,
    pub(crate) nullable: bool,
    pub(crate) is_final: bool,
}

impl std::fmt::Display for DartArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        if self.required {
            s.push_str("required");
        }
        s.push_str(&format!("{} {}", self.arg_type, self.name));
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for DartType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DartType::Int => "int",
            DartType::Double => "double",
            DartType::String => "String",
            DartType::Bool => "bool",
            DartType::Dynamic => "dynamic",
            DartType::List(v) => &format!("List<{}>", v),
            DartType::Map(k, v) => &format!("Map<{}, {}>", k, v),
            DartType::Set(v) => &format!("Set<{}>", v),
            DartType::Future(v) => &format!("Future<{}>", v),
            DartType::Stream(v) => &format!("Stream<{}>", v),
            DartType::NewClass(v) => &pascal_case(&v.name).to_string(),
            DartType::RefClass(v) => &pascal_case(&v.name).to_string(),
            DartType::Void => "void",
            DartType::Function(v) => &format!(
                "{} Function({})",
                v.return_type,
                v.args
                    .iter()
                    .map(|f| format!("{{{}}}", f))
                    .collect::<Vec<String>>()
                    .join(", "),
            ),
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
        application_config: ApplicationConfig {
            use_cases: vec![
                UseCaseConfig {
                    name: String::from("signIn"),
                    dir_name: Some(String::from("auth")),
                    return_type: DartType::Future(Box::new(DartType::RefClass(DartClassRef {
                        name: String::from("SignInUser"),
                        path: String::from("lib/domain/entity/auth"),
                    }))),
                    args: Some(vec![
                        DartField {
                            name: String::from("email"),
                            dart_type: DartType::String,
                            nullable: false,
                            is_final: true,
                        },
                        DartField {
                            name: String::from("password"),
                            dart_type: DartType::String,
                            nullable: false,
                            is_final: true,
                        },
                    ]),
                    exceptions: vec![
                        ExceptionConfig {
                            name: String::from("Sample"),
                            description: None,
                            fields: Some(vec![DartField {
                                name: String::from("message"),
                                dart_type: DartType::String,
                                nullable: true,
                                is_final: true,
                            }]),
                        },
                        ExceptionConfig {
                            name: String::from("Sample2"),
                            description: None,
                            fields: Some(vec![DartField {
                                name: String::from("message"),
                                dart_type: DartType::String,
                                nullable: true,
                                is_final: true,
                            }]),
                        },
                    ],
                    use_case_type: UseCaseType::Query,
                },
                UseCaseConfig {
                    name: String::from("SignOut"),
                    dir_name: Some(String::from("auth")),
                    return_type: DartType::Void,
                    args: None,
                    exceptions: vec![
                        ExceptionConfig {
                            name: String::from("Sample"),
                            description: None,
                            fields: Some(vec![DartField {
                                name: String::from("message"),
                                dart_type: DartType::String,
                                nullable: true,
                                is_final: true,
                            }]),
                        },
                        ExceptionConfig {
                            name: String::from("Sample2"),
                            description: None,
                            fields: Some(vec![DartField {
                                name: String::from("message"),
                                dart_type: DartType::String,
                                nullable: true,
                                is_final: true,
                            }]),
                        },
                    ],
                    use_case_type: UseCaseType::Command,
                },
                UseCaseConfig {
                    name: String::from("ForgotPassword"),
                    dir_name: Some(String::from("auth")),
                    return_type: DartType::Void,
                    args: Some(vec![DartField {
                        name: String::from("email"),
                        dart_type: DartType::String,
                        nullable: false,
                        is_final: true,
                    }]),
                    exceptions: vec![ExceptionConfig {
                        name: String::from("Sample"),
                        description: None,
                        fields: Some(vec![DartField {
                            name: String::from("message"),
                            dart_type: DartType::String,
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
                    shell_index: ShellIndexType::Enum(vec![
                        String::from("SampleShell1"),
                        String::from("SampleShell2"),
                        String::from("SampleShell3"),
                    ]),
                    shells: (vec![
                        (
                            String::from("SampleShell1"),
                            RouteConfigType::RoutePath(RoutePathConfig {
                                name: String::from("Sample1"),
                                uri: Some(String::from("/sample1-url")),
                                path_reg_exp: Some(String::from("^/sample1-url$")),
                                children: None,
                                dir_name: Some(String::from("main")),
                            }),
                        ),
                        (
                            String::from("SampleShell1"),
                            RouteConfigType::RoutePath(RoutePathConfig {
                                name: String::from("Sample2"),
                                uri: Some(String::from("/sample2-url")),
                                path_reg_exp: Some(String::from("^/sample2-url$")),
                                children: None,
                                dir_name: Some(String::from("main")),
                            }),
                        ),
                        (
                            String::from("SampleShell1"),
                            RouteConfigType::RoutePath(RoutePathConfig {
                                name: String::from("Sample3"),
                                uri: Some(String::from("/sample3-url")),
                                path_reg_exp: Some(String::from("^/sample3-url$")),
                                children: None,
                                dir_name: Some(String::from("main")),
                            }),
                        ),
                    ])
                    .into_iter()
                    .collect(),
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
        riverpod_config: RiverpodConfig {
            providers: Some(vec![
                ProviderConfig {
                    name: String::from("Sample"),
                    dir_name: Some(String::from("auth")),
                    state: DartType::String,
                    auto_dispose: true,
                    family_type: None,
                    provider_type: ProviderType::Provider,
                },
                ProviderConfig {
                    name: String::from("Sample2"),
                    dir_name: Some(String::from("main")),
                    state: DartType::RefClass(DartClassRef {
                        name: String::from("sample"),
                        path: String::from("lib/domain/entity"),
                    }),
                    auto_dispose: true,
                    family_type: None,
                    provider_type: ProviderType::Provider,
                },
                ProviderConfig {
                    name: String::from("Sample2"),
                    dir_name: Some(String::from("main")),
                    state: DartType::NewClass(DartClass {
                        name: String::from("ClassName"),
                        fields: Some(vec![DartField {
                            name: String::from("sampleField"),
                            dart_type: DartType::Int,
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
                fields: Some(vec![DartField {
                    name: String::from("sampleField"),
                    dart_type: DartType::Int,
                    nullable: false,
                    is_final: true,
                }]),
            }]),
            repositories: Some(vec![RepositoryConfig {
                name: String::from("smple"),
                exceptions: Some(vec![ExceptionConfig {
                    name: String::from("sampleException"),
                    description: Some(String::from("これはサンプルの説明")),
                    fields: Some(vec![DartField {
                        name: String::from("field1"),
                        dart_type: DartType::Int,
                        nullable: false,
                        is_final: true,
                    }]),
                }]),
            }]),
            services: Some(vec![ServiceConfig {
                name: String::from("smple"),
                exceptions: Some(vec![ExceptionConfig {
                    name: String::from("sampleException"),
                    description: Some(String::from("これはサンプルの説明")),
                    fields: Some(vec![DartField {
                        name: String::from("field1"),
                        dart_type: DartType::Int,
                        nullable: false,
                        is_final: true,
                    }]),
                }]),
            }]),
            common_exceptions: Some(vec![ExceptionConfig {
                name: String::from("sampleException"),
                description: Some(String::from("これはサンプルの説明")),
                fields: Some(vec![DartField {
                    name: String::from("field1"),
                    dart_type: DartType::Int,
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
