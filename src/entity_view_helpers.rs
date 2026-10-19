use crate::views::{view, NativeView, StringView, ValueView, AR_VALUE_STRUCT, AR_VALUE_U64};

impl<T: NativeView> NativeView for acadrust::entities::LegacyBorderOverrides<T> {
    fn kind(&self) -> i32 {
        AR_VALUE_STRUCT
    }
    fn len(&self) -> usize {
        2
    }
    fn child(&self, index: usize) -> Option<(StringView, ValueView)> {
        match index {
            0 => Some((StringView::new("flags"), view(&self.flags))),
            1 => Some((StringView::new("values"), view(&self.values))),
            _ => None,
        }
    }
}

impl NativeView for acadrust::xdata::ExtendedData {
    fn kind(&self) -> i32 {
        AR_VALUE_STRUCT
    }
    fn len(&self) -> usize {
        self.records().len() + 1
    }
    fn child(&self, index: usize) -> Option<(StringView, ValueView)> {
        if index == 0 {
            Some((StringView::new("raw_dwg_eed"), view(&self.raw_dwg_eed)))
        } else {
            self.records()
                .get(index - 1)
                .map(|record| (StringView::new(&record.application_name), view(record)))
        }
    }
}

macro_rules! flag_view {
    ($($ty:path),* $(,)?) => { $(
        impl NativeView for $ty {
            fn kind(&self) -> i32 { AR_VALUE_U64 }
            fn number(&self) -> Option<i64> { Some(self.bits() as i64) }
        }
    )* };
}

flag_view!(
    acadrust::entities::InvisibleEdgeFlags,
    acadrust::entities::BoundaryPathFlags,
    acadrust::entities::PolylineFlags,
    acadrust::entities::VertexFlags,
);

impl NativeView for acadrust::entities::mtext_format::MTextStroke {
    fn kind(&self) -> i32 {
        AR_VALUE_U64
    }
    fn number(&self) -> Option<i64> {
        Some(
            i64::from(self.underline())
                | (i64::from(self.strikethrough()) << 1)
                | (i64::from(self.overline()) << 2),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::views::{AR_VALUE_MAP, AR_VALUE_OPAQUE};
    use crate::{ar_value_child, ar_value_kind, ar_value_len, ar_value_type_name, Status};

    unsafe fn text(value: StringView) -> String {
        String::from_utf8_lossy(std::slice::from_raw_parts(value.data.cast(), value.len))
            .into_owned()
    }

    unsafe fn verify_tree(value: ValueView, path: &str) {
        let mut kind = 0;
        assert_eq!(ar_value_kind(value, &mut kind), Status::Ok);
        let mut type_name = StringView::new("");
        assert_eq!(ar_value_type_name(value, &mut type_name), Status::Ok);
        assert_ne!(
            kind,
            AR_VALUE_OPAQUE,
            "opaque native field {path}: {}",
            text(type_name)
        );
        let mut count = 0;
        assert_eq!(ar_value_len(value, &mut count), Status::Ok);
        if !matches!(
            kind,
            crate::AR_VALUE_STRUCT
                | crate::AR_VALUE_ENUM
                | crate::AR_VALUE_SEQUENCE
                | crate::AR_VALUE_OPTION
                | AR_VALUE_MAP
        ) {
            return;
        }
        if kind == AR_VALUE_MAP {
            count *= 2;
        }
        for index in 0..count {
            let mut name = StringView::new("");
            let mut child = ValueView::default();
            assert_eq!(
                ar_value_child(value, index, &mut name, &mut child),
                Status::Ok
            );
            verify_tree(child, &format!("{path}/{}", text(name)));
        }
    }

    #[test]
    fn every_default_entity_is_fully_native() {
        for name in crate::entity_features::ENTITY_NAMES {
            let value = crate::entity_features::make_entity(name).unwrap();
            unsafe {
                verify_tree(view(&value), name);
            }
        }
    }

    #[test]
    fn native_views_include_storage_and_private_xdata_records() {
        let mut spline = acadrust::entities::Spline {
            dwg_scenario: Some(2),
            ..Default::default()
        };
        spline.common.graphic_data = Some(vec![1, 2, 3]);
        let mut record = acadrust::xdata::ExtendedDataRecord::new("APP");
        record.add_value(acadrust::xdata::XDataValue::String("value".into()));
        spline.common.extended_data.add_record(record);
        let entity = acadrust::EntityType::Spline(spline);
        unsafe {
            verify_tree(view(&entity), "Spline");
        }
        let mut spline_view = ValueView::default();
        let mut name = StringView::new("");
        unsafe {
            assert_eq!(
                ar_value_child(view(&entity), 0, &mut name, &mut spline_view),
                Status::Ok
            );
            let mut scenario = ValueView::default();
            assert_eq!(
                crate::ar_value_field(spline_view, c"dwg_scenario".as_ptr(), &mut scenario),
                Status::Ok
            );
            let mut scenario_value = ValueView::default();
            assert_eq!(
                ar_value_child(scenario, 0, &mut name, &mut scenario_value),
                Status::Ok
            );
            let mut data = std::ptr::null();
            let (mut len, mut stride) = (0, 0);
            assert_eq!(
                crate::ar_value_data(scenario_value, &mut data, &mut len, &mut stride),
                Status::Ok
            );
            assert_eq!(*data.cast::<i32>(), 2);
            assert_eq!((len, stride), (1, 4));
        }
    }

    #[test]
    fn scalar_geometry_views_borrow_original_storage() {
        let vector = acadrust::Vector3::new(1.0, 2.0, 3.0);
        let (_, x) = vector.child(0).unwrap();
        unsafe {
            let mut data = std::ptr::null();
            let (mut len, mut stride) = (0, 0);
            assert_eq!(
                crate::ar_value_data(x, &mut data, &mut len, &mut stride),
                Status::Ok
            );
            assert_eq!(data, std::ptr::from_ref(&vector.x).cast());
            assert_eq!((len, stride), (1, 8));
        }
    }
}
