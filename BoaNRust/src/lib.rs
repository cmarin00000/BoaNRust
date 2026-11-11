use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::ptr;
use std::panic::{catch_unwind, AssertUnwindSafe};

fn validate(
    root_bytes: &[u8],
    candidate_bytes: &[u8],
) -> Result<Vec<u8>, c_int> {

    let root_path =
        Path::new(std::ffi::OsStr::from_bytes(root_bytes));

    let candidate_path =
        Path::new(std::ffi::OsStr::from_bytes(candidate_bytes));

    // Obtener las rutas físicas reales
    let canonical_root =
        std::fs::canonicalize(root_path).map_err(|_| -2)?;

    let canonical_candidate =
        std::fs::canonicalize(candidate_path).map_err(|_| -2)?;

    // Comprobar que el destino permanece dentro del DocumentRoot
    if !canonical_candidate.starts_with(&canonical_root) {
        return Err(-3);
    }

    // Permitir únicamente archivos normales o directorios
    let metadata =
        std::fs::metadata(&canonical_candidate).map_err(|_| -2)?;

    if !(metadata.is_file() || metadata.is_dir()) {
        return Err(-4);
    }

    Ok(canonical_candidate.as_os_str().as_bytes().to_vec())
}

#[no_mangle]
pub unsafe extern "C" fn boa_validate_path(
    document_root: *const c_char,
    candidate_path: *const c_char,
    out: *mut c_char,
    out_len: usize,
) -> c_int {

    if document_root.is_null()
        || candidate_path.is_null()
        || out.is_null()
        || out_len == 0 {
        return -1;
    }

    let result = catch_unwind(AssertUnwindSafe(|| {
        let root = unsafe { CStr::from_ptr(document_root) };
        let candidate = unsafe { CStr::from_ptr(candidate_path) };

        let canonical =
            validate(root.to_bytes(), candidate.to_bytes())?;

        if canonical.len() + 1 > out_len {
            return Err(-5);
        }

        unsafe {
            ptr::copy_nonoverlapping(
                canonical.as_ptr(),
                out.cast::<u8>(),
                canonical.len(),
            );

            *out.add(canonical.len()) = 0;
        }

        Ok(())
    }));

    match result {
        Ok(Ok(())) => 0,
        Ok(Err(code)) => code,
        Err(_) => -6,
    }
}
