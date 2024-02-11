use nom::{self, IResult, bytes::complete::tag, character::{complete::{multispace0, alpha1, alphanumeric1}}};

#[derive(Debug, PartialEq)]
pub enum GIRType {
    Executable,
    DynamicLib,
    StaticLib
}

#[derive(Debug, PartialEq)]
struct Header {
    gir_type: GIRType,
    entrypoint: String,
    abi: String
}

/*fn parser(input: &str) -> IResult<&str, Header>{
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, header) = header(input)?;
    Ok((input))
}*/

fn header(i: &str) -> IResult<&str, Header> {
    let (i, _) = multispace0(i)?;
    let (i, _) = tag("(")(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = tag("header!")(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = tag("{")(i)?;
    
    let (i, _) = multispace0(i)?;

    let (i, _) = tag("type")(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = tag("=")(i)?;
    let (i, _) = multispace0(i)?;
    let (i, gir_type) = match alpha1(i)? {
        (_, "Executable")=>(i, GIRType::Executable),
        (_, "DynamicLib")=>(i, GIRType::DynamicLib),
        (_, "StaticLib")=>(i, GIRType::StaticLib),
        _=>panic!("Failed to parse file because of bad GIR file type")
    };
    let (i, _) = tag(",")(i)?;

    let (i, _) = multispace0(i)?;
    
    let (i, _) = tag("entrypoint")(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = tag("=")(i)?;
    let (i, _) = multispace0(i)?;
    let (i, entrypoint) = alpha1(i)?;
    let entrypoint = entrypoint.to_string();
    let (i, _) = tag(",")(i)?;
    
    let (i, _) = multispace0(i)?;
    
    let (i, _) = tag("abi")(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = tag("=")(i)?;
    let (i, _) = multispace0(i)?;
    let (i, abi) = alpha1(i)?;
    let abi = abi.to_string();

    let (i, _) = multispace0(i)?;
    
    let (i, _) = tag("}")(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = tag(")")(i)?;
    let (i, _) = multispace0(i)?;
    Ok((i, Header{
        gir_type,
        entrypoint,
        abi
    }))
}

#[test]
fn header_test() {
    let test_in = "(header! {
        type=Executable,
        entrypoint=main,
        abi=stdgrond
    })";
    println!("{:?}", header(test_in));
    assert_eq!(
        ("", Header {
            gir_type: GIRType::Executable,
            entrypoint: "main".into(),
            abi: "stdgrond".into()
        }),
        (header(test_in).unwrap())
    )
}