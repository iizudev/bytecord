//! This module provies [`ByteCordBuilder`].

/// ByteCordBuilder
pub struct ByteCordBuilder {
    inner: Vec<u8>,
    alignment: usize,
}

impl ByteCordBuilder {
    /// Returns a new builder with specified capacity.
    #[inline]
    pub fn with_capacity(capacity: usize, alignment: usize) -> Self {
        assert!(
            alignment.is_power_of_two() || alignment == 1,
            "alignment must be either a power of two or 1"
        );
        Self {
            inner: Vec::with_capacity(capacity),
            alignment,
        }
    }

    /// Returns a new builder.
    #[inline]
    pub fn new(alignment: usize) -> Self {
        Self::with_capacity(0, alignment)
    }

    /// Appends contents of a slice to this builder, padding with zeros
    /// to maintain alignment.
    pub fn append_from_slice(&mut self, slice: &[u8]) {
        self.inner.extend_from_slice(slice);
        let padding = self.inner.len().next_multiple_of(self.alignment) - self.inner.len();
        if padding > 0 {
            self.inner.resize(self.inner.len() + padding, 0);
        }
    }

    /// Appends `bytes` to this builder.
    #[inline]
    pub fn append<const S: usize>(&mut self, bytes: &[u8; S]) {
        self.append_from_slice(&bytes[..]);
    }

    /// Coverts this builder into [`Box<[u8]>`].
    #[inline]
    pub fn into_boxed_slice(self) -> Box<[u8]> {
        self.inner.into_boxed_slice()
    }
}

impl ByteCordBuilder {
    #[inline]
    #[allow(missing_docs)]
    pub fn append_u8(&mut self, value: u8) {
        self.append(&[value]);
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn append_i8(&mut self, value: i8) {
        self.append(&[value as u8]);
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn append_be_u16(&mut self, value: u16) {
        self.append(&value.to_be_bytes());
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn append_le_u16(&mut self, value: u16) {
        self.append(&value.to_le_bytes());
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn append_be_u32(&mut self, value: u32) {
        self.append(&value.to_be_bytes());
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn append_le_u32(&mut self, value: u32) {
        self.append(&value.to_le_bytes());
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn append_be_u64(&mut self, value: u64) {
        self.append(&value.to_be_bytes());
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn append_le_u64(&mut self, value: u64) {
        self.append(&value.to_le_bytes());
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn append_be_u128(&mut self, value: u128) {
        self.append(&value.to_be_bytes());
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn append_le_u128(&mut self, value: u128) {
        self.append(&value.to_le_bytes());
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn append_be_i16(&mut self, value: i16) {
        self.append(&value.to_be_bytes());
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn append_le_i16(&mut self, value: i16) {
        self.append(&value.to_le_bytes());
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn append_be_i32(&mut self, value: i32) {
        self.append(&value.to_be_bytes());
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn append_le_i32(&mut self, value: i32) {
        self.append(&value.to_le_bytes());
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn append_be_i64(&mut self, value: i64) {
        self.append(&value.to_be_bytes());
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn append_le_i64(&mut self, value: i64) {
        self.append(&value.to_le_bytes());
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn append_be_i128(&mut self, value: i128) {
        self.append(&value.to_be_bytes());
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn append_le_i128(&mut self, value: i128) {
        self.append(&value.to_le_bytes());
    }
}
