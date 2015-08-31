/// Define typed accessor methods for a C-union like structure.
/// TODO document, see src/d3d11/d3d11/structs.rs for usage examples.
macro_rules! union {
    ($name:ident.$field:ident {
        $(
            $(#[$getter_attr:meta])*
            fn $getter:ident() -> $rt:ty,
            $(#[$setter_attr:meta])*
            fn $setter:ident($param:ident: $param_ty:ty)
        ),+
    }) => (
        impl $name {
            $(
            $(#[$getter_attr])*
            pub fn $getter(&self) -> $rt {
                let x : &$rt = unsafe { ::std::mem::transmute(&self.$field) };
                *x
            }
            $(#[$setter_attr])*
            pub fn $setter(&mut self, $param:$param_ty) {
                let x: &mut $param_ty = unsafe {
                    ::std::mem::transmute(&mut self.$field)
                };
                *x = $param;
            }
            )+
        }
    )
}

/// Define typed accessor methods for a bitfield structure
/// TODO document how to use this, quite messy and very likely to change as
///      it is used more often.
macro_rules! bitfields {
    ($struct_id:ident.$member:ident: $member_ty:ty {
        $($field:tt),+
    }) => (
        impl $struct_id {
            $(
                __bitfields_field!(
                    $member, $member_ty,
                    $field
                );
            )+
        }
    )
}

macro_rules! __bitfields_field {
    // Store all bools as u8 because !bool isn't a bitwise operation
    // e.g. (0, 1 => bool, foo, set_foo)
    (
        $member_id:ident, $member_ty:ty,
        ($offset:expr, $size:expr => bool, $get:ident, $set:ident)
    )  => (
        __bitfields_field!(
            $member_id, $member_ty,
            ($offset, $size => bool as u8, $get, $set)
        );
    );

    // Store struct in field, this uses transmute to convert so the struct
    // must be the same size as the bitfield.
    // e.g. (0, 4 => struct Foo, foo, set_foo)
    (
        $member_id:ident, $member_ty:ty,
        ($offset:expr, $size:expr => struct $ty:ty, $get:ident, $set:ident)
    ) => (
        pub fn $get(&self) -> $ty {
            let mask = (1 << $size) - 1;
            let x = (self.$member_id & (mask << $offset)) >> $offset;
            unsafe { ::std::mem::transmute(x) }
        }
        pub fn $set(&mut self, value: $ty) {
            let mask = (1 << $size) - 1;
            let x: $member_ty = unsafe { ::std::mem::transmute(value) };
            self.$member_id = (self.$member_id & !(mask << $offset)) |
                              ((x & mask) << $offset);
        }
    );

    // Use machine types directly, should be fine as they convert easily
    // e.g. (0, 4 => u8, foo, set_foo)
    (
        $member_id:ident, $member_ty:ty,
        ($offset:expr, $size:expr => $ty:ty, $get:ident, $set:ident)
    )  => (
        __bitfields_field!(
            $member_id, $member_ty,
            ($offset, $size => $ty as $ty, $get, $set)
        );
    );

    // Store type as specified, only works for machine types.
    // e.g. (0, 4 => u32 as u8, foo, set_foo)
    (
        $member_id:ident, $member_ty:ty,
        ($offset:expr, $size:expr => $ty:ty as $ty2:ty, $get:ident, $set:ident)
    )  => (
        pub fn $get(&self) -> $ty {
            let mask = (1 << $size) - 1;
            let x = (self.$member_id & (mask << $offset)) >> $offset;
            unsafe { ::std::mem::transmute(x as $ty2) }
        }
        pub fn $set(&mut self, value: $ty) {
            let mask = (1 << $size) - 1;
            self.$member_id = (self.$member_id & !(mask << $offset)) |
                               ((value as $member_ty & mask) << $offset);
        }
    )
}

macro_rules! guid {
    ($(#[$iid_attr:meta])*
    $name:ident = $d1:expr, $d2:expr, $d3:expr, $($d4:expr),*) => (
        $(#[$iid_attr])*
        pub const $name: ::winapi::GUID = ::winapi::GUID {
            Data1: $d1,
            Data2: $d2,
            Data3: $d3,
            Data4: [$($d4),*],
        };
    )
}
