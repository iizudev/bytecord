//! This module provies [`ByteCordReader`].

use crate::ByteCord;

/// Reader of an aligned [`ByteCord`].
///
/// # Examples
/// ```
/// # use bytecord::{ByteCord, ByteCordReader};
///
/// let data = vec![0u8; 100];
/// let cord = ByteCord::new(data);
/// let mut reader = ByteCordReader::with_alignment(&cord, 4);
///
/// // Read 10 bytes (position 0 → 10, aligned to 12)
/// let slice = reader.next_n(10);
///
/// // Read a u32 (position 12 → 16)
/// let num = reader.next::<4>();
/// ```
pub struct ByteCordReader<'a, T> {
    cord: &'a ByteCord<T>,
    alignment: usize,
    position: usize,
}

impl<'a, T> ByteCordReader<'a, T> {
    /// Returns a new reader with specified alignment.
    ///
    /// ## Panics
    ///
    /// Panics if alignment is 0 or not a power of two
    #[inline]
    pub fn with_alignment(cord: &'a ByteCord<T>, alignment: usize) -> Self {
        assert!(
            alignment.is_power_of_two() || alignment == 1,
            "alignment must be either a power of two or 1"
        );

        ByteCordReader {
            cord,
            alignment,
            position: 0,
        }
    }

    /// Returns a new reader with default 1-byte alignment (no alignment
    /// constraints).
    #[inline]
    pub fn new(cord: &'a ByteCord<T>) -> Self {
        Self::with_alignment(cord, 1)
    }
}

impl<'a, T: AsRef<[u8]>> ByteCordReader<'a, T> {
    /// Returns `length` bytes at its currect position and advances the
    /// position to next aligned offset, or None if out of bounds.
    #[inline]
    pub fn next_n(&mut self, length: usize) -> Option<&'a [u8]> {
        if let Some(result) = self.cord.at_n(self.position, length) {
            self.position = (self.position + length).next_multiple_of(self.alignment);
            Some(result)
        } else {
            None
        }
    }

    /// Returns an array of size S at its current position and advances the
    /// position to next aligned offset or `None` if out of bounds.
    ///
    /// # Safety
    ///
    /// The unsafe conversion is safe because:
    /// 1. Bounds are checked by at_n()
    /// 2. The slice length is guaranteed to be S
    #[inline]
    #[allow(clippy::should_implement_trait)]
    pub fn next<const S: usize>(&mut self) -> Option<&'a [u8; S]> {
        self.next_n(S)
            .map(|slice| unsafe { &*(slice.as_ptr() as *const [u8; S]) })
    }

    /// Advances the position by `length` bytes, aligning the final position.
    ///
    /// Returns `true` if the skip was successful (enough bytes remaining).
    #[inline]
    pub fn skip(&mut self, length: usize) -> bool {
        self.next_n(length).is_some()
    }

    /// Returns count of unread bytes.
    #[inline]
    pub fn remaining(&self) -> usize {
        self.cord.len().saturating_sub(self.position)
    }
}

impl<T: AsRef<[u8]>> ByteCordReader<'_, T> {
    #[inline]
    #[allow(missing_docs)]
    pub fn next_u8(&mut self) -> Option<u8> {
        self.next::<1>().map(|arr| arr[0])
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn next_i8(&mut self) -> Option<i8> {
        self.next::<1>().map(|arr| arr[0] as i8)
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn next_be_u16(&mut self) -> Option<u16> {
        self.next::<2>().map(|arr| u16::from_be_bytes(*arr))
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn next_le_u16(&mut self) -> Option<u16> {
        self.next::<2>().map(|arr| u16::from_le_bytes(*arr))
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn next_be_u32(&mut self) -> Option<u32> {
        self.next::<4>().map(|arr| u32::from_be_bytes(*arr))
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn next_le_u32(&mut self) -> Option<u32> {
        self.next::<4>().map(|arr| u32::from_le_bytes(*arr))
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn next_be_u64(&mut self) -> Option<u64> {
        self.next::<8>().map(|arr| u64::from_be_bytes(*arr))
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn next_le_u64(&mut self) -> Option<u64> {
        self.next::<8>().map(|arr| u64::from_le_bytes(*arr))
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn next_be_u128(&mut self) -> Option<u128> {
        self.next::<16>().map(|arr| u128::from_be_bytes(*arr))
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn next_le_u128(&mut self) -> Option<u128> {
        self.next::<16>().map(|arr| u128::from_le_bytes(*arr))
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn next_be_i16(&mut self) -> Option<i16> {
        self.next::<2>().map(|arr| i16::from_be_bytes(*arr))
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn next_le_i16(&mut self) -> Option<i16> {
        self.next::<2>().map(|arr| i16::from_le_bytes(*arr))
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn next_be_i32(&mut self) -> Option<i32> {
        self.next::<4>().map(|arr| i32::from_be_bytes(*arr))
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn next_le_i32(&mut self) -> Option<i32> {
        self.next::<4>().map(|arr| i32::from_le_bytes(*arr))
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn next_be_i64(&mut self) -> Option<i64> {
        self.next::<8>().map(|arr| i64::from_be_bytes(*arr))
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn next_le_i64(&mut self) -> Option<i64> {
        self.next::<8>().map(|arr| i64::from_le_bytes(*arr))
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn next_be_i128(&mut self) -> Option<i128> {
        self.next::<16>().map(|arr| i128::from_be_bytes(*arr))
    }

    #[inline]
    #[allow(missing_docs)]
    pub fn next_le_i128(&mut self) -> Option<i128> {
        self.next::<16>().map(|arr| i128::from_le_bytes(*arr))
    }
}
