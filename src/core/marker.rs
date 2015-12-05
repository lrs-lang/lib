// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ops::{Eq};

/// Objects whose size known at compile time.
///
/// = Remarks
///
/// This includes `&[T]` and `&Trait` but not `[T]` or `Trait`.
#[lang = "sized"]
#[fundamental]
pub trait Sized { }

impl Sized for .. { }

/// Objects that are safe to use if they contain a random bit pattern.
///
/// = Remarks
///
/// That is, types without invariants. For example, immutable slices are not `Pod` for the
/// same reason `slice::from_ptr` is not safe. Note that only structs and primitives can
/// be `Pod`.
#[fundamental]
pub trait Pod : Copy { }

/// Objects that can safely be copied via `memcpy`.
///
/// = Remarks
///
/// That is, objects which you can copy and use both the copy and the original. For
/// example, immutable slices are `Copy`. This is a weaker form of `Pod`.
#[lang = "copy"]
#[fundamental]
pub trait Copy { }

/// Objects that allow immutable access from threads other than their owning thread.
///
/// = Remarks
///
/// For example, `RefCell` is `!Sync`.
#[lang = "sync"]
#[rustc_on_unimplemented = "`{Self}` cannot be shared safely between threads"]
pub unsafe trait Sync: Interrupt { }

unsafe impl Sync for .. { }

impl<T> !Sync for *const T { }
impl<T> !Sync for *mut T { }

/// A dummy object that is `!Sync`.
///
/// = Remarks
///
/// This can be embedded in other objects to make them `!Sync`.
pub struct NoSync;

impl !Sync for NoSync { }

/// Objects that allow immutable access from signal handlers.
///
/// = Remarks
///
/// For example, `RefCell` is `!Interrupt`.
#[rustc_on_unimplemented = "`{Self}` cannot be used safely in signal handlers"]
pub unsafe trait Interrupt { }

unsafe impl Interrupt for .. { }
unsafe impl<T: Sync> Interrupt for T { }

impl<T> !Interrupt for *const T { }
impl<T> !Interrupt for *mut T { }

/// A dummy object that is `!Interrupt`.
///
/// = Remarks
///
/// This can be embedded in other objects to make them `!Interrupt`.
pub struct NoInterrupt;

impl !Interrupt for NoInterrupt { }
impl !Sync for NoInterrupt { }

/// Objects whose ownership can be moved from one thread to another.
///
/// = Remarks
///
/// For example, types using a thread-local allocator are often `Sync` but never `Send`
/// because they must be destroyed in the thread they were created in. 
pub unsafe trait Send { }

unsafe impl Send for .. { }

impl<T> !Send for *const T { }
impl<T> !Send for *mut T { }

/// A dummy object that is `!Send`
///
/// = Remarks
///
/// This can be embedded in other objects to make them `!Send`.
pub struct NoSend;

impl !Send for NoSend { }

/// Objects that can be leaked without causing memory unsafety.
///
/// = Remarks
///
/// In normal, safe code, the compiler inserts calls to destructors at the end of object's
/// lifetimes, e.g,
///
/// ----
/// {
///     let x = X;
/// }
/// ----
///
/// inserts a call to the `X` destructor at the end of the block. However, the compiler
/// cannot do this if it doesn't know the lifetime of an object. This happens when you use
/// raw pointers which opt out of lifetimes. In those cases it is the job of the
/// programmer to insert destructor calls. For example, a vector looks roughly like this:
///
/// ----
/// struct Vec<T> {
///     ptr: *mut T,
///     len: usize,
///     cap: usize,
/// }
/// ----
///
/// Since the `T`s contained in the vector are behind a raw pointer, the compiler does not
/// destroy them automatically at the end of their lifetime. The author of the `Vec`
/// structure has to manually call the destructor for all of its `T` objects in the
/// destructor of `Vec`:
///
/// ----
/// fn drop(&mut self) {
///     for i in 0..self.len {
///         unsafe {
///             // Bring the contained object back from behind the pointer so that it's
///             // once again managed by the compiler which will call the destructor of
///             // `_t` at the end of the `unsafe` block.
///             let _t = ptr::read(self.ptr.add(i));
///         }
///     }
///     // Deallocate `self.ptr` here
///     // ...
/// }
/// ----
///
/// In the case of vectors this is very easy: Since the lifetime of the vector is always
/// shorter than the lifetime of the contained objects, we can rely on the `drop` code
/// above running before the end of the lifetimes of the `T`.
///
/// There are, however, some data structures for which this is not easy. For example, the
/// destructor of `Rc` looks like this:
///
/// ----
/// fn drop(&mut self) {
///     self.number_of_references -= 1;
///     if self.number_of_references == 0 {
///         // Call the destructor of the contained object
///     }
/// }
/// ----
///
/// As you can see, we don't always call the destructor of the contained object at the end
/// of the lifetime of the `Rc`. At first it seems that this is no problem since, in order
/// to have `number_of_references > 1` you have to have cloned the `Rc` and all of the
/// clones are bound by the lifetime of `T`. Once the last cloned `Rc` is dropped, it
/// should call the destructor of the `T`.
///
/// But this doesn't account for the possibility of cycles. If we put a clone of the `Rc`
/// into the `Rc` itself, then the destructor that destroys the last `Rc` clone will never
/// run and thus the `T` will never be destroyed. Here is an example that generates such a
/// cycle:
///
/// ----
/// struct T;
/// 
/// impl Drop for T {
///     fn drop(&mut self) {
///         println!("dropped");
///     }
/// }
/// 
/// struct X<T> {
///     _t: T,
///     rc: Option<Rc<RefCell<X<T>>>>,
/// }
/// 
/// fn main() {
///     let rc = Rc::new(RefCell::new(X { _t: T, rc: None }));
///     rc.borrow_mut().rc = Some(rc.clone());
/// }
/// ----
///
/// You will notice that the `dropped` message will never be printed even though the end
/// of the lifetime of `T` is reached at the end of the `main` block.
///
/// Most of the time this is not a problem, even if destructors don't run, this cannot
/// cause memory unsafety. However, the safety of some structures depends on the guarantee
/// that destructors run at the end of object's lifetimes. For example, the `JoinGuard`
/// returned by `thread::scoped` must have its destructor run at the end of its lifetime
/// or the behavior is undefined.
///
/// For this reason we introduce the `Leak` trait which marks objects that don't need to
/// have their destructor run at the end of their lifetime. By default, every object is
/// `Leak`. If your object contains an object that is `!Leak`, then it's automatically
/// `!Leak` itself but you can opt into `Leak` by implementing `Leak` explicitly. If you
/// want to explicitly opt out of `Leak`, then you have to implement `!Leak` for your
/// object. For example, `JoinGuard` explicitly implements `!Leak`.
///
/// If you create a new container such as `Vec` or `Rc` which owns objects behind raw
/// pointers and if you cannot guarantee that the object's destructors will be run at the
/// end of their lifetimes, then you have to add the `Leak` bound to your trait bounds.
/// For example, the `Rc` type can be defined as follows:
///
/// ----
/// struct Rc<T: Leak> {
///     data: *mut Inner<T>,
/// }
/// ----
/// 
/// where `Inner<T>` is an allocated object that contains the `T` and the reference count.
///
/// The simple criterion is as follows:
///
/// **If the destructor of your object calls the destructor of the contained object only
/// if a certain condition is met, add the `Leak` bound.**
pub unsafe trait Leak { }

unsafe impl Leak for .. { }

unsafe impl<'a, T: ?Sized> Leak for &'a T { }
unsafe impl<'a, T: ?Sized> Leak for &'a mut T { }

/// A dummy type for unused type parameters.
///
/// = Remarks
///
/// Normally, all type parameters have to be used in some way. If a type has type
/// parameters that are not used directly in the type definition, it has to use this
/// object instead:
///
/// ----
/// struct X<T> {
///     _dummy: PhantomData<T>,
/// }
/// ----
#[lang = "phantom_data"]
pub struct PhantomData<T: ?Sized>;

impl<T: ?Sized> Copy for PhantomData<T> { }
impl<T: ?Sized> Eq for PhantomData<T> { fn eq(&self, _: &PhantomData<T>) -> bool { true } }

/// Objects that can be converted to an unsized type.
#[lang="unsize"]
pub trait Unsize<T: ?Sized> { }
