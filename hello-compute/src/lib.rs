use anyhow::Result;
use std::str;

#[no_mangle]
pub extern "C" fn hello_compute(r#in: wasi::Fd, out: wasi::Fd) -> Result<()> {
    let mut buf = vec![0u8; 1000];

    // Read in from the input Fd
    let iovs = wasi::Iovec {
        buf: buf.as_mut_ptr(),
        buf_len: buf.len(),
    };
    let nread = unsafe { wasi::fd_read(out, &[iovs])? };

    // Convert to uppercase
    let to_str = str::from_utf8(&buf[..nread])?.to_uppercase();

    // Write out to the output Fd
    let ciovs = wasi::Ciovec {
        buf: to_str.as_ptr(),
        buf_len: to_str.len(),
    };
    unsafe { wasi::fd_write(r#in, &[ciovs])? };

    Ok(())
}
