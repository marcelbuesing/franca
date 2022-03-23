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

    Ok(FdModel {
        name,
        imports,
        specifications,
        deployments,
    })
}

pub enum Import {
    ImportUri(String),
    ImportedSpec(String),
}

pub fn import(s: &str) -> IResult<&str, Import> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("import")(s)?;
    let (s, _) = multispace1(s)?;

}

pub fn fqn(s: &str) -> IResult<&str, &str> {
    let (s, e_str) = take_till(|c: char| !c.is_alphanum() && c != '_' && c != '-')(s)?;
    Ok((s, e_str))
}

pub fn fqn_with_selector(s: &str) -> IResult<&str, &str> {
    let (s, _) = fqn(s)?;
    let (s, _) = tag(":")?;
    let (s, _) = id(":")?;
}
