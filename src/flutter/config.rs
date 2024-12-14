use crate::utils::create_file;
use change_case::pascal_case;
use serde::de::{self, MapAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{self, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Deserialize, Serialize)]
pub(crate) struct Config {
    pub(crate) application_name: String,
    pub(crate) copy_source: Option<Vec<CopySource>>,
    pub(crate) route_path_config: Option<NavigationConfig>,
    pub(crate) application_config: Option<ApplicationConfig>,
    pub(crate) riverpod_config: Option<RiverpodConfig>,
}

#[derive(Deserialize, Serialize)]
pub(crate) enum CopySource {
    GitHub(GitHubSource),
    CodeCommit(CodeCommitSource),
    Local(LocalSource),
}

#[derive(Deserialize, Serialize)]
pub(crate) struct GitHubSource {
    repository_name: String,
    path: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct CodeCommitSource {
    account_id: String,
    repository_name: String,
    path: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct LocalSource {
    path: Vec<String>,
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
    String,
    Int,
    Enum(Vec<String>),
}

#[derive(Deserialize, Serialize)]
pub(crate) struct RoutePathConfig {
    pub(crate) name: String,
    pub(crate) dir_name: Option<String>,
    pub(crate) uri: Option<String>,
    pub(crate) path_reg_exp: Option<String>,
    pub(crate) fields: Option<Vec<DartField>>,
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

#[derive(Debug, Clone)]
pub(crate) enum DartType {
    Int,
    Double,
    String,
    Bool,
    Dynamic,
    Tuple(Vec<(String, DartType)>),
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

impl serde::Serialize for DartType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("DartType", 2)?;
        match self {
            DartType::Int => state.serialize_field("type", "Int")?,
            DartType::Double => state.serialize_field("type", "Double")?,
            DartType::String => state.serialize_field("type", "String")?,
            DartType::Bool => state.serialize_field("type", "Bool")?,
            DartType::Dynamic => state.serialize_field("type", "Dynamic")?,
            DartType::List(inner) => {
                state.serialize_field("type", "List")?;
                state.serialize_field("inner", inner)?;
            }
            DartType::Map(key, value) => {
                state.serialize_field("type", "Map")?;
                state.serialize_field("key", key)?;
                state.serialize_field("value", value)?;
            }
            DartType::Set(inner) => {
                state.serialize_field("type", "Set")?;
                state.serialize_field("inner", inner)?;
            }
            DartType::Future(inner) => {
                state.serialize_field("type", "Future")?;
                state.serialize_field("inner", inner)?;
            }
            DartType::Stream(inner) => {
                state.serialize_field("type", "Stream")?;
                state.serialize_field("inner", inner)?;
            }
            DartType::NewClass(class) => {
                state.serialize_field("type", "NewClass")?;
                state.serialize_field("class", class)?;
            }
            DartType::RefClass(class_ref) => {
                state.serialize_field("type", "RefClass")?;
                state.serialize_field("class_ref", class_ref)?;
            }
            DartType::Void => state.serialize_field("type", "Void")?,
            DartType::Function(func) => {
                state.serialize_field("type", "Function")?;
                state.serialize_field("func", func)?;
            }
            DartType::Tuple(v) => {
                state.serialize_field("type", "Tuple")?;
                state.serialize_field("tuple", v)?;
            }
        }
        state.end()
    }
}

impl<'de> Deserialize<'de> for DartType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Type,
            Inner,
            Key,
            Value,
            Class,
            ClassRef,
            Func,
            Tuple,
        }

        struct DartTypeVisitor;

        impl<'de> Visitor<'de> for DartTypeVisitor {
            type Value = DartType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid DartType")
            }

            fn visit_map<V>(self, mut map: V) -> Result<DartType, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut type_name = None;
                let mut inner = None;
                let mut key_ = None;
                let mut value = None;
                let mut class = None;
                let mut class_ref = None;
                let mut func = None;
                let mut tuple_ = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Type => {
                            if type_name.is_some() {
                                return Err(de::Error::duplicate_field("type"));
                            }
                            type_name = Some(map.next_value()?);
                        }
                        Field::Inner => {
                            if inner.is_some() {
                                return Err(de::Error::duplicate_field("inner"));
                            }
                            inner = Some(map.next_value()?);
                        }
                        Field::Key => {
                            if key_.is_some() {
                                return Err(de::Error::duplicate_field("key"));
                            }
                            key_ = Some(map.next_value()?);
                        }
                        Field::Value => {
                            if value.is_some() {
                                return Err(de::Error::duplicate_field("value"));
                            }
                            value = Some(map.next_value()?);
                        }
                        Field::Class => {
                            if class.is_some() {
                                return Err(de::Error::duplicate_field("class"));
                            }
                            class = Some(map.next_value()?);
                        }
                        Field::ClassRef => {
                            if class_ref.is_some() {
                                return Err(de::Error::duplicate_field("class_ref"));
                            }
                            class_ref = Some(map.next_value()?);
                        }
                        Field::Func => {
                            if func.is_some() {
                                return Err(de::Error::duplicate_field("func"));
                            }
                            func = Some(map.next_value()?);
                        }
                        Field::Tuple => {
                            if tuple_.is_some() {
                                return Err(de::Error::duplicate_field("tuple"));
                            }
                            tuple_ = Some(map.next_value()?);
                        }
                    }
                }

                let type_name: String =
                    type_name.ok_or_else(|| de::Error::missing_field("type"))?;
                match type_name.as_str() {
                    "Int" => Ok(DartType::Int),
                    "Double" => Ok(DartType::Double),
                    "String" => Ok(DartType::String),
                    "Bool" => Ok(DartType::Bool),
                    "Dynamic" => Ok(DartType::Dynamic),
                    "List" => {
                        let inner = inner.ok_or_else(|| de::Error::missing_field("inner"))?;
                        Ok(DartType::List(Box::new(inner)))
                    }
                    "Map" => {
                        let key = key_.ok_or_else(|| de::Error::missing_field("key"))?;
                        let value = value.ok_or_else(|| de::Error::missing_field("value"))?;
                        Ok(DartType::Map(Box::new(key), Box::new(value)))
                    }
                    "Set" => {
                        let inner = inner.ok_or_else(|| de::Error::missing_field("inner"))?;
                        Ok(DartType::Set(Box::new(inner)))
                    }
                    "Future" => {
                        let inner = inner.ok_or_else(|| de::Error::missing_field("inner"))?;
                        Ok(DartType::Future(Box::new(inner)))
                    }
                    "Stream" => {
                        let inner = inner.ok_or_else(|| de::Error::missing_field("inner"))?;
                        Ok(DartType::Stream(Box::new(inner)))
                    }
                    "NewClass" => {
                        let class = class.ok_or_else(|| de::Error::missing_field("class"))?;
                        Ok(DartType::NewClass(class))
                    }
                    "RefClass" => {
                        let class_ref =
                            class_ref.ok_or_else(|| de::Error::missing_field("class_ref"))?;
                        Ok(DartType::RefClass(class_ref))
                    }
                    "Void" => Ok(DartType::Void),
                    "Function" => {
                        let func = func.ok_or_else(|| de::Error::missing_field("func"))?;
                        Ok(DartType::Function(Box::new(func)))
                    }
                    "Tuple" => {
                        let tuple = tuple_.ok_or_else(|| de::Error::missing_field("tuple"))?;
                        Ok(DartType::Tuple(tuple))
                    }
                    _ => Err(de::Error::unknown_variant(
                        &type_name,
                        &[
                            "Int", "Double", "String", "Bool", "Dynamic", "List", "Map", "Set",
                            "Future", "Stream", "NewClass", "RefClass", "Void", "Function",
                        ],
                    )),
                }
            }
        }

        const FIELDS: &[&str] = &[
            "type",
            "inner",
            "key",
            "value",
            "class",
            "class_ref",
            "func",
        ];
        deserializer.deserialize_struct("DartType", FIELDS, DartTypeVisitor)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct DartFunc {
    pub(crate) return_type: DartType,
    pub(crate) args: Vec<DartArg>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct DartArg {
    pub(crate) name: String,
    pub(crate) required: bool,
    pub(crate) arg_type: DartType,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct DartClassRef {
    pub(crate) name: String,
    pub(crate) path: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct DartClass {
    pub(crate) name: String,
    pub(crate) is_immutable: bool,
    pub(crate) fields: Option<Vec<DartField>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
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
            DartType::Tuple(v) => &format!(
                "({{{}}})",
                v.iter()
                    .map(|f| format!("{} {}", f.1, f.0))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        };
        write!(f, "{}", s)
    }
}

pub(crate) fn generate_sample_config(
    application_name: &str,
    file_name: &str,
    overwrite_conflict: bool,
    skip_conflict: bool,
) -> Result<(), std::io::Error> {
    let config = Config {
        application_name: application_name.to_string(),
        application_config: Some(ApplicationConfig {
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
                            name: String::from("InternalServerError"),
                            description: None,
                            fields: Some(vec![DartField {
                                name: String::from("message"),
                                dart_type: DartType::String,
                                nullable: true,
                                is_final: true,
                            }]),
                        },
                        ExceptionConfig {
                            name: String::from("NetworkNotAvailable"),
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
                            name: String::from("InternalServerError"),
                            description: None,
                            fields: Some(vec![DartField {
                                name: String::from("message"),
                                dart_type: DartType::String,
                                nullable: true,
                                is_final: true,
                            }]),
                        },
                        ExceptionConfig {
                            name: String::from("NetworkNotAvailable"),
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
                        name: String::from("NetworkNotAvailable"),
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
        }),
        route_path_config: Some(NavigationConfig {
            route_paths: vec![
                RouteConfigType::RoutePath(RoutePathConfig {
                    name: String::from("SignIn"),
                    uri: Some(String::from("/sign-in")),
                    path_reg_exp: Some(String::from("^/sign-in$")),
                    dir_name: Some(String::from("auth")),
                    fields: None,
                    children: Some(vec![
                        RouteConfigType::RoutePath(RoutePathConfig {
                            name: String::from("SignUp"),
                            uri: Some(String::from("/sign-up")),
                            path_reg_exp: Some(String::from("^/sign-up$")),
                            dir_name: Some(String::from("auth")),
                            fields: None,
                            children: None,
                        }),
                        RouteConfigType::RoutePath(RoutePathConfig {
                            name: String::from("ForgotPassword"),
                            uri: Some(String::from("/forgot-password")),
                            path_reg_exp: Some(String::from("^/forgot-password$")),
                            fields: None,
                            dir_name: Some(String::from("auth")),
                            children: Some(vec![RouteConfigType::RoutePath(RoutePathConfig {
                                name: String::from("ResetPassword"),
                                uri: Some(String::from("/reset-password")),
                                dir_name: Some(String::from("auth")),
                                fields: None,
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
                    fields: None,
                    children: None,
                }),
                RouteConfigType::RoutePath(RoutePathConfig {
                    name: String::from("FetchLoading"),
                    uri: None,
                    path_reg_exp: None,
                    children: None,
                    fields: None,
                    dir_name: Some(String::from("auth")),
                }),
                RouteConfigType::RoutePath(RoutePathConfig {
                    name: String::from("SessionTimeout"),
                    uri: None,
                    path_reg_exp: None,
                    children: None,
                    fields: None,
                    dir_name: Some(String::from("auth")),
                }),
                RouteConfigType::ShellRoute(ShellRoutePathConfig {
                    name: String::from("Main"),
                    dir_name: Some(String::from("main")),
                    shell_index: ShellIndexType::Enum(vec![
                        String::from("One"),
                        String::from("Two"),
                        String::from("Three"),
                    ]),
                    shells: (vec![
                        (
                            String::from("One"),
                            RouteConfigType::RoutePath(RoutePathConfig {
                                name: String::from("One"),
                                uri: Some(String::from("/one-url1?id=$id")),
                                path_reg_exp: Some(String::from("^/one-url?id=.+$")),
                                children: None,
                                fields: Some(vec![DartField {
                                    name: String::from("id"),
                                    dart_type: DartType::String,
                                    nullable: false,
                                    is_final: true,
                                }]),
                                dir_name: Some(String::from("main")),
                            }),
                        ),
                        (
                            String::from("Two"),
                            RouteConfigType::RoutePath(RoutePathConfig {
                                name: String::from("Two"),
                                uri: Some(String::from("/two-url2")),
                                path_reg_exp: Some(String::from("^/two-url$")),
                                fields: None,
                                children: None,
                                dir_name: Some(String::from("main")),
                            }),
                        ),
                        (
                            String::from("Three"),
                            RouteConfigType::RoutePath(RoutePathConfig {
                                name: String::from("Three"),
                                uri: Some(String::from("/three-url")),
                                path_reg_exp: Some(String::from("^/three-url$")),
                                fields: None,
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
                fields: None,
                children: None,
            }),
        }),
        riverpod_config: Some(RiverpodConfig {
            providers: Some(vec![
                ProviderConfig {
                    name: String::from("SignInFormState"),
                    dir_name: Some(String::from("auth")),
                    state: DartType::NewClass(DartClass {
                        name: String::from("SignInForm"),
                        is_immutable: true,
                        fields: Some(vec![
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
                    }),
                    auto_dispose: true,
                    family_type: None,
                    provider_type: ProviderType::NotifierProvider,
                },
                ProviderConfig {
                    name: String::from("xxx"),
                    dir_name: Some(String::from("main")),
                    state: DartType::RefClass(DartClassRef {
                        name: String::from("sample"),
                        path: String::from("lib/domain/entity"),
                    }),
                    auto_dispose: true,
                    family_type: None,
                    provider_type: ProviderType::Provider,
                },
            ]),
            use_riverpod_generator: false,
        }),
        copy_source: Some(vec![
            CopySource::CodeCommit(CodeCommitSource {
                account_id: String::from("000000000000"),
                repository_name: String::from("sample_repository"),
                path: vec![String::from("lib/domain/")],
            }),
            CopySource::GitHub(GitHubSource {
                repository_name: String::from("sample/sample"),
                path: vec![
                    String::from("lib/domain/service/sample"),
                    String::from("lib/domain/repository/sample.dart"),
                    String::from("test/domain/repository/sample.dart"),
                ],
            }),
        ]),
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
