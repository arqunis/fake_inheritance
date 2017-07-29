//! A little utility macro for a "faked" inheritance.
//! It generates shortcuts from the inner struct's fields to the parent struct,
//! to make the access of these fields easier to use. But with minimal effort.
#![no_std]

/// Input of this macro follows: (parent_struct_name, field_name_for_inner_struct, fields = [
/// field_name: field_type; ...]) (The ... indicates that the pattern can be used infinitely).
/// For the macro to use the inner struct's accesor functions (e. g `inner.a()`); use
/// "fields = [f(field_name): field_type; ...]" 
///
/// # Examples
/// ```rust,ignore
/// #[macro_use]
/// extern crate fake_inheritance;
/// 
/// struct Inner {
///    a: i32, b: i32
/// }
///
/// struct Parent { inner: Inner }
///
/// fake_inheritance!(Parent, inner, fields = [a: i32; b: i32;]);
///
/// let parent = Parent { inner: Inner { a: 1, b: 2 }}; 
/// assert_eq!(parent.a(), 1);
/// assert_eq!(parent.b(), 2);
/// ```
#[macro_export]
macro_rules! fake_inheritance {
    ($sname:ident, $fname:ident, fields = [$($ffname:ident:$return_val:ident;)*]) => {
        impl $sname {
            $(
                pub fn $ffname(&self) -> $return_val {
                    self.$fname.$ffname
                }
            )*
        }
    };
    ($sname:ident, $fname:ident, fields = [$(f($ffname:ident):$return_val:ident;)*]) => {
        impl $sname {
            $(
                pub fn $ffname(&self) -> $return_val {
                    self.$fname.$ffname()
                }
            )*
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_the_macro_1() {
        struct FakeInheritance {
            a: i32,
            b: i32,
        }
        struct A {
            fi: FakeInheritance,
        }
        
        fake_inheritance!(A, fi, fields = [a: i32; b: i32;]);
        let a = A { fi: FakeInheritance { a: 42, b: 42 }};
        assert_eq!(a.a(), 42);
        assert_eq!(a.b(), 42);
    }
    #[test]
    fn test_the_macro_2() {
        struct FakeInheritance {
            a: i32,
            b: i32,
        }
        impl FakeInheritance {
            fn a(&self) -> i32 {
                self.a
            }
            fn b(&self) -> i32 {
                self.b
            }
        }
        struct A {
            fi: FakeInheritance,
        }
        
        fake_inheritance!(A, fi, fields = [f(a): i32; f(b): i32;]);
        let a = A { fi: FakeInheritance { a: 42, b: 42 }};
        assert_eq!(a.a(), 42);
        assert_eq!(a.b(), 42);
    }
}