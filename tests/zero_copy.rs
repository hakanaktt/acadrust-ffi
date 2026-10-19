use acadrust_ffi::*;
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::ptr;

struct TrackedAllocator;
thread_local! {
    static TRACK: Cell<bool> = const { Cell::new(false) };
    static ALLOCATIONS: Cell<usize> = const { Cell::new(0) };
}

fn allocation() {
    let _ = TRACK.try_with(|active| {
        if active.get() {
            let _ = ALLOCATIONS.try_with(|n| n.set(n.get() + 1));
        }
    });
}

unsafe impl GlobalAlloc for TrackedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        allocation();
        System.alloc(layout)
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        allocation();
        System.alloc_zeroed(layout)
    }
    unsafe fn realloc(&self, p: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        allocation();
        System.realloc(p, layout, size)
    }
    unsafe fn dealloc(&self, p: *mut u8, layout: Layout) {
        System.dealloc(p, layout)
    }
}

#[global_allocator]
static ALLOCATOR: TrackedAllocator = TrackedAllocator;

#[test]
fn borrowed_document_and_entity_traversal_performs_no_allocations() {
    unsafe {
        let mut doc = ptr::null_mut();
        assert_eq!(ar_document_new(1032, &mut doc), Status::Ok);
        let mut handle = 0;
        assert_eq!(
            ar_entity_add_default(doc, c"TEXT".as_ptr(), &mut handle),
            Status::Ok
        );
        let mut root = ValueView::default();
        assert_eq!(ar_document_view(doc, &mut root), Status::Ok);
        ALLOCATIONS.with(|n| n.set(0));
        TRACK.with(|active| active.set(true));
        for _ in 0..100 {
            let mut header = ValueView::default();
            let mut project = ValueView::default();
            assert_eq!(
                ar_value_field(root, c"header".as_ptr(), &mut header),
                Status::Ok
            );
            assert_eq!(
                ar_value_field(header, c"project_name".as_ptr(), &mut project),
                Status::Ok
            );
            let mut data = ptr::null();
            let mut len = 0;
            let mut stride = 0;
            assert_eq!(
                ar_value_data(project, &mut data, &mut len, &mut stride),
                Status::Ok
            );
            assert_eq!(stride, 1);
            let mut entity = ValueView::default();
            assert_eq!(ar_entity_view(doc, handle, &mut entity), Status::Ok);
            let mut name = StringView {
                data: ptr::null(),
                len: 0,
            };
            let mut payload = ValueView::default();
            assert_eq!(
                ar_value_child(entity, 0, &mut name, &mut payload),
                Status::Ok
            );
            let mut text = ValueView::default();
            assert_eq!(
                ar_value_field(payload, c"value".as_ptr(), &mut text),
                Status::Ok
            );
            let mut first = ptr::null();
            let mut second = ptr::null();
            assert_eq!(
                ar_value_data(text, &mut first, &mut len, &mut stride),
                Status::Ok
            );
            assert_eq!(
                ar_value_data(text, &mut second, &mut len, &mut stride),
                Status::Ok
            );
            assert_eq!(first, second);
        }
        TRACK.with(|active| active.set(false));
        let allocations = ALLOCATIONS.with(Cell::get);
        ar_document_free(doc);
        assert_eq!(allocations, 0, "borrowed traversal allocated");
    }
}
