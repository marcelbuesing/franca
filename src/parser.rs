use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_till1},
    character::complete::{self, char, line_ending, multispace0, multispace1, space0, space1},
    combinator::{opt, value},
    multi::many0,
    AsChar, IResult,
};

use crate::{
    FArgument, FBasicTypeId, FBroadcast, FInterface, FMethod, FModel, FTypeRef, FVersion, Import,
};

#[cfg(test)]
mod tests {
    use crate::{
        FArgument, FBasicTypeId, FBroadcast, FInterface, FMethod, FModel, FTypeRef, FVersion,
        Import,
    };

    use super::*;

    #[test]
    pub fn f_model_test() {
        let def = "package test

interface HelloWorld {
    version { major 1 minor 0 }

    method sayHello {
        in {
            String name
        }
        out {
            String value_a
            Double value_b
            UInt32 value_c
        }
    }
}
";

        let in_args = vec![FArgument {
            name: "name".to_string(),
            comment: None,
            r#type: FTypeRef {
                predefined: Some(FBasicTypeId::String),
            },
            array: None,
        }];

        let out_args = vec![
            FArgument {
                name: "value_a".to_string(),
                comment: None,
                r#type: FTypeRef {
                    predefined: Some(FBasicTypeId::String),
                },
                array: None,
            },
            FArgument {
                name: "value_b".to_string(),
                comment: None,
                r#type: FTypeRef {
                    predefined: Some(FBasicTypeId::Double),
                },
                array: None,
            },
            FArgument {
                name: "value_c".to_string(),
                comment: None,
                r#type: FTypeRef {
                    predefined: Some(FBasicTypeId::UInt32),
                },
                array: None,
            },
        ];

        let method = FMethod {
            name: "sayHello".to_string(),
            comment: None,
            fire_and_forget: None,
            in_args,
            out_args,
            error_enum: None,
            errors: None,
        };

        let interface = FInterface {
            name: "HelloWorld".to_string(),
            comment: None,
            version: Some(FVersion { major: 1, minor: 0 }),
            attributes: Vec::new(),
            methods: vec![method],
            broadcasts: Vec::new(),
        };

        let exp_model = FModel {
            name: "test".to_string(),
            imports: Vec::new(),
            interfaces: vec![interface],
            type_collections: Vec::new(),
        };

        let (_, model) = f_model(def).unwrap();
        assert_eq!(model, exp_model);
    }

    #[test]
    pub fn f_method_test() {
        let def = "method sayHello {
        in {
            String name
        }
        out {
            String value_a
            Double value_b
            UInt32 value_c
        }
    }
}
";

        let in_args = vec![FArgument {
            name: "name".to_string(),
            comment: None,
            r#type: FTypeRef {
                predefined: Some(FBasicTypeId::String),
            },
            array: None,
        }];

        let out_args = vec![
            FArgument {
                name: "value_a".to_string(),
                comment: None,
                r#type: FTypeRef {
                    predefined: Some(FBasicTypeId::String),
                },
                array: None,
            },
            FArgument {
                name: "value_b".to_string(),
                comment: None,
                r#type: FTypeRef {
                    predefined: Some(FBasicTypeId::Double),
                },
                array: None,
            },
            FArgument {
                name: "value_c".to_string(),
                comment: None,
                r#type: FTypeRef {
                    predefined: Some(FBasicTypeId::UInt32),
                },
                array: None,
            },
        ];

        let exp_method = FMethod {
            name: "sayHello".to_string(),
            comment: None,
            fire_and_forget: None,
            in_args,
            out_args,
            error_enum: None,
            errors: None,
        };

        let (_, method) = f_method(def).unwrap();
        assert_eq!(method, exp_method);
    }

    #[test]
    pub fn f_version_test() {
        let def = "version { major 1 minor 0 }\n";

        let exp_version = FVersion { major: 1, minor: 0 };

        let (_, version) = f_version(def).unwrap();
        assert_eq!(version, exp_version);
    }

    #[test]
    pub fn f_broadcast_test() {
        let def = "broadcast Greeting {
    out {
        String message
    }
}
";
        let out_args = vec![FArgument {
            name: "message".to_string(),
            comment: None,
            r#type: FTypeRef {
                predefined: Some(FBasicTypeId::String),
            },
            array: None,
        }];

        let exp_broadcast = FBroadcast {
            name: "Greeting".to_string(),
            comment: None,
            out_args,
            selective: None,
        };

        let (_, broadcast) = f_broadcast(def).unwrap();
        assert_eq!(broadcast, exp_broadcast);
    }

    #[test]
    pub fn import_uri_test() {
        let (_, ns) = imported_namespace(" org.franca*.examples.demo.* from").unwrap();
        assert_eq!(ns, "org.franca*.examples.demo.*");
        let def = r##"import org.franca.examples.demo.* from "basic.fidl""##;

        let exp_import = Import {
            imported_namespace: Some("org.franca.examples.demo.*".to_string()),
            import_uri: Some("basic.fidl".to_string()),
        };

        let (_, broadcast) = import(def).unwrap();
        assert_eq!(broadcast, exp_import);
    }

    #[test]
    pub fn import_uri_test_without_from() {
        let def = r##"import "platform:/plugin/abc.fdepl"""##;

        let exp_import = Import {
            imported_namespace: None,
            import_uri: Some("platform:/plugin/abc.fdepl".to_string()),
        };

        let (_, broadcast) = import(def).unwrap();
        assert_eq!(broadcast, exp_import);
    }
}

pub fn f_model(s: &str) -> IResult<&str, FModel> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("package")(s)?;
    let (s, _) = space1(s)?;
    let (s, name) = e_string(s)?;

    let (s, imports) = many0(import)(s)?;
    let (s, interfaces) = many0(f_interface)(s)?;
    Ok((
        s,
        FModel {
            name: name.to_string(),
            imports,
            interfaces,
            type_collections: Vec::new(),
        },
    ))
}

pub fn f_interface(s: &str) -> IResult<&str, FInterface> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("interface")(s)?;
    let (s, _) = space1(s)?;
    let (s, name) = e_string(s)?;
    let (s, _) = multispace1(s)?;

    let (s, _) = brc_open(s)?;
    let (s, version) = opt(f_version)(s)?;
    let (s, methods) = many0(f_method)(s)?;
    let (s, broadcasts) = many0(f_broadcast)(s)?;
    let (s, _) = brc_close(s)?;

    Ok((
        s,
        FInterface {
            name: name.to_string(),
            comment: None,
            version,
            attributes: Vec::new(),
            methods,
            broadcasts,
        },
    ))
}

pub fn f_broadcast(s: &str) -> IResult<&str, FBroadcast> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("broadcast")(s)?;
    let (s, _) = space1(s)?;
    let (s, name) = e_string(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = brc_open(s)?;
    let (s, out_args) = f_method_out_args(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = brc_close(s)?;
    Ok((
        s,
        FBroadcast {
            name: name.to_string(),
            comment: None,
            out_args,
            selective: None,
        },
    ))
}

pub fn e_string(s: &str) -> IResult<&str, &str> {
    let (s, e_str) = take_till(|c: char| !c.is_alphanum() && c != '_' && c != '-' && c != '.')(s)?;
    Ok((s, e_str))
}

pub fn brc_open(s: &str) -> IResult<&str, char> {
    char('{')(s)
}

pub fn brc_close(s: &str) -> IResult<&str, char> {
    char('}')(s)
}

pub fn f_argument(s: &str) -> IResult<&str, FArgument> {
    let (s, _) = multispace0(s)?;
    let (s, r#type) = f_type_ref(s)?;
    let (s, _) = space1(s)?;
    let (s, name) = e_string(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = line_ending(s)?;
    Ok((
        s,
        FArgument {
            name: name.to_string(),
            comment: None,
            r#type,
            array: None,
        },
    ))
}

pub fn f_method_in_args(s: &str) -> IResult<&str, Vec<FArgument>> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("in")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = brc_open(s)?;
    let (s, in_args) = many0(f_argument)(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = brc_close(s)?;
    Ok((s, in_args))
}

pub fn f_method_out_args(s: &str) -> IResult<&str, Vec<FArgument>> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("out")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = brc_open(s)?;
    let (s, in_args) = many0(f_argument)(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = brc_close(s)?;
    Ok((s, in_args))
}

pub fn f_method(s: &str) -> IResult<&str, FMethod> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("method")(s)?;
    let (s, _) = space1(s)?;
    let (s, name) = e_string(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = brc_open(s)?;
    let (s, in_args) = f_method_in_args(s)?;
    let (s, out_args) = f_method_out_args(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = brc_close(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = line_ending(s)?;
    Ok((
        s,
        FMethod {
            name: name.to_string(),
            comment: None,
            fire_and_forget: None,
            in_args,
            out_args,
            error_enum: None,
            errors: None,
        },
    ))
}

pub fn f_version(s: &str) -> IResult<&str, FVersion> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("version")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, _) = brc_open(s)?;

    let (s, _) = multispace1(s)?;
    let (s, _) = tag("major")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, major) = complete::u16(s)?;

    let (s, _) = multispace1(s)?;
    let (s, _) = tag("minor")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, minor) = complete::u16(s)?;

    let (s, _) = multispace1(s)?;
    let (s, _) = brc_close(s)?;
    let (s, _) = line_ending(s)?;

    Ok((s, FVersion { major, minor }))
}

fn imported_namespace(s: &str) -> IResult<&str, &str> {
    let (s, _) = space1(s)?;
    let (s, imported_namespace) = take_till1(|c: char| {
        !(c.is_alphanum() || c == '_' || c == '-' || c == '\u{002A}' || c == '*' || c == '.')
    })(s)?;

    let (s, _) = space1(s)?;
    let (s, _) = tag("from")(s)?;
    Ok((s, imported_namespace))
}

fn import_uri(s: &str) -> IResult<&str, &str> {
    let (s, _) = char('"')(s)?;
    let (s, import_uri) = take_till(|c: char| c == '"' || c == '\n')(s)?;
    let (s, _) = char('"')(s)?;
    Ok((s, import_uri))
}

fn import(s: &str) -> IResult<&str, Import> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("import")(s)?;

    // TODO model import currently not supported since it's not documented very well
    let (s, imported_namespace) = opt(imported_namespace)(s)?;
    let (s, _) = space1(s)?;

    let (s, import_uri) = import_uri(s)?;
    Ok((
        s,
        Import {
            imported_namespace: imported_namespace.map(|x| x.to_string()),
            import_uri: Some(import_uri.to_string()),
        },
    ))
}

pub fn f_type_ref(s: &str) -> IResult<&str, FTypeRef> {
    let (s, predefined) = opt(f_basic_type_id)(s)?;
    // let (s, derived) = opt(f_type)(s)?;
    Ok((
        s,
        FTypeRef {
            predefined,
            // derived,
        },
    ))
}

pub fn f_basic_type_id(s: &str) -> IResult<&str, FBasicTypeId> {
    alt((
        value(FBasicTypeId::UInt8, tag("UInt8")),
        value(FBasicTypeId::Int8, tag("Int8")),
        value(FBasicTypeId::UInt16, tag("UInt16")),
        value(FBasicTypeId::Int16, tag("Int16")),
        value(FBasicTypeId::UInt32, tag("UInt32")),
        value(FBasicTypeId::Int32, tag("Int32")),
        value(FBasicTypeId::UInt64, tag("UInt64")),
        value(FBasicTypeId::Int64, tag("Int64")),
        // TODO ranges
        value(FBasicTypeId::Integer, tag("Integer")),
        value(FBasicTypeId::Boolean, tag("Boolean")),
        value(FBasicTypeId::Float, tag("Float")),
        value(FBasicTypeId::Double, tag("Double")),
        value(FBasicTypeId::String, tag("String")),
        value(FBasicTypeId::ByteBuffer, tag("ByteBuffer")),
    ))(s)
}
