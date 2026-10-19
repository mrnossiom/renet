use nix::{
	ioctl_write_ptr_bad,
	libc::{self, ifreq},
};
use std::{
	ffi::CStr,
	fs::{self, File},
	io,
	mem::{MaybeUninit, transmute},
	os::fd::AsRawFd,
};

ioctl_write_ptr_bad!(tun_set_interface, libc::TUNSETIFF, ifreq);

pub fn open_tap(if_name: &CStr) -> io::Result<File> {
	let file = fs::OpenOptions::new()
		.read(true)
		.write(true)
		// .custom_flags(libc::O_NONBLOCK)
		.open("/dev/net/tun")?;

	unsafe {
		let mut if_req = MaybeUninit::<ifreq>::zeroed().assume_init();

		let raw_name = transmute::<&[u8], &[i8]>(if_name.to_bytes_with_nul());
		if_req.ifr_name[..raw_name.len()].clone_from_slice(raw_name);

		if_req.ifr_ifru.ifru_flags |= (libc::IFF_TAP | libc::IFF_NO_PI) as i16;

		tun_set_interface(file.as_raw_fd(), &raw const if_req).unwrap();
	}

	Ok(file)
}
