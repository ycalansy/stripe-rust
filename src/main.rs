use std::{collections::HashMap, io::Write};

use openapiv3::{OpenAPI, ReferenceOr, Schema, SchemaKind, Type};
use quote::{format_ident, quote, ToTokens};

fn main() {
    let data = std::fs::read_to_string("openapi.json").unwrap();
    let spec = serde_json::from_str::<OpenAPI>(&data).unwrap();

    let mut generated_code_memo = HashMap::new();

    let components = spec.components.unwrap();
    let schemas = &components.schemas;

    for (schema_name, schema) in schemas {
        let schema = schema.as_item().unwrap();
        if !schema.schema_data.extensions.contains_key("x-stripeEvent") {
            parse_schema(schema, None, schema_name, &mut generated_code_memo).unwrap();
        }
    }

    // let account = schemas
    //     .get("account")
    //     .and_then(|schema| schema.as_item())
    //     .unwrap();
    //
    // parse_schema(account, None, "account", &mut generated_code_memo).unwrap();

    for code in generated_code_memo.values() {
        println!("{}", code);
    }

    // Dump all generated code to a file.
    let mut file = std::fs::File::create("generated.rs").unwrap();

    for code in generated_code_memo.values() {
        file.write_all(code.as_bytes()).unwrap();
        file.write(b"\n").unwrap();
    }
}

fn parse_schema(schema: &Schema, parent_key: Option<&str>, current_key: &str, generated_code_memo: &mut HashMap<String, String>) -> anyhow::Result<(syn::Type, bool)> {
    let composite_key = match parent_key {
        Some(parent_key) => format!("{}_{}", parent_key, current_key),
        None => current_key.to_owned(),
    };

    let type_name = snake_case_to_pascal_case(&composite_key);
    let mut type_name = force_parse_type(&type_name);

    let mut is_collection = false;

    let schema_data = &schema.schema_data;
    let schema_kind = &schema.schema_kind;

    match schema_kind {
        SchemaKind::Type(ty) => match ty {
            Type::String(ty) => {
                let variants = ty
                    .enumeration
                    .iter()
                    .flatten()
                    .map(|variant| {
                        let variant = snake_case_to_pascal_case(variant);

                        let variant = match syn::parse_str::<syn::Variant>(&variant) {
                            Ok(variant) => variant,
                            Err(_) => {
                                let is_numeric_start = variant
                                    .chars()
                                    .next()
                                    .map(|c| c.is_numeric())
                                    .unwrap_or_default();

                                if is_numeric_start {
                                    force_parse_enum_variant(&format!("No_{}", variant))
                                } else {
                                    force_parse_enum_variant(&format!("{}_", variant))
                                }
                            }
                        };

                        quote! {
                            #variant,
                        }
                    })
                    .collect::<Vec<_>>();

                if variants.is_empty() || (current_key == "object" && variants.len() == 1) {
                    type_name = force_parse_type("String");
                } else {
                    let code = quote! {
                        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
                        #[serde(rename_all = "snake_case")]
                        pub enum #type_name {
                            #(#variants)*
                        }
                    };

                    let syntax_tree = syn::parse2::<syn::File>(code).unwrap();
                    let formatted_code = prettyplease::unparse(&syntax_tree);

                    generated_code_memo.insert(composite_key, formatted_code);
                }
            }
            Type::Number(_) => {
                type_name = force_parse_type("f64");
            }
            Type::Integer(_) => {
                type_name = force_parse_type("i64");
            }
            Type::Object(ty) => {
                if current_key == "metadata" && ty.additional_properties.is_some() {
                    type_name = force_parse_type("HashMap<String, String>");
                    return Ok((type_name, true));
                }

                let required_field_memo = ty
                    .required
                    .iter()
                    .map(|field| (field, ()))
                    .collect::<HashMap<_, _>>();

                let fields = ty
                    .properties
                    .iter()
                    .map(|(property_name, property_schema)| {
                        let field_name = match syn::parse_str::<syn::Ident>(property_name) {
                            Ok(field_name) => field_name,
                            Err(_) => format_ident!("r#{}", property_name),
                        };

                        let (field_type, is_collection) = match property_schema {
                            ReferenceOr::Reference { reference } => {
                                let field_type = reference
                                    .split('/')
                                    .last()
                                    .map(|chunk| snake_case_to_pascal_case(chunk))
                                    .map(|field_type| force_parse_type(&field_type))
                                    .unwrap();
                                (field_type, false)
                            }
                            ReferenceOr::Item(field_schema) => parse_schema(
                                field_schema,
                                Some(&composite_key),
                                property_name,
                                generated_code_memo,
                            )
                            .unwrap(),
                        };

                        if required_field_memo.contains_key(property_name) || is_collection {
                            quote! {
                                pub #field_name: #field_type,
                            }
                        } else {
                            quote! {
                                pub #field_name: Option<#field_type>,
                            }
                        }
                    });

                let code = quote! {
                    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
                    pub struct #type_name {
                        #(#fields)*
                    }
                };

                let syntax_tree = syn::parse2::<syn::File>(code).unwrap();
                let formatted_code = prettyplease::unparse(&syntax_tree);

                generated_code_memo.insert(composite_key, formatted_code);
            }
            Type::Array(ty) => {
                let item_type_name = match ty.items.as_ref().unwrap() {
                    ReferenceOr::Reference { reference } => {
                        let item_type_name = reference
                            .split('/')
                            .last()
                            .map(|chunk| snake_case_to_pascal_case(chunk))
                            .map(|field_type| force_parse_type(&field_type))
                            .unwrap();
                        item_type_name
                    }
                    ReferenceOr::Item(item_schema) => {
                        parse_schema(
                            item_schema,
                            Some(&composite_key),
                            "item",
                            generated_code_memo,
                        )
                        .unwrap()
                        .0
                    }
                };

                type_name = syn::parse_quote!(Vec<#item_type_name>);
            }
            Type::Boolean(_) => type_name = force_parse_type("bool"),
        },
        SchemaKind::AnyOf { any_of } => type_name = force_parse_type("ReplaceMeWithAnyOfSpec"),
        _ => unimplemented!(),
    };

    Ok((type_name, is_collection))
}

fn to_title_case(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn snake_case_to_pascal_case(s: &str) -> String {
    let parts: Vec<&str> = s.split(&['_', '.', '-']).collect();
    let mut output = to_title_case(parts[0]);
    for part in parts.into_iter().skip(1) {
        output.push_str(&to_title_case(part));
    }
    output
}

fn force_parse_syntax<T: syn::parse::Parse>(s: &str) -> T {
    syn::parse_str::<T>(s).expect(&format!("Failed to parse syntax: {}", s))
}

fn force_parse_type(s: &str) -> syn::Type {
    force_parse_syntax::<syn::Type>(s)
}

fn force_parse_enum_variant(s: &str) -> syn::Variant {
    force_parse_syntax::<syn::Variant>(s)
}
