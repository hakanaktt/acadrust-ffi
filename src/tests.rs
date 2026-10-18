use super::*;
use std::ffi::CStr;

#[test]
fn panic_is_reported_without_unwinding_into_c() {
    assert_eq!(ffi_call(|| panic!("test panic")), Status::Panic);
    let error = unsafe { CStr::from_ptr(ar_last_error()) }.to_str().unwrap();
    assert!(error.contains("test panic"));
    assert_eq!(ffi_call(|| Ok(())), Status::Ok);
    assert_eq!(unsafe { CStr::from_ptr(ar_last_error()) }.to_bytes(), b"");
}

#[test]
fn errors_are_local_to_the_calling_thread() {
    ffi_call(|| Err(invalid("parent thread")));
    std::thread::spawn(|| {
        assert_eq!(unsafe { CStr::from_ptr(ar_last_error()) }.to_bytes(), b"");
        ffi_call(|| Err(invalid("child thread")));
    })
    .join()
    .unwrap();
    assert_eq!(
        unsafe { CStr::from_ptr(ar_last_error()) }.to_bytes(),
        b"parent thread"
    );
}

#[test]
fn document_and_buffer_contracts() {
    unsafe {
        let mut doc = ptr::null_mut();
        assert_eq!(ar_document_new(99999, &mut doc), Status::InvalidArgument);
        assert!(doc.is_null());
        assert_eq!(
            ar_document_new(1032, ptr::null_mut()),
            Status::InvalidArgument
        );
        assert_eq!(ar_document_new(1032, &mut doc), Status::Ok);
        let mut code = 0;
        assert_eq!(ar_document_get_version(doc, &mut code), Status::Ok);
        assert_eq!(code, 1032);

        let line = Line {
            start: Vector3::default(),
            end: Vector3 {
                x: 10.0,
                y: 20.0,
                z: 3.0,
            },
        };
        let mut handle = 0;
        assert_eq!(ar_line_add(doc, &line, &mut handle), Status::Ok);
        let mut cloned = ptr::null_mut();
        assert_eq!(ar_document_clone(doc, &mut cloned), Status::Ok);
        assert_eq!(ar_entity_remove(doc, handle), Status::Ok);
        let mut value = Line::default();
        assert_eq!(ar_line_get(cloned, handle, &mut value), Status::Ok);
        assert_eq!(value, line);

        for format in [AR_FORMAT_DXF_ASCII, AR_FORMAT_DXF_BINARY, AR_FORMAT_DWG] {
            let mut buffer = Buffer::default();
            assert_eq!(
                ar_document_write_buffer(cloned, format, &mut buffer),
                Status::Ok
            );
            assert!(buffer.len > 100);
            for input_format in [AR_FORMAT_AUTO, format] {
                let mut restored = ptr::null_mut();
                assert_eq!(
                    ar_document_read_buffer(buffer.data, buffer.len, input_format, &mut restored),
                    Status::Ok
                );
                assert_eq!(ar_line_get(restored, handle, &mut value), Status::Ok);
                assert_eq!(value, line);
                ar_document_free(restored);
            }
            ar_buffer_free(&mut buffer);
            assert!(buffer.data.is_null());
            assert_eq!(buffer.len, 0);
            ar_buffer_free(&mut buffer);
        }
        let mut buffer = Buffer::default();
        assert_eq!(
            ar_document_write_buffer(cloned, AR_FORMAT_AUTO, &mut buffer),
            Status::InvalidArgument
        );
        assert!(buffer.data.is_null());
        ar_document_free(doc);
        doc = ptr::null_mut();
        assert_eq!(
            ar_document_read_buffer(ptr::null(), 100, AR_FORMAT_AUTO, &mut doc),
            Status::InvalidArgument
        );
        assert!(doc.is_null());
        ar_document_free(cloned);
    }
}

#[test]
fn malformed_input_and_nulls_report_errors() {
    unsafe {
        let mut out = ptr::null_mut();
        assert_eq!(
            ar_document_read_file(ptr::null(), &mut out),
            Status::InvalidArgument
        );
        assert_eq!(
            ar_document_read_file(c"__acadrust_missing_test_file__.dwg".as_ptr(), &mut out),
            Status::IoError
        );
        assert!(out.is_null());
        assert_eq!(
            ar_document_read_buffer(b"invalid".as_ptr(), 7, 500, &mut out),
            Status::InvalidArgument
        );
        assert_eq!(
            ar_document_read_buffer(ptr::null(), 0, AR_FORMAT_AUTO, &mut out),
            Status::InvalidArgument
        );
        assert_eq!(
            ar_document_get_version(ptr::null(), ptr::null_mut()),
            Status::InvalidArgument
        );
        ar_document_free(ptr::null_mut());
        ar_buffer_free(ptr::null_mut());
        ar_string_free(ptr::null_mut());
    }
}

#[test]
fn parallel_file_io_uses_configured_rayon_pool() {
    unsafe {
        assert_eq!(ar_io_set_threads(1), Status::Ok);
        let mut threads = 0;
        assert_eq!(ar_io_get_threads(&mut threads), Status::Ok);
        assert_eq!(threads, 1);
        assert_eq!(ar_io_set_threads(2), Status::Ok);
        threads = 0;
        assert_eq!(ar_io_get_threads(&mut threads), Status::Ok);
        assert_eq!(threads, 2);
        assert_eq!(ar_io_set_threads(0), Status::Ok);
        assert_eq!(ar_io_get_threads(&mut threads), Status::Ok);
        assert!(threads >= 1);
        assert_eq!(ar_io_set_threads(usize::MAX), Status::InvalidArgument);
        assert_eq!(ar_io_set_threads(2), Status::Ok);

        let root = std::env::temp_dir().join(format!(
            "acadrust_ffi_parallel_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&root).unwrap();
        let input = root.join("input.dxf");
        let output_a = root.join("a.dxf");
        let output_b = root.join("b.dxf");
        let input = CString::new(input.to_string_lossy().as_bytes()).unwrap();
        let output_a = CString::new(output_a.to_string_lossy().as_bytes()).unwrap();
        let output_b = CString::new(output_b.to_string_lossy().as_bytes()).unwrap();

        let mut source = ptr::null_mut();
        assert_eq!(ar_document_new(1032, &mut source), Status::Ok);
        let line = Line {
            start: Vector3::default(),
            end: Vector3 {
                x: 10.0,
                y: 20.0,
                z: 30.0,
            },
        };
        let mut handle = 0;
        assert_eq!(ar_line_add(source, &line, &mut handle), Status::Ok);
        assert_eq!(
            ar_document_write_file(source, input.as_ptr(), AR_FORMAT_DXF_ASCII),
            Status::Ok
        );
        ar_document_free(source);

        let paths = [input.as_ptr(), input.as_ptr()];
        let mut documents = [ptr::null_mut(); 2];
        let mut statuses = [Status::Panic; 2];
        assert_eq!(
            ar_documents_read_files(
                paths.as_ptr(),
                paths.len(),
                documents.as_mut_ptr(),
                statuses.as_mut_ptr()
            ),
            Status::Ok
        );
        assert_eq!(statuses, [Status::Ok, Status::Ok]);
        assert!(documents.iter().all(|document| !document.is_null()));

        let output_paths = [output_a.as_ptr(), output_b.as_ptr()];
        let document_refs = [
            documents[0] as *const Document,
            documents[1] as *const Document,
        ];
        statuses = [Status::Panic; 2];
        assert_eq!(
            ar_documents_write_files(
                document_refs.as_ptr(),
                output_paths.as_ptr(),
                output_paths.len(),
                AR_FORMAT_DXF_BINARY,
                statuses.as_mut_ptr(),
            ),
            Status::Ok
        );
        assert_eq!(statuses, [Status::Ok, Status::Ok]);
        assert!(std::fs::metadata(output_a.to_str().unwrap()).unwrap().len() > 100);
        assert!(std::fs::metadata(output_b.to_str().unwrap()).unwrap().len() > 100);

        for document in documents {
            ar_document_free(document);
        }
        let missing = CString::new(root.join("missing.dxf").to_string_lossy().as_bytes()).unwrap();
        let error_paths = [missing.as_ptr(), input.as_ptr()];
        let mut error_documents = [ptr::null_mut(); 2];
        statuses = [Status::Ok; 2];
        assert_eq!(
            ar_documents_read_files(
                error_paths.as_ptr(),
                2,
                error_documents.as_mut_ptr(),
                statuses.as_mut_ptr()
            ),
            Status::IoError
        );
        assert_eq!(statuses[0], Status::IoError);
        assert_eq!(statuses[1], Status::Ok);
        ar_document_free(error_documents[1]);
        std::fs::remove_dir_all(root).unwrap();
    }
}
