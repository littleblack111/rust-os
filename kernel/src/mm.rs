use core::{ops::Deref, sync::atomic::Ordering};

use atomic::Atomic;
use bytemuck::{Pod, Zeroable};

static KERNEL_VIRTUAL_BASE: Atomic<PhysicalMemoryAddress> = Atomic::new(PhysicalMemoryAddress(0));

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(transparent)]
pub struct VirtualMemoryAddress(pub usize);

impl Deref for VirtualMemoryAddress {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<usize> for VirtualMemoryAddress {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<VirtualMemoryAddress> for usize {
    fn from(value: VirtualMemoryAddress) -> Self {
        *value
    }
}

impl From<PhysicalMemoryAddress> for VirtualMemoryAddress {
    fn from(value: PhysicalMemoryAddress) -> Self {
        VirtualMemoryAddress(*value + *KERNEL_VIRTUAL_BASE.load(Ordering::Relaxed))
    }
}

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(transparent)]
pub struct PhysicalMemoryAddress(pub usize);

impl Deref for PhysicalMemoryAddress {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<usize> for PhysicalMemoryAddress {
    fn from(value: usize) -> Self {
        PhysicalMemoryAddress(value)
    }
}

impl From<PhysicalMemoryAddress> for usize {
    fn from(value: PhysicalMemoryAddress) -> Self {
        *value
    }
}

impl From<VirtualMemoryAddress> for PhysicalMemoryAddress {
    fn from(value: VirtualMemoryAddress) -> Self {
        PhysicalMemoryAddress(*value - *KERNEL_VIRTUAL_BASE.load(Ordering::Relaxed))
    }
}

#[derive(Clone, Copy)]
pub enum MemoryAddress {
    Virtual(VirtualMemoryAddress),
    Physical(PhysicalMemoryAddress),
}

impl From<VirtualMemoryAddress> for MemoryAddress {
    fn from(value: VirtualMemoryAddress) -> Self {
        MemoryAddress::Virtual(value)
    }
}

impl From<PhysicalMemoryAddress> for MemoryAddress {
    fn from(value: PhysicalMemoryAddress) -> Self {
        MemoryAddress::Physical(value)
    }
}

impl From<MemoryAddress> for usize {
    fn from(value: MemoryAddress) -> Self {
        match value {
            MemoryAddress::Virtual(v) => *v,
            MemoryAddress::Physical(p) => *p,
        }
    }
}

impl From<MemoryAddress> for VirtualMemoryAddress {
    fn from(value: MemoryAddress) -> Self {
        match value {
            MemoryAddress::Virtual(v) => v,
            MemoryAddress::Physical(p) => p.into(),
        }
    }
}

impl From<MemoryAddress> for PhysicalMemoryAddress {
    fn from(value: MemoryAddress) -> Self {
        match value {
            MemoryAddress::Virtual(v) => v.into(),
            MemoryAddress::Physical(p) => p,
        }
    }
}
