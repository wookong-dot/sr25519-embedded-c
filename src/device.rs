#[cfg(any(feature = "embedded"))]
use core::panic::PanicInfo;

#[cfg(any(feature = "embedded"))]
#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
	loop {}
}