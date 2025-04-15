//! # bytecord
//!
//! Provides zero-copy access to byte data with guaranteed alignment and bounds checking.
//! Designed for parsing binary formats, network protocols, and memory-mapped I/O.
//!
//! ## Features
//! - Alignment-aware operations (`align=1|2|4|8|16|...`)
//! - Bounds-checked access
//! - Zero-copy views into data
//! - Supports both owned and borrowed buffers
//!
//! # Examples
//!
//! reading:
//! ```
//! use bytecord::{ByteCord, ByteCordReader};
//!
//! let data = vec![0u8; 1024];
//! let cord = ByteCord::new(data);
//!
//! // Read with 4-byte alignment
//! let mut reader = cord.read_with_alignment(4);
//! let header = reader.next_n(16).unwrap();
//! ```
//!
//! building:
//! ```
//! use bytecord::ByteCordBuilder;
//!
//! // a new builer with alignment of 1 byte (meaning no alignment)
//! let mut data = ByteCordBuilder::new(1);
//! data.append_le_u32(1111);
//! data.append_u8(1);
//! data.append_le_i64(-3919);
//! let slice = data.into_boxed_slice();
//! ```
//!
//! ## Safety
//! All operations are bounds-checked. Unsafe code is strictly contained and documented.

#![warn(missing_docs)]

pub mod builder;
pub mod reader;

pub use builder::ByteCordBuilder;
pub use reader::ByteCordReader;

/// ByteCord.
///
/// # Examples
///
/// ```
/// # use bytecord::ByteCord;
///
/// let data = vec![0u8; 1024];
/// let cord = ByteCord::new(&data);
///
/// // returns a reference to four bytes array at 0 position;
/// let num = cord.at::<4>(0);
/// ```
///
/// ```
/// # use bytecord::ByteCord;
///
/// let mut data = vec![0u8; 1024];
/// let mut cord = ByteCord::new(&mut data);
///
/// // returns a mutable reference to four bytes array at 0 position;
/// let num = cord.at_mut::<4>(0);
/// if let Some(mut num) = num {
///     num[0] = 1;
/// }
/// ```
pub struct ByteCord<T> {
    data: T,
}

impl<T> ByteCord<T> {
    /// Returns a new [`ByteCord`] wrapping the provided data.
    pub fn new(data: T) -> Self {
        ByteCord { data }
    }

    /// Returns a new reader with default alignment (1 byte).
    pub fn read(&self) -> ByteCordReader<T> {
        ByteCordReader::new(self)
    }

    /// Returns a new reader with specific alignment.
    ///
    /// # Panics
    ///
    /// Panics if the alignment is not a power of 2 or equal to 0.
    pub fn read_with_alignment(&self, alignment: usize) -> ByteCordReader<T> {
        ByteCordReader::with_alignment(self, alignment)
    }
}

impl<T: AsRef<[u8]>> ByteCord<T> {
    /// Returns a byte slice starting at position with given length
    /// or None if out of bounds.
    pub fn at_n(&self, position: usize, length: usize) -> Option<&[u8]> {
        let data = self.data.as_ref();
        if position + length > data.len() {
            None
        } else {
            Some(&data[position..position + length])
        }
    }

    /// Returns a reference to an array of size S starting at position
    /// or None if out of bounds
    ///
    /// # Safety
    ///
    /// The unsafe block is safe because bounds are checked and the
    /// conversion preserves the original slice's lifetime.
    pub fn at<const S: usize>(&self, position: usize) -> Option<&[u8; S]> {
        self.at_n(position, S)
            .map(|slice| unsafe { &*(slice.as_ptr() as *const [u8; S]) })
    }

    /// Returns length of this cord.
    pub fn len(&self) -> usize {
        self.data.as_ref().len()
    }

    /// Returns `true` if the underlying data is empty.
    pub fn is_empty(&self) -> bool {
        self.data.as_ref().is_empty()
    }
}

impl<T: AsMut<[u8]>> ByteCord<T> {
    /// Returns a mutable byte slice starting at position with given length
    /// or None if out of bounds.
    pub fn at_n_mut(&mut self, position: usize, length: usize) -> Option<&mut [u8]> {
        let data = self.data.as_mut();
        if position + length >= data.len() {
            None
        } else {
            Some(&mut data[position..position + length])
        }
    }

    /// Returns a mutable reference to an array of size S starting at position
    /// or None if out of bounds.
    ///
    /// # Safety
    ///
    /// The unsafe block is safe because bounds are checked and the
    /// conversion preserves the original slice's lifetime.
    pub fn at_mut<const S: usize>(&mut self, position: usize) -> Option<&mut [u8; S]> {
        self.at_n_mut(position, S)
            .map(|slice| unsafe { &mut *(slice.as_mut_ptr() as *mut [u8; S]) })
    }
}
