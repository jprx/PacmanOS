// Everything related to virtualization under Qemu goes here
// This file should be the only location (besides very early bringup) where
// any virtualization-related hacks are present as I iron out the kinks in modding Qemu
// to act just like real iBoot-compliant hardware

// What mode are we currently running in?
#[derive(Copy,Clone,Debug)]
pub enum VirtMode {
    Baremetal,
    Qemu,
}

pub static mut CurrentMode : VirtMode = VirtMode::Baremetal;
