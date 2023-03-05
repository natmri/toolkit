extern crate napi_build;
use cfg_aliases::cfg_aliases;

fn main() {
  cfg_aliases! {
      windows_platform:  { target_os = "windows" },
      linux_platform: { target_os = "linux" },
      macos_platform: { target_os = "macos" },
  }

  napi_build::setup();
}
