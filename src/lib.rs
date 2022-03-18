#![doc = include_str!("../README.md")]

pub mod parser;

/// Primitive types
/// Franca IDL Ref: 5.1.1
pub enum FPrimitiveType {
    UInt8(u8),
    Int8(i8),
    UInt16(u16),
    Int16(i16),
    UInt32(u32),
    Int32(i32),
    UInt64(u64),
    Int64(i64),
    Boolean(bool),
    Float(f32),
    Double(f64),
    String(String),
    ByteBuffer(Vec<u8>),
}

/// Root of franca model.
/// Franca IDL Ref: 8.4.1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FModel {
    /// Package declaration
    pub name: String,
    pub imports: Vec<Import>,
    pub interfaces: Vec<FInterface>,
    pub type_collections: Vec<FTypeCollection>,
}

/// Collection of Franca type definitions.
/// Franca IDL Ref: 8.4.2
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FTypeCollection {
    pub name: Option<String>,
    pub comment: Option<FAnnotationBlock>,
    pub version: Option<FVersion>,
    // TODO
    // types: Vec<FType>,
}

/// Franca IDL Ref: 8.4.3
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FInterface {
    pub name: String,
    pub comment: Option<FAnnotationBlock>,
    pub version: Option<FVersion>,
    // types: Vec<FType>,
    pub attributes: Vec<FAttribute>,
    pub methods: Vec<FMethod>,
    pub broadcasts: Vec<FBroadcast>,
    // contract: Option<FContract>,
    // TODO recursive type
    //base: Option<FInterface>,
    // TODO recursive type
    // managed_interfaces: Vec<FInterface>,
}

/// Franca IDL Ref: 8.4.4
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FBroadcast {
    pub name: String,
    pub comment: Option<FAnnotationBlock>,
    pub out_args: Vec<FArgument>,
    pub selective: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FArgument {
    pub name: String,
    pub comment: Option<FAnnotationBlock>,
    pub r#type: FTypeRef,
    pub array: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FMethod {
    pub name: String,
    pub comment: Option<FAnnotationBlock>,
    pub fire_and_forget: Option<String>,
    pub in_args: Vec<FArgument>,
    pub out_args: Vec<FArgument>,
    pub error_enum: Option<FEnumerationType>,
    pub errors: Option<FEnumerationType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FAttribute {
    pub name: String,
    pub comment: Option<FAnnotationBlock>,
    pub r#type: FTypeRef,
    pub array: Option<String>,
    pub readonly: Option<bool>,
    pub no_subscriptions: Option<bool>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FVersion {
    pub major: u16,
    pub minor: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Import {
    /// Namespace addressed by import
    pub imported_namespace: Option<String>,
    /// Uri of the imported resource
    pub import_uri: Option<String>,
}

// TODO
// Abstract
// pub struct FType {
//     name: String,
//     comment: Option<FAnnotationBlock>,
// }

// pub fn f_type(s: &str) -> IResult<&str, FBasicTypeId> {

// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FMapType {
    pub name: String,
    pub comment: Option<FAnnotationBlock>,
    pub key_type: FTypeRef,
    pub value_type: FTypeRef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FTypeRef {
    pub predefined: Option<FBasicTypeId>,
    // derived: Option<FType>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FBasicTypeId {
    Undefined,
    UInt8,
    Int8,
    UInt16,
    Int16,
    UInt32,
    Int32,
    UInt64,
    Int64,
    Integer,
    Boolean,
    Float,
    Double,
    String,
    ByteBuffer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FEnumerationType {
    pub name: String,
    pub comment: Option<FAnnotationBlock>,
    pub enumerators: Vec<FEnumerator>,
    // TODO recursive type
    // base: Option<FEnumerationType>,
}

// TODO
// pub fn f_enumeration_type(s: &str) -> IResult<&str, FEnumerationType> {
//     let (s, _) = tag("enumeration ");
//     let (s; name) =
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FEnumerator {
    pub name: String,
    pub comment: Option<FAnnotationBlock>,
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FTypeDef {
    pub name: String,
    pub comment: Option<FAnnotationBlock>,
    pub actual_type: FTypeRef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Abstract Type
pub struct FCompoundType {
    pub name: String,
    pub comment: Option<FAnnotationBlock>,
    pub elements: Vec<FField>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FUnionType {
    pub name: String,
    pub comment: Option<FAnnotationBlock>,
    pub elements: Vec<FField>,
    // TODO recursive type
    // base: Option<FUnionType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FStructType {
    pub name: String,
    pub comment: Option<FAnnotationBlock>,
    pub elements: Vec<FField>,
    // TODO recursive type
    // base: Option<Fpub structType>,
    pub polymorphic: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FField {
    pub name: String,
    pub comment: Option<FAnnotationBlock>,
    pub r#type: FTypeRef,
    pub array: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FArrayType {
    pub name: String,
    pub comment: Option<FAnnotationBlock>,
    pub element_type: FTypeRef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FAnnotationBlock {
    pub elements: Vec<FAnnotation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FAnnotation {
    pub r#type: Option<FAnnotationType>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FAnnotationType {
    Description,
    Author,
    Experimental,
    Deprecated,
    See,
    Param,
    HighVolume,
    HighFrequency,
    SourceUri,
    SourceAlias,
    Details,
}
