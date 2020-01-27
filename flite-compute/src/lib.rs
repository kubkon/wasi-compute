use anyhow::{anyhow, Result};
use std::{
    convert::{TryFrom, TryInto},
    mem::{self, MaybeUninit},
    ptr::{self, NonNull},
    slice,
};

#[repr(C)]
struct CstVoice {
    _private: [u8; 0],
}

#[repr(C)]
struct CstWave {
    r#type: *const libc::c_char,
    sample_rate: libc::c_int,
    num_samples: libc::c_int,
    num_channels: libc::c_int,
    samples: *const libc::c_short,
}

extern "C" {
    fn register_cmu_us_kal(voxdir: *const libc::c_char) -> *mut CstVoice;
    fn flite_init() -> libc::c_int;
    fn flite_text_to_wave(text: *const libc::c_char, voice: *const CstVoice) -> *mut CstWave;
}

fn write_wave(fd: wasi::Fd, wave: NonNull<CstWave>) -> Result<()> {
    let sample_rate: u32 = unsafe { wave.as_ref().sample_rate.try_into()? };
    let num_samples: u32 = unsafe { wave.as_ref().num_samples.try_into()? };
    let num_channels: u32 = unsafe { wave.as_ref().num_channels.try_into()? };
    let mut buf: Vec<u8> = vec![];
    buf.extend_from_slice("RIFF".as_bytes());
    // num bytes in the whole file
    let sample_size: u32 = mem::size_of::<u16>().try_into()?;
    let num_bytes: u32 = (num_samples * num_channels * sample_size) + 8 + 16 + 12;
    buf.extend_from_slice(&num_bytes.to_le_bytes());
    buf.extend_from_slice("WAVE".as_bytes());
    buf.extend_from_slice("fmt ".as_bytes());
    // size of header
    buf.extend_from_slice(&(16u32).to_le_bytes());
    // sample type
    buf.extend_from_slice(&(0x0001u16).to_le_bytes());
    // number of channels
    buf.extend_from_slice(&u16::try_from(num_channels)?.to_le_bytes());
    // sample rate
    buf.extend_from_slice(&sample_rate.to_le_bytes());
    // averate bytes per second
    let num_bytes = sample_rate * num_channels * sample_size;
    buf.extend_from_slice(&num_bytes.to_le_bytes());
    // block align
    let num_bytes: u16 = (num_channels * sample_size).try_into()?;
    buf.extend_from_slice(&num_bytes.to_le_bytes());
    // bits per sample
    buf.extend_from_slice(&(2 * 8u16).to_le_bytes());
    // data
    buf.extend_from_slice("data".as_bytes());
    let data = unsafe {
        slice::from_raw_parts(
            wave.as_ref().samples,
            (num_channels * num_samples).try_into()?,
        )
    };
    buf.extend(data.iter().map(|x| x.to_le_bytes().to_vec()).flatten());

    let ciovs = wasi::Ciovec {
        buf: buf.as_ptr(),
        buf_len: buf.len(),
    };
    unsafe { wasi::fd_write(fd, &[ciovs])? };
    Ok(())
}

#[no_mangle]
pub extern "C" fn compute(r#in: wasi::Fd, out: wasi::Fd) -> Result<()> {
    let mut buf = vec![0u8; 1000];

    // Read in from the input Fd
    let iovs = wasi::Iovec {
        buf: buf.as_mut_ptr(),
        buf_len: buf.len(),
    };
    let _nread = unsafe { wasi::fd_read(r#in, &[iovs])? };

    let mut output = MaybeUninit::<NonNull<CstWave>>::uninit();
    unsafe {
        flite_init();
        let voice = register_cmu_us_kal(ptr::null());
        let res = NonNull::new(flite_text_to_wave(buf.as_ptr() as *const _, voice))
            .ok_or(anyhow!("flite_text_to_wave returned a NULL pointer"))?;
        output.as_mut_ptr().write(res);
    }
    let output = unsafe { output.assume_init() };
    write_wave(out, output)
}
