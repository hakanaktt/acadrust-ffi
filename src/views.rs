//! Borrowed native reflection. No serialization or allocation is used to read a view.
use crate::*;
use std::collections::{BTreeMap, HashMap};
use std::ffi::c_void;
use std::hash::{BuildHasher, Hash};
use std::sync::Arc;

pub const AR_VALUE_OPAQUE: i32 = 0;
pub const AR_VALUE_STRUCT: i32 = 1;
pub const AR_VALUE_ENUM: i32 = 2;
pub const AR_VALUE_SEQUENCE: i32 = 3;
pub const AR_VALUE_STRING: i32 = 4;
pub const AR_VALUE_BOOL: i32 = 5;
pub const AR_VALUE_I8: i32 = 6;
pub const AR_VALUE_U8: i32 = 7;
pub const AR_VALUE_I16: i32 = 8;
pub const AR_VALUE_U16: i32 = 9;
pub const AR_VALUE_I32: i32 = 10;
pub const AR_VALUE_U32: i32 = 11;
pub const AR_VALUE_I64: i32 = 12;
pub const AR_VALUE_U64: i32 = 13;
pub const AR_VALUE_F32: i32 = 14;
pub const AR_VALUE_F64: i32 = 15;
pub const AR_VALUE_OPTION: i32 = 16;
pub const AR_VALUE_MAP: i32 = 17;
pub const AR_VALUE_CHAR: i32 = 18;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct StringView {
    pub data: *const c_char,
    pub len: usize,
}

impl StringView {
    pub(crate) fn new(value: &str) -> Self {
        Self {
            data: value.as_ptr().cast(),
            len: value.len(),
        }
    }
}

/// Both pointers are private implementation details, even though C can copy the record.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ValueView {
    pub data: *const c_void,
    pub descriptor: *const c_void,
}
pub type View = ValueView;

impl Default for ValueView {
    fn default() -> Self {
        Self {
            data: ptr::null(),
            descriptor: ptr::null(),
        }
    }
}

pub(crate) trait NativeView: Sized {
    fn kind(&self) -> i32;
    fn len(&self) -> usize {
        0
    }
    fn child(&self, _index: usize) -> Option<(StringView, ValueView)> {
        None
    }
    fn bytes(&self) -> Option<(*const c_void, usize, usize)> {
        None
    }
    fn variant(&self) -> Option<&'static str> {
        None
    }
    fn number(&self) -> Option<i64> {
        None
    }
}

struct Descriptor {
    kind: unsafe fn(*const c_void) -> i32,
    len: unsafe fn(*const c_void) -> usize,
    child: unsafe fn(*const c_void, usize) -> Option<(StringView, ValueView)>,
    bytes: unsafe fn(*const c_void) -> Option<(*const c_void, usize, usize)>,
    variant: unsafe fn(*const c_void) -> Option<&'static str>,
    number: unsafe fn(*const c_void) -> Option<i64>,
    type_name: fn() -> &'static str,
}

struct TypeDescriptor<T>(std::marker::PhantomData<T>);
impl<T: NativeView> TypeDescriptor<T> {
    const DESCRIPTOR: Descriptor = Descriptor {
        kind: |p| unsafe { (&*p.cast::<T>()).kind() },
        len: |p| unsafe { (&*p.cast::<T>()).len() },
        child: |p, i| unsafe { (&*p.cast::<T>()).child(i) },
        bytes: |p| unsafe { (&*p.cast::<T>()).bytes() },
        variant: |p| unsafe { (&*p.cast::<T>()).variant() },
        number: |p| unsafe { (&*p.cast::<T>()).number() },
        type_name: std::any::type_name::<T>,
    };
}

pub(crate) fn view<T: NativeView>(value: &T) -> ValueView {
    ValueView {
        data: std::ptr::from_ref(value).cast(),
        descriptor: std::ptr::from_ref(&TypeDescriptor::<T>::DESCRIPTOR).cast(),
    }
}

macro_rules! view_struct {
    ($ty:path { $($field:tt),* $(,)? }) => {
        #[allow(unused_imports, unused_mut)]
        impl $crate::views::NativeView for $ty {
            fn kind(&self) -> i32 { $crate::views::AR_VALUE_STRUCT }
            fn len(&self) -> usize { let names: &[&str] = &[$(stringify!($field)),*]; names.len() }
            fn child(&self, index: usize) -> Option<($crate::views::StringView, $crate::views::ValueView)> {
                let mut position = 0usize;
                $(if index == position { return Some(($crate::views::StringView::new(stringify!($field)), $crate::views::view(&self.$field))); } position += 1;)*
                let _ = (index, position);
                None
            }
        }
        $crate::mutations::mut_struct!($ty { $($field),* });
    };
}
pub(crate) use view_struct;

macro_rules! view_enum {
    ($ty:path { $($variant:ident $(($($payload:ident),*))? $({$($field:ident),*})?),* $(,)? }) => {
        #[allow(unused_variables, unused_imports)]
        impl $crate::views::NativeView for $ty {
            fn kind(&self) -> i32 { $crate::views::AR_VALUE_ENUM }
            fn len(&self) -> usize { match self { $(Self::$variant $(($($payload),*))? $({$($field),*})? => { 0 $($(+ { let _ = $payload; 1 })*)? $($(+ { let _ = $field; 1 })*)? }),* } }
            fn variant(&self) -> Option<&'static str> { Some(match self { $(Self::$variant $(($($payload),*))? $({$($field),*})? => stringify!($variant)),* }) }
            fn child(&self, index: usize) -> Option<($crate::views::StringView, $crate::views::ValueView)> {
                match self { $(Self::$variant $(($($payload),*))? $({$($field),*})? => {
                    let children: &[($crate::views::StringView, $crate::views::ValueView)] = &[
                        $($(($crate::views::StringView::new(stringify!($payload)), $crate::views::view($payload)),)*)?
                        $($(($crate::views::StringView::new(stringify!($field)), $crate::views::view($field)),)*)?
                    ];
                    children.get(index).copied()
                }),* }
            }
        }
        $crate::mutations::mut_enum!($ty { $($variant $(($($payload),*))? $({$($field),*})?),* });
    };
}
pub(crate) use view_enum;

macro_rules! primitive {
    ($($ty:ty => $kind:ident),* $(,)?) => { $(impl NativeView for $ty {
        fn kind(&self) -> i32 { $kind }
        fn bytes(&self) -> Option<(*const c_void, usize, usize)> { Some((std::ptr::from_ref(self).cast(), 1, std::mem::size_of::<Self>())) }
    })* };
}
primitive!(bool => AR_VALUE_BOOL, i8 => AR_VALUE_I8, u8 => AR_VALUE_U8, i16 => AR_VALUE_I16, u16 => AR_VALUE_U16, i32 => AR_VALUE_I32, u32 => AR_VALUE_U32, i64 => AR_VALUE_I64, u64 => AR_VALUE_U64, f32 => AR_VALUE_F32, f64 => AR_VALUE_F64, char => AR_VALUE_CHAR);
#[cfg(target_pointer_width = "64")]
primitive!(usize => AR_VALUE_U64, isize => AR_VALUE_I64);
#[cfg(target_pointer_width = "32")]
primitive!(usize => AR_VALUE_U32, isize => AR_VALUE_I32);

impl NativeView for String {
    fn kind(&self) -> i32 {
        AR_VALUE_STRING
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn bytes(&self) -> Option<(*const c_void, usize, usize)> {
        Some((self.as_ptr().cast(), self.len(), 1))
    }
}
impl<T: NativeView> NativeView for Vec<T> {
    fn kind(&self) -> i32 {
        AR_VALUE_SEQUENCE
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn child(&self, index: usize) -> Option<(StringView, ValueView)> {
        self.get(index).map(|v| (StringView::new(""), view(v)))
    }
    fn bytes(&self) -> Option<(*const c_void, usize, usize)> {
        let v = self.first()?;
        if matches!(v.kind(), AR_VALUE_BOOL..=AR_VALUE_F64 | AR_VALUE_CHAR) && v.bytes().is_some() {
            Some((self.as_ptr().cast(), self.len(), std::mem::size_of::<T>()))
        } else {
            None
        }
    }
}
impl<T: NativeView, const N: usize> NativeView for [T; N] {
    fn kind(&self) -> i32 {
        AR_VALUE_SEQUENCE
    }
    fn len(&self) -> usize {
        N
    }
    fn child(&self, index: usize) -> Option<(StringView, ValueView)> {
        self.get(index).map(|v| (StringView::new(""), view(v)))
    }
    fn bytes(&self) -> Option<(*const c_void, usize, usize)> {
        let v = self.first()?;
        if matches!(v.kind(), AR_VALUE_BOOL..=AR_VALUE_F64 | AR_VALUE_CHAR) && v.bytes().is_some() {
            Some((self.as_ptr().cast(), N, std::mem::size_of::<T>()))
        } else {
            None
        }
    }
}
impl<T: NativeView> NativeView for Option<T> {
    fn kind(&self) -> i32 {
        AR_VALUE_OPTION
    }
    fn len(&self) -> usize {
        usize::from(self.is_some())
    }
    fn child(&self, index: usize) -> Option<(StringView, ValueView)> {
        if index == 0 {
            self.as_ref().map(|v| (StringView::new("value"), view(v)))
        } else {
            None
        }
    }
}
impl<T: NativeView> NativeView for Box<T> {
    fn kind(&self) -> i32 {
        (**self).kind()
    }
    fn len(&self) -> usize {
        (**self).len()
    }
    fn child(&self, index: usize) -> Option<(StringView, ValueView)> {
        (**self).child(index)
    }
    fn bytes(&self) -> Option<(*const c_void, usize, usize)> {
        (**self).bytes()
    }
    fn variant(&self) -> Option<&'static str> {
        (**self).variant()
    }
    fn number(&self) -> Option<i64> {
        (**self).number()
    }
}
impl<T: NativeView> NativeView for Arc<T> {
    fn kind(&self) -> i32 {
        (**self).kind()
    }
    fn len(&self) -> usize {
        (**self).len()
    }
    fn child(&self, index: usize) -> Option<(StringView, ValueView)> {
        (**self).child(index)
    }
    fn bytes(&self) -> Option<(*const c_void, usize, usize)> {
        (**self).bytes()
    }
    fn variant(&self) -> Option<&'static str> {
        (**self).variant()
    }
    fn number(&self) -> Option<i64> {
        (**self).number()
    }
}
impl<A: NativeView, B: NativeView> NativeView for (A, B) {
    fn kind(&self) -> i32 {
        AR_VALUE_STRUCT
    }
    fn len(&self) -> usize {
        2
    }
    fn child(&self, i: usize) -> Option<(StringView, ValueView)> {
        match i {
            0 => Some((StringView::new("0"), view(&self.0))),
            1 => Some((StringView::new("1"), view(&self.1))),
            _ => None,
        }
    }
}
impl<A: NativeView, B: NativeView, C: NativeView> NativeView for (A, B, C) {
    fn kind(&self) -> i32 {
        AR_VALUE_STRUCT
    }
    fn len(&self) -> usize {
        3
    }
    fn child(&self, i: usize) -> Option<(StringView, ValueView)> {
        match i {
            0 => Some((StringView::new("0"), view(&self.0))),
            1 => Some((StringView::new("1"), view(&self.1))),
            2 => Some((StringView::new("2"), view(&self.2))),
            _ => None,
        }
    }
}
impl<K: NativeView + Eq + Hash, V: NativeView, S: BuildHasher> NativeView for HashMap<K, V, S> {
    fn kind(&self) -> i32 {
        AR_VALUE_MAP
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn child(&self, i: usize) -> Option<(StringView, ValueView)> {
        let (k, v) = self.iter().nth(i / 2)?;
        if i.is_multiple_of(2) {
            Some((StringView::new("key"), view(k)))
        } else {
            Some((StringView::new("value"), view(v)))
        }
    }
}
impl<K: NativeView + Ord, V: NativeView> NativeView for BTreeMap<K, V> {
    fn kind(&self) -> i32 {
        AR_VALUE_MAP
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn child(&self, i: usize) -> Option<(StringView, ValueView)> {
        let (k, v) = self.iter().nth(i / 2)?;
        if i.is_multiple_of(2) {
            Some((StringView::new("key"), view(k)))
        } else {
            Some((StringView::new("value"), view(v)))
        }
    }
}

impl NativeView for acadrust::Handle {
    fn kind(&self) -> i32 {
        AR_VALUE_U64
    }
    fn number(&self) -> Option<i64> {
        Some(self.value() as i64)
    }
}

unsafe fn descriptor(value: ValueView) -> FfiResult<&'static Descriptor> {
    if value.data.is_null() {
        return Err(invalid("value view is empty"));
    }
    value
        .descriptor
        .cast::<Descriptor>()
        .as_ref()
        .ok_or_else(|| invalid("value view is empty"))
}

#[no_mangle]
pub unsafe extern "C" fn ar_value_kind(value: ValueView, out: *mut i32) -> Status {
    ffi_call(|| {
        *output(out)? = (descriptor(value)?.kind)(value.data);
        Ok(())
    })
}
#[no_mangle]
pub unsafe extern "C" fn ar_value_type_name(value: ValueView, out: *mut StringView) -> Status {
    ffi_call(|| {
        *output(out)? = StringView::new((descriptor(value)?.type_name)());
        Ok(())
    })
}
#[no_mangle]
pub unsafe extern "C" fn ar_value_len(value: ValueView, out: *mut usize) -> Status {
    ffi_call(|| {
        *output(out)? = (descriptor(value)?.len)(value.data);
        Ok(())
    })
}
#[no_mangle]
pub unsafe extern "C" fn ar_value_child(
    value: ValueView,
    index: usize,
    name: *mut StringView,
    out: *mut ValueView,
) -> Status {
    ffi_call(|| {
        let name = output(name)?;
        let out = output(out)?;
        let (n, v) = (descriptor(value)?.child)(value.data, index)
            .ok_or_else(|| not_found("value child index is out of range"))?;
        *name = n;
        *out = v;
        Ok(())
    })
}
#[no_mangle]
pub unsafe extern "C" fn ar_value_field(
    value: ValueView,
    name: *const c_char,
    out: *mut ValueView,
) -> Status {
    ffi_call(|| {
        let out = output(out)?;
        let name = input_str(name)?;
        let d = descriptor(value)?;
        let count = (d.len)(value.data);
        for i in 0..count {
            if let Some((n, v)) = (d.child)(value.data, i) {
                if std::slice::from_raw_parts(n.data.cast::<u8>(), n.len) == name.as_bytes() {
                    *out = v;
                    return Ok(());
                }
            }
        }
        Err(not_found(format!("field {name:?} was not found")))
    })
}
#[no_mangle]
pub unsafe extern "C" fn ar_value_data(
    value: ValueView,
    out_data: *mut *const c_void,
    out_len: *mut usize,
    out_stride: *mut usize,
) -> Status {
    ffi_call(|| {
        let data = output(out_data)?;
        let len = output(out_len)?;
        let stride = output(out_stride)?;
        let (p, n, s) = (descriptor(value)?.bytes)(value.data).ok_or_else(|| {
            type_mismatch("value has no C-compatible contiguous storage; enumerate its children")
        })?;
        *data = p;
        *len = n;
        *stride = s;
        Ok(())
    })
}
#[no_mangle]
pub unsafe extern "C" fn ar_value_variant(value: ValueView, out: *mut StringView) -> Status {
    ffi_call(|| {
        *output(out)? = StringView::new(
            (descriptor(value)?.variant)(value.data)
                .ok_or_else(|| type_mismatch("value is not an enum"))?,
        );
        Ok(())
    })
}
#[no_mangle]
pub unsafe extern "C" fn ar_value_read_integer(value: ValueView, out: *mut i64) -> Status {
    ffi_call(|| {
        *output(out)? = (descriptor(value)?.number)(value.data).ok_or_else(|| {
            type_mismatch("value has no computed integer representation; use ar_value_data")
        })?;
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_view(
    doc: *const Document,
    handle: u64,
    out: *mut ValueView,
) -> Status {
    ffi_call(|| {
        let out = output(out)?;
        let entity = document(doc)?
            .get_entity(acadrust::Handle::new(handle))
            .ok_or_else(|| not_found("entity was not found"))?;
        *out = view(entity);
        Ok(())
    })
}
