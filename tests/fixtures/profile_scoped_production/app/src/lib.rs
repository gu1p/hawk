#[cfg(feature = "debug-web")]
#[unsafe(no_mangle)]
pub extern "C" fn flutter_entry() {}

#[cfg(not(feature = "debug-web"))]
#[unsafe(no_mangle)]
pub extern "C" fn flutter_entry() {
    internal::profile_union_api();
}
