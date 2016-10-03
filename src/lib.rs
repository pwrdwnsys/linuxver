extern crate nix;
extern crate semver;

use nix::sys::utsname::*;
use semver::Version;


/// Return a Result with a SemVer representation of the running Linux Kernel version.
pub fn version() -> Result<semver::Version, String> {

    let sysinfo = uname();

    match Version::parse(sysinfo.release()) {
        Ok(ver) => return Ok(ver),
        Err(err) => return Err(err.to_string())
    };


}
