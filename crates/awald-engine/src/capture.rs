/// stdout / stderr capture via Python `sys` module redirection.
///
/// Installs an `io.StringIO` buffer as `sys.stdout` / `sys.stderr`,
/// runs the user code, then drains and restores the originals.
use pyo3::prelude::*;
use pyo3::types::PyAny;

use crate::Result;

/// Returns (stdout_capture, stderr_capture) StringIO objects.
pub fn install(py: Python<'_>) -> Result<(Py<PyAny>, Py<PyAny>)> {
    let io  = py.import("io")?;
    let sys = py.import("sys")?;

    let out = io.call_method0("StringIO")?;
    let err = io.call_method0("StringIO")?;

    sys.setattr("stdout", &out)?;
    sys.setattr("stderr", &err)?;

    Ok((out.unbind(), err.unbind()))
}

/// Drain a StringIO capture buffer, returning its contents as a String.
pub fn drain(cap: &Py<PyAny>, py: Python<'_>) -> Result<String> {
    let bound  = cap.bind(py);
    let s: String = bound.call_method0("getvalue")?.extract()?;
    Ok(s)
}

/// Restore sys.stdout and sys.stderr to the original `__stdout__`/`__stderr__`.
pub fn restore(py: Python<'_>) -> Result<()> {
    let sys = py.import("sys")?;
    sys.setattr("stdout", sys.getattr("__stdout__")?)?;
    sys.setattr("stderr", sys.getattr("__stderr__")?)?;
    Ok(())
}
