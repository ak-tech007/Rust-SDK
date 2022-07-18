use fuels_types::{
    errors::Error,
    param_types::{EnumVariants, ParamType},
    Property,
};
use std::str::FromStr;

/// Turns a JSON property into ParamType
pub fn parse_param_type_from_property(prop: &Property) -> Result<ParamType, Error> {
    match ParamType::from_str(&prop.type_field) {
        // Simple case (primitive types, no arrays, including string)
        Ok(param_type) => Ok(param_type),
        Err(_) => {
            if prop.type_field == "()" {
                return Ok(ParamType::Unit);
            }
            if prop.type_field.contains('[') && prop.type_field.contains(']') {
                // Try to parse array ([T; M]) or string (str[M])
                if prop.type_field.contains("str[") {
                    return parse_string_param(prop);
                }
                return parse_array_param(prop);
            }
            if prop.type_field.starts_with('(') && prop.type_field.ends_with(')') {
                // Try to parse tuple (T, T, ..., T)
                return parse_tuple_param(prop);
            }
            // Try to parse a free form enum or struct (e.g. `struct MySTruct`, `enum MyEnum`).
            parse_custom_type_param(prop)
        }
    }
}

pub fn parse_tuple_param(prop: &Property) -> Result<ParamType, Error> {
    let mut params: Vec<ParamType> = Vec::new();

    for tuple_component in prop
        .components
        .as_ref()
        .expect("tuples should have components")
    {
        params.push(parse_param_type_from_property(tuple_component)?);
    }

    Ok(ParamType::Tuple(params))
}

pub fn parse_string_param(prop: &Property) -> Result<ParamType, Error> {
    // Split "str[n]" string into "str" and "[n]"
    let split: Vec<&str> = prop.type_field.split('[').collect();
    if split.len() != 2 || !split[0].eq("str") {
        return Err(Error::InvalidType(format!(
            "Expected parameter type `str[n]`, found `{}`",
            prop.type_field
        )));
    }
    // Grab size in between brackets, i.e the `n` in "[n]"
    let size: usize = split[1][..split[1].len() - 1].parse()?;
    Ok(ParamType::String(size))
}

pub fn parse_array_param(prop: &Property) -> Result<ParamType, Error> {
    // Split "[T; n]" string into "T" and "n"
    let split: Vec<&str> = prop.type_field.split("; ").collect();
    if split.len() != 2 {
        return Err(Error::InvalidType(format!(
            "Expected parameter type `[T; n]`, found `{}`",
            prop.type_field
        )));
    }
    let (type_field, size) = (split[0], split[1]);
    let type_field = type_field[1..].to_string();

    let param_type = match ParamType::from_str(&type_field) {
        Ok(param_type) => param_type,
        Err(_) => parse_custom_type_param(
            prop.components
                .as_ref()
                .expect("array should have components")
                .first()
                .expect("components in array should have at least one component"),
        )?,
    };

    // Grab size the `n` in "[T; n]"
    let size: usize = size[..size.len() - 1].parse()?;
    Ok(ParamType::Array(Box::new(param_type), size))
}

pub fn parse_custom_type_param(prop: &Property) -> Result<ParamType, Error> {
    let mut params: Vec<ParamType> = vec![];
    match &prop.components {
        Some(c) => {
            for component in c {
                params.push(parse_param_type_from_property(component)?)
            }
            if prop.is_struct_type() {
                return Ok(ParamType::Struct(params));
            }
            if prop.is_enum_type() {
                return Ok(ParamType::Enum(EnumVariants::new(params)?));
            }
            Err(Error::InvalidType(prop.type_field.clone()))
        }
        None => Err(Error::InvalidType(
            "cannot parse custom type with no components".into(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fuels_types::{errors::Error, param_types::ParamType};

    #[test]
    fn parse_string_and_array_param() -> Result<(), Error> {
        let array_prop = Property {
            name: "some_array".to_string(),
            type_field: "[bool; 4]".to_string(),
            components: None,
        };
        let expected = "Array(Box::new(ParamType::Bool),4)";
        let result = parse_array_param(&array_prop)?.to_string();
        assert_eq!(result, expected);

        let string_prop = Property {
            name: "some_array".to_string(),
            type_field: "str[5]".to_string(),
            components: None,
        };
        let expected = "String(5)";
        let result = parse_string_param(&string_prop)?.to_string();
        assert_eq!(result, expected);

        let expected = "Invalid type: Expected parameter type `str[n]`, found `[bool; 4]`";
        let result = parse_string_param(&array_prop).unwrap_err().to_string();
        assert_eq!(result, expected);

        let expected = "Invalid type: Expected parameter type `[T; n]`, found `str[5]`";
        let result = parse_array_param(&string_prop).unwrap_err().to_string();
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_parse_custom_type_params() -> Result<(), Error> {
        let components = vec![
            Property {
                name: "vodka".to_string(),
                type_field: "u64".to_string(),
                components: None,
            },
            Property {
                name: "redbull".to_string(),
                type_field: "bool".to_string(),
                components: None,
            },
        ];

        // STRUCT
        let some_struct = Property {
            name: String::from("something_you_drink"),
            type_field: String::from("struct Cocktail"),
            components: Some(components.clone()),
        };
        let struct_result = parse_custom_type_param(&some_struct)?;
        // Underlying value comparison
        let expected = ParamType::Struct(vec![ParamType::U64, ParamType::Bool]);
        assert_eq!(struct_result, expected);
        let expected_string = "Struct(vec![ParamType::U64,ParamType::Bool])";
        // String format comparison
        assert_eq!(struct_result.to_string(), expected_string);

        // ENUM
        let some_enum = Property {
            name: String::from("something_you_drink"),
            type_field: String::from("enum Cocktail"),
            components: Some(components),
        };
        let enum_result = parse_custom_type_param(&some_enum)?;
        // Underlying value comparison
        let expected = ParamType::Enum(EnumVariants::new(vec![ParamType::U64, ParamType::Bool])?);
        assert_eq!(enum_result, expected);
        let expected_string =
            "Enum(EnumVariants::new(vec![ParamType::U64,ParamType::Bool]).unwrap())";
        // String format comparison
        assert_eq!(enum_result.to_string(), expected_string);
        Ok(())
    }
}
