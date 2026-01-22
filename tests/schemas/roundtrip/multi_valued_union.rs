use crate::schemas::{multi_valued_union::Contact, utils::serde::serde_assert};

#[test]
fn none() {
    let contact = Contact {
        extra: None,
    };
    serde_assert(contact);
}

#[test]
fn some_string() {
    let contact = Contact {
        extra: Some(crate::schemas::multi_valued_union::UnionStringLongDoubleBoolean::String("string".into())),
    };
    serde_assert(contact);
}

#[test]
fn some_long() {
    let contact = Contact {
        extra: Some(crate::schemas::multi_valued_union::UnionStringLongDoubleBoolean::Long(i64::MAX)),
    };
    serde_assert(contact);
}

#[test]
fn some_double() {
    let contact = Contact {
        extra: Some(crate::schemas::multi_valued_union::UnionStringLongDoubleBoolean::Double(f64::INFINITY)),
    };
    serde_assert(contact);
}

#[test]
fn some_boolean() {
    let contact = Contact {
        extra: Some(crate::schemas::multi_valued_union::UnionStringLongDoubleBoolean::Boolean(true)),
    };
    serde_assert(contact);
}
