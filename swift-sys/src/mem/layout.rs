/// The memory layout of a type, describing its size, stride, and alignment.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryLayout {
    /// The type's size.
    pub size: usize,

    /// The type's stride.
    pub stride: usize,

    /// The type's alignment.
    pub align: usize,
}
