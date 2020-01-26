use anyhow::{Context, Result};

unsafe fn fd_get_rights(fd: wasi::Fd) -> Result<(wasi::Rights, wasi::Rights)> {
    let fdstat = wasi::fd_fdstat_get(fd).context("could read fdstat from descriptor")?;
    Ok((fdstat.fs_rights_base, fdstat.fs_rights_inheriting))
}

#[no_mangle]
pub extern "C" fn compute(r#in: wasi::Fd, out: wasi::Fd) -> Result<()> {
    // Get rights for 'in' file descriptor
    let (base, inheriting) = unsafe { fd_get_rights(r#in)? };
    assert_eq!(base, wasi::RIGHTS_FD_READ);
    assert_eq!(inheriting, 0);

    // Get rights for 'out' file descriptor
    let (base, inheriting) = unsafe { fd_get_rights(out)? };
    assert_eq!(base, wasi::RIGHTS_FD_WRITE);
    assert_eq!(inheriting, 0);

    Ok(())
}
