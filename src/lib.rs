use std::{
    any::{Any, TypeId},
    marker::PhantomData,
    mem,
};

/// Any type that can be downcast to a specific lifetime.
///
/// TODO: I'm pretty sure this can be automatically derived
///
/// # SAFETY
///
/// The associated type `LoweredType` be the same as `Self`, but shorten all
/// lifetime parameters to `'a`. `'static` references that are required to be
/// static for ALL subtypes of this type do not need to be downcasted.
pub unsafe trait NonStaticType<'a> {
    /// A version of this type where all lifetime references have been replaced with `'a`
    type LoweredType: 'a;
}
// TODO: provide implementations for useful std types
unsafe impl<'a> NonStaticType<'a> for &str {
    type LoweredType = &'a str;
}

pub fn non_static<'a, T: NonStaticType<'a> + 'a>(val: &'a T) -> &'a T::LoweredType {
    // SAFETY: this is safe, since it's only shortening lifetimes
    unsafe { std::mem::transmute(val) }
}

fn non_static_type_id<T: ?Sized>() -> TypeId {
    struct HiddenType<T: ?Sized>(PhantomData<T>);
    trait NonStaticAny {
        fn get_type_id(&self) -> TypeId
        where
            Self: 'static;
    }

    impl<T: ?Sized> NonStaticAny for HiddenType<T> {
        fn get_type_id(&self) -> TypeId
        where
            Self: 'static,
        {
            // Note: ensures `non_static_typeid` of `T` != TypeId::of::<T>
            // This should help prevent mis-use, since we don't actually want
            // them to be compatible
            TypeId::of::<Self>()
        }
    }

    let phantom_data = HiddenType::<T>(PhantomData);
    // SAFETY: phantom_data is a ZST, and therefore contains no data. It is therefore
    // safe to cast the lifetime of inner state, no such state actually exists
    NonStaticAny::get_type_id(unsafe {
        mem::transmute::<&dyn NonStaticAny, &(dyn NonStaticAny + 'static)>(&phantom_data)
    })
}

/// Much like `Any`, but implemented for non-`'static` types.
pub trait NonStaticAny {
    /// Fetch a non-static type id. Note that the returned TypeId is incompatible with
    /// the `std::any::TypeId::of::<Self>()`.
    fn non_static_typeid(&self) -> TypeId;
}
impl<T: ?Sized> NonStaticAny for T {
    fn non_static_typeid(&self) -> TypeId {
        non_static_type_id::<Self>()
    }
}

pub fn downcast_non_static<'a, T: NonStaticType<'a> + NonStaticAny + 'a>(
    val: &'a dyn NonStaticAny,
) -> Option<&'a T::LoweredType> {
    // SAFETY: this is safe, since it shortens lifetimes, and we have verified
    // that is is otherwise the same type.
    if val.non_static_typeid() == non_static_type_id::<T>() {
        Some(unsafe { &*(val as *const dyn NonStaticAny).cast() })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct WithLife<'a> {
        v: &'a str,
    }

    /// Compile test, to demonstrate that a time is always safe to downcast lifetimes
    fn downcast<'a, 'b>(val: &'a WithLife<'b>) -> &'a WithLife<'a>
    where
        WithLife<'b>: 'a,
    {
        val
    }

    #[test]
    fn it_works() {
        
    }
}
