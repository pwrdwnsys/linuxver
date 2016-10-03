extern crate linuxver;
extern crate semver;

// use linuxver;
use semver::Version;

const SECCOMP_MIN_KERNEL_VERSION: &'static str = "2.6.12";

fn main() {

    println!("[+] Checking Linux kernel version...");

    let ver = linuxver::version().unwrap();

    println!("[+] System is running Linux kernel: {}.{}.{}", ver.major, ver.minor, ver.patch);

    if ver >= Version::parse(SECCOMP_MIN_KERNEL_VERSION).unwrap() {
        println!("[+] Your kernel version should support SECCOMP.");
    } else {
        println!("[+] Your kernel version does not support SECCOMP");
    };


}
