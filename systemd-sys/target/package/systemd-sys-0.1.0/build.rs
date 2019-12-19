fn main() {
   pkg_config::find_library("libsystemd").expect("systemd not found via pkg-config");
}