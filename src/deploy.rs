use nom::{
    bytes::complete::{tag, take_till},
    character::complete::{multispace0, multispace1},
    multi::many0,
    AsChar, IResult,
};

use crate::{
    FArgument, FArrayType, FAttribute, FBroadcast, FEnumerationType, FEnumerator, FField, FMapType,
    FMethod, FStructType, FUnionType,
};

/// Deployment model root
pub struct FdModel {
    name: String,
    imports: Vec<Import>,
    specifications: Vec<FdSpecification>,
    deployments: Vec<FdRootElement>,
}

pub fn fd_model(s: &str) -> IResult<&str, FdModel> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("package")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, name) = fqn(s)?;
    let (s, imports) = many0(import)(s)?;
    let (s, specifications) = many0(fd_specification)(s)?;
    let (s, deployments) = many0(fd_root_element)(s)?;

    Ok((
        s,
        FdModel {
            name: name.to_string(),
            imports,
            specifications,
            deployments,
        },
    ))
}

pub enum Import {
    ImportUri(String),
    ImportedSpec(String),
}

// Import :
// 	'import' (importURI=STRING|importedSpec=FQN);
pub fn import(s: &str) -> IResult<&str, Import> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("import")(s)?;
    let (s, _) = multispace1(s)?;
    todo!();
}

pub struct FdRootElement {}

pub fn fd_root_element(s: &str) -> IResult<&str, FdRootElement> {
    todo!()
}

// pub struct FdExtensionRoot {
//     spec: FdSpecification,
//     tag: Id,
//     name: String,
//     r#use: Vec<FdRootElement>,
//     properties: FdPropertySet,
//     elements: Vec<FdExtensionElement>,
// }

pub struct FdInterface {
    spec: FdSpecification,
    target: Option<FInterface>,
    name: String,
    r#use: Vec<FdRootElement>,
    properties: FdPropertySet,
    elements: Vec<FdAttribute>,
    methods: Vec<FdMethod>,
    broadcasts: Vec<FdBroadcast>,
    types: Vec<FdTypeDefinition>,
}

pub struct FInterface {}

pub struct FdAttribute {
    target: FAttribute,
    properties: FdPropertySet,
    overwrites: Option<FdTypeOverwrites>,
}

pub enum FdOperation {
    FdMethod(FdMethod),
    FdBroadcast(FdBroadcast),
}

pub struct FdMethod {
    target: FMethod,
    properties: FdPropertySet,
    r#in: Option<FdArgumentList>,
    out: Option<FdArgumentList>,
}

pub struct FdBroadcast {
    target: FBroadcast,
    properties: FdPropertySet,
    out: Option<FdArgumentList>,
}

pub struct FdArgumentList {
    arguments: Vec<FdArgument>,
}

pub struct FdArgument {
    target: FArgument,
    properties: FdPropertySet,
    overwrites: Option<FdTypeOverwrites>,
}

pub enum FdTypeDefinition {
    FdArray(FdArray),
    FdCompound(FdCompound),
    FdEnumeration(FdEnumeration),
    FdTypeDef(FdTypeDef),
    FdMap(FdMap),
}

pub enum FdCompound {
    FdStruct(FdStruct),
    FdUnion(FdUnion),
}

pub struct FdArray {
    target: FArrayType,
    properties: FdPropertySet,
    overwrites: Option<FdTypeOverwrites>,
}

pub struct FdStruct {
    target: FStructType,
    properties: FdPropertySet,
    fields: Vec<FdField>,
}

pub struct FdUnion {
    target: FUnionType,
    properties: FdPropertySet,
    fields: Vec<FdField>,
}

pub struct FdTypeDef {
    target: FUnionType,
    properties: FdPropertySet,
}

pub struct FdField {
    target: FField,
    properties: FdPropertySet,
    overwrites: Option<FdTypeOverwrites>,
}

pub struct FdEnumeration {
    target: FEnumerationType,
    properties: FdPropertySet,
    enumerators: FdEnumValue,
}

pub struct FdEnumValue {
    target: FEnumerator,
    properties: FdPropertySet,
}

pub struct FdMap {
    target: FMapType,
    properties: FdPropertySet,
    key: Option<FdMapKey>,
    value: Option<FdMapValue>,
}

pub struct FdMapKey {
    properties: FdPropertySet,
}

pub struct FdMapValue {
    properties: FdPropertySet,
}

pub enum FdTypeOverwrites {
    FdPlainTypeOverwrites(FdPlainTypeOverwrites),
    FdStructOverwrites(FdStructOverwrites),
    FdUnionOverwrites(FdUnionOverwrites),
    FdEnumerationOverwrites(FdEnumerationOverwrites),
}

pub struct FdPlainTypeOverwrites {}

pub struct FdStructOverwrites {}
pub struct FdUnionOverwrites {}

pub struct FdEnumerationOverwrites {}

pub struct FdSpecification {}
pub fn fd_specification(s: &str) -> IResult<&str, FdSpecification> {
    todo!()
}

pub struct FdExtensionElement {}
pub fn fd_extension_element(s: &str) -> IResult<&str, FdExtensionElement> {
    todo!()
}

pub struct FdPropertySet {
    items: Vec<FdProperty>,
}

pub struct FdProperty {
    decl: FdPropertyDecl,
    value: FdValue,
}

pub struct FdPropertyDecl {}

pub enum FdValue {
    FdInteger(i32),
    FdString(String),
    FdBoolean(bool),
    FdInterfaceRef(FdInterfaceRef),
    // FdGeneric(FdGeneric),
}

pub struct FdInterfaceRef {
    value: FInterface,
}

// pub struct FdGeneric {
//     value: EObject,
// }

pub fn fd_property_set(s: &str) -> IResult<&str, FdPropertySet> {
    todo!()
}

pub fn fqn(s: &str) -> IResult<&str, &str> {
    let (s, e_str) = take_till(|c: char| !c.is_alphanum() && c != '_' && c != '-')(s)?;
    Ok((s, e_str))
}

pub fn fqn_with_selector(s: &str) -> IResult<&str, &str> {
    let (s, _) = fqn(s)?;
    let (s, _) = tag(":")(s)?;
    let (s, _) = id(s)?;
    todo!()
}

pub struct Id {}
pub fn id(s: &str) -> IResult<&str, Id> {
    todo!()
}
