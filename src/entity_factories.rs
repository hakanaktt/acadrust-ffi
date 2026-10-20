//! Named entry points generated from the complete entity catalog.
use crate::*;

macro_rules! entity_factories {
    ($($kind:literal => ($new:ident, $create:ident)),* $(,)?) => {
        $(
            #[no_mangle]
            pub unsafe extern "C" fn $new(out: *mut *mut EntityBuilder) -> Status {
                ar_entity_builder_new(concat!($kind, "\0").as_ptr().cast(), out)
            }
            #[no_mangle]
            pub unsafe extern "C" fn $create(doc: *mut Document, edits: *const EntityEdit, count: usize, out: *mut u64) -> Status {
                ar_entity_create(doc, concat!($kind, "\0").as_ptr().cast(), edits, count, out)
            }
        )*
    };
}

include!("entity_factories_catalog.rs");
