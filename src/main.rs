use std::{cell::RefCell, collections::HashMap, rc::Rc};

use openapiv3::{AdditionalProperties, OpenAPI, ReferenceOr, SchemaKind, Type as SchemaType};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

struct Schema<'s> {
    inner: &'s openapiv3::Schema,
    parsed: Option<DataType>,
}

impl<'s> Schema<'s> {
    pub fn new(inner: &'s openapiv3::Schema) -> Self {
        Self { inner, parsed: None }
    }
}

#[derive(Clone)]
struct DataType {
    name: String,
    token_stream: TokenStream,
    collection: bool,
    expandable: bool,
}

type Reference<T> = Rc<RefCell<T>>;

fn reference_of<T>(value: T) -> Reference<T> {
    Rc::new(RefCell::new(value))
}

fn main() {
    let data = std::fs::read_to_string("openapi.json").unwrap();
    let spec = serde_json::from_str::<OpenAPI>(&data).unwrap();

    let components = spec.components.unwrap();
    let schemas = &components.schemas;

    let mut schema_memo = HashMap::with_capacity(schemas.len());

    for (schema_name, schema) in schemas {
        let schema = schema.as_item().unwrap();
        if !schema.schema_data.extensions.contains_key("x-stripeEvent") {
            let schema = reference_of(Schema::new(schema));
            schema_memo.insert(schema_name.clone(), schema);
        }
    }

    let schema_memo_clone = schema_memo.clone();

    for (schema_name, schema) in &schema_memo_clone {
        parse(schema, None, Some(schema_name), &mut schema_memo);
    }

    for (schema_name, schema) in &schema_memo {
        let schema = schema.borrow();
        match &schema.parsed {
            Some(data_type) => {
                println!("{}", data_type.token_stream.to_string());
            }
            None => {
                println!("Got None for {}", schema_name);
            }
        }
    }

    let code = quote! {
        #[derive(Debug, Default, Clone, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Expandable<T> {
            Id(String),
            Resource(T),
        }
    };

    println!("{}", code.to_string());

    let subscription = schema_memo.get("subscription").unwrap().borrow();

    match &subscription.parsed {
        Some(data_type) => {
            println!("{}", data_type.token_stream.to_string());
        }
        None => {
            println!("Got None for subscription");
        }
    }
}

fn parse<'s>(schema: &Reference<Schema<'s>>, parent_schema_name: Option<&str>, current_schema_name: Option<&str>, schema_memo: &mut HashMap<String, Reference<Schema<'s>>>) -> DataType {
    let fully_qualified_schema_name = vec![
        parent_schema_name,
        current_schema_name,
    ];

    let fully_qualified_schema_name = fully_qualified_schema_name
        .into_iter()
        .filter_map(|x| x)
        .collect::<Vec<&str>>()
        .join("_");

    let mut data_type = DataType {
        name: delimited_string_to_pascal_case(&fully_qualified_schema_name),
        token_stream: TokenStream::new(),
        collection: false,
        expandable: false,
    };

    let data_schema = schema.borrow().inner;

    let schema_data = &data_schema.schema_data;
    let schema_kind = &data_schema.schema_kind;

    let expandable_property_memo = schema_data
        .extensions
        .get("x-expandableFields")
        .and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok())
        .unwrap_or_default()
        .into_iter()
        .map(|field| (field, ()))
        .collect::<HashMap<_, _>>();

    match schema_kind {
        SchemaKind::Type(schema_kind_type) => match schema_kind_type {
            SchemaType::String(_) => {
                data_type.name = String::from("String");
            }
            SchemaType::Number(_) => {
                data_type.name = String::from("f64");
            }
            SchemaType::Integer(_) => {
                data_type.name = String::from("i64");
            }
            SchemaType::Object(schema_type_object) => {
                if let Some(additional_properties) = &schema_type_object.additional_properties {
                    match additional_properties {
                        AdditionalProperties::Schema(additional_properties_schema) => {
                            let additional_properties_data_type = match &**additional_properties_schema {
                                ReferenceOr::Reference { reference } => {
                                    let schema_name = reference.split('/').last().unwrap();
                                    let reference_schema = Rc::clone(schema_memo.get(schema_name).unwrap());
                                    parse(
                                        &reference_schema,
                                        Some(&fully_qualified_schema_name),
                                        None,
                                        schema_memo,
                                    )
                                }
                                ReferenceOr::Item(item_schema) => parse(
                                    &reference_of(Schema::new(item_schema)),
                                    Some(&fully_qualified_schema_name),
                                    None,
                                    schema_memo,
                                ),
                            };

                            data_type.name = format!("HashMap<String, {}>", additional_properties_data_type.name);
                            data_type.collection = true;

                            return data_type;
                        }
                        _ => {
                            unreachable!("Unknown AdditionalProperties, expected AdditionalProperties::Schema");
                        }
                    }
                }

                let required_property_memo = schema_type_object
                    .required
                    .iter()
                    .map(|property| (property, ()))
                    .collect::<HashMap<_, _>>();

                let properties = schema_type_object
                    .properties
                    .iter()
                    .map(|(property_name, property_schema)| {
                        let parsed_property_name = match syn::parse_str::<syn::Ident>(property_name) {
                            Ok(field_name) => field_name,
                            Err(_) => format_ident!("r#{}", property_name),
                        };

                        let property_data_type = match property_schema {
                            ReferenceOr::Reference { reference } => {
                                let schema_name = reference.split('/').last().unwrap();
                                let reference_schema = Rc::clone(schema_memo.get(schema_name).unwrap());
                                parse(
                                    &reference_schema,
                                    Some(&fully_qualified_schema_name),
                                    Some(property_name),
                                    schema_memo,
                                )
                            }
                            ReferenceOr::Item(item_schema) => parse(
                                &reference_of(Schema::new(item_schema)),
                                Some(&fully_qualified_schema_name),
                                Some(property_name),
                                schema_memo,
                            ),
                        };

                        if property_name == "id" {
                            data_type.expandable = true;
                        }

                        let mut property_derive_input = None;
                        let mut property_data_type_name = property_data_type.name.clone();

                        // if expandable_property_memo.contains_key(property_name) {
                        //     property_data_type_name = format!("Expandable<{}>", property_data_type_name);
                        // }
                        if property_data_type.collection {
                            let derive_input = syn::parse_str::<TokenStream>("#[serde(default)]").unwrap();
                            property_derive_input = Some(derive_input);
                        } else {
                            if required_property_memo.contains_key(property_name) {
                                property_data_type_name = format!("Option<{}>", property_data_type_name);
                            }
                        }

                        let property_data_type_name = syn::parse_str::<syn::Type>(&property_data_type_name).unwrap();

                        quote! {
                            #property_derive_input
                            pub #parsed_property_name: #property_data_type_name,
                        }
                    });

                let data_type_name = format_ident!("{}", data_type.name);

                let code = quote! {
                    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
                    pub struct #data_type_name {
                        #(#properties)*
                    }
                };

                data_type.token_stream = code;
                schema.borrow_mut().parsed = Some(data_type.clone());

                if !schema_memo.contains_key(&fully_qualified_schema_name) {
                    schema_memo.insert(fully_qualified_schema_name, schema.clone());
                }
            }
            SchemaType::Array(schema_type_array) => {
                let item_data_type = match schema_type_array.items.as_ref().unwrap() {
                    ReferenceOr::Reference { reference } => {
                        let schema_name = reference.split('/').last().unwrap();
                        let reference_schema = Rc::clone(schema_memo.get(schema_name).unwrap());
                        parse(
                            &reference_schema,
                            Some(&fully_qualified_schema_name),
                            None,
                            schema_memo,
                        )
                    }
                    ReferenceOr::Item(item_schema) => parse(
                        &reference_of(Schema::new(item_schema)),
                        Some(&fully_qualified_schema_name),
                        None,
                        schema_memo,
                    ),
                };

                data_type.name = format!("Vec<{}>", item_data_type.name);
                data_type.collection = true;
            }
            SchemaType::Boolean(_) => {
                data_type.name = String::from("bool");
            }
        },
        SchemaKind::AnyOf { any_of: schema_kind_any_of } => {
            data_type.name = String::from("Temp");
        }
        _ => unreachable!("Unknown SchemaKind, expected either SchemaKind::Type or SchemaKind::AnyOf"),
    }

    data_type
}

fn delimited_string_to_pascal_case(s: &str) -> String {
    let mut converted = String::with_capacity(s.len());
    let mut next_uppercase = true;

    for c in s.chars() {
        match c {
            '-' | '_' | '.' => {
                next_uppercase = true;
            }
            _ => match next_uppercase {
                true => {
                    next_uppercase = false;
                    converted.push(c.to_ascii_uppercase());
                }
                false => {
                    converted.push(c);
                }
            },
        }
    }

    converted
}
