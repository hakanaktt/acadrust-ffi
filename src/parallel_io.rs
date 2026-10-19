//! Parallel file I/O through a dedicated Rayon pool.
//!
//! The pool is separate from Rayon's global pool, so callers can choose a
//! worker count without changing the host application's global Rayon setup.
//! While a batch operation runs inside this pool, acadrust's own DWG page and
//! object parallelism uses the same pool and configured worker count.

use crate::{
    catch_ffi, document, ffi_call, input_str, invalid, output, read_file, write_file, Document,
    FfiError, FfiResult, Status,
};
use rayon::prelude::*;
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::ffi::c_char;
use std::ptr;
use std::sync::{Arc, OnceLock, RwLock};

static IO_POOL: OnceLock<RwLock<Arc<ThreadPool>>> = OnceLock::new();

fn default_threads() -> usize {
    std::thread::available_parallelism()
        .map(|value| value.get())
        .unwrap_or(1)
        .max(1)
}

fn build_pool(threads: usize) -> FfiResult<Arc<ThreadPool>> {
    ThreadPoolBuilder::new()
        .num_threads(threads)
        .thread_name(|index| format!("acadrust-ffi-io-{index}"))
        .build()
        .map(Arc::new)
        .map_err(|error| invalid(format!("could not create Rayon I/O pool: {error}")))
}

fn pool() -> FfiResult<Arc<ThreadPool>> {
    let lock = IO_POOL.get_or_init(|| {
        // The default is always valid because available_parallelism is at least
        // one. If the platform cannot report it, default_threads returns one.
        RwLock::new(build_pool(default_threads()).expect("one Rayon worker must be valid"))
    });
    lock.read()
        .map(|pool| Arc::clone(&pool))
        .map_err(|_| invalid("Rayon I/O pool lock is poisoned"))
}

/// Run one I/O operation in the configured pool. This also routes acadrust's
/// nested DWG parallel stages to the configured pool for single-file calls.
pub(crate) fn install<T: Send>(operation: impl FnOnce() -> T + Send) -> FfiResult<T> {
    Ok(pool()?.install(operation))
}

fn validate_array<T>(pointer: *const T, count: usize, name: &str) -> FfiResult<()> {
    if count > 0 && pointer.is_null() {
        return Err(invalid(format!("{name} is NULL")));
    }
    if count > isize::MAX as usize {
        return Err(invalid(format!("{name} count is too large")));
    }
    Ok(())
}

unsafe fn copy_paths(paths: *const *const c_char, count: usize) -> FfiResult<Vec<String>> {
    validate_array(paths, count, "paths")?;
    (0..count)
        .map(|index| input_str(*paths.add(index)).map(str::to_owned))
        .collect()
}

fn first_error(statuses: &[Status], errors: &[Option<FfiError>]) -> FfiResult<()> {
    if let Some((index, error)) = errors
        .iter()
        .enumerate()
        .find_map(|(index, error)| error.as_ref().map(|error| (index, error)))
    {
        Err(FfiError {
            status: error.status,
            message: format!("parallel I/O item {index}: {}", error.message),
        })
    } else if let Some(status) = statuses
        .iter()
        .copied()
        .find(|status| *status != Status::Ok)
    {
        Err(FfiError {
            status,
            message: "parallel I/O operation failed".to_owned(),
        })
    } else {
        Ok(())
    }
}

/// Set the worker count for subsequent parallel I/O calls. Zero selects the
/// platform's available parallelism. Existing calls continue on their old
/// pool; a new call observes the replacement atomically.
#[no_mangle]
pub extern "C" fn ar_io_set_threads(threads: usize) -> Status {
    ffi_call(|| {
        let threads = if threads == 0 {
            default_threads()
        } else {
            if threads > isize::MAX as usize {
                return Err(invalid("thread count is too large"));
            }
            threads
        };
        let replacement = build_pool(threads)?;
        let lock = IO_POOL.get_or_init(|| {
            RwLock::new(build_pool(default_threads()).expect("one Rayon worker must be valid"))
        });
        *lock
            .write()
            .map_err(|_| invalid("Rayon I/O pool lock is poisoned"))? = replacement;
        Ok(())
    })
}

/// Return the configured worker count for the parallel I/O pool.
#[no_mangle]
pub unsafe extern "C" fn ar_io_get_threads(out_threads: *mut usize) -> Status {
    ffi_call(|| {
        *output(out_threads)? = pool()?.current_num_threads();
        Ok(())
    })
}

/// Read independent CAD files concurrently. `documents` and `statuses` must
/// have `count` entries. Each document is initialized to NULL before work; an
/// item error leaves its document NULL and records its status. The overall
/// status is the first item error, if any. Inputs are copied before parallel
/// work starts, so caller strings may be changed after this call returns.
#[no_mangle]
pub unsafe extern "C" fn ar_documents_read_files(
    paths: *const *const c_char,
    count: usize,
    documents: *mut *mut Document,
    statuses: *mut Status,
) -> Status {
    ffi_call(|| {
        validate_array(documents, count, "documents")?;
        validate_array(statuses, count, "statuses")?;
        for index in 0..count {
            documents.add(index).write(ptr::null_mut());
            statuses.add(index).write(Status::Panic);
        }
        let paths = copy_paths(paths, count)?;
        if count == 0 {
            return Ok(());
        }
        let pool = pool()?;
        let results: Vec<FfiResult<Document>> = pool.install(|| {
            paths
                .par_iter()
                .map(|path| catch_ffi(|| Ok(Document::new(read_file(path)?))))
                .collect()
        });
        let mut statuses_copy = Vec::with_capacity(count);
        let mut errors = Vec::with_capacity(count);
        for (index, result) in results.into_iter().enumerate() {
            match result {
                Ok(document) => {
                    statuses.add(index).write(Status::Ok);
                    documents
                        .add(index)
                        .write(Box::into_raw(Box::new(document)));
                    statuses_copy.push(Status::Ok);
                    errors.push(None);
                }
                Err(error) => {
                    statuses.add(index).write(error.status);
                    statuses_copy.push(error.status);
                    errors.push(Some(error));
                }
            }
        }
        first_error(&statuses_copy, &errors)
    })
}

/// Write independent documents concurrently. The document pointers must stay
/// alive and must not be mutated for the duration of this call. `statuses`
/// receives one status per output path; the overall status is the first error.
#[no_mangle]
pub unsafe extern "C" fn ar_documents_write_files(
    documents: *const *const Document,
    paths: *const *const c_char,
    count: usize,
    format: i32,
    statuses: *mut Status,
) -> Status {
    ffi_call(|| {
        validate_array(documents, count, "documents")?;
        validate_array(statuses, count, "statuses")?;
        for index in 0..count {
            statuses.add(index).write(Status::Panic);
        }
        let paths = copy_paths(paths, count)?;
        let documents: Vec<&acadrust::CadDocument> = (0..count)
            .map(|index| document(*documents.add(index)))
            .collect::<FfiResult<_>>()?;
        if count == 0 {
            return Ok(());
        }
        let pool = pool()?;
        let results: Vec<FfiResult<()>> = pool.install(|| {
            documents
                .par_iter()
                .zip(paths.par_iter())
                .map(|(document, path)| catch_ffi(|| write_file(document, path, format)))
                .collect()
        });
        let mut statuses_copy = Vec::with_capacity(count);
        let mut errors = Vec::with_capacity(count);
        for (index, result) in results.into_iter().enumerate() {
            match result {
                Ok(()) => {
                    statuses.add(index).write(Status::Ok);
                    statuses_copy.push(Status::Ok);
                    errors.push(None);
                }
                Err(error) => {
                    statuses.add(index).write(error.status);
                    statuses_copy.push(error.status);
                    errors.push(Some(error));
                }
            }
        }
        first_error(&statuses_copy, &errors)
    })
}
