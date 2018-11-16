// Bitcoin secp256k1 bindings
// Written in 2014 by
//   Dawid Ciężarkiewicz
//   Andrew Poelstra
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

// This is a macro that routinely comes in handy
macro_rules! impl_array_newtype {
    ($thing:ident, $ty:ty, $len:expr) => {
        impl Copy for $thing {}

        impl $thing {
            #[inline]
            /// Converts the object to a raw pointer for FFI interfacing
            pub fn as_ptr(&self) -> *const $ty {
                let &$thing(ref dat) = self;
                dat.as_ptr()
            }

            #[inline]
            /// Converts the object to a mutable raw pointer for FFI interfacing
            pub fn as_mut_ptr(&mut self) -> *mut $ty {
                let &mut $thing(ref mut dat) = self;
                dat.as_mut_ptr()
            }

            #[inline]
            /// Returns the length of the object as an array
            pub fn len(&self) -> usize { $len }

            #[inline]
            /// Returns whether the object as an array is empty
            pub fn is_empty(&self) -> bool { false }
        }

        impl PartialEq for $thing {
            #[inline]
            fn eq(&self, other: &$thing) -> bool {
                &self[..] == &other[..]
            }
        }

        impl Eq for $thing {}

        impl Clone for $thing {
            #[inline]
            fn clone(&self) -> $thing {
                unsafe {
                    cfg_if! {
                        if #[cfg(feature = "std")] {
                            use std::intrinsics::copy_nonoverlapping;
                            use std::mem;
                        } else {
                            use core::intrinsics::copy_nonoverlapping;
                            use core::mem;
                        }
                    }
                    let mut ret: $thing = mem::uninitialized();
                    copy_nonoverlapping(self.as_ptr(),
                                        ret.as_mut_ptr(),
                                        mem::size_of::<$thing>());
                    ret
                }
            }
        }

        impl ::ops::Index<usize> for $thing {
            type Output = $ty;

            #[inline]
            fn index(&self, index: usize) -> &$ty {
                let &$thing(ref dat) = self;
                &dat[index]
            }
        }

        impl ::ops::Index<::ops::Range<usize>> for $thing {
            type Output = [$ty];

            #[inline]
            fn index(&self, index: ::ops::Range<usize>) -> &[$ty] {
                let &$thing(ref dat) = self;
                &dat[index]
            }
        }

        impl ::ops::Index<::ops::RangeTo<usize>> for $thing {
            type Output = [$ty];

            #[inline]
            fn index(&self, index: ::ops::RangeTo<usize>) -> &[$ty] {
                let &$thing(ref dat) = self;
                &dat[index]
            }
        }

        impl ::ops::Index<::ops::RangeFrom<usize>> for $thing {
            type Output = [$ty];

            #[inline]
            fn index(&self, index: ::ops::RangeFrom<usize>) -> &[$ty] {
                let &$thing(ref dat) = self;
                &dat[index]
            }
        }

        impl ::ops::Index<::ops::RangeFull> for $thing {
            type Output = [$ty];

            #[inline]
            fn index(&self, _: ::ops::RangeFull) -> &[$ty] {
                let &$thing(ref dat) = self;
                &dat[..]
            }
        }
    }
}

macro_rules! impl_pretty_debug {
    ($thing:ident) => {
        impl ::fmt::Debug for $thing {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                try!(write!(f, "{}(", stringify!($thing)));
                for i in self[..].iter().cloned() {
                    try!(write!(f, "{:02x}", i));
                }
                write!(f, ")")
            }
        }
     }
}

macro_rules! impl_raw_debug {
    ($thing:ident) => {
        impl ::fmt::Debug for $thing {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                for i in self[..].iter().cloned() {
                    try!(write!(f, "{:02x}", i));
                }
                Ok(())
            }
        }
     }
}
