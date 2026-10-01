use crate::ffi::StringCopy;
use core::ptr;

const PLACEHOLDER_COUNT: usize = 8;
const CHAR_DYNAMIC: u8 = 0xf7;
const EOS: u8 = 0xff;

unsafe extern "C" {}

#[unsafe(link_section = "ewram_data")]
static mut STRING_POINTERS: [*const u8; PLACEHOLDER_COUNT] = [ptr::null(); PLACEHOLDER_COUNT];

#[unsafe(no_mangle)]
pub unsafe fn DynamicPlaceholderTextUtil_Reset() {
    let base = (&raw mut STRING_POINTERS).cast::<*const u8>();
    let mut index = 0;
    while index < PLACEHOLDER_COUNT {
        unsafe { base.add(index).write(ptr::null()) };
        index += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe fn DynamicPlaceholderTextUtil_SetPlaceholderPtr(index: u8, value: *const u8) {
    if usize::from(index) < PLACEHOLDER_COUNT {
        unsafe {
            (&raw mut STRING_POINTERS)
                .cast::<*const u8>()
                .add(usize::from(index))
                .write(value)
        };
    }
}

#[unsafe(no_mangle)]
pub unsafe fn DynamicPlaceholderTextUtil_ExpandPlaceholders(
    mut destination: *mut u8,
    mut source: *const u8,
) -> *mut u8 {
    while unsafe { source.read() } != EOS {
        if unsafe { source.read() } != CHAR_DYNAMIC {
            unsafe { destination.write(source.read()) };
            destination = unsafe { destination.add(1) };
            source = unsafe { source.add(1) };
        } else {
            source = unsafe { source.add(1) };
            let index = usize::from(unsafe { source.read() });
            let placeholder = unsafe {
                (&raw const STRING_POINTERS)
                    .cast::<*const u8>()
                    .add(index)
                    .read()
            };
            if !placeholder.is_null() {
                destination = unsafe { StringCopy(destination, placeholder) };
            }
            source = unsafe { source.add(1) };
        }
    }
    unsafe { destination.write(EOS) };
    destination
}

#[unsafe(no_mangle)]
pub unsafe fn DynamicPlaceholderTextUtil_GetPlaceholderPtr(index: u8) -> *const u8 {
    unsafe {
        (&raw const STRING_POINTERS)
            .cast::<*const u8>()
            .add(usize::from(index))
            .read()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reset_and_set_preserve_the_pointer_table_contract() {
        let text = [1_u8, 2, EOS];
        unsafe {
            DynamicPlaceholderTextUtil_Reset();
            assert!(DynamicPlaceholderTextUtil_GetPlaceholderPtr(3).is_null());
            DynamicPlaceholderTextUtil_SetPlaceholderPtr(3, text.as_ptr());
            assert_eq!(
                DynamicPlaceholderTextUtil_GetPlaceholderPtr(3),
                text.as_ptr()
            );
            DynamicPlaceholderTextUtil_SetPlaceholderPtr(PLACEHOLDER_COUNT as u8, text.as_ptr());
            assert!(DynamicPlaceholderTextUtil_GetPlaceholderPtr(0).is_null());
        }
    }
}
