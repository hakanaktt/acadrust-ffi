//! Native mutable projections. Borrowed read tokens are never used for mutation.

use std::collections::{BTreeMap, HashMap};
use std::ffi::{c_char, c_void};
use std::fmt::Display;
use std::hash::{BuildHasher, Hash};
use std::marker::PhantomData;
use std::sync::Arc;

use crate::*;
use acadrust::{CadDocument, Handle};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeWrite {
    pub kind: i32,
    pub data: *const c_void,
    pub len: usize,
}

pub(crate) struct WriteInput<'a> {
    kind: i32,
    bytes: &'a [u8],
    len: usize,
    stride: usize,
}

pub(crate) struct StagedWrite {
    kind: i32,
    bytes: Vec<u8>,
    len: usize,
    stride: usize,
}
impl StagedWrite {
    pub(crate) fn input(&self) -> WriteInput<'_> {
        WriteInput {
            kind: self.kind,
            bytes: &self.bytes,
            len: self.len,
            stride: self.stride,
        }
    }
}

impl WriteInput<'_> {
    fn element(&self, index: usize) -> WriteInput<'_> {
        WriteInput {
            kind: self.kind,
            bytes: &self.bytes[index * self.stride..(index + 1) * self.stride],
            len: 1,
            stride: self.stride,
        }
    }
    fn scalar<T: Copy>(&self, kind: i32) -> FfiResult<T> {
        if self.kind != kind || self.len != 1 || self.stride != std::mem::size_of::<T>() {
            return Err(type_mismatch(
                "write kind or element count does not match the native scalar",
            ));
        }
        Ok(unsafe { std::ptr::read_unaligned(self.bytes.as_ptr().cast::<T>()) })
    }
    fn text(&self) -> FfiResult<&str> {
        if self.kind != AR_VALUE_STRING {
            return Err(type_mismatch("expected UTF-8 string input"));
        }
        std::str::from_utf8(self.bytes).map_err(|_| invalid("write string is not valid UTF-8"))
    }
}

pub(crate) unsafe fn stage_write(input: *const NativeWrite) -> FfiResult<StagedWrite> {
    let input = input
        .as_ref()
        .ok_or_else(|| invalid("write input is NULL"))?;
    let stride = match input.kind {
        AR_VALUE_STRING | AR_VALUE_BOOL | AR_VALUE_I8 | AR_VALUE_U8 => 1,
        AR_VALUE_I16 | AR_VALUE_U16 => 2,
        AR_VALUE_I32 | AR_VALUE_U32 | AR_VALUE_F32 | AR_VALUE_CHAR => 4,
        AR_VALUE_I64 | AR_VALUE_U64 | AR_VALUE_F64 => 8,
        _ => {
            return Err(invalid(
                "write kind must describe a primitive scalar, array, or UTF-8 string",
            ))
        }
    };
    let bytes_len = input
        .len
        .checked_mul(stride)
        .filter(|n| *n <= isize::MAX as usize)
        .ok_or_else(|| invalid("write input is too large"))?;
    let bytes = if bytes_len == 0 {
        &[]
    } else {
        if input.data.is_null() {
            return Err(invalid("write data is NULL"));
        }
        std::slice::from_raw_parts(input.data.cast::<u8>(), bytes_len)
    };
    Ok(StagedWrite {
        kind: input.kind,
        bytes: bytes.to_vec(),
        len: input.len,
        stride,
    })
}

unsafe fn write_input(input: *const NativeWrite) -> FfiResult<StagedWrite> {
    stage_write(input)
}

pub(crate) fn protected(field: &str) -> bool {
    matches!(
        field,
        "handle"
            | "owner"
            | "owner_handle"
            | "handle_seed"
            | "entity_handles"
            | "insert_handles"
            | "block_entity_handle"
            | "block_end_handle"
            | "seqend_handle"
            | "xdictionary_handle"
            | "reactors"
            | "entity_mode"
            | "layout"
            | "block_record"
            | "dwg_source_version"
            | "source_path"
    ) || field.starts_with("raw_")
        || field.ends_with("_control_handle")
}

pub(crate) fn ensure_field(field: &str) -> FfiResult<()> {
    if protected(field) {
        Err(invalid(format!(
            "native identity or storage field {field:?} requires its dedicated API"
        )))
    } else {
        Ok(())
    }
}

pub(crate) trait NativeMut {
    fn write_integer(&mut self, value: i64) -> FfiResult<()> {
        let bytes = value.to_ne_bytes();
        self.write(&WriteInput {
            kind: AR_VALUE_I64,
            bytes: &bytes,
            len: 1,
            stride: 8,
        })
    }
    fn write_unsigned(&mut self, value: u64) -> FfiResult<()> {
        let bytes = value.to_ne_bytes();
        self.write(&WriteInput {
            kind: AR_VALUE_U64,
            bytes: &bytes,
            len: 1,
            stride: 8,
        })
    }
    fn accepts_f64(&self) -> bool {
        false
    }
    fn write_vector(&mut self, value: Vector3) -> FfiResult<()> {
        if [value.x, value.y, value.z].iter().any(|v| !v.is_finite()) {
            return Err(invalid("vector coordinates must be finite"));
        }
        for name in ["x", "y", "z"] {
            if !self.child_mut(name).is_ok_and(|value| value.accepts_f64()) {
                return Err(type_mismatch("target is not a native 3D vector"));
            }
        }
        for (name, component) in [("x", value.x), ("y", value.y), ("z", value.z)] {
            self.child_mut(name)?.write(&WriteInput {
                kind: AR_VALUE_F64,
                bytes: &component.to_ne_bytes(),
                len: 1,
                stride: 8,
            })?;
        }
        Ok(())
    }
    fn write_vectors(&mut self, _values: &[Vector3]) -> FfiResult<()> {
        Err(type_mismatch("target is not a vector sequence"))
    }
    fn writable_nested(
        &mut self,
        _parts: &[&str],
    ) -> Option<FfiResult<crate::writable::RawStorage>> {
        None
    }
    fn writable_storage(&mut self) -> FfiResult<crate::writable::RawStorage> {
        Err(type_mismatch("native value has no directly writable C storage; use a native setter or borrow a numeric/string child"))
    }
    fn writable_kind() -> Option<i32>
    where
        Self: Sized,
    {
        None
    }
    fn edit_nested(&mut self, _parts: &[&str], _edit: &Edit<'_>) -> Option<FfiResult<()>> {
        None
    }
    fn child_mut(&mut self, _segment: &str) -> FfiResult<&mut dyn NativeMut> {
        Err(not_found("native field or element was not found"))
    }
    fn write(&mut self, _input: &WriteInput<'_>) -> FfiResult<()> {
        Err(type_mismatch("native value cannot accept primitive input"))
    }
    fn resize(&mut self, _len: usize) -> FfiResult<()> {
        Err(type_mismatch("native value is not a resizable vector"))
    }
    fn select(&mut self, _variant: &str) -> FfiResult<()> {
        Err(type_mismatch("native value is not a selectable enum"))
    }
    fn option(&mut self, _enabled: bool) -> FfiResult<()> {
        Err(type_mismatch("native value is not optional"))
    }
    fn variant_name(&self) -> Option<&'static str> {
        None
    }
    fn invalidate(&mut self) {}
    fn after_child(&mut self, _segment: &str) {
        self.invalidate();
    }
    fn clear_storage(&mut self) {}
    fn create() -> FfiResult<Self>
    where
        Self: Sized,
    {
        Err(type_mismatch("native type has no default constructor; select a supported variant or use its dedicated constructor"))
    }
    fn from_input(_input: &WriteInput<'_>) -> FfiResult<Self>
    where
        Self: Sized,
    {
        Err(type_mismatch(
            "native element cannot be constructed from primitive input",
        ))
    }
    fn input_kind() -> Option<i32>
    where
        Self: Sized,
    {
        None
    }
}

pub(crate) struct DefaultOf<T>(pub PhantomData<T>);
impl<T> Copy for DefaultOf<T> {}
impl<T> Clone for DefaultOf<T> {
    fn clone(&self) -> Self {
        *self
    }
}
pub(crate) trait DefaultDispatch<T> {
    fn default_value(self) -> FfiResult<T>;
}
impl<T: Default> DefaultDispatch<T> for &DefaultOf<T> {
    fn default_value(self) -> FfiResult<T> {
        Ok(T::default())
    }
}
impl<T> DefaultDispatch<T> for DefaultOf<T> {
    fn default_value(self) -> FfiResult<T> {
        Err(type_mismatch("native type has no Default constructor"))
    }
}
pub(crate) trait NativeConstruct: Sized {
    fn construct() -> FfiResult<Self>;
}
pub(crate) struct ConstructOf<T>(pub PhantomData<T>);
impl<T> Copy for ConstructOf<T> {}
impl<T> Clone for ConstructOf<T> {
    fn clone(&self) -> Self {
        *self
    }
}
pub(crate) trait ConstructDispatch<T> {
    fn construct_value(self) -> FfiResult<T>;
}
impl<T: NativeConstruct> ConstructDispatch<T> for &ConstructOf<T> {
    fn construct_value(self) -> FfiResult<T> {
        T::construct()
    }
}
impl<T> ConstructDispatch<T> for ConstructOf<T> {
    fn construct_value(self) -> FfiResult<T> {
        Err(type_mismatch("native type has no available constructor"))
    }
}

pub(crate) fn invalidate_field<T: NativeMut>(name: &str, field: &mut T) {
    if matches!(
        name,
        "raw_record"
            | "raw_dwg_data"
            | "raw_dwg_version"
            | "raw_dxf_codes"
            | "raw_dxf_version"
            | "raw_plot_settings_codes"
            | "raw_data"
    ) {
        field.clear_storage();
    }
    if matches!(name, "common" | "base") {
        field.invalidate();
    }
}

pub(crate) fn invalidate_dependency<T: NativeMut>(edited: &str, name: &str, field: &mut T) {
    if matches!(
        (edited, name),
        ("layer", "layer_handle")
            | ("linetype", "linetype_handle")
            | ("color", "color_name")
            | ("color", "book_name")
    ) {
        field.clear_storage();
    }
}

macro_rules! mut_struct {
    ($ty:path { $($field:tt),* $(,)? }) => {
        #[allow(unused_imports, unused_variables)]
        impl $crate::mutations::NativeMut for $ty {
            fn child_mut(&mut self, segment: &str) -> $crate::FfiResult<&mut dyn $crate::mutations::NativeMut> {
                $crate::mutations::ensure_field(segment)?;
                match segment { $(stringify!($field) => Ok(&mut self.$field),)* _ => Err($crate::not_found("native field was not found")) }
            }
            fn invalidate(&mut self) { $($crate::mutations::invalidate_field(stringify!($field), &mut self.$field);)* }
            fn after_child(&mut self, segment: &str) { self.invalidate(); $($crate::mutations::invalidate_dependency(segment,stringify!($field),&mut self.$field);)* }
            fn create() -> $crate::FfiResult<Self> {
                use $crate::mutations::DefaultDispatch as _;
                use $crate::mutations::ConstructDispatch as _;
                if let Ok(value)=(&$crate::mutations::DefaultOf::<Self>(std::marker::PhantomData)).default_value(){return Ok(value);}
                (&$crate::mutations::ConstructOf::<Self>(std::marker::PhantomData)).construct_value()
            }
        }
    };
}
pub(crate) use mut_struct;

macro_rules! mut_enum {
    ($ty:path { $($variant:ident $(($($payload:ident),*))? $({$($field:ident),*})?),* $(,)? }) => {
        #[allow(unused_variables, unused_mut, unused_imports, unreachable_code, clippy::redundant_closure_call)]
        impl $crate::mutations::NativeMut for $ty {
            fn child_mut(&mut self, segment: &str) -> $crate::FfiResult<&mut dyn $crate::mutations::NativeMut> {
                $crate::mutations::ensure_field(segment)?;
                match self { $(Self::$variant $(($($payload),*))? $({$($field),*})? => {
                    let count = 0 $($(+ { let _ = &$payload; 1 })*)?;
                    let mut position = 0usize;
                    $($(if segment == stringify!($payload) || segment.parse::<usize>().ok() == Some(position) || (count == 1 && segment == "value") { return Ok($payload); } position += 1;)*)?
                    $($(if segment == stringify!($field) { return Ok($field); })*)?
                    $($(if count == 1 { return $payload.child_mut(segment); })*)?
                    let _ = position;
                    Err($crate::not_found("native enum payload field was not found"))
                }),* }
            }
            fn select(&mut self, variant: &str) -> $crate::FfiResult<()> {
                let value = match variant {
                    $(stringify!($variant) => Self::$variant $(($($crate::mutations::create_field(stringify!($payload))?),*))? $({$($field: $crate::mutations::create_field(stringify!($field))?),*})?,)*
                    _ => return Err($crate::invalid("native enum variant was not found")),
                };
                *self = value; Ok(())
            }
            fn variant_name(&self) -> Option<&'static str> { Some(match self { $(Self::$variant $(($($payload),*))? $({$($field),*})? => stringify!($variant)),* }) }
            fn invalidate(&mut self) { match self { $(Self::$variant $(($($payload),*))? $({$($field),*})? => { $($($payload.invalidate();)*)? $($($crate::mutations::invalidate_field(stringify!($field), $field);)*)? }),* } }
            fn after_child(&mut self, segment: &str) { match self { $(Self::$variant $(($($payload),*))? $({$($field),*})? => { $($($payload.after_child(segment);)*)? $($($crate::mutations::invalidate_dependency(segment,stringify!($field),$field);)*)? }),* } self.invalidate(); }
            fn create() -> $crate::FfiResult<Self> {
                use $crate::mutations::DefaultDispatch as _;
                if let Ok(value) = (&$crate::mutations::DefaultOf::<Self>(std::marker::PhantomData)).default_value() { return Ok(value); }
                $(let value: $crate::FfiResult<Self> = (|| Ok(Self::$variant $(($($crate::mutations::create_field(stringify!($payload))?),*))? $({$($field: $crate::mutations::create_field(stringify!($field))?),*})?))(); if value.is_ok() { return value; })*
                Err($crate::type_mismatch("native enum has no constructible variant"))
            }
        }
    };
}
pub(crate) use mut_enum;
pub(crate) fn create_field<T: NativeMut>(_name: &str) -> FfiResult<T> {
    T::create()
}

macro_rules! construct_struct {
    ($ty:path { $($field:ident),* $(,)? })=>{impl NativeConstruct for $ty{fn construct()->FfiResult<Self>{Ok(Self{$($field:create_field(stringify!($field))?,)*})}}};
    ($ty:path => ( $($field:tt),* $(,)? ))=>{impl NativeConstruct for $ty{fn construct()->FfiResult<Self>{Ok(Self($(create_field(stringify!($field))?,)*))}}};
}

macro_rules! numeric_mut {
    ($($ty:ty => $kind:ident),* $(,)?) => { $(
        impl NativeMut for $ty {
            fn write_integer(&mut self, value: i64) -> FfiResult<()> { *self = Self::try_from(value).map_err(|_| invalid("integer exceeds target range"))?; Ok(()) }
            fn write_unsigned(&mut self, value: u64) -> FfiResult<()> { *self = Self::try_from(value).map_err(|_| invalid("integer exceeds target range"))?; Ok(()) }
            fn writable_storage(&mut self) -> FfiResult<crate::writable::RawStorage> { Ok(crate::writable::RawStorage::primitive(std::ptr::from_mut(self), 1, $kind)) }
            fn writable_kind() -> Option<i32> { Some($kind) }
            fn write(&mut self, input: &WriteInput<'_>) -> FfiResult<()> { *self = Self::from_input(input)?; Ok(()) }
            fn create() -> FfiResult<Self> { Ok(Self::default()) }
            fn from_input(input: &WriteInput<'_>) -> FfiResult<Self> { input.scalar($kind) }
            fn input_kind() -> Option<i32> { Some($kind) }
        }
    )* };
}
numeric_mut!(i8=>AR_VALUE_I8,u8=>AR_VALUE_U8,i16=>AR_VALUE_I16,u16=>AR_VALUE_U16,i32=>AR_VALUE_I32,u32=>AR_VALUE_U32,i64=>AR_VALUE_I64,u64=>AR_VALUE_U64);
#[cfg(target_pointer_width = "64")]
numeric_mut!(usize=>AR_VALUE_U64,isize=>AR_VALUE_I64);
#[cfg(target_pointer_width = "32")]
numeric_mut!(usize=>AR_VALUE_U32,isize=>AR_VALUE_I32);

macro_rules! float_mut {
    ($($ty:ty => $kind:ident),* $(,)?) => { $(impl NativeMut for $ty {
        fn accepts_f64(&self) -> bool { $kind == AR_VALUE_F64 }
        fn writable_storage(&mut self) -> FfiResult<crate::writable::RawStorage> { Ok(crate::writable::RawStorage::primitive(std::ptr::from_mut(self), 1, $kind)) }
        fn writable_kind() -> Option<i32> { Some($kind) }
        fn write(&mut self, input: &WriteInput<'_>) -> FfiResult<()> { *self = Self::from_input(input)?; Ok(()) }
        fn create() -> FfiResult<Self> { Ok(0.0) }
        fn from_input(input: &WriteInput<'_>) -> FfiResult<Self> { let value: Self=input.scalar($kind)?; if !value.is_finite() { return Err(invalid("native floating-point writes must be finite")); } Ok(value) }
        fn input_kind() -> Option<i32> { Some($kind) }
    })* };
}
float_mut!(f32=>AR_VALUE_F32,f64=>AR_VALUE_F64);

impl NativeMut for bool {
    fn write(&mut self, input: &WriteInput<'_>) -> FfiResult<()> {
        *self = Self::from_input(input)?;
        Ok(())
    }
    fn create() -> FfiResult<Self> {
        Ok(false)
    }
    fn from_input(input: &WriteInput<'_>) -> FfiResult<Self> {
        match input.scalar::<u8>(AR_VALUE_BOOL)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(invalid("boolean input must be 0 or 1")),
        }
    }
    fn input_kind() -> Option<i32> {
        Some(AR_VALUE_BOOL)
    }
}
impl NativeMut for char {
    fn write(&mut self, input: &WriteInput<'_>) -> FfiResult<()> {
        *self = Self::from_input(input)?;
        Ok(())
    }
    fn create() -> FfiResult<Self> {
        Ok('\0')
    }
    fn from_input(input: &WriteInput<'_>) -> FfiResult<Self> {
        char::from_u32(input.scalar(AR_VALUE_CHAR)?)
            .ok_or_else(|| invalid("character input is not a Unicode scalar"))
    }
    fn input_kind() -> Option<i32> {
        Some(AR_VALUE_CHAR)
    }
}
impl NativeMut for Handle {
    fn write_integer(&mut self, value: i64) -> FfiResult<()> {
        *self =
            Handle::new(u64::try_from(value).map_err(|_| invalid("handle must be nonnegative"))?);
        Ok(())
    }
    fn write_unsigned(&mut self, value: u64) -> FfiResult<()> {
        *self = Handle::new(value);
        Ok(())
    }
    fn write(&mut self, input: &WriteInput<'_>) -> FfiResult<()> {
        *self = Self::from_input(input)?;
        Ok(())
    }
    fn create() -> FfiResult<Self> {
        Ok(Handle::NULL)
    }
    fn from_input(input: &WriteInput<'_>) -> FfiResult<Self> {
        Ok(Handle::new(input.scalar(AR_VALUE_U64)?))
    }
    fn input_kind() -> Option<i32> {
        Some(AR_VALUE_U64)
    }
}
impl NativeMut for String {
    fn writable_storage(&mut self) -> FfiResult<crate::writable::RawStorage> {
        Ok(crate::writable::RawStorage::string(self))
    }
    fn write(&mut self, input: &WriteInput<'_>) -> FfiResult<()> {
        *self = Self::from_input(input)?;
        Ok(())
    }
    fn create() -> FfiResult<Self> {
        Ok(String::new())
    }
    fn from_input(input: &WriteInput<'_>) -> FfiResult<Self> {
        Ok(input.text()?.to_string())
    }
    fn clear_storage(&mut self) {
        self.clear();
    }
}

fn index(segment: &str) -> FfiResult<usize> {
    segment
        .parse()
        .map_err(|_| invalid("native sequence path segment must be an unsigned index"))
}

impl<T: NativeMut> NativeMut for Vec<T> {
    fn write_vectors(&mut self, values: &[Vector3]) -> FfiResult<()> {
        if values.is_empty() {
            let mut element = T::create()?;
            element.write_vector(Vector3::default())?;
        }
        let mut result = Vec::with_capacity(values.len());
        for value in values {
            let mut item = T::create()?;
            item.write_vector(*value)?;
            result.push(item);
        }
        *self = result;
        Ok(())
    }
    fn writable_storage(&mut self) -> FfiResult<crate::writable::RawStorage> {
        let kind = T::writable_kind().ok_or_else(|| {
            type_mismatch("borrow a numeric or byte array, or one of this sequence's children")
        })?;
        Ok(crate::writable::RawStorage::primitive(
            self.as_mut_ptr(),
            self.len(),
            kind,
        ))
    }
    fn child_mut(&mut self, segment: &str) -> FfiResult<&mut dyn NativeMut> {
        self.get_mut(index(segment)?)
            .map(|v| v as &mut dyn NativeMut)
            .ok_or_else(|| not_found("native vector index was not found"))
    }
    fn write(&mut self, input: &WriteInput<'_>) -> FfiResult<()> {
        *self = Self::from_input(input)?;
        Ok(())
    }
    fn resize(&mut self, len: usize) -> FfiResult<()> {
        if len <= self.len() {
            self.truncate(len);
            return Ok(());
        }
        let additional = len - self.len();
        let mut values = Vec::new();
        values
            .try_reserve_exact(additional)
            .map_err(|_| invalid("native vector size is too large"))?;
        for _ in 0..additional {
            values.push(T::create()?);
        }
        self.try_reserve(additional)
            .map_err(|_| invalid("native vector size is too large"))?;
        self.append(&mut values);
        Ok(())
    }
    fn clear_storage(&mut self) {
        self.clear();
    }
    fn create() -> FfiResult<Self> {
        Ok(Vec::new())
    }
    fn from_input(input: &WriteInput<'_>) -> FfiResult<Self> {
        if T::input_kind() != Some(input.kind) {
            return Err(type_mismatch(
                "native vector element kind does not match input",
            ));
        }
        let mut values = Vec::new();
        values
            .try_reserve_exact(input.len)
            .map_err(|_| invalid("native vector size is too large"))?;
        for i in 0..input.len {
            values.push(T::from_input(&input.element(i))?);
        }
        Ok(values)
    }
}
impl<T: NativeMut, const N: usize> NativeMut for [T; N] {
    fn writable_storage(&mut self) -> FfiResult<crate::writable::RawStorage> {
        let kind = T::writable_kind().ok_or_else(|| {
            type_mismatch("borrow a numeric or byte array, or one of this array's children")
        })?;
        Ok(crate::writable::RawStorage::primitive(
            self.as_mut_ptr(),
            N,
            kind,
        ))
    }
    fn child_mut(&mut self, segment: &str) -> FfiResult<&mut dyn NativeMut> {
        self.get_mut(index(segment)?)
            .map(|v| v as &mut dyn NativeMut)
            .ok_or_else(|| not_found("native array index was not found"))
    }
    fn write(&mut self, input: &WriteInput<'_>) -> FfiResult<()> {
        *self = Self::from_input(input)?;
        Ok(())
    }
    fn from_input(input: &WriteInput<'_>) -> FfiResult<Self> {
        if input.len != N {
            return Err(invalid("fixed native array length does not match input"));
        }
        Vec::<T>::from_input(input)?
            .try_into()
            .map_err(|_| invalid("fixed native array length does not match input"))
    }
    fn create() -> FfiResult<Self> {
        let mut values = Vec::with_capacity(N);
        for _ in 0..N {
            values.push(T::create()?);
        }
        values
            .try_into()
            .map_err(|_| invalid("fixed array creation failed"))
    }
}
impl<T: NativeMut> NativeMut for Option<T> {
    fn writable_storage(&mut self) -> FfiResult<crate::writable::RawStorage> {
        self.as_mut()
            .ok_or_else(|| not_found("optional value is absent"))?
            .writable_storage()
    }
    fn child_mut(&mut self, segment: &str) -> FfiResult<&mut dyn NativeMut> {
        let value = self
            .as_mut()
            .ok_or_else(|| not_found("optional native value is absent; enable it first"))?;
        if matches!(segment, "value" | "0") {
            Ok(value)
        } else {
            value.child_mut(segment)
        }
    }
    fn option(&mut self, enabled: bool) -> FfiResult<()> {
        if enabled {
            if self.is_none() {
                *self = Some(T::create()?);
            }
        } else {
            *self = None;
        }
        Ok(())
    }
    fn clear_storage(&mut self) {
        *self = None;
    }
    fn create() -> FfiResult<Self> {
        Ok(None)
    }
}
impl<T: NativeMut> NativeMut for Box<T> {
    fn writable_storage(&mut self) -> FfiResult<crate::writable::RawStorage> {
        (**self).writable_storage()
    }
    fn child_mut(&mut self, s: &str) -> FfiResult<&mut dyn NativeMut> {
        (**self).child_mut(s)
    }
    fn write(&mut self, v: &WriteInput<'_>) -> FfiResult<()> {
        (**self).write(v)
    }
    fn resize(&mut self, n: usize) -> FfiResult<()> {
        (**self).resize(n)
    }
    fn select(&mut self, s: &str) -> FfiResult<()> {
        (**self).select(s)
    }
    fn option(&mut self, b: bool) -> FfiResult<()> {
        (**self).option(b)
    }
    fn variant_name(&self) -> Option<&'static str> {
        (**self).variant_name()
    }
    fn invalidate(&mut self) {
        (**self).invalidate();
    }
    fn create() -> FfiResult<Self> {
        Ok(Box::new(T::create()?))
    }
}
impl<T: NativeMut + Clone> NativeMut for Arc<T> {
    fn writable_storage(&mut self) -> FfiResult<crate::writable::RawStorage> {
        Arc::make_mut(self).writable_storage()
    }
    fn child_mut(&mut self, s: &str) -> FfiResult<&mut dyn NativeMut> {
        Arc::make_mut(self).child_mut(s)
    }
    fn write(&mut self, v: &WriteInput<'_>) -> FfiResult<()> {
        Arc::make_mut(self).write(v)
    }
    fn resize(&mut self, n: usize) -> FfiResult<()> {
        Arc::make_mut(self).resize(n)
    }
    fn select(&mut self, s: &str) -> FfiResult<()> {
        Arc::make_mut(self).select(s)
    }
    fn option(&mut self, b: bool) -> FfiResult<()> {
        Arc::make_mut(self).option(b)
    }
    fn variant_name(&self) -> Option<&'static str> {
        (**self).variant_name()
    }
    fn invalidate(&mut self) {
        Arc::make_mut(self).invalidate();
    }
    fn create() -> FfiResult<Self> {
        Ok(Arc::new(T::create()?))
    }
}
macro_rules! tuple_mut { ($($ty:ident:$field:tt),+) => {impl<$($ty:NativeMut),+> NativeMut for ($($ty,)+) {
    fn child_mut(&mut self,s:&str)->FfiResult<&mut dyn NativeMut> {match s {$(stringify!($field)=>Ok(&mut self.$field),)+_=>Err(not_found("native tuple field was not found"))}}
    fn create()->FfiResult<Self> {Ok(($(<$ty>::create()?,)+))}
}}; }
tuple_mut!(A:0,B:1);
tuple_mut!(A:0,B:1,C:2);
impl<K: Eq + Hash + Display, V: NativeMut, S: BuildHasher> NativeMut for HashMap<K, V, S> {
    fn child_mut(&mut self, s: &str) -> FfiResult<&mut dyn NativeMut> {
        self.iter_mut()
            .find(|(k, _)| k.to_string() == s)
            .map(|(_, v)| v as &mut dyn NativeMut)
            .ok_or_else(|| not_found("native map key was not found"))
    }
}
impl<K: Ord + Display, V: NativeMut> NativeMut for BTreeMap<K, V> {
    fn child_mut(&mut self, s: &str) -> FfiResult<&mut dyn NativeMut> {
        self.iter_mut()
            .find(|(k, _)| k.to_string() == s)
            .map(|(_, v)| v as &mut dyn NativeMut)
            .ok_or_else(|| not_found("native map key was not found"))
    }
}

pub(crate) enum Edit<'a> {
    Integer(i64),
    Unsigned(u64),
    Vector(Vector3),
    Vectors(&'a [Vector3]),
    Touch,
    Write(WriteInput<'a>),
    Resize(usize),
    Select(&'a str),
    Option(bool),
}

pub(crate) fn edit_native_value(
    value: &mut dyn NativeMut,
    path: &str,
    edit: Edit<'_>,
) -> FfiResult<()> {
    let parts = path_parts(path)?;
    let parts = if parts.first().copied() == value.variant_name() {
        &parts[1..]
    } else {
        &parts[..]
    };
    let parts = if matches!(parts.first(), Some(&"v0" | &"0")) {
        &parts[1..]
    } else {
        parts
    };
    if parts.is_empty() {
        return Err(invalid("native mutation requires a property path"));
    }
    apply(value.child_mut("v0")?, parts, &edit)
}

fn apply(value: &mut dyn NativeMut, parts: &[&str], edit: &Edit<'_>) -> FfiResult<()> {
    if let Some(result) = value.edit_nested(parts, edit) {
        return result;
    }
    if let Some((segment, rest)) = parts.split_first() {
        if value.variant_name() == Some(*segment) {
            return apply(value, rest, edit);
        }
        let child = value.child_mut(segment)?;
        apply(child, rest, edit)?;
        value.after_child(segment);
    } else {
        match edit {
            Edit::Integer(number) => value.write_integer(*number)?,
            Edit::Unsigned(number) => value.write_unsigned(*number)?,
            Edit::Vector(vector) => value.write_vector(*vector)?,
            Edit::Vectors(vectors) => value.write_vectors(vectors)?,
            Edit::Touch => {}
            Edit::Write(input) => value.write(input)?,
            Edit::Resize(len) => value.resize(*len)?,
            Edit::Select(variant) => value.select(variant)?,
            Edit::Option(enabled) => value.option(*enabled)?,
        }
        value.invalidate();
    }
    Ok(())
}

fn path_parts(path: &str) -> FfiResult<Vec<&str>> {
    let path = path.trim_matches('/');
    if path.is_empty() {
        return Ok(Vec::new());
    }
    let parts = path.split('/').collect::<Vec<_>>();
    if parts.iter().any(|p| p.is_empty()) {
        return Err(invalid("native paths must not contain empty segments"));
    }
    Ok(parts)
}

pub(crate) fn borrow_target(
    doc: &mut CadDocument,
    target: &crate::writable::WriteTarget,
    path: &str,
) -> FfiResult<crate::writable::RawStorage> {
    use crate::writable::{storage_at, WriteTarget};
    let parts = path_parts(path)?;
    if parts.is_empty() {
        return Err(invalid("writable borrow requires a property path"));
    }
    match target {
        WriteTarget::Document => {
            let (first, rest) = parts.split_first().expect("nonempty path");
            let root: &mut dyn NativeMut = match *first {
                "header" => &mut doc.header,
                "summary_info" => &mut doc.summary_info,
                "maintenance_version" => &mut doc.maintenance_version,
                "preview" => &mut doc.preview,
                "section_view_style" => &mut doc.section_view_style,
                _ => {
                    return Err(invalid(
                        "document collection or identity requires its dedicated API",
                    ))
                }
            };
            storage_at(root, rest)
        }
        WriteTarget::Entity(handle) => {
            let key = Handle::new(*handle);
            let source_record = doc
                .get_entity(key)
                .ok_or_else(|| not_found("entity was not found"))?
                .common()
                .raw_record
                .clone();
            let entity = doc.get_entity_mut(key).expect("validated entity");
            match storage_at(entity, &parts) {
                Ok(storage) => Ok(storage),
                Err(error) => {
                    entity.common_mut().raw_record = source_record;
                    Err(error)
                }
            }
        }
        WriteTarget::Object(handle) => storage_at(
            doc.objects
                .get_mut(&Handle::new(*handle))
                .ok_or_else(|| not_found("object was not found"))?,
            &parts,
        ),
        WriteTarget::Table(kind, name) => {
            macro_rules! table {
                ($field:ident) => {
                    storage_at(
                        doc.$field
                            .get_mut(name)
                            .ok_or_else(|| not_found("table entry was not found"))?,
                        &parts,
                    )
                };
            }
            match *kind {
                AR_TABLE_LAYER => table!(layers),
                AR_TABLE_LINETYPE => table!(line_types),
                AR_TABLE_TEXT_STYLE => table!(text_styles),
                AR_TABLE_BLOCK_RECORD => table!(block_records),
                AR_TABLE_DIM_STYLE => table!(dim_styles),
                AR_TABLE_APP_ID => table!(app_ids),
                AR_TABLE_VIEW => table!(views),
                AR_TABLE_VPORT => table!(vports),
                AR_TABLE_UCS => table!(ucss),
                AR_TABLE_VX => table!(vx_table),
                _ => Err(invalid("unknown symbol table kind")),
            }
        }
        WriteTarget::TableIndex(kind, index) => {
            macro_rules! table {
                ($field:ident) => {
                    storage_at(
                        doc.$field
                            .iter_mut()
                            .nth(*index)
                            .ok_or_else(|| not_found("table entry index was not found"))?,
                        &parts,
                    )
                };
            }
            match *kind {
                AR_TABLE_LAYER => table!(layers),
                AR_TABLE_LINETYPE => table!(line_types),
                AR_TABLE_TEXT_STYLE => table!(text_styles),
                AR_TABLE_BLOCK_RECORD => table!(block_records),
                AR_TABLE_DIM_STYLE => table!(dim_styles),
                AR_TABLE_APP_ID => table!(app_ids),
                AR_TABLE_VIEW => table!(views),
                AR_TABLE_VPORT => table!(vports),
                AR_TABLE_UCS => table!(ucss),
                AR_TABLE_VX => table!(vx_table),
                _ => Err(invalid("unknown symbol table kind")),
            }
        }
    }
}

pub(crate) fn finish_target(
    doc: &mut CadDocument,
    target: &crate::writable::WriteTarget,
    path: &str,
) -> FfiResult<()> {
    use crate::writable::WriteTarget;
    match target {
        WriteTarget::Document => edit_document(doc, path, &Edit::Touch),
        WriteTarget::Entity(handle) => edit_entity(doc, *handle, path, &Edit::Touch),
        WriteTarget::Object(handle) => edit_object(doc, *handle, path, &Edit::Touch),
        WriteTarget::Table(kind, name) => edit_table(doc, *kind, name, path, &Edit::Touch),
        WriteTarget::TableIndex(kind, index) => {
            edit_table_at(doc, *kind, *index, path, &Edit::Touch)
        }
    }
}

fn edit_document(doc: &mut CadDocument, path: &str, edit: &Edit<'_>) -> FfiResult<()> {
    let parts = path_parts(path)?;
    let (first, rest) = parts
        .split_first()
        .ok_or_else(|| invalid("document mutation requires a field path"))?;
    match *first {
        "header" => apply(&mut doc.header, rest, edit),
        "summary_info" => apply(&mut doc.summary_info, rest, edit),
        "maintenance_version" => apply(&mut doc.maintenance_version, rest, edit),
        "preview" => apply(&mut doc.preview, rest, edit),
        "section_view_style" => apply(&mut doc.section_view_style, rest, edit),
        _ => Err(invalid(
            "document collection or identity requires its dedicated API",
        )),
    }
}

fn edit_entity(doc: &mut CadDocument, handle: u64, path: &str, edit: &Edit<'_>) -> FfiResult<()> {
    let parts = path_parts(path)?;
    if parts.is_empty() {
        return Err(invalid("entity mutation requires a property path"));
    }
    if parts.last() == Some(&"layer") {
        if let Edit::Write(input) = edit {
            if !doc.layers.contains(input.text()?) {
                return Err(not_found("entity layer was not found"));
            }
        }
    }
    if parts.last() == Some(&"radius") {
        if let Edit::Write(input) = edit {
            if input.scalar::<f64>(AR_VALUE_F64)? <= 0.0 {
                return Err(invalid("entity radius must be positive"));
            }
        }
    }
    let original = doc
        .get_entity(Handle::new(handle))
        .ok_or_else(|| not_found("entity handle was not found"))?;
    if matches!(edit, Edit::Touch) {
        let entity = doc
            .get_entity_mut(Handle::new(handle))
            .expect("validated entity");
        entity.common_mut().raw_record = None;
        return Ok(());
    }
    if parts.len() == 1 && original.variant_name() == Some(parts[0]) {
        return Err(invalid("entity mutation requires a property path"));
    }
    let identity = (original.common().handle, original.common().owner_handle);
    let mut candidate = original.clone();
    apply(&mut candidate, &parts, edit)?;
    candidate.common_mut().handle = identity.0;
    candidate.common_mut().owner_handle = identity.1;
    candidate.common_mut().raw_record = None;
    crate::entities::validate_entity(doc, &candidate)?;
    *doc.get_entity_mut(Handle::new(handle))
        .expect("validated entity") = candidate;
    Ok(())
}

fn edit_object(doc: &mut CadDocument, handle: u64, path: &str, edit: &Edit<'_>) -> FfiResult<()> {
    let parts = path_parts(path)?;
    if parts.is_empty() {
        return Err(invalid("object mutation requires a property path"));
    }
    let key = Handle::new(handle);
    let current = doc
        .objects
        .get(&key)
        .ok_or_else(|| not_found("object handle was not found"))?;
    if parts.len() > 1
        && parts.first().copied() == current.variant_name()
        && parts.get(1).copied() == current.variant_name()
    {
        return Err(invalid(
            "object mutation path cannot repeat the root variant",
        ));
    }
    let direct = parts
        .iter()
        .copied()
        .filter(|p| Some(*p) != current.variant_name() && !matches!(*p, "value" | "v0" | "0"))
        .collect::<Vec<_>>();
    let rename = if direct == ["name"] {
        if let (Some(old), Edit::Write(input)) = (object_name(current), edit) {
            let new = input.text()?.to_string();
            if new.is_empty() {
                return Err(invalid("object name must not be empty"));
            }
            let old = old.to_string();
            let owner = doc.object_owner(key).unwrap_or(Handle::NULL);
            let entries = match doc.objects.get(&owner) {
                Some(acadrust::objects::ObjectType::Dictionary(v)) => Some(&v.entries),
                Some(acadrust::objects::ObjectType::DictionaryWithDefault(v)) => Some(&v.entries),
                _ => None,
            };
            if entries.is_some_and(|entries| {
                entries.iter().any(|(name, h)| {
                    *h != key
                        && acadrust::tables::normalize_name(name)
                            == acadrust::tables::normalize_name(&new)
                })
            }) {
                return Err(invalid("object dictionary name already exists"));
            }
            Some((owner, old, new))
        } else {
            None
        }
    } else {
        None
    };
    let object = doc
        .objects
        .get_mut(&Handle::new(handle))
        .ok_or_else(|| not_found("object handle was not found"))?;
    if parts.len() == 1 && object.variant_name() == Some(parts[0]) {
        return Err(invalid("object mutation requires a property path"));
    }
    apply(object, &parts, edit)?;
    invalidate_object(object);
    crate::database_mutations::sync_object_side_views(doc, key);
    if let Some((owner, old, new)) = rename {
        match doc.objects.get_mut(&owner) {
            Some(acadrust::objects::ObjectType::Dictionary(dict)) => {
                for (name, h) in &mut dict.entries {
                    if *h == key && *name == old {
                        *name = new.clone();
                    }
                }
                for name in &mut dict.hard_owner_entries {
                    if *name == old {
                        *name = new.clone();
                    }
                }
            }
            Some(acadrust::objects::ObjectType::DictionaryWithDefault(dict)) => {
                for (name, h) in &mut dict.entries {
                    if *h == key && *name == old {
                        *name = new.clone();
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn object_name(object: &acadrust::objects::ObjectType) -> Option<&str> {
    use acadrust::objects::ObjectType::*;
    match object {
        Layout(v) => Some(&v.name),
        Group(v) => Some(&v.name),
        MLineStyle(v) => Some(&v.name),
        MultiLeaderStyle(v) => Some(&v.name),
        TableStyle(v) => Some(&v.name),
        Scale(v) => Some(&v.name),
        DictionaryVariable(v) => Some(&v.name),
        XRecord(v) => Some(&v.name),
        Material(v) => Some(&v.name),
        _ => None,
    }
}

fn edit_table(
    doc: &mut CadDocument,
    kind: i32,
    name: &str,
    path: &str,
    edit: &Edit<'_>,
) -> FfiResult<()> {
    let parts = path_parts(path)?;
    if parts.is_empty() || parts.contains(&"name") {
        return Err(invalid(
            "table mutation requires a property path; rename entries with the dedicated API",
        ));
    }
    macro_rules! apply_table {
        ($field:ident) => {
            apply(
                doc.$field
                    .get_mut(name)
                    .ok_or_else(|| not_found("table entry was not found"))?,
                &parts,
                edit,
            )
        };
    }
    match kind {
        AR_TABLE_LAYER => apply_table!(layers),
        AR_TABLE_LINETYPE => apply_table!(line_types),
        AR_TABLE_TEXT_STYLE => apply_table!(text_styles),
        AR_TABLE_BLOCK_RECORD => apply_table!(block_records),
        AR_TABLE_DIM_STYLE => apply_table!(dim_styles),
        AR_TABLE_APP_ID => apply_table!(app_ids),
        AR_TABLE_VIEW => apply_table!(views),
        AR_TABLE_VPORT => apply_table!(vports),
        AR_TABLE_UCS => apply_table!(ucss),
        AR_TABLE_VX => apply_table!(vx_table),
        _ => Err(invalid("unknown symbol table kind")),
    }
}

fn edit_table_at(
    doc: &mut CadDocument,
    kind: i32,
    entry_index: usize,
    path: &str,
    edit: &Edit<'_>,
) -> FfiResult<()> {
    let parts = path_parts(path)?;
    if parts.is_empty() || parts.contains(&"name") {
        return Err(invalid(
            "table mutation requires a property path; indexed names require their dedicated API",
        ));
    }
    macro_rules! entry {
        ($field:ident) => {
            apply(
                doc.$field
                    .iter_mut()
                    .nth(entry_index)
                    .ok_or_else(|| not_found("table entry index was not found"))?,
                &parts,
                edit,
            )
        };
    }
    match kind {
        AR_TABLE_LAYER => entry!(layers),
        AR_TABLE_LINETYPE => entry!(line_types),
        AR_TABLE_TEXT_STYLE => entry!(text_styles),
        AR_TABLE_BLOCK_RECORD => entry!(block_records),
        AR_TABLE_DIM_STYLE => entry!(dim_styles),
        AR_TABLE_APP_ID => entry!(app_ids),
        AR_TABLE_VIEW => entry!(views),
        AR_TABLE_VPORT => entry!(vports),
        AR_TABLE_UCS => entry!(ucss),
        AR_TABLE_VX => entry!(vx_table),
        _ => Err(invalid("unknown symbol table kind")),
    }
}

fn invalidate_object(object: &mut acadrust::objects::ObjectType) {
    use acadrust::objects::*;
    match object {
        ObjectType::Layout(v) => v.raw_plot_settings_codes = None,
        ObjectType::XRecord(v) => {
            v.raw_dwg_data = None;
            v.raw_dwg_version = None;
            v.raw_data.clear();
        }
        ObjectType::TableStyle(v) => v.raw_dxf_codes = None,
        ObjectType::SortEntitiesTable(v) => {
            v.raw_dxf_codes = None;
            v.raw_dxf_version = None;
        }
        ObjectType::RegisteredClass(v) => {
            v.raw_dwg_data = None;
            v.raw_dwg_version = None;
        }
        ObjectType::ClassObject(v) => {
            if let ClassObjectData::CsacDocumentOptions(options) = &mut v.data {
                options.raw_dwg_data = None;
                options.raw_dwg_version = None;
            }
        }
        ObjectType::Unknown {
            raw_dxf_codes,
            raw_dwg_data,
            raw_dwg_version,
            ..
        } => {
            *raw_dxf_codes = None;
            *raw_dwg_data = None;
            *raw_dwg_version = None;
        }
        _ => {}
    }
}

macro_rules! exports {
    ($write:ident,$resize:ident,$select:ident,$option:ident,$edit:ident $(,$arg:ident:$ty:ty)*) => {
        #[no_mangle] pub unsafe extern "C" fn $write(doc:*mut Document,$($arg:$ty,)*path:*const c_char,input:*const NativeWrite)->Status {ffi_call(||{let input=write_input(input)?;let path=input_str(path)?;$edit(document_mut(doc)?,$($arg,)*path,&Edit::Write(input.input()))})}
        #[no_mangle] pub unsafe extern "C" fn $resize(doc:*mut Document,$($arg:$ty,)*path:*const c_char,len:usize)->Status {ffi_call(||{let path=input_str(path)?;$edit(document_mut(doc)?,$($arg,)*path,&Edit::Resize(len))})}
        #[no_mangle] pub unsafe extern "C" fn $select(doc:*mut Document,$($arg:$ty,)*path:*const c_char,variant:*const c_char)->Status {ffi_call(||{let path=input_str(path)?;let variant=input_str(variant)?;$edit(document_mut(doc)?,$($arg,)*path,&Edit::Select(variant))})}
        #[no_mangle] pub unsafe extern "C" fn $option(doc:*mut Document,$($arg:$ty,)*path:*const c_char,enabled:u8)->Status {ffi_call(||{if enabled>1{return Err(invalid("optional enabled flag must be 0 or 1"));}let path=input_str(path)?;$edit(document_mut(doc)?,$($arg,)*path,&Edit::Option(enabled==1))})}
    };
}
exports!(
    ar_document_write,
    ar_document_resize,
    ar_document_select,
    ar_document_option,
    edit_document
);
exports!(ar_entity_write,ar_entity_resize,ar_entity_select,ar_entity_option,edit_entity,handle:u64);
exports!(ar_object_write,ar_object_resize,ar_object_select,ar_object_option,edit_object,handle:u64);
exports!(ar_class_write,ar_class_resize,ar_class_select,ar_class_option,edit_class,class_index:usize);
exports!(ar_table_entry_write,ar_table_entry_resize,ar_table_entry_select,ar_table_entry_option,edit_table_at,kind:i32,entry_index:usize);

fn edit_class(doc: &mut CadDocument, index: usize, path: &str, edit: &Edit<'_>) -> FfiResult<()> {
    let parts = path_parts(path)?;
    if parts.is_empty()
        || parts
            .iter()
            .any(|p| matches!(*p, "dxf_name" | "class_number"))
    {
        return Err(invalid(
            "class identity requires the class creation/removal API",
        ));
    }
    let mut classes = doc.classes.iter().cloned().collect::<Vec<_>>();
    let class = classes
        .get_mut(index)
        .ok_or_else(|| not_found("class index was not found"))?;
    apply(class, &parts, edit)?;
    doc.classes.clear();
    for class in classes {
        doc.classes.push_preserving(class);
    }
    Ok(())
}

unsafe fn table_edit(
    doc: *mut Document,
    kind: i32,
    name: *const c_char,
    path: *const c_char,
    edit: Edit<'_>,
) -> FfiResult<()> {
    let name = input_str(name)?;
    let path = input_str(path)?;
    edit_table(document_mut(doc)?, kind, name, path, &edit)
}
#[no_mangle]
pub unsafe extern "C" fn ar_table_write(
    doc: *mut Document,
    kind: i32,
    name: *const c_char,
    path: *const c_char,
    input: *const NativeWrite,
) -> Status {
    ffi_call(|| {
        let input = write_input(input)?;
        table_edit(doc, kind, name, path, Edit::Write(input.input()))
    })
}
#[no_mangle]
pub unsafe extern "C" fn ar_table_resize(
    doc: *mut Document,
    kind: i32,
    name: *const c_char,
    path: *const c_char,
    len: usize,
) -> Status {
    ffi_call(|| table_edit(doc, kind, name, path, Edit::Resize(len)))
}
#[no_mangle]
pub unsafe extern "C" fn ar_table_select(
    doc: *mut Document,
    kind: i32,
    name: *const c_char,
    path: *const c_char,
    variant: *const c_char,
) -> Status {
    ffi_call(|| table_edit(doc, kind, name, path, Edit::Select(input_str(variant)?)))
}
#[no_mangle]
pub unsafe extern "C" fn ar_table_option(
    doc: *mut Document,
    kind: i32,
    name: *const c_char,
    path: *const c_char,
    enabled: u8,
) -> Status {
    ffi_call(|| {
        if enabled > 1 {
            return Err(invalid("optional enabled flag must be 0 or 1"));
        }
        table_edit(doc, kind, name, path, Edit::Option(enabled == 1))
    })
}

impl<T: NativeMut + acadrust::tables::TableEntry> NativeMut for acadrust::tables::Table<T> {
    fn child_mut(&mut self, s: &str) -> FfiResult<&mut dyn NativeMut> {
        self.get_mut(s)
            .map(|v| v as &mut dyn NativeMut)
            .ok_or_else(|| not_found("native table entry was not found"))
    }
    fn create() -> FfiResult<Self> {
        Ok(Self::new())
    }
}
impl NativeMut for acadrust::classes::DxfClassCollection {}
impl NativeMut for acadrust::notification::NotificationCollection {}
impl NativeMut for acadrust::BlockGraph {}

impl<T: NativeMut> NativeMut for acadrust::entities::LegacyBorderOverrides<T> {
    fn child_mut(&mut self, s: &str) -> FfiResult<&mut dyn NativeMut> {
        match s {
            "flags" => Ok(&mut self.flags),
            "values" => Ok(&mut self.values),
            _ => Err(not_found("legacy border field was not found")),
        }
    }
}

impl NativeMut for acadrust::xdata::ExtendedData {
    fn writable_nested(
        &mut self,
        parts: &[&str],
    ) -> Option<FfiResult<crate::writable::RawStorage>> {
        let (app, rest) = parts.split_first()?;
        Some((|| {
            if *app == "raw_dwg_eed" {
                return Err(invalid("raw native storage is read-only"));
            }
            let position = self
                .records()
                .iter()
                .position(|r| r.application_name == *app)
                .ok_or_else(|| not_found("XDATA application record was not found"))?;
            let mut record = Box::new(self.remove_record(app).expect("existing XDATA record"));
            match crate::writable::storage_at(&mut *record, rest) {
                Ok(storage) => Ok(storage.with_record(self, position, record)),
                Err(error) => {
                    crate::writable::restore_record(self, position, *record);
                    Err(error)
                }
            }
        })())
    }
    fn edit_nested(&mut self, parts: &[&str], edit: &Edit<'_>) -> Option<FfiResult<()>> {
        if matches!(edit, Edit::Touch) {
            self.raw_dwg_eed.clear();
            return Some(Ok(()));
        }
        let (app, rest) = parts.split_first()?;
        Some((|| {
            if *app == "raw_dwg_eed" {
                return Err(invalid("raw native storage is read-only"));
            }
            if rest.first() == Some(&"application_name") {
                return Err(invalid(
                    "XDATA application names require their dedicated API",
                ));
            }
            // Upstream keeps records private and offers replacement instead of a mutable accessor.
            let mut record = self
                .get_record(app)
                .ok_or_else(|| not_found("XDATA application record was not found"))?
                .clone();
            apply(&mut record, rest, edit)?;
            self.upsert_record(record);
            self.raw_dwg_eed.clear();
            Ok(())
        })())
    }
    fn create() -> FfiResult<Self> {
        Ok(Self::new())
    }
}

#[repr(transparent)]
struct SortEntriesMut(acadrust::objects::SortEntitiesTable);
impl NativeMut for SortEntriesMut {
    fn child_mut(&mut self, s: &str) -> FfiResult<&mut dyn NativeMut> {
        self.0
            .entries_mut()
            .nth(index(s)?)
            .map(|v| v as &mut dyn NativeMut)
            .ok_or_else(|| not_found("sort entry index was not found"))
    }
    fn invalidate(&mut self) {
        self.0.rebuild_map();
    }
}
impl NativeMut for acadrust::objects::SortEntitiesTable {
    fn child_mut(&mut self, s: &str) -> FfiResult<&mut dyn NativeMut> {
        ensure_field(s)?;
        match s {
            "block_owner_handle" => Err(invalid("sort table ownership requires its dedicated API")),
            "entries" => {
                // A transparent mutable adapter preserves the exclusive native borrow.
                Ok(unsafe { &mut *std::ptr::from_mut(self).cast::<SortEntriesMut>() })
            }
            _ => Err(not_found("sort table field was not found")),
        }
    }
    fn create() -> FfiResult<Self> {
        Ok(Self::new())
    }
}

macro_rules! flags_mut {($($ty:path),* $(,)?)=>{$(
    impl NativeMut for $ty {
        fn write(&mut self,input:&WriteInput<'_>)->FfiResult<()> {*self=Self::from_input(input)?;Ok(())}
        fn create()->FfiResult<Self>{Ok(Self::empty())}
        fn from_input(input:&WriteInput<'_>)->FfiResult<Self>{let bits=input.scalar::<u64>(AR_VALUE_U64)?;let width=std::mem::size_of_val(&Self::empty().bits())*8;if width<64 && bits>(u64::MAX>>(64-width)){return Err(invalid("flag input exceeds its native bit width"));}Ok(Self::from_bits_retain(bits as _))}
    }
)*};}
flags_mut!(
    acadrust::objects::MultiLeaderPropertyOverrideFlags,
    acadrust::objects::LeaderLinePropertyOverrideFlags,
    acadrust::objects::TableCellStylePropertyFlags,
    acadrust::objects::TableBorderPropertyFlags,
    acadrust::objects::TableStyleFlags,
    acadrust::entities::mline::MLineFlags,
    acadrust::entities::mline::MLineStyleFlags,
    acadrust::entities::multileader::LeaderLinePropertyOverrideFlags,
    acadrust::entities::multileader::MultiLeaderPropertyOverrideFlags,
    acadrust::entities::polyface_mesh::PolyfaceMeshFlags,
    acadrust::entities::polyface_mesh::PolyfaceVertexFlags,
    acadrust::entities::polygon_mesh::PolygonMeshFlags,
    acadrust::entities::raster_image::ImageDisplayFlags,
    acadrust::entities::table::CellEdgeFlags,
    acadrust::entities::table::CellStateFlags,
    acadrust::entities::table::CellStylePropertyFlags,
    acadrust::entities::table::BorderPropertyFlags,
    acadrust::entities::table::ContentLayoutFlags,
    acadrust::entities::table::BreakOptionFlags,
    acadrust::entities::underlay::UnderlayDisplayFlags,
    acadrust::entities::wipeout::WipeoutDisplayFlags,
);
macro_rules! native_flags_mut {($($ty:path),* $(,)?)=>{$(
    impl NativeMut for $ty {
        fn write(&mut self,input:&WriteInput<'_>)->FfiResult<()> {*self=Self::from_input(input)?;Ok(())}
        fn create()->FfiResult<Self>{Ok(Self::new())}
        fn from_input(input:&WriteInput<'_>)->FfiResult<Self>{let bits=input.scalar::<u64>(AR_VALUE_U64)?;let width=std::mem::size_of_val(&Self::new().bits())*8;if width<64 && bits>(u64::MAX>>(64-width)){return Err(invalid("flag input exceeds its native bit width"));}Ok(Self::from_bits(bits as _))}
    }
)*};}
native_flags_mut!(
    acadrust::entities::InvisibleEdgeFlags,
    acadrust::entities::BoundaryPathFlags,
    acadrust::entities::PolylineFlags,
    acadrust::entities::VertexFlags
);
impl NativeMut for acadrust::LayerStateMask {
    fn write(&mut self, input: &WriteInput<'_>) -> FfiResult<()> {
        *self = Self::from_bits(input.scalar::<u64>(AR_VALUE_U64)? as u32);
        Ok(())
    }
    fn create() -> FfiResult<Self> {
        Ok(Self::NONE)
    }
}
impl NativeMut for acadrust::entities::mtext_format::MTextStroke {
    fn write(&mut self, input: &WriteInput<'_>) -> FfiResult<()> {
        let bits = input.scalar::<u64>(AR_VALUE_U64)?;
        if bits > 7 {
            return Err(invalid("MTextStroke supports only bits 0 through 2"));
        }
        self.set_underline(bits & 1 != 0);
        self.set_strikethrough(bits & 2 != 0);
        self.set_overline(bits & 4 != 0);
        Ok(())
    }
    fn create() -> FfiResult<Self> {
        Ok(Self::new())
    }
}
mut_enum!(acadrust::tables::linetype::LineTypeComplexContent {
    Shape { shape_number },
    Text { text },
});

mut_enum!(acadrust::objects::AssocConstraintNodeData {
    None,
    Geometrical { owner_id, is_implied, is_active },
    Composite { owner_id, is_implied, is_active, owned_constraint_ids },
    HelpParameter { value, reserved },
    Angle { owner_id, is_implied, is_active, value_dependency, dimension_dependency, sector_type },
    Parallel { owner_id, is_implied, is_active, datum_line_index },
    Distance { owner_id, is_implied, is_active, value_dependency, dimension_dependency, direction_type, distance },
    RadiusDiameter { owner_id, is_implied, is_active, value_dependency, dimension_dependency, mode },
    ImplicitPoint { geometry_dependency, geometry_node_id, point, point_type, point_index, curve_id },
    Point { geometry_dependency, geometry_node_id, point },
    RigidSet { geometry_dependency, geometry_node_id, reserved, transform, geometry_ids },
    Line { geometry_dependency, geometry_node_id, point, direction },
    BoundedLine { geometry_dependency, geometry_node_id, point, direction, is_ray, start_point, end_point },
    Circle { geometry_dependency, geometry_node_id, center, normal, direction, radius, start_parameter, end_parameter, reserved },
    Arc { geometry_dependency, geometry_node_id, center, normal, direction, radius, start_parameter, end_parameter, reserved, start_point, end_point },
    Ellipse { geometry_dependency, geometry_node_id, center, major_axis, axis_ratio },
    BoundedEllipse { geometry_dependency, geometry_node_id, center, major_axis, axis_ratio, start_point, end_point },
    Spline { geometry_dependency, geometry_node_id, rational, periodic, degree, knot_tolerance, knot_physical_length, knot_grow_length, knots, weight_physical_length, weight_grow_length, weights, control_point_physical_length, control_point_grow_length, control_points, implicit_point_ids },
});

mut_enum!(acadrust::objects::ViewRepSketchGeometry {
    None,
    Line { type_code, first, second },
    Circle { type_code, center, normal, direction, radius, start_parameter, end_parameter, reserved },
    Nurb { type_code, flags, degree, tolerance, knot_header, knots, weight_header, weights, point_header, control_points },
});

mut_enum!(acadrust::objects::DgnLineStyleData {
    Definition { description, version, style_number, component_uid, is_continuous, unit_definition, unit_scale, units_type, is_element, is_physical, is_scale_independent, is_snappable, root_component, properties },
    Component { kind, description, version, component_uid, scale, property_flags, component, properties },
    Registered { dxf_name, properties, payload, object_ids },
});

mut_enum!(acadrust::objects::ObjectType {
    Dictionary(v0),
    Layout(v0),
    XRecord(v0),
    Group(v0),
    MLineStyle(v0),
    ImageDefinition(v0),
    UnderlayDefinition(v0),
    PlotSettings(v0),
    MultiLeaderStyle(v0),
    TableStyle(v0),
    TableContent(v0),
    Scale(v0),
    ObjectContextData(v0),
    SortEntitiesTable(v0),
    DictionaryVariable(v0),
    VisualStyle(v0),
    Material(v0),
    ImageDefinitionReactor(v0),
    GeoData(v0),
    SpatialFilter(v0),
    RasterVariables(v0),
    BookColor(v0),
    PlaceHolder(v0),
    DictionaryWithDefault(v0),
    WipeoutVariables(v0),
    BlockVisibilityParameter(v0),
    DynamicBlock(v0),
    Associative(v0),
    ClassObject(v0),
    DataObject(v0),
    Field(v0),
    FieldList(v0),
    RegisteredClass(v0),
    DgnLineStyle(v0),
    ProxyObject(v0),
    Unknown { type_name, handle, owner, raw_dxf_codes, raw_dwg_data, raw_dwg_handle_bits, raw_dwg_version },
});

mut_enum!(acadrust::objects::ObjectContextKind {
    AnnotScale,
    BlkRef { rotation, insertion, scale_factor },
    Text { horizontal_mode, rotation, insertion, alignment },
    MText(v0),
    Dim(v0),
    MLeader(v0),
    MTextAttribute(v0),
    Leader(v0),
    Fcf { location, horizontal_direction },
    HatchScale(v0),
    HatchView(v0),
    Opaque,
});

mut_enum!(acadrust::objects::DimSubtype {
    Aligned { dimline_pt },
    Angular { arc_pt },
    Diametric { first_arc_pt, def_pt },
    Radial { first_arc_pt },
    RadialLarge { ovr_center, jog_point },
    Ordinate { feature_location_pt, leader_endpt },
});

mut_enum!(acadrust::objects::XRecordValue {
    String(v0),
    Double(v0),
    Int16(v0),
    Int32(v0),
    Int64(v0),
    Byte(v0),
    Bool(v0),
    Handle(v0),
    Point3D(v0, v1, v2),
    Chunk(v0),
});

// Construction of fully public native records without a Default implementation.
construct_struct!(acadrust::types::bounds::BoundingBox2D { min, max });
construct_struct!(acadrust::types::bounds::BoundingBox3D { min, max });
construct_struct!(acadrust::types::transform::Matrix3 { m });
construct_struct!(acadrust::types::transform::Matrix4 { m });
construct_struct!(acadrust::types::transform::Transform { matrix });
construct_struct!(acadrust::types::vector::Vector2 { x, y });
construct_struct!(acadrust::types::vector::Vector3 { x, y, z });
construct_struct!(acadrust::xdata::ExtendedDataRecord {
    application_name,
    values
});
construct_struct!(acadrust::entities::arc::Arc {
    common,
    center,
    radius,
    start_angle,
    end_angle,
    thickness,
    normal
});
construct_struct!(acadrust::entities::attribute_definition::AttributeFlags {
    invisible,
    constant,
    verify,
    preset,
    locked_position,
    annotative
});
construct_struct!(
    acadrust::entities::attribute_definition::AttributeDefinition {
        common,
        tag,
        prompt,
        default_value,
        insertion_point,
        alignment_point,
        height,
        rotation,
        width_factor,
        oblique_angle,
        text_style,
        text_generation_flags,
        horizontal_alignment,
        vertical_alignment,
        flags,
        field_length,
        normal,
        mtext_flag,
        is_multiline,
        line_count,
        embedded_mtext,
        lock_position
    }
);
construct_struct!(acadrust::entities::attribute_entity::AttributeEntity {
    common,
    tag,
    value,
    insertion_point,
    alignment_point,
    height,
    rotation,
    width_factor,
    oblique_angle,
    text_style,
    text_generation_flags,
    horizontal_alignment,
    vertical_alignment,
    flags,
    field_length,
    normal,
    mtext_flag,
    is_multiline,
    line_count,
    embedded_mtext,
    attdef_handle,
    lock_position
});
construct_struct!(acadrust::entities::block::Block {
    common,
    name,
    base_point,
    description,
    xref_path
});
construct_struct!(acadrust::entities::block::BlockEnd { common });
construct_struct!(acadrust::entities::centerline::CenterLineSource {
    handle,
    kind,
    segment_index,
    pick_point
});
construct_struct!(acadrust::entities::centerline::CenterLineAssociation {
    first,
    second,
    plane_origin,
    plane_x,
    plane_y,
    start_extension,
    end_extension,
    start_length_adjustment,
    end_length_adjustment,
    associated
});
construct_struct!(acadrust::entities::centerline::CenterMarkSource {
    handle,
    kind,
    segment_index,
    pick_point
});
construct_struct!(acadrust::entities::centerline::CenterMarkAssociation {
    source,
    plane_origin,
    plane_x,
    plane_y,
    center,
    radius,
    cross_size,
    cross_gap,
    cross_size_relative,
    cross_gap_relative,
    extension_length,
    length_adjustments,
    overshoots,
    show_extensions,
    associated
});
construct_struct!(acadrust::entities::circle::Circle {
    common,
    center,
    radius,
    thickness,
    normal
});
construct_struct!(acadrust::entities::dimension::DimensionBase {
    common,
    definition_point,
    text_middle_point,
    insertion_point,
    dimension_type,
    attachment_point,
    text,
    user_text,
    normal,
    text_rotation,
    horizontal_direction,
    style_name,
    actual_measurement,
    version,
    block_name,
    line_spacing_factor,
    line_spacing_style,
    insertion_scale,
    insertion_rotation,
    dwg_unknown_bit,
    flip_arrow1,
    flip_arrow2,
    dwg_flags_byte,
    text_user_positioned
});
construct_struct!(acadrust::entities::dimension::DimensionAligned {
    base,
    first_point,
    second_point,
    definition_point,
    ext_line_rotation
});
construct_struct!(acadrust::entities::dimension::DimensionLinear {
    base,
    first_point,
    second_point,
    definition_point,
    rotation,
    ext_line_rotation
});
construct_struct!(acadrust::entities::dimension::DimensionRadius {
    base,
    definition_point,
    angle_vertex,
    leader_length
});
construct_struct!(acadrust::entities::dimension::DimensionDiameter {
    base,
    definition_point,
    angle_vertex,
    leader_length
});
construct_struct!(acadrust::entities::dimension::DimensionAngular2Ln {
    base,
    dimension_arc,
    first_point,
    second_point,
    angle_vertex,
    definition_point
});
construct_struct!(acadrust::entities::dimension::DimensionAngular3Pt {
    base,
    definition_point,
    first_point,
    second_point,
    angle_vertex
});
construct_struct!(acadrust::entities::dimension::DimensionOrdinate {
    base,
    definition_point,
    feature_location,
    leader_endpoint,
    is_ordinate_type_x
});
construct_struct!(acadrust::entities::dimension::DimensionArc {
    base,
    definition_point,
    first_extension_point,
    second_extension_point,
    center_point,
    is_partial,
    arc_start_parameter,
    arc_end_parameter,
    has_leader,
    first_leader_point,
    second_leader_point
});
construct_struct!(acadrust::entities::dimension::DimensionLargeRadial {
    base,
    definition_point,
    chord_point,
    jog_angle,
    override_center,
    jog_point
});
construct_struct!(acadrust::entities::ellipse::Ellipse {
    common,
    center,
    major_axis,
    minor_axis_ratio,
    start_parameter,
    end_parameter,
    normal
});
construct_struct!(acadrust::entities::extended_entity::ExtendedEntity { common, data });
construct_struct!(acadrust::entities::extended_entity::SectionObjectData {
    state,
    flags,
    name,
    vertical_direction,
    top_height,
    bottom_height,
    indicator_alpha,
    indicator_color,
    vertices,
    back_line_vertices,
    settings_handle
});
construct_struct!(acadrust::entities::extended_entity::ArcAlignedTextData {
    text,
    font_name,
    big_font_name,
    style_name,
    center,
    radius,
    x_scale,
    text_size,
    character_spacing,
    offset_from_arc,
    right_offset,
    left_offset,
    start_angle,
    end_angle,
    reverse,
    text_direction,
    alignment,
    text_position,
    bold,
    italic,
    underlined,
    character_set,
    pitch_and_family,
    is_shx,
    text_color,
    normal,
    wizard_flag,
    arc_handle
});
construct_struct!(acadrust::entities::extended_entity::RemoteTextData {
    position,
    normal,
    rotation,
    height,
    style_handle,
    style_name,
    flags,
    text
});
construct_struct!(acadrust::entities::extended_entity::GeoPositionMarkerData {
    class_version,
    position,
    radius,
    notes,
    landing_gap,
    mtext_visible,
    text_alignment,
    enable_frame_text,
    embedded_mtext
});
construct_struct!(acadrust::entities::extended_entity::CoordinationModelData {
    flags,
    definition_handle,
    transform,
    unit_factor
});
construct_struct!(acadrust::entities::extended_entity::PointCloudClip {
    inverted,
    clip_type,
    vertices,
    z_min,
    z_max
});
construct_struct!(acadrust::entities::extended_entity::PointCloudData {
    class_version,
    origin,
    saved_filename,
    source_files,
    extents_min,
    extents_max,
    point_count,
    ucs_name,
    ucs_origin,
    ucs_x_direction,
    ucs_y_direction,
    ucs_z_direction,
    definition_handle,
    reactor_handle,
    show_intensity,
    intensity_scheme,
    minimum_intensity,
    maximum_intensity,
    low_intensity_threshold,
    high_intensity_threshold,
    show_clipping,
    clippings
});
construct_struct!(acadrust::entities::extended_entity::PointCloudExCrop {
    crop_type,
    inside,
    inverted,
    plane,
    x_direction,
    y_direction,
    points
});
construct_struct!(acadrust::entities::extended_entity::PointCloudExData {
    class_version,
    extents_min,
    extents_max,
    ucs_origin,
    ucs_x_direction,
    ucs_y_direction,
    ucs_z_direction,
    locked,
    definition_handle,
    reactor_handle,
    name,
    show_intensity,
    show_cropping,
    hidden_scans,
    hidden_regions,
    stylization_type,
    intensity_color_scheme,
    current_color_scheme,
    classification_color_scheme,
    elevation_min,
    elevation_max,
    intensity_min,
    intensity_max,
    intensity_out_of_range_behavior,
    elevation_out_of_range_behavior,
    elevation_apply_to_fixed_range,
    intensity_as_gradient,
    elevation_as_gradient,
    croppings
});
construct_struct!(acadrust::entities::extended_entity::ProxyEntityData {
    proxy_id,
    class_id,
    dxf_subclass,
    version,
    dwg_version,
    maintenance_version,
    from_dxf,
    graphics,
    payload,
    text_payload,
    object_ids
});
construct_struct!(acadrust::entities::extended_entity::OleFrameData {
    flag,
    mode,
    storage
});
construct_struct!(acadrust::entities::extended_entity::LayoutPrintConfigData {
    class_version,
    flag,
    raw_dwg_data,
    raw_dwg_handle_bits,
    raw_dwg_version
});
construct_struct!(acadrust::entities::extended_entity::FormatData {
    raw_dwg_data,
    raw_dwg_handle_bits,
    raw_dwg_version,
    raw_dxf_codes
});
construct_struct!(
    acadrust::entities::extended_entity::RegisteredClassEntityData {
        dxf_name,
        cpp_class_name,
        properties,
        payload,
        object_ids
    }
);
construct_struct!(acadrust::entities::face3d::Face3D {
    common,
    first_corner,
    second_corner,
    third_corner,
    fourth_corner,
    invisible_edges
});
construct_struct!(acadrust::entities::hatch::LineEdge { start, end });
construct_struct!(acadrust::entities::hatch::CircularArcEdge {
    center,
    radius,
    start_angle,
    end_angle,
    counter_clockwise
});
construct_struct!(acadrust::entities::hatch::EllipticArcEdge {
    center,
    major_axis_endpoint,
    minor_axis_ratio,
    start_angle,
    end_angle,
    counter_clockwise
});
construct_struct!(acadrust::entities::hatch::SplineEdge {
    degree,
    rational,
    periodic,
    knots,
    control_points,
    fit_points,
    start_tangent,
    end_tangent
});
construct_struct!(acadrust::entities::hatch::PolylineEdge {
    vertices,
    is_closed
});
construct_struct!(acadrust::entities::hatch::BoundaryPath {
    flags,
    edges,
    boundary_handles
});
construct_struct!(acadrust::entities::hatch::HatchPatternLine {
    angle,
    base_point,
    offset,
    dash_lengths
});
construct_struct!(acadrust::entities::hatch::HatchPattern {
    name,
    description,
    lines
});
construct_struct!(acadrust::entities::hatch::GradientColorEntry {
    value,
    color,
    color_name,
    book_name
});
construct_struct!(acadrust::entities::hatch::HatchGradientPattern {
    enabled,
    reserved,
    angle,
    shift,
    is_single_color,
    color_tint,
    colors,
    name
});
construct_struct!(acadrust::entities::hatch::Hatch {
    common,
    is_mpolygon,
    elevation,
    normal,
    pattern,
    is_solid,
    is_associative,
    pattern_type,
    pattern_angle,
    pattern_scale,
    is_double,
    style,
    paths,
    seed_points,
    pixel_size,
    gradient_color,
    mpolygon_hatch_color,
    mpolygon_x_direction,
    mpolygon_invalid_loops
});
construct_struct!(acadrust::entities::helix::Helix {
    common,
    spline,
    major_version,
    maintenance_version,
    axis_base_point,
    start_point,
    axis_vector,
    radius,
    turns,
    turn_height,
    handedness,
    constraint
});
construct_struct!(acadrust::entities::leader::Leader {
    common,
    dimension_style,
    arrow_enabled,
    path_type,
    creation_type,
    hookline_direction,
    hookline_enabled,
    text_height,
    text_width,
    vertices,
    override_color,
    annotation_handle,
    normal,
    horizontal_direction,
    block_offset,
    annotation_offset,
    origin,
    dimension_gap,
    arrowhead_type,
    arrow_size,
    byblock_color,
    dwg_unknown_bit1,
    dwg_unknown_bit2,
    dwg_unknown_bit3,
    dwg_unknown_bit4,
    dwg_unknown_bit5,
    dwg_unknown_short1
});
construct_struct!(acadrust::entities::light::LightPhotometricData {
    has_web_file,
    web_file,
    physical_intensity_method,
    physical_intensity,
    illuminance_distance,
    lamp_color_type,
    lamp_color_temperature,
    lamp_color_preset,
    web_rotation,
    extended_light_shape,
    extended_light_length,
    extended_light_width,
    extended_light_radius,
    web_file_type,
    web_symmetry,
    has_target_grip,
    web_flux,
    web_angles,
    glyph_display_type
});
construct_struct!(acadrust::entities::light::Light {
    common,
    name,
    class_version,
    light_type,
    position,
    target,
    status,
    light_color,
    plot_glyph,
    intensity,
    attenuation_type,
    use_attenuation_limits,
    attenuation_start_limit,
    attenuation_end_limit,
    hotspot_angle,
    falloff_angle,
    cast_shadows,
    shadow_type,
    shadow_map_size,
    shadow_map_softness,
    photometric_mode,
    photometric_data
});
construct_struct!(acadrust::entities::line::Line {
    common,
    start,
    end,
    thickness,
    normal
});
construct_struct!(acadrust::entities::lwpolyline::LwVertex {
    location,
    bulge,
    start_width,
    end_width,
    vertex_id
});
construct_struct!(acadrust::entities::lwpolyline::LwPolyline {
    common,
    vertices,
    is_closed,
    plinegen,
    constant_width,
    elevation,
    thickness,
    normal
});
construct_struct!(acadrust::entities::mesh::MeshEdge { start, end, crease });
construct_struct!(acadrust::entities::mesh::MeshFace { vertices });
construct_struct!(acadrust::entities::mesh::Mesh {
    common,
    version,
    blend_crease,
    subdivision_level,
    vertices,
    faces,
    edges,
    override_option
});
construct_struct!(acadrust::entities::mline::MLineStyleElement {
    offset,
    color,
    line_type_handle,
    line_type_name
});
construct_struct!(acadrust::entities::mline::MLineStyle {
    handle,
    name,
    description,
    flags,
    fill_color,
    start_angle,
    end_angle,
    elements
});
construct_struct!(acadrust::entities::mline::MLineSegment {
    parameters,
    area_fill_parameters
});
construct_struct!(acadrust::entities::mline::MLineVertex {
    position,
    direction,
    miter,
    segments
});
construct_struct!(acadrust::entities::mline::MLine {
    common,
    flags,
    justification,
    normal,
    scale_factor,
    start_point,
    style_handle,
    style_name,
    style_element_count,
    vertices
});
construct_struct!(acadrust::entities::RawRecord {
    data,
    handle_bits,
    version
});
construct_struct!(acadrust::entities::R12EntityMetadata {
    viewport,
    child_viewports
});
construct_struct!(acadrust::entities::EntityCommon {
    handle,
    layer,
    layer_handle,
    color,
    line_weight,
    linetype,
    linetype_handle,
    linetype_scale,
    transparency,
    color_name,
    invisible,
    extended_data,
    graphic_data,
    reactors,
    xdictionary_handle,
    owner_handle,
    color_book_handle,
    full_visual_style_handle,
    face_visual_style_handle,
    edge_visual_style_handle,
    material_flags,
    material_handle,
    shadow_flags,
    plotstyle_flags,
    plotstyle_handle,
    entity_mode,
    r12_metadata,
    has_ds_data,
    raw_record
});
construct_struct!(acadrust::entities::mtext::MTextColumnData {
    column_type,
    column_count,
    flow_reversed,
    auto_height,
    width,
    gutter,
    heights
});
construct_struct!(acadrust::entities::mtext::MText {
    common,
    value,
    insertion_point,
    height,
    rectangle_width,
    rectangle_height,
    rotation,
    dwg_x_direction,
    style,
    attachment_point,
    drawing_direction,
    line_spacing_factor,
    line_spacing_style,
    normal,
    background_fill_flags,
    background_scale,
    background_color,
    background_transparency,
    is_annotative,
    column_data,
    extents_width,
    extents_height
});
construct_struct!(acadrust::entities::multileader::StartEndPointPair {
    start_point,
    end_point
});
construct_struct!(acadrust::entities::multileader::LeaderLineBreakInfo {
    segment_index,
    break_points
});
construct_struct!(acadrust::entities::multileader::LeaderLine {
    index,
    segment_index,
    points,
    break_points,
    break_info_count,
    break_infos,
    path_type,
    line_color,
    line_type_handle,
    line_weight,
    arrowhead_handle,
    arrowhead_size,
    override_flags
});
construct_struct!(acadrust::entities::multileader::LeaderRoot {
    leader_index,
    content_valid,
    unknown,
    connection_point,
    direction,
    break_points,
    lines,
    landing_distance,
    text_attachment_direction
});
construct_struct!(acadrust::entities::multileader::BlockAttribute {
    attribute_definition_handle,
    index,
    width,
    text
});
construct_struct!(
    acadrust::entities::multileader::MultiLeaderArrowheadOverride {
        index,
        is_default,
        arrowhead_handle
    }
);
construct_struct!(acadrust::entities::multileader::MultiLeaderAnnotContext {
    leader_roots,
    standalone_flags,
    standalone_uses_root_flags,
    scale_factor,
    content_base_point,
    has_text_contents,
    text_string,
    text_normal,
    text_location,
    text_direction,
    text_rotation,
    text_height,
    text_width,
    text_boundary_height,
    line_spacing_factor,
    line_spacing_style,
    text_color,
    text_attachment_point,
    text_flow_direction,
    text_alignment,
    text_left_attachment,
    text_right_attachment,
    text_top_attachment,
    text_bottom_attachment,
    text_height_automatic,
    word_break,
    dwg_unknown_text_bit,
    text_style_handle,
    has_block_contents,
    block_content_handle,
    block_content_normal,
    block_content_location,
    block_content_scale,
    block_rotation,
    block_content_color,
    block_connection_type,
    column_type,
    column_width,
    column_gutter,
    column_flow_reversed,
    column_sizes,
    background_fill_enabled,
    background_mask_fill_on,
    background_fill_color,
    background_scale_factor,
    background_transparency,
    base_point,
    base_direction,
    base_vertical,
    normal_reversed,
    arrowhead_size,
    landing_gap,
    transform_matrix,
    scale_handle
});
construct_struct!(acadrust::entities::multileader::MultiLeader {
    common,
    dwg_version,
    style_handle,
    content_type,
    context,
    block_attributes,
    arrowhead_overrides,
    path_type,
    line_color,
    line_type_handle,
    line_weight,
    enable_landing,
    enable_dogleg,
    dogleg_length,
    arrowhead_handle,
    arrowhead_size,
    text_style_handle,
    text_color,
    text_frame,
    text_height,
    text_left_attachment,
    text_right_attachment,
    text_top_attachment,
    text_bottom_attachment,
    text_attachment_direction,
    text_attachment_point,
    text_alignment,
    text_angle_type,
    text_direction_negative,
    text_align_in_ipe,
    block_content_handle,
    block_content_color,
    block_connection_type,
    block_rotation,
    block_scale,
    scale_factor,
    property_override_flags,
    enable_annotation_scale,
    extend_leader_to_text
});
construct_struct!(acadrust::entities::ole2frame::Ole2Frame {
    common,
    version,
    source_application,
    upper_left_corner,
    lower_right_corner,
    ole_object_type,
    is_paper_space,
    storage,
    envelope,
    dwg_mode,
    lock_aspect
});
construct_struct!(acadrust::entities::point::Point {
    common,
    location,
    thickness,
    normal,
    x_axis_angle
});
construct_struct!(acadrust::entities::polyface_mesh::PolyfaceVertex {
    common,
    location,
    flags,
    bulge,
    start_width,
    end_width,
    curve_tangent,
    id
});
construct_struct!(acadrust::entities::polyface_mesh::PolyfaceFace {
    common,
    flags,
    index1,
    index2,
    index3,
    index4,
    color
});
construct_struct!(acadrust::entities::polyface_mesh::PolyfaceMesh {
    common,
    elevation,
    flags,
    normal,
    start_width,
    end_width,
    smooth_surface,
    thickness,
    vertices,
    faces,
    seqend_handle
});
construct_struct!(acadrust::entities::polygon_mesh::PolygonMeshVertex {
    common,
    location,
    flags
});
construct_struct!(acadrust::entities::polygon_mesh::PolygonMesh {
    common,
    flags,
    m_vertex_count,
    n_vertex_count,
    m_smooth_density,
    n_smooth_density,
    smooth_type,
    elevation,
    normal,
    vertices
});
construct_struct!(acadrust::entities::polyline::Vertex2D {
    location,
    flags,
    start_width,
    end_width,
    bulge,
    curve_tangent,
    id
});
construct_struct!(acadrust::entities::polyline::Vertex3D { location, flags });
construct_struct!(acadrust::entities::polyline::Polyline2D {
    common,
    flags,
    smooth_surface,
    start_width,
    end_width,
    thickness,
    elevation,
    normal,
    vertices
});
construct_struct!(acadrust::entities::polyline::Polyline {
    common,
    flags,
    vertices
});
construct_struct!(acadrust::entities::polyline3d::Polyline3DFlags {
    closed,
    spline_fit,
    is_3d,
    is_3d_mesh,
    mesh_closed_n,
    is_polyface_mesh,
    linetype_continuous
});
construct_struct!(acadrust::entities::polyline3d::Vertex3DPolyline {
    handle,
    layer,
    position,
    flags
});
construct_struct!(acadrust::entities::polyline3d::Polyline3D {
    common,
    flags,
    smooth_type,
    default_start_width,
    default_end_width,
    mesh_m_count,
    mesh_n_count,
    smooth_m_density,
    smooth_n_density,
    elevation,
    normal,
    vertices
});
construct_struct!(acadrust::entities::proxy_graphics::ProxyGraphics { records });
construct_struct!(acadrust::entities::proxy_graphics::ProxyUnicodeText {
    position,
    normal,
    direction,
    height,
    width_factor,
    oblique_angle,
    text
});
construct_struct!(acadrust::entities::raster_image::ClipBoundary {
    clip_type,
    clip_mode,
    vertices
});
construct_struct!(acadrust::entities::raster_image::RasterImage {
    common,
    class_version,
    insertion_point,
    u_vector,
    v_vector,
    size,
    flags,
    clipping_enabled,
    brightness,
    contrast,
    fade,
    clip_boundary,
    definition_handle,
    definition_reactor_handle,
    file_path
});
construct_struct!(acadrust::entities::raster_image::ImageDefinition {
    handle,
    class_version,
    file_path,
    size,
    pixel_size,
    is_loaded,
    resolution_unit
});
construct_struct!(acadrust::entities::ray::Ray {
    common,
    base_point,
    direction
});
construct_struct!(acadrust::entities::section_symbol::SectionSymbolPoint {
    point,
    bulge,
    label,
    label_offset,
    raw_flag_280
});
construct_struct!(acadrust::entities::section_symbol::SectionSymbol {
    common,
    view_symbol_version,
    style_handle,
    symbol_scale,
    view_rep_handle,
    raw_view_symbol_70,
    version,
    raw_point_count_90,
    raw_flags_90,
    raw_point_record_count,
    points,
    end_a,
    end_b,
    tick_a,
    tick_b,
    label
});
construct_struct!(acadrust::entities::section_symbol::SectionViewStyle {
    show_arrows,
    show_plane_line,
    show_end_lines,
    arrow_size,
    arrow_extension,
    label_height,
    label_offset,
    label_position,
    arrow_position,
    end_line_length,
    end_line_overshoot,
    arrow_start_handle,
    arrow_end_handle,
    arrow_is_default
});
construct_struct!(acadrust::entities::seqend::Seqend { common });
construct_struct!(acadrust::entities::shape::Shape {
    common,
    insertion_point,
    size,
    shape_name,
    shape_number,
    rotation,
    relative_x_scale,
    oblique_angle,
    normal,
    thickness,
    style_name,
    style_handle
});
construct_struct!(acadrust::entities::solid::Solid {
    common,
    first_corner,
    second_corner,
    third_corner,
    fourth_corner,
    normal,
    thickness,
    is_trace
});
construct_struct!(acadrust::entities::solid3d::Wire {
    acis_index,
    wire_type,
    selection_marker,
    color,
    points,
    has_transform,
    has_rotation,
    has_shear,
    has_reflection,
    scale,
    translation,
    x_axis,
    y_axis,
    z_axis
});
construct_struct!(acadrust::entities::solid3d::Silhouette {
    viewport_id,
    view_direction,
    up_vector,
    target,
    is_perspective,
    has_wires,
    wires
});
construct_struct!(acadrust::entities::solid3d::AcisRevision {
    has_guid,
    major,
    minor1,
    minor2,
    bytes,
    end_marker
});
construct_struct!(acadrust::entities::solid3d::AcisMaterial {
    array_index,
    absolute_reference,
    material_handle
});
construct_struct!(acadrust::entities::solid3d::AcisData {
    version,
    sat_data,
    sab_data,
    is_binary,
    revision,
    materials,
    wireframe_data_present,
    wireframe_point_present,
    wireframe_isoline_present,
    acis_empty_bit,
    extra_acis_data,
    wireframe_isolines
});
construct_struct!(acadrust::entities::solid3d::Solid3D {
    common,
    uid,
    point_of_reference,
    acis_data,
    wires,
    silhouettes,
    history_handle
});
construct_struct!(acadrust::entities::solid3d::Region {
    common,
    uid,
    point_of_reference,
    acis_data,
    wires,
    silhouettes,
    history_handle
});
construct_struct!(acadrust::entities::solid3d::Body {
    common,
    uid,
    point_of_reference,
    acis_data,
    wires,
    silhouettes,
    history_handle
});
construct_struct!(acadrust::entities::spline::SplineFlags {
    closed,
    periodic,
    rational,
    planar,
    linear
});
construct_struct!(acadrust::entities::spline::Spline {
    common,
    degree,
    flags,
    knots,
    control_points,
    weights,
    fit_points,
    normal,
    knot_tolerance,
    control_tolerance,
    fit_tolerance,
    begin_tangent,
    end_tangent,
    knot_parameterization,
    cv_frame_visible,
    dwg_flags1,
    dwg_scenario,
    dxf_flags
});
construct_struct!(acadrust::entities::surface::SurfaceSweepOptions {
    draft_angle,
    draft_start_distance,
    draft_end_distance,
    twist_angle,
    scale_factor,
    align_angle,
    sweep_entity_transform,
    path_entity_transform,
    is_solid,
    sweep_alignment_flags,
    path_flags,
    align_start,
    bank,
    base_point_set,
    sweep_entity_transform_computed,
    path_entity_transform_computed,
    reference_vector
});
construct_struct!(acadrust::entities::surface::Surface {
    common,
    kind,
    acis_data,
    modeler_format_version,
    u_isolines,
    v_isolines,
    surface_data,
    point_of_reference,
    history_handle,
    wires,
    silhouettes
});
construct_struct!(acadrust::entities::table::CellBorder {
    border_type,
    color,
    line_weight,
    line_type_handle,
    invisible,
    double_spacing,
    override_flags
});
construct_struct!(acadrust::entities::table::CellValue {
    value_type,
    raw_type_code,
    unit_type,
    raw_unit_type_code,
    flags,
    data_size,
    text,
    format,
    formatted_value,
    numeric_value,
    handle_value,
    point_value,
    binary_value
});
construct_struct!(acadrust::entities::table::CellContent {
    content_type,
    value,
    block_handle,
    text_style_handle,
    text_style_name,
    color,
    rotation,
    scale,
    text_height,
    field_handle,
    attributes,
    format_override_flags,
    format_property_flags,
    format_value_data_type,
    format_value_unit_type,
    value_format,
    alignment,
    geometry
});
construct_struct!(acadrust::entities::table::TableAttribute {
    definition_handle,
    value,
    index
});
construct_struct!(acadrust::entities::table::CellContentGeometry {
    distance_to_top_left,
    distance_to_center,
    width,
    height,
    outer_width,
    outer_height,
    flags
});
construct_struct!(acadrust::entities::table::TableCustomData { name, value });
construct_struct!(acadrust::entities::table::CellStyle {
    style_type,
    property_flags,
    override_flags,
    legacy_override_bits,
    content_format_override_flags,
    content_property_flags,
    value_data_type,
    value_unit_type,
    value_format,
    background_color,
    content_color,
    text_style_handle,
    text_style_name,
    text_height,
    rotation,
    scale,
    alignment,
    fill_enabled,
    layout_flags,
    margin_override_flags,
    margin_left,
    margin_top,
    margin_right,
    margin_bottom,
    horizontal_spacing,
    vertical_spacing,
    top_border,
    right_border,
    bottom_border,
    left_border,
    applied_border_edges,
    additional_borders
});
construct_struct!(acadrust::entities::table::TableCell {
    cell_type,
    state,
    contents,
    style,
    tooltip,
    rotation,
    geometry_scale,
    auto_fit,
    merge_width,
    merge_height,
    flag,
    merged,
    virtual_edge,
    has_linked_data,
    custom_data,
    edge_flags,
    value_handle,
    block_scale,
    custom_data_items,
    data_link_handle,
    data_link_rows,
    data_link_columns,
    data_link_unknown,
    style_id,
    geometry_data_flag,
    geometry_width_with_gap,
    geometry_height_with_gap,
    geometry_flags,
    geometry_handle,
    geometry,
    geometries
});
construct_struct!(acadrust::entities::table::TableRow {
    height,
    cells,
    style,
    custom_data,
    custom_data_items,
    style_id
});
construct_struct!(acadrust::entities::table::TableColumn {
    name,
    width,
    style,
    custom_data,
    custom_data_items,
    style_id
});
construct_struct!(acadrust::entities::table::CellRange {
    top_row,
    left_col,
    bottom_row,
    right_col
});
construct_struct!(acadrust::entities::table::TableBreakData {
    position,
    height,
    flags
});
construct_struct!(acadrust::entities::table::TableBreakRange {
    position,
    start_row,
    end_row
});
construct_struct!(acadrust::entities::table::LegacyTableStyleOverride {
    flags,
    title_suppressed,
    header_suppressed,
    flow_direction,
    horizontal_cell_margin,
    vertical_cell_margin,
    row_colors,
    row_fill_none,
    row_fill_colors,
    row_alignments,
    text_style_names,
    text_style_handles,
    row_heights
});
construct_struct!(acadrust::entities::table::Table {
    common,
    insertion_point,
    horizontal_direction,
    normal,
    table_style_handle,
    block_record_handle,
    block_name,
    data_version,
    value_flags,
    override_flag,
    override_border_color,
    override_border_line_weight,
    override_border_visibility,
    legacy_style_override,
    legacy_border_colors,
    legacy_border_line_weights,
    legacy_border_visibility,
    rows,
    columns,
    break_options,
    break_flow_direction,
    break_spacing,
    name,
    description,
    field_handles,
    base_style,
    merged_ranges,
    break_data,
    break_ranges,
    dwg_unknown_byte,
    dwg_unknown_handle,
    dwg_unknown_long1,
    dwg_r2010_unknown_bit,
    dwg_unknown_long2,
    dwg_unknown_short
});
construct_struct!(acadrust::entities::text::Text {
    common,
    value,
    insertion_point,
    alignment_point,
    height,
    rotation,
    width_factor,
    oblique_angle,
    style,
    horizontal_alignment,
    vertical_alignment,
    normal,
    thickness,
    generation_flags
});
construct_struct!(acadrust::entities::tolerance::Tolerance {
    common,
    insertion_point,
    direction,
    normal,
    text,
    dimension_style_name,
    dimension_style_handle,
    text_height,
    dimension_gap,
    dwg_unknown_short
});
construct_struct!(acadrust::entities::underlay::UnderlayDefinition {
    handle,
    owner_handle,
    underlay_type,
    file_path,
    page_name,
    name,
    reactors,
    unloaded
});
construct_struct!(acadrust::entities::underlay::Underlay {
    common,
    underlay_type,
    definition_handle,
    insertion_point,
    x_scale,
    y_scale,
    z_scale,
    rotation,
    normal,
    flags,
    contrast,
    fade,
    clip_boundary_vertices,
    clip_inverted
});
construct_struct!(acadrust::entities::unknown_entity::UnknownEntity {
    common,
    dxf_name,
    dwg_type_code,
    raw_dwg_data,
    dwg_handle_bits,
    raw_dxf_codes,
    dwg_source_version
});
construct_struct!(acadrust::entities::view_border::ViewBorder {
    common,
    version,
    min,
    max,
    center,
    scale,
    rotation_angle,
    active_viewport,
    scale_handle
});
construct_struct!(acadrust::entities::viewport::ViewportStatusFlags {
    non_rectangular_clipping,
    is_off,
    is_on,
    perspective,
    front_clipping,
    back_clipping,
    ucs_follow,
    front_clip_not_at_eye,
    ucs_icon_visible,
    ucs_icon_at_origin,
    fast_zoom,
    snap_on,
    grid_on,
    isometric_snap,
    hide_plot,
    iso_pair_top,
    iso_pair_right,
    locked
});
construct_struct!(acadrust::entities::viewport::GridFlags {
    beyond_limits,
    adaptive,
    subdivision,
    follow_dynamic
});
construct_struct!(acadrust::entities::viewport::Viewport {
    common,
    center,
    width,
    height,
    status,
    id,
    dxf_status,
    off_screen,
    view_center,
    snap_base,
    snap_spacing,
    grid_spacing,
    view_direction,
    view_target,
    lens_length,
    front_clip_z,
    back_clip_z,
    view_height,
    snap_angle,
    twist_angle,
    circle_sides,
    frozen_layers,
    clip_boundary_handle,
    render_mode,
    style_sheet,
    ucs_at_origin,
    ucs_per_viewport,
    ucs_icon_visible,
    ucs_origin,
    ucs_x_axis,
    ucs_y_axis,
    ucs_handle,
    base_ucs_handle,
    ucs_ortho_type,
    elevation,
    shade_plot_mode,
    grid_flags,
    grid_major,
    background_handle,
    shade_plot_handle,
    visual_style_handle,
    sun_handle,
    default_lighting,
    default_lighting_type,
    brightness,
    contrast,
    ambient_color,
    custom_scale
});
construct_struct!(acadrust::entities::wipeout::Wipeout {
    common,
    class_version,
    insertion_point,
    u_vector,
    v_vector,
    size,
    flags,
    clipping_enabled,
    brightness,
    contrast,
    fade,
    clip_mode,
    clip_type,
    clip_boundary_vertices,
    definition_handle,
    definition_reactor_handle
});
construct_struct!(acadrust::entities::xline::XLine {
    common,
    base_point,
    direction
});
impl NativeConstruct for acadrust::entities::acis::parser::SatParser {
    fn construct() -> FfiResult<Self> {
        Ok(Self)
    }
}
impl NativeConstruct for acadrust::entities::acis::sab::SabWriter {
    fn construct() -> FfiResult<Self> {
        Ok(Self)
    }
}
impl NativeConstruct for acadrust::entities::acis::sab::SabReader {
    fn construct() -> FfiResult<Self> {
        Ok(Self)
    }
}
construct_struct!(acadrust::entities::acis::types::SatVersion {
    major,
    minor,
    patch
});
construct_struct!(acadrust::entities::acis::types::SatHeader {
    version,
    num_records,
    num_bodies,
    has_history,
    product_id,
    product_version,
    date,
    spatial_resolution,
    normal_tolerance,
    resfit_tolerance
});
construct_struct!(acadrust::entities::acis::types::SatPointer => (0));
construct_struct!(acadrust::entities::acis::types::SatRecord {
    index,
    entity_type,
    sub_type,
    attribute,
    subtype_id,
    tokens,
    raw_text
});
construct_struct!(acadrust::entities::acis::types::SatBSplineSurface {
    rational,
    degree_u,
    degree_v,
    u_closure,
    v_closure,
    u_singularity,
    v_singularity,
    u_knots,
    v_knots,
    control_count_u,
    control_count_v,
    control_points,
    fit_tolerance
});
construct_struct!(acadrust::entities::acis::types::SatDocument { header, records });
impl NativeConstruct for acadrust::entities::acis::writer::SatWriter {
    fn construct() -> FfiResult<Self> {
        Ok(Self)
    }
}
construct_struct!(acadrust::entities::mtext_format::types::StackingData {
    numerator,
    denominator,
    stacking_type
});
construct_struct!(
    acadrust::entities::mtext_format::types::ParagraphProperties {
        alignment,
        first_line_indent,
        left_margin,
        right_margin,
        spacing_before,
        spacing_after,
        line_spacing,
        tab_stops
    }
);
construct_struct!(acadrust::entities::mtext_format::types::MTextFont {
    name,
    style,
    bold,
    italic,
    bold_explicit,
    italic_explicit,
    charset,
    pitch,
    extra
});
construct_struct!(acadrust::entities::mtext_format::types::SpanProperties {
    color,
    color_rgb,
    second_color,
    font,
    height,
    stroke,
    line_align,
    tracking,
    width_factor,
    oblique_angle
});
construct_struct!(acadrust::entities::mtext_format::types::MTextSpan {
    text,
    properties,
    stacking
});
construct_struct!(acadrust::entities::mtext_format::types::MTextParagraph {
    properties,
    spans,
    starts_column
});
construct_struct!(acadrust::entities::mtext_format::types::MTextDocument { paragraphs });
construct_struct!(acadrust::tables::appid::AppId { handle, name });
construct_struct!(acadrust::tables::block_record::BlockFlags {
    anonymous,
    has_attributes,
    is_xref,
    is_xref_overlay,
    is_external,
    is_xref_unloaded
});
construct_struct!(acadrust::tables::block_record::BlockRecord {
    handle,
    block_entity_handle,
    block_end_handle,
    name,
    flags,
    layout,
    units,
    explodable,
    scale_uniformly,
    entity_handles,
    xref_path,
    description,
    insert_count_bytes,
    preview_data,
    insert_handles,
    base_point
});
construct_struct!(acadrust::tables::dimstyle::DimStyle {
    handle,
    name,
    xref_reference,
    xref_resolved,
    xref_dependent,
    xref_handle,
    dimclrd,
    dimclrd_true_color,
    dimdle,
    dimdli,
    dimgap,
    dimlwd,
    dimsd1,
    dimsd2,
    dimclre,
    dimclre_true_color,
    dimexe,
    dimexo,
    dimlwe,
    dimse1,
    dimse2,
    dimfxl,
    dimfxlon,
    dimasz,
    dimblk,
    dimblk1,
    dimblk2,
    dimldrblk,
    dimblk_name,
    dimblk1_name,
    dimblk2_name,
    dimsah,
    dimcen,
    dimtsz,
    dimclrt,
    dimclrt_true_color,
    dimtxt,
    dimjust,
    dimtad,
    dimtvp,
    dimtih,
    dimtoh,
    dimtix,
    dimsoxd,
    dimtfill,
    dimtfillclr,
    dimtfillclr_true_color,
    dimtmove,
    dimtxtdirection,
    dimaltmzf,
    dimaltmzs,
    dimmzf,
    dimmzs,
    dimtxsty_handle,
    dimtxsty,
    dimscale,
    dimlfac,
    dimlunit,
    dimdec,
    dimrnd,
    dimdsep,
    dimzin,
    dimaunit,
    dimadec,
    dimazin,
    dimalt,
    dimaltf,
    dimaltd,
    dimaltu,
    dimalttd,
    dimaltrnd,
    dimapost,
    dimaltz,
    dimalttz,
    dimtol,
    dimlim,
    dimtp,
    dimtm,
    dimtdec,
    dimtfac,
    dimtolj,
    dimtzin,
    dimatfit,
    dimtofl,
    dimupt,
    dimfit,
    dimpost,
    dimfrac,
    dimarcsym,
    dimjogang,
    dimltex_handle,
    dimltex1_handle,
    dimltex2_handle,
    dimunit,
    annotative
});
construct_struct!(acadrust::tables::layer::LayerFlags {
    frozen,
    locked,
    frozen_in_new_viewport,
    off,
    xref_dependent
});
construct_struct!(acadrust::tables::layer::Layer {
    handle,
    name,
    flags,
    color,
    color_name,
    book_name,
    line_type,
    line_weight,
    plot_style,
    is_plottable,
    transparency,
    description,
    material,
    plotstyle_handle,
    xref_block_record_handle
});
construct_struct!(acadrust::tables::linetype::LineTypeElement { length, complex });
construct_struct!(acadrust::tables::linetype::LineType {
    handle,
    name,
    description,
    elements,
    pattern_length,
    alignment,
    xref_dependent,
    xref_block_record_handle
});
construct_struct!(acadrust::tables::linetype::LineTypeComplexData {
    content,
    style_handle,
    scale,
    rotation,
    absolute_rotation,
    source_flags,
    offset
});
construct_struct!(acadrust::tables::textstyle::TextGenerationFlags {
    backward,
    upside_down
});
construct_struct!(acadrust::tables::textstyle::TextStyle {
    handle,
    name,
    flags,
    height,
    width_factor,
    oblique_angle,
    last_height,
    font_file,
    big_font_file,
    true_type_font,
    true_type_font_flags,
    xref_dependent,
    xref_block_record_handle,
    is_shape_file,
    is_vertical,
    annotative
});
construct_struct!(acadrust::tables::ucs::Ucs {
    handle,
    name,
    origin,
    x_axis,
    y_axis,
    elevation,
    ortho_view_type,
    ortho_type,
    named_ucs_handle,
    base_ucs_handle,
    xref_reference,
    xref_resolved,
    xref_dependent,
    xref_handle
});
construct_struct!(acadrust::tables::view::View {
    handle,
    name,
    center,
    height,
    width,
    direction,
    target,
    lens_length,
    front_clip,
    back_clip,
    twist_angle,
    perspective,
    front_clipping,
    back_clipping,
    front_clip_at_eye,
    render_mode,
    use_default_lights,
    default_lighting_type,
    brightness,
    contrast,
    ambient_color,
    paper_space,
    ucs_associated,
    ucs_origin,
    ucs_x_axis,
    ucs_y_axis,
    ucs_elevation,
    ucs_ortho_type,
    camera_plottable,
    xref_reference,
    xref_resolved,
    xref_dependent,
    xref_handle,
    background_handle,
    live_section_handle,
    visual_style_handle,
    sun_handle,
    named_ucs_handle,
    base_ucs_handle
});
construct_struct!(acadrust::tables::vport::VPort {
    handle,
    name,
    lower_left,
    upper_right,
    view_center,
    snap_base,
    snap_spacing,
    grid_spacing,
    view_direction,
    view_target,
    view_height,
    aspect_ratio,
    lens_length,
    view_twist,
    front_clip,
    back_clip,
    ucsfollow,
    circle_zoom,
    fast_zoom,
    grid_on,
    snap_on,
    snap_style,
    snap_isopair,
    snap_rotation,
    render_mode,
    perspective,
    front_clipping,
    back_clipping,
    front_clip_at_eye,
    ucsicon_lower,
    ucsicon_origin,
    use_default_lights,
    default_lighting_type,
    brightness,
    contrast,
    ambient_color,
    ucs_at_origin,
    ucs_per_viewport,
    ucs_origin,
    ucs_x_axis,
    ucs_y_axis,
    ucs_elevation,
    ucs_ortho_type,
    grid_flags,
    grid_major,
    xref_reference,
    xref_resolved,
    xref_dependent,
    xref_handle,
    background_handle,
    visual_style_handle,
    sun_handle,
    named_ucs_handle,
    base_ucs_handle
});
construct_struct!(acadrust::tables::vx::VxTableRecord {
    handle,
    name,
    is_xref_reference,
    is_xref_resolved,
    is_xref_dependent,
    xref_handle,
    is_on,
    viewport,
    previous_entry,
    legacy_viewport_entity_address,
    legacy_viewport_index,
    legacy_previous_entry_index
});
construct_struct!(acadrust::objects::AssociativeObject {
    handle,
    owner,
    reactors,
    xdictionary_handle,
    dxf_name,
    cpp_class_name,
    data,
    source_version
});
construct_struct!(acadrust::objects::AssocDependency {
    class_version,
    status,
    is_read_dependency,
    is_write_dependency,
    is_attached_to_object,
    is_delegating_to_owning_action,
    order,
    dependent_on,
    name,
    read_dependency,
    node,
    dependency_body,
    dependency_body_id
});
construct_struct!(acadrust::objects::AssocValueDependency {
    dependency,
    class_version,
    name,
    value
});
construct_struct!(acadrust::objects::AssocPersistentSubentId {
    class_name,
    dependent_on_compound_object,
    class_code,
    values,
    leading_flag
});
construct_struct!(acadrust::objects::AssocGeomDependency {
    dependency,
    class_version,
    enabled,
    persistent_subent
});
construct_struct!(acadrust::objects::AssocEvalVariant { code, value });
construct_struct!(acadrust::objects::AssocValueParamVariable { value, handle });
construct_struct!(acadrust::objects::AssocValueParam {
    class_version,
    name,
    unit_type,
    variables,
    controlled_object_dependency
});
construct_struct!(acadrust::objects::AssocActionDependency {
    is_owned,
    dependency
});
construct_struct!(acadrust::objects::AssocAction {
    class_version,
    geometry_status,
    owning_network,
    action_body,
    action_index,
    max_dependency_index,
    dependencies,
    owned_parameters,
    values
});
construct_struct!(acadrust::objects::AssocNetwork {
    action,
    network_version,
    network_action_index,
    actions,
    owned_actions
});
construct_struct!(acadrust::objects::AssocActionParam {
    is_r2013,
    version,
    name
});
construct_struct!(acadrust::objects::AssocActionBody { version });
construct_struct!(acadrust::objects::AssocParamBasedActionBody {
    version,
    minor,
    dependencies,
    marker,
    values,
    empty_value_marker,
    dependency
});
construct_struct!(acadrust::objects::AssocSurfaceBody {
    version,
    dependency,
    is_semi_associative,
    marker,
    is_semi_override,
    grip_status
});
construct_struct!(acadrust::objects::AssocSurfaceActionBody {
    kind,
    action_body,
    parameter_body,
    surface_body,
    path_status,
    class_version,
    option,
    flags,
    status,
    secondary_status,
    distance,
    first_point,
    second_point
});
construct_struct!(acadrust::objects::AssocAnnotationBase {
    action_body,
    parameter_body,
    version,
    dependency
});
construct_struct!(acadrust::objects::AssocAnnotationDependency {
    dependency_id,
    dependency
});
construct_struct!(acadrust::objects::AssocAnnotationActionBody {
    kind,
    annotation,
    action_body,
    class_version,
    entity,
    actions,
    read_node,
    dimension_node,
    dependency
});
construct_struct!(acadrust::objects::AssocPersSubentManager {
    class_version,
    markers,
    values,
    final_flag
});
construct_struct!(acadrust::objects::AssocSingleDependencyActionParam {
    action_param,
    dependency_class_version,
    dependency,
    class_version
});
construct_struct!(acadrust::objects::AssocSmartCenterActionBody {
    action_body,
    parameter_body,
    version
});
construct_struct!(acadrust::objects::AssocViewRepActionBody {
    action_body,
    class_version,
    view_rep,
    view_type,
    rotation
});
construct_struct!(acadrust::objects::AssocViewObjectActionParam {
    kind,
    single_dependency,
    class_version
});
construct_struct!(acadrust::objects::AssocViewRepHatchManagerItem {
    first_id,
    second_id,
    status,
    parameter
});
construct_struct!(acadrust::objects::AssocViewRepHatchManager {
    compound,
    class_version,
    items
});
construct_struct!(acadrust::objects::AssocViewRepHatchActionParam {
    single_dependency,
    class_version,
    normal,
    hatch_index,
    flags
});
construct_struct!(acadrust::objects::AssocViewLabelActionParam {
    single_dependency,
    class_version,
    label_version,
    offset,
    flag
});
construct_struct!(acadrust::objects::AssocEdgeActionParam {
    single_dependency,
    parameter,
    has_action,
    action_type,
    subcurve_kind,
    curve
});
construct_struct!(acadrust::objects::AssocConstraintNode {
    node_id,
    status,
    connections,
    class_name,
    registry_flag,
    data
});
construct_struct!(acadrust::objects::Assoc2dConstraintGroup {
    action,
    version,
    flag,
    work_plane,
    dependency,
    actions,
    next_node_id,
    nodes
});
construct_struct!(acadrust::objects::AssocVariable {
    action,
    class_version,
    name,
    expression,
    evaluator,
    description,
    value,
    has_cached_value,
    cached_value,
    flag,
    dependencies
});
construct_struct!(acadrust::objects::AssocVariableDependency { dependency, flags });
construct_struct!(acadrust::objects::AssocCompoundActionParam {
    action_param,
    class_version,
    status,
    parameters,
    child_parameter
});
construct_struct!(acadrust::objects::AssocChildParameter {
    status,
    id,
    parameter,
    secondary_parameter,
    marker,
    tertiary_parameter
});
construct_struct!(acadrust::objects::AssocOsnapPointRefActionParam {
    compound,
    status,
    osnap_mode,
    parameter
});
construct_struct!(acadrust::objects::AssocPathActionParam { compound, version });
construct_struct!(acadrust::objects::AssocDimDependencyBody {
    dependency_body_version,
    base_version,
    name,
    class_version
});
construct_struct!(acadrust::objects::AssocFaceActionParam {
    single_dependency,
    index
});
construct_struct!(acadrust::objects::AssocVertexActionParam {
    single_dependency,
    point
});
construct_struct!(acadrust::objects::AssocAsmBodyActionParam {
    single_dependency,
    acis_data,
    point_of_reference,
    wires,
    silhouettes,
    history
});
construct_struct!(acadrust::objects::AssocArrayItem {
    class_version,
    location,
    flags,
    uses_default_transform,
    x_direction,
    transform,
    relative_transform,
    first_handle,
    second_handle
});
construct_struct!(acadrust::objects::AssocArrayParameters {
    version,
    class_name,
    items,
    item_count,
    row_count,
    level_count
});
construct_struct!(acadrust::objects::AssocArrayActionBody {
    action_body,
    parameter_body,
    version,
    parameter_block,
    item_list_version,
    item_class,
    items,
    transform
});
construct_struct!(acadrust::objects::AssocArrayModifyActionBody {
    body,
    status,
    item_locations
});
construct_struct!(acadrust::objects::AssocDimensionReference {
    class_name,
    osnap_type,
    xrefs,
    main_subent_type,
    main_gs_marker,
    xref_paths,
    osnap_distance,
    osnap_point,
    intersection_objects,
    intersection_subent_type,
    intersection_gs_marker,
    intersection_xref_paths,
    has_last_point_reference
});
construct_struct!(acadrust::objects::AssocDimensionAssociation {
    associativity,
    trans_space,
    rotated_type,
    dimension,
    references
});
construct_struct!(acadrust::objects::PersSubentManager {
    class_version,
    marker_zero,
    marker_two,
    associative_step_count,
    associative_subent_count,
    values
});
construct_struct!(acadrust::objects::BlockParameterConnection { code, name });
construct_struct!(acadrust::objects::BlockParameterPropertyInfo { connections });
construct_struct!(acadrust::objects::BlockVisibilityState {
    name,
    visible_blocks,
    visible_params
});
construct_struct!(acadrust::objects::BlockVisibilityParameter {
    handle,
    owner,
    eval_parent_id,
    eval_major,
    eval_minor,
    eval_value_code,
    eval_value,
    eval_node_id,
    element_name,
    element_major,
    element_minor,
    element_eed_1071,
    show_properties,
    chain_actions,
    name,
    description,
    def_point,
    property_info,
    property_info_count,
    is_initialized,
    unknown_bool,
    all_blocks,
    states
});
construct_struct!(acadrust::objects::ClassObject {
    handle,
    owner,
    reactors,
    xdictionary_handle,
    data
});
construct_struct!(acadrust::objects::ViewRepSourceManager {
    has_source,
    source,
    status
});
construct_struct!(acadrust::objects::ViewRepStandard { values });
construct_struct!(acadrust::objects::ViewRepOrientation {
    camera,
    target,
    normal
});
construct_struct!(acadrust::objects::ViewRepSectionDefinition {
    version,
    section_depth,
    flags
});
construct_struct!(acadrust::objects::ViewRepModelSpaceViewSelectionSet { version, entities });
construct_struct!(acadrust::objects::ViewRepGuid {
    data1,
    data2,
    data3,
    data4
});
construct_struct!(acadrust::objects::ViewRepModelSpaceSource {
    enabled,
    header_values,
    transform,
    source_version,
    source_status,
    model,
    guid,
    references,
    tail_values,
    orientation
});
construct_struct!(acadrust::objects::ViewRepSketchReference { object, flag });
construct_struct!(acadrust::objects::ViewRepSketch {
    id,
    version,
    references,
    reserved,
    enabled,
    geometry,
    final_flag
});
construct_struct!(acadrust::objects::ViewRepObjectPath {
    class_name,
    objects
});
construct_struct!(acadrust::objects::ViewRepBlockPathEntry { flag, kind, object });
construct_struct!(acadrust::objects::ViewRepBlockPath {
    class_name,
    version,
    entries
});
construct_struct!(acadrust::objects::ViewRep {
    header_values,
    name,
    scale,
    header_status,
    description,
    source_id,
    source_enabled,
    source_version,
    model_id,
    guid,
    marker,
    transform,
    transform_version,
    database_id,
    geometry_version,
    geometry_marker,
    sketches,
    related_objects,
    source_manager,
    owned_objects,
    optional_objects,
    position,
    rotation,
    orientation,
    is_active,
    projection,
    linked_views,
    section_sketches,
    action_mode,
    action,
    has_parent,
    parent,
    tail_version,
    tail_state,
    tail_id,
    path_count,
    path_version,
    path_id,
    has_block_path,
    block_path,
    style
});
construct_struct!(acadrust::objects::AcMeCommandHistory { class_version });
construct_struct!(acadrust::objects::AcMeScope { class_version });
construct_struct!(acadrust::objects::AcMeStateManager { class_version });
construct_struct!(acadrust::objects::CsacDocumentOptions {
    class_version,
    raw_dwg_data,
    raw_dwg_handle_bits,
    raw_dwg_version
});
construct_struct!(acadrust::objects::SpatialIndex {
    last_updated_julian_day,
    last_updated_milliseconds,
    min_corner,
    max_corner,
    indexed_objects
});
construct_struct!(acadrust::objects::LayerFilter { names });
construct_struct!(acadrust::objects::PartialViewingIndexEntry {
    extents_min,
    extents_max,
    object
});
construct_struct!(acadrust::objects::PartialViewingIndex {
    has_entries,
    entries
});
construct_struct!(acadrust::objects::VbaProject { storage });
construct_struct!(acadrust::objects::SectionManager { is_live, sections });
construct_struct!(acadrust::objects::SectionGeometrySettings {
    geometry_count,
    index,
    flags,
    color,
    layer,
    linetype,
    linetype_scale,
    plot_style,
    lineweight,
    face_transparency,
    edge_transparency,
    hatch_type,
    hatch_pattern,
    hatch_angle,
    hatch_spacing,
    hatch_scale
});
construct_struct!(acadrust::objects::SectionTypeSettings {
    section_type,
    generation,
    sources,
    destination_block,
    destination_file,
    geometry
});
construct_struct!(acadrust::objects::SectionSettings {
    current_type,
    types
});
construct_struct!(acadrust::objects::LightListEntry { handle, name });
construct_struct!(acadrust::objects::LightList {
    class_version,
    lights
});
construct_struct!(acadrust::objects::Sun {
    class_version,
    is_on,
    color,
    intensity,
    has_shadow,
    julian_day,
    milliseconds,
    is_daylight_savings_on,
    shadow_type,
    shadow_map_size,
    shadow_softness
});
construct_struct!(acadrust::objects::RenderSettings {
    class_version,
    name,
    fog_enabled,
    fog_background_enabled,
    backfaces_enabled,
    environment_image_enabled,
    environment_image_filename,
    description,
    display_index,
    has_predefined
});
construct_struct!(acadrust::objects::MentalRayRenderSettings {
    base,
    version,
    sampling_min,
    sampling_max,
    sampling_filter,
    sampling_filter_width,
    sampling_filter_height,
    sampling_contrast,
    shadow_mode,
    shadow_maps_enabled,
    ray_tracing_enabled,
    ray_trace_depth,
    global_illumination_enabled,
    global_illumination_sample_count,
    global_illumination_sample_radius_enabled,
    global_illumination_sample_radius,
    photons_per_light,
    photon_trace_depth,
    final_gathering_enabled,
    final_gathering_ray_count,
    final_gathering_sample_radius_state,
    final_gathering_sample_radius,
    light_luminance_scale,
    diagnostics_mode,
    diagnostics_grid_mode,
    diagnostics_grid_size,
    diagnostics_photon_mode,
    diagnostics_bsp_mode,
    export_mi_enabled,
    description,
    tile_size,
    tile_order,
    memory_limit,
    diagnostics_samples_mode,
    energy_multiplier
});
construct_struct!(acadrust::objects::RapidRtRenderSettings {
    base,
    version,
    render_target,
    render_level,
    render_time,
    lighting_model,
    filter_type,
    filter_width,
    filter_height
});
construct_struct!(acadrust::objects::GradientBackground {
    class_version,
    color_top,
    color_middle,
    color_bottom,
    horizon,
    height,
    rotation
});
construct_struct!(acadrust::objects::GroundPlaneBackground {
    class_version,
    color_sky_zenith,
    color_sky_horizon,
    color_underground_horizon,
    color_underground_azimuth,
    color_near,
    color_far
});
construct_struct!(acadrust::objects::IblBackground {
    class_version,
    enabled,
    name,
    rotation,
    display_image,
    secondary_background
});
construct_struct!(acadrust::objects::ImageBackground {
    class_version,
    filename,
    fit_to_screen,
    maintain_aspect_ratio,
    use_tiling,
    offset,
    scale
});
construct_struct!(acadrust::objects::SkyLightBackground { class_version, sun });
construct_struct!(acadrust::objects::SolidBackground {
    class_version,
    color
});
construct_struct!(acadrust::objects::RenderEntry {
    class_version,
    image_filename,
    preset_name,
    view_name,
    width,
    height,
    start_year,
    start_month,
    start_day,
    start_hour,
    start_minute,
    start_second,
    start_millisecond,
    end_year,
    end_month,
    end_day,
    end_hour,
    end_minute,
    end_second,
    end_millisecond,
    render_time,
    memory_amount,
    material_count,
    light_count,
    triangle_count,
    display_index
});
construct_struct!(acadrust::objects::RenderEnvironment {
    class_version,
    fog_enabled,
    fog_background_enabled,
    fog_color,
    fog_density_near,
    fog_density_far,
    fog_distance_near,
    fog_distance_far,
    environment_image_enabled,
    environment_image_filename
});
construct_struct!(acadrust::objects::RenderGlobal {
    class_version,
    procedure,
    destination,
    save_enabled,
    save_filename,
    image_width,
    image_height,
    predefined_presets_first,
    high_level_info
});
construct_struct!(acadrust::objects::MotionPath {
    class_version,
    camera_path,
    target_path,
    view,
    frames,
    frame_rate,
    corner_deceleration
});
construct_struct!(acadrust::objects::CurvePath {
    class_version,
    entity
});
construct_struct!(acadrust::objects::PointPath {
    class_version,
    point
});
construct_struct!(acadrust::objects::TvDeviceProperties {
    flags,
    max_regen_threads,
    use_lut_palette,
    alternate_highlight,
    alternate_highlight_color,
    geometry_shader_usage,
    blending_mode,
    antialiasing_level,
    reserved_double
});
construct_struct!(acadrust::objects::PointCloudDefinition {
    class_version,
    source_filename,
    is_loaded,
    point_count,
    extents_min,
    extents_max
});
construct_struct!(acadrust::objects::PointCloudDefinitionReactor { class_version });
construct_struct!(acadrust::objects::PointCloudColorRamp {
    id,
    class_version,
    colors,
    name
});
construct_struct!(acadrust::objects::PointCloudRampColor { color, visible });
construct_struct!(acadrust::objects::PointCloudColorMap {
    class_version,
    default_intensity_scheme,
    default_elevation_scheme,
    default_classification_scheme,
    color_ramps,
    classification_color_ramps
});
construct_struct!(acadrust::objects::NavisworksModelDefinition {
    flags,
    path,
    status,
    extents_min,
    extents_max,
    host_drawing_visibility
});
construct_struct!(acadrust::objects::ContextDataEntry { item, name });
construct_struct!(acadrust::objects::ContextDataSubManager { handle, entries });
construct_struct!(acadrust::objects::ContextDataManager {
    object_context,
    sub_managers
});
construct_struct!(acadrust::objects::SunStudyDate {
    julian_day,
    milliseconds
});
construct_struct!(acadrust::objects::SunStudy {
    class_version,
    setup_name,
    description,
    output_type,
    use_subset,
    sheet_set_name,
    sheet_subset_name,
    select_dates_from_calendar,
    dates,
    select_range_of_dates,
    start_time,
    end_time,
    interval,
    hours,
    shade_plot_type,
    viewport_count,
    rows,
    columns,
    spacing,
    lock_viewports,
    label_viewports,
    page_setup_wizard,
    view,
    visual_style,
    text_style
});
construct_struct!(acadrust::objects::DataTableValue {
    integer,
    real,
    text,
    point,
    handle
});
construct_struct!(acadrust::objects::DataTableColumn {
    value_type,
    name,
    rows
});
construct_struct!(acadrust::objects::DataTable {
    flags,
    name,
    row_count,
    columns
});
construct_struct!(acadrust::objects::DataLinkCustomData { target, value });
construct_struct!(acadrust::objects::DataLink {
    data_adapter,
    description,
    tooltip,
    connection_string,
    option,
    update_option,
    flags,
    year,
    month,
    day,
    hour,
    minute,
    second,
    millisecond,
    path_option,
    status_flags,
    update_status,
    custom_data,
    hard_owner
});
construct_struct!(acadrust::objects::PersistentSubentityManager {
    class_version,
    reserved_zero,
    reserved_two,
    associated_step_count,
    associated_subentity_count,
    steps,
    subentities
});
construct_struct!(acadrust::objects::GeoMapImage {
    class_version,
    origin,
    image_size,
    display_properties,
    clipping_enabled,
    brightness,
    contrast,
    fade
});
construct_struct!(acadrust::objects::ModelDocViewStyle {
    class_version,
    description,
    modified_for_recompute,
    display_name,
    flags
});
construct_struct!(acadrust::objects::DetailViewStyle {
    base,
    class_version,
    flags,
    identifier_style,
    identifier_color,
    identifier_height,
    identifier_excluded_characters,
    identifier_offset,
    identifier_placement,
    arrow_symbol,
    arrow_symbol_color,
    arrow_symbol_size,
    boundary_linetype,
    boundary_lineweight,
    boundary_color,
    view_label_text_style,
    view_label_text_color,
    view_label_text_height,
    view_label_attachment,
    view_label_offset,
    view_label_alignment,
    view_label_pattern,
    connection_linetype,
    connection_lineweight,
    connection_color,
    border_linetype,
    border_lineweight,
    border_color,
    model_edge
});
construct_struct!(acadrust::objects::SectionViewStyle {
    base,
    class_version,
    flags,
    identifier_style,
    identifier_color,
    identifier_height,
    arrow_start_symbol,
    arrow_end_symbol,
    arrow_symbol_color,
    arrow_symbol_size,
    identifier_excluded_characters,
    arrow_symbol_extension_length,
    plane_linetype,
    plane_lineweight,
    plane_color,
    bend_linetype,
    bend_lineweight,
    bend_color,
    bend_line_length,
    end_line_length,
    view_label_text_style,
    view_label_text_color,
    view_label_text_height,
    view_label_attachment,
    view_label_offset,
    view_label_alignment,
    view_label_pattern,
    hatch_color,
    hatch_background_color,
    hatch_pattern,
    hatch_scale,
    hatch_transparency,
    reserved_flags,
    identifier_position,
    identifier_offset,
    arrow_position,
    end_line_overshoot,
    hatch_angles
});
construct_struct!(acadrust::objects::DataObject {
    handle,
    owner,
    reactors,
    xdictionary_handle,
    data
});
construct_struct!(acadrust::objects::BreakData {
    version,
    dimension_reference,
    reserved_reference,
    point_references
});
construct_struct!(acadrust::objects::BreakPointReference {
    version,
    reserved,
    reference_type,
    flags,
    identifier,
    first_point,
    second_point,
    trailing_version
});
construct_struct!(acadrust::objects::IdBuffer { flags, object_ids });
construct_struct!(acadrust::objects::Index {
    last_updated_julian_day,
    last_updated_milliseconds
});
construct_struct!(acadrust::objects::LayerIndexEntry {
    layer_count,
    name,
    id_buffer
});
construct_struct!(acadrust::objects::LayerIndex {
    last_updated_julian_day,
    last_updated_milliseconds,
    entries
});
impl NativeConstruct for acadrust::objects::PartialViewingFilter {
    fn construct() -> FfiResult<Self> {
        Ok(Self)
    }
}
construct_struct!(acadrust::objects::CellStyleMap { cells });
construct_struct!(acadrust::objects::TableGeometryCell {
    geometry_data_flag,
    width_with_gap,
    height_with_gap,
    table_geometry,
    geometry
});
construct_struct!(acadrust::objects::TableGeometry {
    rows,
    columns,
    cells
});
construct_struct!(acadrust::objects::DgnLineStyleObject {
    handle,
    owner,
    reactors,
    xdictionary_handle,
    data
});
construct_struct!(acadrust::objects::DgnLsSymbolComponent {
    stored_unit_scale,
    unit_scale,
    has_unit_scale,
    is_3d,
    block
});
construct_struct!(acadrust::objects::DgnLsCompoundComponent { entries });
construct_struct!(acadrust::objects::DgnLsCompoundEntry { component, offset });
construct_struct!(acadrust::objects::DgnLsStrokePattern {
    has_iteration_limit,
    is_single_segment,
    iteration_limit,
    auto_phase,
    phase,
    phase_mode,
    strokes
});
construct_struct!(acadrust::objects::DgnLsStroke {
    is_dash,
    bypass_corner,
    can_be_scaled,
    invert_at_origin,
    invert_at_end,
    length,
    start_width,
    end_width,
    width_mode,
    cap_mode
});
construct_struct!(acadrust::objects::DgnLsPointComponent {
    stroke_component,
    symbols
});
construct_struct!(acadrust::objects::DgnLsSymbolReference {
    symbol_component,
    partial_strokes,
    clip_partial,
    allow_stretch,
    partial_projected,
    use_symbol_color,
    use_symbol_lineweight,
    justify,
    rotation_type,
    vertex_mask,
    x_offset,
    y_offset,
    angle,
    stroke_number
});
construct_struct!(acadrust::objects::DgnLsInternalComponent {
    pattern,
    internal_version,
    hardware_style,
    is_hardware_style,
    line_code
});
construct_struct!(acadrust::objects::DgnLsDefinition {
    handle,
    name,
    root_component
});
construct_struct!(acadrust::objects::DgnLsComponent {
    handle,
    component_type,
    description,
    refs,
    scale
});
construct_struct!(acadrust::objects::DictionaryVariable {
    handle,
    owner_handle,
    schema_number,
    value,
    name
});
construct_struct!(acadrust::objects::DynamicBlockObject {
    handle,
    owner,
    reactors,
    xdictionary_handle,
    dxf_name,
    cpp_class_name,
    data
});
construct_struct!(acadrust::objects::BlockEvalExpression {
    parent_id,
    major,
    minor,
    value_code,
    value,
    node_id
});
construct_struct!(acadrust::objects::BlockElement {
    eval,
    name,
    major,
    minor,
    eed_1071
});
construct_struct!(acadrust::objects::BlockConnection { code, name });
construct_struct!(acadrust::objects::BlockParameterProperty { connections });
construct_struct!(acadrust::objects::BlockParameterValueSet {
    description,
    flags,
    minimum,
    maximum,
    increment,
    values
});
construct_struct!(acadrust::objects::BlockParameter {
    element,
    show_properties,
    chain_actions
});
construct_struct!(acadrust::objects::BlockOnePointParameter {
    parameter,
    definition_point,
    properties,
    property_count
});
construct_struct!(acadrust::objects::BlockTwoPointParameter {
    parameter,
    definition_base_point,
    definition_end_point,
    properties,
    property_states,
    parameter_base_location,
    updated_base_point,
    base_point,
    updated_end_point,
    end_point
});
construct_struct!(acadrust::objects::BlockGrip {
    element,
    flags_91,
    flags_92,
    location,
    insert_cycling,
    insert_cycling_weight
});
construct_struct!(acadrust::objects::BlockGripExpression {
    eval,
    grip_type,
    expression
});
construct_struct!(acadrust::objects::BlockOrientedGrip { grip, orientation });
construct_struct!(acadrust::objects::BlockFlipGrip {
    grip,
    combined_state,
    orientation
});
construct_struct!(acadrust::objects::BlockAction {
    element,
    display_location,
    dependencies,
    action_ids
});
construct_struct!(acadrust::objects::BlockActionOffsets {
    distance_multiplier,
    angle_offset,
    flags
});
construct_struct!(acadrust::objects::BlockActionWithBasePoint {
    action,
    offset,
    connections,
    dependent,
    base_point
});
construct_struct!(acadrust::objects::BlockConstraintParameter {
    parameter,
    dependency
});
construct_struct!(acadrust::objects::BlockLinearConstraintParameter {
    constraint,
    expression_name,
    expression_description,
    value,
    value_set
});
construct_struct!(acadrust::objects::BlockRepresentationData { flags, block });
construct_struct!(acadrust::objects::BlockAlignmentParameter {
    parameter,
    align_perpendicular
});
construct_struct!(acadrust::objects::BlockLinearParameter {
    parameter,
    distance_name,
    distance_description,
    distance,
    value_set
});
construct_struct!(acadrust::objects::BlockBasePointParameter {
    parameter,
    point,
    base_point
});
construct_struct!(acadrust::objects::BlockFlipParameter {
    parameter,
    flip_label,
    flip_label_description,
    base_state_label,
    flipped_state_label,
    definition_label_point,
    flags_96,
    tooltip
});
construct_struct!(acadrust::objects::BlockXYParameter {
    parameter,
    x_label,
    x_label_description,
    y_label,
    y_label_description,
    x_value,
    y_value,
    x_value_set,
    y_value_set
});
construct_struct!(acadrust::objects::BlockRotationParameter {
    parameter,
    definition_base_angle_point,
    angle_name,
    angle_description,
    angle,
    value_set
});
construct_struct!(acadrust::objects::BlockPolarParameter {
    parameter,
    angle_name,
    angle_description,
    distance_name,
    distance_description,
    offset,
    angle_value_set,
    distance_value_set
});
construct_struct!(acadrust::objects::BlockLookupParameter {
    parameter,
    index,
    lookup_name,
    lookup_description
});
construct_struct!(acadrust::objects::BlockPointParameter {
    parameter,
    position_name,
    position_description,
    definition_label_point
});
construct_struct!(acadrust::objects::BlockUserParameter {
    parameter,
    flags,
    associated_variable,
    expression,
    value_code,
    value,
    value_type
});
construct_struct!(acadrust::objects::BlockAngularConstraintParameter {
    constraint,
    center_point,
    end_point,
    expression_name,
    expression_description,
    angle,
    orientation_on_both_grips,
    value_set
});
construct_struct!(acadrust::objects::BlockAngularConstraintParameterEntity {
    constraint,
    center_point,
    label_point,
    expression_name,
    expression_description,
    angle,
    orientation_on_both_grips,
    value_set
});
construct_struct!(acadrust::objects::BlockDistanceConstraintParameter {
    constraint,
    expression_name,
    expression_description,
    distance,
    value_set
});
construct_struct!(acadrust::objects::BlockParameterDependencyBody {
    dependency_body_version,
    dimension_base_version,
    name,
    class_version
});
construct_struct!(acadrust::objects::BlockMoveAction {
    action,
    connections,
    offsets
});
construct_struct!(acadrust::objects::BlockFlipAction {
    action,
    connections
});
construct_struct!(acadrust::objects::BlockBasePointAction {
    action,
    connections,
    dwg_tail_byte
});
construct_struct!(acadrust::objects::BlockArrayAction {
    action,
    connections,
    column_offset,
    row_offset
});
construct_struct!(acadrust::objects::BlockLookupColumn {
    node_id,
    value_type,
    property_type,
    lookup_property,
    unmatched_name,
    writable,
    connection_name
});
construct_struct!(acadrust::objects::BlockLookupAction {
    action,
    row_count,
    column_count,
    expressions,
    columns,
    flag_280
});
construct_struct!(acadrust::objects::BlockStretchHandle { handle, indexes });
construct_struct!(acadrust::objects::BlockStretchCode { code, indexes });
construct_struct!(acadrust::objects::BlockStretchAction {
    action,
    connections,
    points,
    handles,
    codes,
    offsets
});
construct_struct!(acadrust::objects::BlockPolarStretchAction {
    action,
    connections,
    points,
    handles,
    bindings,
    codes,
    distance_multiplier,
    angle_offset,
    extra
});
construct_struct!(acadrust::objects::BlockEvaluationNode {
    id,
    edge_flags,
    next_id,
    expression,
    node_data,
    active_cycles
});
construct_struct!(acadrust::objects::BlockEvaluationEdge {
    id,
    next_id,
    incoming_edge,
    source_node,
    destination_node,
    outgoing_edges
});
construct_struct!(acadrust::objects::BlockEvaluationGraph {
    first_node_id,
    first_node_id_copy,
    nodes,
    edges
});
construct_struct!(acadrust::objects::SolidHistory {
    major,
    minor,
    owner,
    history_node_id,
    show_history,
    record_history
});
construct_struct!(acadrust::objects::SolidHistoryNodeBase {
    eval,
    major,
    minor,
    transform,
    color,
    step_id,
    material
});
construct_struct!(acadrust::objects::SolidHistoryBox {
    base,
    operation_major,
    operation_minor,
    length,
    width,
    height
});
construct_struct!(acadrust::objects::SolidHistorySphere {
    base,
    operation_major,
    operation_minor,
    radius
});
construct_struct!(acadrust::objects::SolidHistoryCylinder {
    base,
    operation_major,
    operation_minor,
    height,
    major_radius,
    minor_radius,
    x_radius
});
construct_struct!(acadrust::objects::SolidHistoryCone {
    base,
    operation_major,
    operation_minor,
    height,
    base_x_radius,
    base_y_radius,
    top_radius
});
construct_struct!(acadrust::objects::SolidHistoryPyramid {
    base,
    operation_major,
    operation_minor,
    height,
    sides,
    radius,
    top_radius
});
construct_struct!(acadrust::objects::SolidHistoryTorus {
    base,
    operation_major,
    operation_minor,
    major_radius,
    minor_radius
});
construct_struct!(acadrust::objects::SolidHistoryBoolean {
    base,
    operation_major,
    operation_minor,
    operation,
    first_operand,
    second_operand
});
construct_struct!(acadrust::objects::SolidHistoryTree {
    operation,
    operands
});
construct_struct!(acadrust::objects::SolidHistoryBrep {
    base,
    operation_major,
    operation_minor,
    acis_data
});
construct_struct!(acadrust::objects::SolidHistoryFillet {
    base,
    operation_major,
    operation_minor,
    method,
    edges,
    radii,
    start_setbacks,
    end_setbacks
});
construct_struct!(acadrust::objects::SolidHistoryChamfer {
    base,
    operation_major,
    operation_minor,
    method,
    base_distance,
    other_distance,
    edges,
    base_face
});
construct_struct!(acadrust::objects::SolidHistorySweep {
    base,
    operation_major,
    operation_minor,
    direction,
    sweep_entity,
    path_entity,
    draft_angle,
    start_draft_distance,
    end_draft_distance,
    scale_factor,
    twist_angle,
    align_angle,
    sweep_entity_transform,
    path_entity_transform,
    align_option,
    miter_option,
    has_align_start,
    align_start,
    bank,
    flags_294_296,
    reference_point,
    dwg_vector
});
construct_struct!(acadrust::objects::SolidHistoryLoft {
    base,
    operation_major,
    operation_minor,
    cross_sections,
    guides,
    parameters,
    path_entity,
    options
});
construct_struct!(acadrust::objects::SolidHistoryLoftOptions {
    surface_option,
    start_draft_angle,
    end_draft_angle,
    start_magnitude,
    end_magnitude,
    flags
});
construct_struct!(acadrust::objects::SolidHistoryLoftParameters {
    path_entity,
    normals,
    start_draft_angle,
    end_draft_angle,
    start_magnitude,
    end_magnitude,
    start_continuity,
    end_continuity,
    start_bulge,
    end_bulge,
    closed,
    periodic,
    surface,
    align_direction,
    section_counts
});
construct_struct!(acadrust::objects::SolidHistoryRevolve {
    base,
    operation_major,
    operation_minor,
    axis_point,
    direction,
    revolve_angle,
    start_angle,
    draft_angle,
    field_44,
    field_45,
    twist_angle,
    flag_290,
    close_to_axis,
    sweep_entity
});
construct_struct!(acadrust::objects::FieldChildValue { key, value });
construct_struct!(acadrust::objects::Field {
    handle,
    owner,
    evaluator_id,
    code,
    format,
    child_fields,
    referenced_objects,
    evaluation_option,
    filing_option,
    state,
    evaluation_status,
    evaluation_error_code,
    evaluation_error_message,
    value,
    value_string,
    value_string_length,
    child_values,
    xdata
});
construct_struct!(acadrust::objects::FieldList {
    handle,
    owner,
    unknown,
    fields
});
construct_struct!(acadrust::objects::Group {
    handle,
    owner,
    name,
    unnamed,
    description,
    entities,
    selectable
});
construct_struct!(acadrust::objects::ImageDefinitionReactor {
    handle,
    owner,
    image_handle
});
construct_struct!(acadrust::objects::ImageDefinition {
    handle,
    owner,
    file_name,
    is_loaded,
    size_in_pixels,
    pixel_size,
    resolution_unit,
    class_version
});
construct_struct!(acadrust::objects::MLineStyleFlags {
    fill_on,
    display_joints,
    start_square_cap,
    start_inner_arcs_cap,
    start_round_cap,
    end_square_cap,
    end_inner_arcs_cap,
    end_round_cap
});
construct_struct!(acadrust::objects::MLineStyleElement {
    offset,
    color,
    linetype
});
construct_struct!(acadrust::objects::MLineStyle {
    handle,
    owner,
    name,
    description,
    elements,
    start_angle,
    end_angle,
    fill_color,
    flags
});
construct_struct!(acadrust::objects::Dictionary {
    handle,
    owner,
    entries,
    hard_owner_entries,
    duplicate_cloning,
    hard_owner,
    reactors,
    xdictionary_handle
});
construct_struct!(acadrust::objects::Layout {
    handle,
    owner,
    name,
    flags,
    tab_order,
    min_limits,
    max_limits,
    insertion_base,
    min_extents,
    max_extents,
    elevation,
    ucs_origin,
    ucs_x_axis,
    ucs_y_axis,
    ucs_ortho_type,
    block_record,
    viewport,
    viewports,
    base_ucs,
    named_ucs,
    reactors,
    xdictionary_handle,
    raw_plot_settings_codes,
    paper_width,
    paper_height,
    plot_rotation,
    plot_flags,
    plot_page_name,
    plot_printer_name,
    paper_size,
    plot_view_name,
    plot_style_sheet,
    plot_margin_left,
    plot_margin_bottom,
    plot_margin_right,
    plot_margin_top,
    plot_origin_x,
    plot_origin_y,
    plot_window_min_x,
    plot_window_min_y,
    plot_window_max_x,
    plot_window_max_y,
    plot_paper_units,
    plot_type,
    plot_scale_numerator,
    plot_scale_denominator,
    plot_scale_type,
    plot_scale_factor,
    paper_image_origin_x,
    paper_image_origin_y,
    shade_plot_mode,
    shade_plot_resolution,
    shade_plot_dpi,
    plot_view_handle,
    visual_style_handle
});
construct_struct!(acadrust::objects::MultiLeaderStyle {
    handle,
    owner_handle,
    name,
    description,
    path_type,
    line_color,
    line_type_handle,
    line_weight,
    enable_landing,
    enable_dogleg,
    landing_distance,
    landing_gap,
    arrowhead_handle,
    arrowhead_size,
    content_type,
    text_style_handle,
    text_left_attachment,
    text_right_attachment,
    text_top_attachment,
    text_bottom_attachment,
    text_attachment_direction,
    text_angle_type,
    text_alignment,
    text_color,
    text_height,
    text_frame,
    text_always_left,
    default_text,
    block_content_handle,
    block_content_color,
    block_content_connection,
    block_content_rotation,
    block_content_scale_x,
    block_content_scale_y,
    block_content_scale_z,
    enable_block_scale,
    enable_block_rotation,
    scale_factor,
    align_space,
    break_gap_size,
    max_leader_points,
    first_segment_angle,
    second_segment_angle,
    leader_draw_order,
    multileader_draw_order,
    is_annotative,
    property_changed,
    unknown_flag_298
});
construct_struct!(acadrust::objects::ObjectContextData {
    handle,
    owner_handle,
    reactors,
    xdictionary_handle,
    class_version,
    is_default,
    scale,
    kind
});
construct_struct!(acadrust::objects::HatchScaleContext {
    pattern_lines,
    pattern_scale,
    pattern_base,
    loops
});
construct_struct!(acadrust::objects::HatchLoopContext {
    loop_type,
    supports_context,
    boundary
});
construct_struct!(acadrust::objects::HatchViewContext {
    hatch,
    view,
    view_normal,
    view_rotation,
    evaluate_hatch
});
construct_struct!(acadrust::objects::MTextContext {
    attachment,
    x_axis_dir,
    insertion,
    rect_width,
    rect_height,
    extents_width,
    extents_height,
    column_type,
    columns
});
construct_struct!(acadrust::objects::MTextAttributeContext {
    horizontal_mode,
    rotation,
    insertion,
    alignment,
    enable_context,
    context
});
construct_struct!(acadrust::objects::EmbeddedMTextContext {
    owner_handle,
    reactors,
    xdictionary_handle,
    has_binary_data,
    class_version,
    is_default,
    scale,
    mtext
});
construct_struct!(acadrust::objects::LeaderContext {
    points,
    x_direction,
    annotation_enabled,
    insertion_offset,
    endpoint_projection
});
construct_struct!(acadrust::objects::DimContext {
    def_pt,
    is_def_textloc,
    text_rotation,
    block,
    b293,
    dimtofl,
    dimosxd,
    dimatfit,
    dimtix,
    dimtmove,
    override_code,
    has_arrow2,
    flip_arrow2,
    flip_arrow1,
    subtype
});
construct_struct!(acadrust::objects::MTextColumns {
    num_heights,
    width,
    gutter,
    auto_height,
    flow_reversed,
    heights
});
construct_struct!(acadrust::objects::PlotFlags {
    unknown_bits,
    plot_viewport_borders,
    show_plot_styles,
    plot_centered,
    plot_hidden,
    use_standard_scale,
    plot_plot_styles,
    scale_lineweights,
    print_lineweights,
    draw_viewports_first,
    model_type,
    update_paper,
    zoom_to_paper_on_update,
    initializing,
    prev_plot_init
});
construct_struct!(acadrust::objects::PaperMargin {
    left,
    bottom,
    right,
    top
});
construct_struct!(acadrust::objects::PlotWindow {
    lower_left_x,
    lower_left_y,
    upper_right_x,
    upper_right_y
});
construct_struct!(acadrust::objects::Scale {
    handle,
    owner_handle,
    name,
    paper_units,
    drawing_units,
    is_unit_scale,
    is_temporary
});
construct_struct!(acadrust::objects::SemanticProperty {
    subclass,
    code,
    value
});
construct_struct!(acadrust::objects::RegisteredClassObject {
    handle,
    owner,
    reactors,
    xdictionary_handle,
    dxf_name,
    cpp_class_name,
    properties,
    payload,
    object_ids,
    raw_dwg_data,
    raw_dwg_handle_bits,
    raw_dwg_version
});
construct_struct!(acadrust::objects::ProxyObject {
    handle,
    owner,
    reactors,
    xdictionary_handle,
    proxy_id,
    class_id,
    dxf_subclass,
    version,
    dwg_version,
    maintenance_version,
    from_dxf,
    payload,
    text_payload,
    object_ids
});
construct_struct!(acadrust::objects::ProxyPayload {
    encoding,
    bit_count,
    records
});
construct_struct!(acadrust::objects::ProxyPayloadRecord {
    sequence,
    bit_offset,
    bit_count,
    data
});
construct_struct!(acadrust::objects::ProxyObjectReference { handle, kind });
construct_struct!(acadrust::objects::SortEntsEntry {
    entity_handle,
    sort_handle
});
construct_struct!(acadrust::objects::VisualStyleProperty { value, enabled });
construct_struct!(acadrust::objects::VisualStyle {
    handle,
    owner,
    reactors,
    xdictionary_handle,
    description,
    style_type,
    face_lighting_model,
    face_lighting_quality,
    face_color_mode,
    face_modifier,
    edge_model,
    edge_style,
    internal_use_only,
    extended_lighting_model,
    properties
});
construct_struct!(acadrust::objects::MaterialColor { flag, factor, rgb });
construct_struct!(acadrust::objects::MaterialTexture {
    mode,
    color1,
    color2,
    procedural,
    table_end
});
construct_struct!(acadrust::objects::MaterialMap {
    blend_factor,
    projection,
    tiling,
    auto_transform,
    transform,
    source,
    file_name,
    texture
});
construct_struct!(acadrust::objects::Material {
    handle,
    owner,
    reactors,
    xdictionary_handle,
    name,
    description,
    ambient_color,
    diffuse_color,
    diffuse_map,
    specular_color,
    specular_map,
    specular_gloss_factor,
    reflection_map,
    opacity_percent,
    opacity_map,
    bump_map,
    refraction_index,
    refraction_map,
    translucence,
    self_illumination,
    reflectivity,
    illumination_model,
    channel_flags,
    mode,
    indirect_bump_scale,
    reflectance_scale,
    transmittance_scale,
    two_sided_material,
    luminance,
    luminance_mode,
    normal_map_method,
    normal_map_strength,
    normal_map,
    is_anonymous,
    global_illumination,
    final_gather,
    color_bleed_scale,
    advanced_data_present
});
construct_struct!(acadrust::objects::GeoDataMeshPoint {
    source,
    destination
});
construct_struct!(acadrust::objects::GeoDataMeshFace {
    first,
    second,
    third
});
construct_struct!(acadrust::objects::GeoData {
    handle,
    owner,
    reactors,
    xdictionary_handle,
    version,
    host_block,
    coordinate_type,
    design_point,
    reference_point,
    obsolete_observation_point,
    obsolete_scale_vector,
    north_direction,
    up_direction,
    horizontal_unit_scale,
    vertical_unit_scale,
    horizontal_units,
    vertical_units,
    scale_estimation_method,
    user_scale_factor,
    sea_level_correction,
    sea_level_elevation,
    coordinate_projection_radius,
    coordinate_system_definition,
    coordinate_system_datum,
    coordinate_system_wkt,
    geo_rss_tag,
    observation_from_tag,
    observation_to_tag,
    observation_coverage_tag,
    mesh_points,
    mesh_faces,
    civil_data_present,
    civil_obsolete_flag,
    civil_reference_point1,
    civil_reference_point2,
    civil_unknown1,
    civil_unknown2,
    civil_unknown_flag1,
    civil_zero_point1,
    civil_zero_point2,
    civil_unknown_flag2,
    civil_north_angle_degrees,
    civil_north_angle_radians
});
construct_struct!(acadrust::objects::SpatialFilter {
    handle,
    owner,
    boundary_points,
    normal,
    origin,
    display_enabled,
    front_clip,
    back_clip,
    inverse_block_transform,
    clip_bound_transform
});
construct_struct!(acadrust::objects::RasterVariables {
    handle,
    owner,
    class_version,
    display_image_frame,
    image_quality,
    units
});
construct_struct!(acadrust::objects::BookColor {
    handle,
    owner,
    color,
    color_name,
    book_name
});
construct_struct!(acadrust::objects::PlaceHolder { handle, owner });
construct_struct!(acadrust::objects::DictionaryWithDefault {
    handle,
    owner,
    entries,
    default_handle,
    duplicate_cloning,
    hard_owner
});
construct_struct!(acadrust::objects::WipeoutVariables {
    handle,
    owner,
    display_frame
});
construct_struct!(acadrust::objects::TableCellBorder {
    property_flags,
    border_type,
    line_weight,
    color,
    is_invisible,
    double_line_spacing
});
construct_struct!(acadrust::objects::RowCellStyle {
    text_style_name,
    text_style_handle,
    text_height,
    alignment,
    text_color,
    fill_color,
    fill_enabled,
    data_type,
    unit_type,
    format_string,
    left_border,
    right_border,
    top_border,
    bottom_border,
    horizontal_inside_border,
    vertical_inside_border
});
construct_struct!(acadrust::objects::TableContentFormat {
    property_override_flags,
    property_flags,
    value_data_type,
    value_unit_type,
    value_format_string,
    rotation,
    block_scale,
    cell_alignment,
    content_color,
    text_style,
    text_height
});
construct_struct!(acadrust::objects::TableGridFormat {
    index_mask,
    border,
    line_type
});
construct_struct!(acadrust::objects::TableCellStyleData {
    style_type,
    data_flags,
    property_override_flags,
    merge_flags,
    background_color,
    content_layout,
    content_format,
    margin_override_flags,
    vertical_margin,
    horizontal_margin,
    bottom_margin,
    right_margin,
    horizontal_spacing,
    vertical_spacing,
    borders
});
construct_struct!(acadrust::objects::NamedTableCellStyle {
    cell_style,
    id,
    style_type,
    name
});
construct_struct!(acadrust::objects::TableStyle {
    handle,
    owner_handle,
    name,
    description,
    version,
    flow_direction,
    flags,
    horizontal_margin,
    vertical_margin,
    title_suppressed,
    header_suppressed,
    data_row_style,
    header_row_style,
    title_row_style,
    modern_unknown_byte,
    modern_unknown_long1,
    modern_unknown_long2,
    modern_cell_style_handle,
    modern_style,
    modern_overrides,
    annotative,
    raw_dxf_codes
});
construct_struct!(acadrust::objects::XRecordEntry { code, value });
construct_struct!(acadrust::objects::XRecordSection { name, entries });
construct_struct!(acadrust::objects::XRecord {
    handle,
    owner,
    reactors,
    xdictionary_handle,
    name,
    cloning_flags,
    entries,
    object_references,
    preserve_object_reference_stream,
    entries_complete,
    raw_data,
    raw_dwg_data,
    raw_dwg_handle_bits,
    raw_dwg_version
});
construct_struct!(acadrust::classes::ProxyFlags => (0));
construct_struct!(acadrust::classes::DxfClass {
    dxf_name,
    cpp_class_name,
    application_name,
    proxy_flags,
    instance_count,
    was_zombie,
    is_an_entity,
    class_number,
    item_class_id,
    dwg_version,
    maintenance_version,
    unknown1,
    unknown2
});
construct_struct!(acadrust::document::SolidHistoryGraph {
    root,
    nodes,
    evaluation_graph
});
construct_struct!(acadrust::document::HeaderVariables {
    legacy,
    required_versions,
    associate_dimensions,
    update_dimensions_while_dragging,
    ortho_mode,
    fill_mode,
    quick_text_mode,
    mirror_text,
    regen_mode,
    limit_check,
    paper_space_limit_check,
    polyline_linetype_generation,
    paper_space_linetype_scaling,
    show_model_space,
    user_timer,
    sketch_type,
    world_view,
    retain_xref_visibility,
    display_silhouette,
    spline_frame,
    delete_objects,
    record_solid_history,
    show_solid_history,
    drag_mode,
    blip_mode,
    attribute_request,
    attribute_dialog,
    linear_unit_format,
    linear_unit_precision,
    angular_unit_format,
    angular_unit_precision,
    insertion_units,
    attribute_visibility,
    point_display_mode,
    user_int1,
    user_int2,
    user_int3,
    user_int4,
    user_int5,
    coords_mode,
    object_snap_mode,
    pick_style,
    spline_type,
    spline_segments,
    spline_segs_surface,
    surface_u_density,
    surface_v_density,
    surface_type,
    surface_tab1,
    surface_tab2,
    shade_edge,
    shade_diffuse,
    max_active_viewports,
    isolines,
    multiline_justification,
    text_quality,
    sort_entities,
    index_control,
    hide_text,
    xclip_frame,
    dwf_frame,
    dgn_frame,
    halo_gap,
    obscured_color,
    obscured_linetype,
    intersection_display,
    intersection_color,
    interference_color,
    real_world_scale,
    light_glyph_display,
    tile_model_light_synch,
    polysolid_width,
    polysolid_height,
    dwf_3d_precision,
    dim_fixed_ext_line_length,
    dim_jog_angle,
    dim_text_fill,
    dim_text_fill_color,
    dimension_associativity,
    project_name,
    linetype_scale,
    text_height,
    trace_width,
    sketch_increment,
    sketch_tolerance,
    thickness,
    point_display_size,
    polyline_width,
    current_entity_linetype_scale,
    view_twist,
    fillet_radius,
    chamfer_distance_a,
    chamfer_distance_b,
    chamfer_length,
    chamfer_angle,
    angle_base,
    angle_direction,
    elevation,
    paper_elevation,
    facet_resolution,
    multiline_scale,
    user_real1,
    user_real2,
    user_real3,
    user_real4,
    user_real5,
    viewport_scale_factor,
    current_annotation_scale,
    annotation_scale_value,
    shadow_plane_location,
    loft_angle1,
    loft_angle2,
    loft_magnitude1,
    loft_magnitude2,
    loft_param,
    loft_normals,
    latitude,
    longitude,
    north_direction,
    timezone,
    steps_per_second,
    step_size,
    lens_length,
    camera_height,
    camera_display,
    current_entity_color,
    current_line_weight,
    current_plotstyle_type,
    end_caps,
    join_style,
    lineweight_display,
    xedit,
    extended_names,
    plotstyle_mode,
    ole_startup,
    dim_scale,
    dim_arrow_size,
    dim_ext_line_offset,
    dim_line_increment,
    dim_ext_line_extension,
    dim_rounding,
    dim_line_extension,
    dim_tolerance_plus,
    dim_tolerance_minus,
    dim_text_height,
    dim_center_mark,
    dim_tick_size,
    dim_alt_scale,
    dim_linear_scale,
    dim_text_vertical_pos,
    dim_tolerance_scale,
    dim_line_gap,
    dim_alt_rounding,
    dim_tolerance,
    dim_limits,
    dim_text_inside_horizontal,
    dim_text_outside_horizontal,
    dim_suppress_ext1,
    dim_suppress_ext2,
    dim_text_above,
    dim_zero_suppression,
    dim_alt_zero_suppression,
    dim_alternate_units,
    dim_alt_decimal_places,
    dim_force_line_inside,
    dim_separate_arrows,
    dim_force_text_inside,
    dim_suppress_outside_ext,
    dim_line_color,
    dim_ext_line_color,
    dim_text_color,
    dim_angular_decimal_places,
    dim_decimal_places,
    dim_tolerance_decimal_places,
    dim_alt_units_format,
    dim_alt_tolerance_decimal_places,
    dim_angular_units,
    dim_fraction_format,
    dim_linear_unit_format,
    dim_decimal_separator,
    dim_text_movement,
    dim_horizontal_justification,
    dim_suppress_line1,
    dim_suppress_line2,
    dim_tolerance_justification,
    dim_tolerance_zero_suppression,
    dim_alt_tolerance_zero_suppression,
    dim_alt_tolerance_zero_tight,
    dim_fit,
    dim_user_positioned_text,
    dim_post,
    dim_alt_post,
    dim_arrow_block,
    dim_arrow_block1,
    dim_arrow_block2,
    dim_leader_arrow_block,
    model_space_insertion_base,
    model_space_extents_min,
    model_space_extents_max,
    model_space_limits_min,
    model_space_limits_max,
    paper_space_insertion_base,
    paper_space_extents_min,
    paper_space_extents_max,
    paper_space_limits_min,
    paper_space_limits_max,
    ucs_base,
    model_space_ucs_name,
    paper_space_ucs_name,
    model_space_ucs_origin,
    model_space_ucs_x_axis,
    model_space_ucs_y_axis,
    paper_space_ucs_origin,
    paper_space_ucs_x_axis,
    paper_space_ucs_y_axis,
    ucs_ortho_ref,
    ucs_ortho_view,
    paper_ucs_ortho_ref,
    paper_ucs_ortho_view,
    handle_seed,
    current_layer_handle,
    current_text_style_handle,
    current_linetype_handle,
    current_dimstyle_handle,
    current_multiline_style_handle,
    current_material_handle,
    dim_text_style_handle,
    dim_linetype_handle,
    dim_linetype1_handle,
    dim_linetype2_handle,
    dim_arrow_block_handle,
    dim_arrow_block1_handle,
    dim_arrow_block2_handle,
    dim_line_weight,
    dim_ext_line_weight,
    block_control_handle,
    layer_control_handle,
    style_control_handle,
    linetype_control_handle,
    view_control_handle,
    ucs_control_handle,
    vport_control_handle,
    appid_control_handle,
    dimstyle_control_handle,
    vpent_hdr_control_handle,
    current_vx_handle,
    named_objects_dict_handle,
    acad_group_dict_handle,
    acad_mlinestyle_dict_handle,
    acad_layout_dict_handle,
    acad_plotsettings_dict_handle,
    acad_plotstylename_dict_handle,
    acad_material_dict_handle,
    acad_color_dict_handle,
    acad_visualstyle_dict_handle,
    model_space_block_handle,
    paper_space_block_handle,
    bylayer_linetype_handle,
    byblock_linetype_handle,
    continuous_linetype_handle,
    create_date_julian,
    update_date_julian,
    universal_create_date_julian,
    universal_update_date_julian,
    total_editing_time,
    user_elapsed_time,
    fingerprint_guid,
    version_guid,
    menu_name,
    code_page,
    last_saved_by,
    hyperlink_base,
    stylesheet,
    measurement,
    proxy_graphics,
    tree_depth,
    multiline_style,
    current_linetype_name,
    current_layer_name,
    current_text_style_name,
    current_dimstyle_name,
    current_table_style_name,
    current_mleader_style_name
});
construct_struct!(acadrust::document::Preview { format, data });
construct_struct!(acadrust::document::FieldDef {
    handle,
    owner,
    evaluator,
    code,
    objects
});
construct_struct!(acadrust::document::SummaryInfo {
    title,
    subject,
    author,
    keywords,
    comments,
    last_saved_by,
    revision_number,
    hyperlink_base,
    custom_properties
});
construct_struct!(acadrust::document::LegacyHeaderState {
    view_center,
    view_height,
    view_aspect_ratio,
    view_direction,
    view_target,
    snap_mode,
    snap_spacing,
    snap_base,
    snap_angle,
    snap_style,
    snap_isopair,
    grid_mode,
    grid_spacing,
    axis_mode,
    axis_spacing,
    circle_zoom,
    fast_zoom,
    front_clip,
    back_clip,
    view_mode,
    flatland,
    ucs_icon,
    unit_mode,
    user_unit_ratios,
    user_unit_names
});
construct_struct!(acadrust::audit::AuditIssue {
    code,
    severity,
    location,
    handle,
    message,
    repairable
});
construct_struct!(acadrust::audit::AuditReport {
    scanned_records,
    scanned_entities,
    scanned_objects,
    scanned_table_entries,
    issues
});
construct_struct!(acadrust::audit::RepairReport {
    before,
    after,
    actions,
    unresolved
});
construct_struct!(acadrust::notification::Notification {
    notification_type,
    message
});
construct_struct!(acadrust::layer_state::LayerStateLayer {
    layer_handle,
    layer_name,
    off,
    frozen,
    locked,
    plottable,
    new_viewport_frozen,
    color,
    color_name,
    book_name,
    line_type,
    line_weight,
    plot_style,
    transparency
});
construct_struct!(acadrust::layer_state::LayerState {
    name,
    description,
    mask,
    hidden,
    current_layer,
    layers
});
construct_struct!(acadrust::compound_file::StructuredStoragePayload {
    leading_records,
    compound_file,
    trailing_records
});
construct_struct!(acadrust::compound_file::BinaryRecord { sequence, data });
construct_struct!(acadrust::compound_file::CompoundFile {
    minor_version,
    major_version,
    transaction_signature,
    root
});
construct_struct!(acadrust::compound_file::CompoundStorage {
    name,
    class_id,
    state_bits,
    creation_time,
    modified_time,
    entries
});
construct_struct!(acadrust::compound_file::CompoundStream {
    name,
    class_id,
    state_bits,
    creation_time,
    modified_time,
    content
});
construct_struct!(acadrust::compound_file::CompoundPropertySetStream {
    version,
    system_identifier,
    class_id,
    sets
});
construct_struct!(acadrust::compound_file::CompoundPropertySet {
    format_id,
    properties
});
construct_struct!(acadrust::compound_file::CompoundProperty { id, value });
construct_struct!(acadrust::compound_file::CompoundDictionaryEntry { property_id, name });
construct_struct!(acadrust::document::DynamicBlockProperty {
    parameter_handle,
    name,
    description,
    type_code,
    units,
    show,
    read_only,
    visible,
    value,
    allowed_values
});
construct_struct!(acadrust::vba::VbaDirectoryRecord { id, value });
construct_struct!(acadrust::vba::VbaModuleDescriptor {
    name,
    stream_name,
    source_offset,
    code_page
});

#[cfg(test)]
mod tests {
    use super::*;

    fn staged_f64(value: &f64) -> StagedWrite {
        unsafe {
            write_input(&NativeWrite {
                kind: AR_VALUE_F64,
                data: std::ptr::from_ref(value).cast(),
                len: 1,
            })
            .unwrap()
        }
    }

    #[test]
    fn successful_entity_edits_discard_storage_and_preserve_unrelated_references() {
        let mut doc = CadDocument::new();
        let handle = doc
            .add_entity(acadrust::EntityType::Line(acadrust::Line::default()))
            .unwrap();
        let common = doc.get_entity_mut(handle).unwrap().common_mut();
        common.layer_handle = Some(Handle::new(777));
        common.raw_record = Some(Arc::new(acadrust::entities::RawRecord {
            data: vec![1, 2, 3],
            handle_bits: 8,
            version: acadrust::DxfVersion::AC1032,
        }));
        let value = 9.0f64;
        let input = staged_f64(&value);
        edit_entity(
            &mut doc,
            handle.value(),
            "start/x",
            &Edit::Write(input.input()),
        )
        .unwrap();
        let common = doc.get_entity(handle).unwrap().common();
        assert_eq!(common.handle, handle);
        assert!(common.raw_record.is_none());
        assert_eq!(common.layer_handle, Some(Handle::new(777)));
        let bytes = b"0";
        let input = WriteInput {
            kind: AR_VALUE_STRING,
            bytes,
            len: 1,
            stride: 1,
        };
        edit_entity(
            &mut doc,
            handle.value(),
            "common/layer",
            &Edit::Write(input),
        )
        .unwrap();
        assert!(doc
            .get_entity(handle)
            .unwrap()
            .common()
            .layer_handle
            .is_none());
    }

    #[test]
    fn invalid_writes_leave_cached_source_data_intact() {
        let mut doc = CadDocument::new();
        let handle = doc
            .add_entity(acadrust::EntityType::Line(acadrust::Line::default()))
            .unwrap();
        doc.get_entity_mut(handle).unwrap().common_mut().raw_record =
            Some(Arc::new(acadrust::entities::RawRecord {
                data: vec![1, 2, 3],
                handle_bits: 8,
                version: acadrust::DxfVersion::AC1032,
            }));
        let value = f64::NAN;
        let input = staged_f64(&value);
        assert!(edit_entity(
            &mut doc,
            handle.value(),
            "start/x",
            &Edit::Write(input.input())
        )
        .is_err());
        assert!(doc
            .get_entity(handle)
            .unwrap()
            .common()
            .raw_record
            .is_some());
    }
}
