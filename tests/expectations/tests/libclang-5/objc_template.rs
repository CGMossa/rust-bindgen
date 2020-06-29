#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]
#![cfg(target_os = "macos")]

#[macro_use]
extern crate objc;
#[allow(non_camel_case_types)]
pub type id = *mut objc::runtime::Object;
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Foo(pub id);
impl std::ops::Deref for Foo {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for Foo {}
impl Foo {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(Foo), alloc) })
    }
}
impl<ObjectType: 'static> IFoo<ObjectType> for Foo {}
pub trait IFoo<ObjectType>: Sized + std::ops::Deref {
    unsafe fn get(self) -> *mut ObjectType
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(self, get)
    }
}
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct FooMultiGeneric(pub id);
impl std::ops::Deref for FooMultiGeneric {
    type Target = objc::runtime::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl objc::Message for FooMultiGeneric {}
impl FooMultiGeneric {
    pub fn alloc() -> Self {
        Self(unsafe { msg_send!(objc::class!(FooMultiGeneric), alloc) })
    }
}
impl<KeyType: 'static, ObjectType: 'static>
    IFooMultiGeneric<KeyType, ObjectType> for FooMultiGeneric
{
}
pub trait IFooMultiGeneric<KeyType, ObjectType>:
    Sized + std::ops::Deref
{
    unsafe fn objectForKey_(self, key: *mut KeyType) -> *mut ObjectType
    where
        <Self as std::ops::Deref>::Target: objc::Message + Sized,
    {
        msg_send!(self, objectForKey: key)
    }
}