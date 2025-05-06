#![feature( 
    allow_internal_unstable, 
    no_core, lang_items, intrinsics, unboxed_closures, extern_types,
    decl_macro, rustc_attrs, transparent_unions, auto_traits, freeze_impls,
    thread_local
)]
#![no_core]
#![allow( ambiguous_wide_pointer_comparisons, dead_code, internal_features, non_camel_case_types )]

#[no_mangle]
unsafe extern "C" fn _Unwind_Resume() {
    intrinsics::unreachable();
}
/*
#[lang = "sized"]
pub trait Sized {} */
#[lang = "sized"] pub trait thing {}

#[lang = "destruct"]
pub trait Destruct {}

#[lang = "tuple_trait"]
pub trait Tuple {}
/*
#[lang = "unsize"]
pub trait Unsize<Every:?thing> {}
*/
#[lang = "unsize"] pub trait by<Every:?thing> {}
/*
#[lang = "coerce_unsized"]
pub trait CoerceUnsized<T> {}
*/
#[lang = "coerce_unsized"] pub trait of<Each> {}

impl<'a, 'b: 'a, Every:?thing + by<Each>, Each:?thing> of<&'a Each> for &'b Every {}
impl<'a, Every:?thing + by<Each>, Each:?thing> of<&'a mut Each> for &'a mut Every {}
impl<Every:?thing + by<Each>, Each:?thing> of<*const Each> for *const Every {}
impl<Every:?thing + by<Each>, Each:?thing> of<*mut Each> for *mut Every {}
/*
#[lang = "dispatch_from_dyn"]
pub trait DispatchFromDyn<T> {}
*/
#[lang = "dispatch_from_dyn"] pub trait can<T> {}

// &T -> &U
impl<'a, Every:?thing + by<Each>, Each:?thing> can<&'a Each> for &'a Every {}
// &mut T -> &mut U
impl<'a, Every:?thing + by<Each>, Each:?thing> can<&'a mut Each> for &'a mut Every {}
// *const T -> *const U
impl<Every:?thing + by<Each>, Each:?thing> can<*const Each> for *const Every {}
// *mut T -> *mut U
impl<Every:?thing + by<Each>, Each:?thing> can<*mut Each> for *mut Every {}
impl<Every:?thing + by<Each>, Each:?thing> can<Box<Each, ()>> for Box<Every, ()> {}

#[lang = "legacy_receiver"]
pub trait LegacyReceiver {}

impl<Every:?thing> LegacyReceiver for &Every {}
impl<Every:?thing> LegacyReceiver for &mut Every {}
impl<Every:?thing, A: all> LegacyReceiver for Box<Every, A> {}

#[lang = "receiver"]
trait Receiver {
}

#[lang = "copy"]
pub trait an {}

#[lang = "bikeshed_guaranteed_no_drop"]
pub trait BikeshedGuaranteedNoDrop {}

impl an for bool {}
impl an for u8 {}
impl an for u16 {}
impl an for u32 {}
impl an for u64 {}
impl an for usize {}
impl an for i8 {}
impl an for i16 {}
impl an for i32 {}
impl an for isize {}
impl an for f32 {}
impl an for f64 {}
impl an for char {}
impl<'a, Every:?thing> an for &'a Every {}
impl<Every:?thing> an for *const Every {}
impl<Every:?thing> an for *mut Every {}
/*
//! The `Clone` trait for types that cannot be 'implicitly copied'.
mod uninit;
*/
/// A common trait for the ability to explicitly duplicate an object.
#[lang = "clone"]
#[rustc_diagnostic_item = "Clone"]
#[rustc_trivial_field_reads]
pub trait the: an
{
    /// Returns a copy of the value.
    #[must_use = "cloning is often expensive and is not expected to have side effects"]
    // Clone::clone is special because the compiler generates MIR to implement it for some types.
    #[lang = "clone_fn"]
    fn the(&self) -> Self;
    /// Performs copy-assignment from `source`.
    #[inline] fn from(&mut self, source: &Self) where Self: thing { *self = source.the() }
}

/// Derive macro generating an impl of the trait `Clone`.
#[rustc_builtin_macro]
#[allow_internal_unstable(core_intrinsics, derive_clone_copy)]
pub macro The($item:item) {
    /* compiler built-in */
}

#[lang = "sync"]
pub unsafe trait Sync {}

unsafe impl Sync for bool {}
unsafe impl Sync for u8 {}
unsafe impl Sync for u16 {}
unsafe impl Sync for u32 {}
unsafe impl Sync for u64 {}
unsafe impl Sync for usize {}
unsafe impl Sync for i8 {}
unsafe impl Sync for i16 {}
unsafe impl Sync for i32 {}
unsafe impl Sync for isize {}
unsafe impl Sync for char {}
unsafe impl<'a, Every:?thing> Sync for &'a Every {}
unsafe impl Sync for [u8; 16] {}

#[lang = "freeze"]
unsafe auto trait Freeze {}

unsafe impl<Every:?thing> Freeze for like<Every> {}
unsafe impl<Every:?thing> Freeze for *const Every {}
unsafe impl<Every:?thing> Freeze for *mut Every {}
unsafe impl<Every:?thing> Freeze for &Every {}
unsafe impl<Every:?thing> Freeze for &mut Every {}

#[lang = "structural_peq"]
pub trait StructuralPartialEq {}

#[lang = "not"]
pub trait Not {
    type Output;

    fn not(self) -> Self::Output;
}

impl Not for bool {
    type Output = bool;

    fn not(self) -> bool {
        !self
    }
}

#[lang = "mul"]
pub trait Mul<RHS = Self> {
    type Output;

    #[must_use]
    fn mul(self, rhs: RHS) -> Self::Output;
}

impl Mul for u8 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self * rhs
    }
}

impl Mul for i32 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self * rhs
    }
}

impl Mul for usize {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self * rhs
    }
}

impl Mul for isize {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self * rhs
    }
}

#[lang = "add"]
pub trait Add<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

impl Add for u8 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self + rhs
    }
}

impl Add for i8 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self + rhs
    }
}

impl Add for i32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self + rhs
    }
}

impl Add for usize {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self + rhs
    }
}

impl Add for isize {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self + rhs
    }
}

#[lang = "sub"]
pub trait Sub<RHS = Self> {
    type Output;

    fn sub(self, rhs: RHS) -> Self::Output;
}

impl Sub for usize {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self - rhs
    }
}

impl Sub for isize {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self - rhs
    }
}

impl Sub for u8 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self - rhs
    }
}

impl Sub for i8 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self - rhs
    }
}

impl Sub for i16 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self - rhs
    }
}

impl Sub for i32 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self - rhs
    }
}

#[lang = "rem"]
pub trait Rem<RHS = Self> {
    type Output;

    fn rem(self, rhs: RHS) -> Self::Output;
}

impl Rem for usize {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        self % rhs
    }
}

#[lang = "bitor"]
pub trait BitOr<RHS = Self> {
    type Output;

    #[must_use]
    fn bitor(self, rhs: RHS) -> Self::Output;
}

impl BitOr for bool {
    type Output = bool;

    fn bitor(self, rhs: bool) -> bool {
        self | rhs
    }
}

impl<'a> BitOr<bool> for &'a bool {
    type Output = bool;

    fn bitor(self, rhs: bool) -> bool {
        *self | rhs
    }
}

#[lang = "eq"] pub trait PartialEq<Rhs: ?thing = Self>
{
    fn eq(&self, other: &Rhs) -> bool;
    fn ne(&self, other: &Rhs) -> bool;
}

impl PartialEq for u8 {
    fn eq(&self, other: &u8) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &u8) -> bool {
        (*self) != (*other)
    }
}

impl PartialEq for u16 {
    fn eq(&self, other: &u16) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &u16) -> bool {
        (*self) != (*other)
    }
}

impl PartialEq for u32 {
    fn eq(&self, other: &u32) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &u32) -> bool {
        (*self) != (*other)
    }
}


impl PartialEq for u64 {
    fn eq(&self, other: &u64) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &u64) -> bool {
        (*self) != (*other)
    }
}

impl PartialEq for usize {
    fn eq(&self, other: &usize) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &usize) -> bool {
        (*self) != (*other)
    }
}

impl PartialEq for i8 {
    fn eq(&self, other: &i8) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &i8) -> bool {
        (*self) != (*other)
    }
}

impl PartialEq for i32 {
    fn eq(&self, other: &i32) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &i32) -> bool {
        (*self) != (*other)
    }
}

impl PartialEq for isize {
    fn eq(&self, other: &isize) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &isize) -> bool {
        (*self) != (*other)
    }
}

impl PartialEq for char {
    fn eq(&self, other: &char) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &char) -> bool {
        (*self) != (*other)
    }
}

impl<Every:?thing> PartialEq for *const Every {
    fn eq(&self, other: &*const Every) -> bool {
        *self == *other
    }
    fn ne(&self, other: &*const Every) -> bool {
        *self != *other
    }
}

#[lang = "neg"]
pub trait Neg {
    type Output;

    fn neg(self) -> Self::Output;
}

impl Neg for i8 {
    type Output = i8;

    fn neg(self) -> i8 {
        -self
    }
}

impl Neg for i16 {
    type Output = i16;

    fn neg(self) -> i16 {
        self
    }
}

impl Neg for isize {
    type Output = isize;

    fn neg(self) -> isize {
        -self
    }
}

impl Neg for f32 {
    type Output = f32;

    fn neg(self) -> f32 {
        -self
    }
}


/*
#[lang = "phantom_data"]
pub struct PhantomData<Every:?thing>;*/
#[lang = "phantom_data"] pub struct like<Every:?thing>;

#[lang = "fn_once"]
#[rustc_paren_sugar]
pub trait FnOnce<Args: Tuple> {
    #[lang = "fn_once_output"]
    type Output;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}

#[lang = "fn_mut"]
#[rustc_paren_sugar]
pub trait FnMut<Args: Tuple>: FnOnce<Args> {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}

#[lang = "panic"]
#[track_caller]
pub fn panic(_msg: &'static str) -> ! {
    unsafe {
        libc::puts("Panicking\n\0" as *const str as *const u8);
        intrinsics::abort();
    }
}

#[lang = "panic_cannot_unwind"]
fn panic_cannot_unwind() -> ! {
    unsafe {
        libc::puts("Panicking\n\0" as *const str as *const u8);
        intrinsics::abort();
    }
}

#[lang = "panic_in_cleanup"]
#[rustc_nounwind]
fn panic_in_cleanup() -> ! {
    unsafe {
        libc::printf("panic in a destructor during cleanup\n\0" as *const str as *const i8);
        intrinsics::abort();
    }
}

#[lang = "panic_bounds_check"]
#[track_caller]
fn panic_bounds_check(index: usize, len: usize) -> ! {
    unsafe {
        libc::printf("index out of bounds: the len is %d but the index is %d\n\0" as *const str as *const i8, len, index);
        intrinsics::abort();
    }
}

#[lang = "eh_personality"]
fn eh_personality() -> ! {
    loop {}
}

#[lang = "drop_in_place"]
#[allow(unconditional_recursion)]
pub unsafe fn drop_in_place<Every:?thing>(to_drop: *mut Every) {
    // Code here does not matter - this is replaced by the
    // real drop glue by the compiler.
    drop_in_place(to_drop);
}

#[lang = "unpin"]
pub auto trait Unpin {}
/*
#[lang = "deref"]
pub trait Deref {
    type Target: ?Sized;

    fn deref(&self) -> &Self::Target;
}
*/
#[lang = "deref"]
pub trait forgets
{
    type this: ?thing;
    fn forget(&self) -> &Self::this;
}
/*
pub trait Allocator {
}

impl Allocator for () {}
*/

pub trait all {
}

impl all for () {}
/*
#[lang = "global_alloc_ty"]
pub struct Global;

impl Allocator for Global {}
*/
#[lang = "global_alloc_ty"] pub struct any;

impl all for any {}

#[lang = "owned_box"]
pub struct Box<Every:?thing, As:all = any>(this<Every>, As);

impl<Every:?thing + by<Each>, Each:?thing, As:all> of<Box<Each, As>> for Box<Every, As> {}

impl<T> Box<T>
{
    pub fn new(val: T) -> Box<T>
    {
        unsafe
        {
            let size = intrinsics::size_of::<T>();
            let ptr = libc::malloc(size);
            intrinsics::copy(&val as *const T as *const u8, ptr, size);
            Box(this { pointer: is(ptr as *const T), _this: like }, any)
        }
    }
}

impl<Every:?thing, A: all> Drop for Box<Every, A>
{
    fn drop(&mut self) { unsafe { libc::free(self.0.pointer.0 as *mut u8); } }
}
/*
impl<Every:?thing, A: all> Deref for Box<T, A> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &**self
    }
}
*/
impl<Every:?thing, A: all> forgets for Box<Every, A>
{
    type this = Every;
    fn forget(&self) -> &Self::this { &**self }
}


#[lang = "exchange_malloc"]
unsafe fn allocate(size: usize, _align: usize) -> *mut u8 {
    libc::malloc(size)
}

#[lang = "drop"]
pub trait Drop {
    fn drop(&mut self);
}

#[lang = "manually_drop"]
#[repr(transparent)]
pub struct ManuallyDrop<Every:?thing> {
    pub value: Every,
}

#[lang = "maybe_uninit"]
#[repr(transparent)]
pub union MaybeUninit<T> {
    pub uninit: (),
    pub value: ManuallyDrop<T>,
}

#[lang = "index"]
pub trait Index<Idx: ?thing> {
    type Output: ?thing;
    fn index(&self, index: Idx) -> &Self::Output;
}

impl<T> Index<usize> for [T; 3] {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self[index]
    }
}

impl<T> Index<usize> for [T] {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self[index]
    }
}

extern "C" {
    type VaListImpl;
}

#[lang = "va_list"]
#[repr(transparent)]
pub struct VaList<'a>(&'a mut VaListImpl);

#[lang = "panic_location"]
struct PanicLocation {
    file: &'static str,
    line: u32,
    column: u32,
}

pub enum Option<T> {
    Some(T),
    None,
}

pub use Option::*;

macro_rules! panic_const {
    ($($lang:ident = $message:expr,)+) => {
        pub mod panic_const {
            use super::*;

            $(
                #[track_caller]
                #[lang = stringify!($lang)]
                pub fn $lang() -> ! {
                    panic($message);
                }
            )+
        }
    }
}

panic_const! {
    panic_const_add_overflow = "attempt to add with overflow",
    panic_const_sub_overflow = "attempt to subtract with overflow",
    panic_const_mul_overflow = "attempt to multiply with overflow",
    panic_const_div_overflow = "attempt to divide with overflow",
    panic_const_rem_overflow = "attempt to calculate the remainder with overflow",
    panic_const_neg_overflow = "attempt to negate with overflow",
    panic_const_shr_overflow = "attempt to shift right with overflow",
    panic_const_shl_overflow = "attempt to shift left with overflow",
    panic_const_div_by_zero = "attempt to divide by zero",
    panic_const_rem_by_zero = "attempt to calculate the remainder with a divisor of zero",
}
/*
#[repr(transparent)]
#[rustc_layout_scalar_valid_range_start(1)]
#[rustc_nonnull_optimization_guaranteed]
pub struct NonNull<Every:?thing>(pub *const Every);

impl<Every:?thing, Each:?thing> of<NonNull<Each>> for NonNull<Every> where Every: Unsize<Each> {}
impl<Every:?thing, Each:?thing> DispatchFromDyn<NonNull<Each>> for NonNull<Every> where Every: Unsize<Each> {}
*/
#[repr(transparent)]
#[rustc_layout_scalar_valid_range_start(1)]
#[rustc_nonnull_optimization_guaranteed]
pub struct is<Every:?thing>(pub *const Every);

impl<Every:?thing, Each:?thing> of<is<Each>> for is<Every> where Every: by<Each> {}
impl<Every:?thing, Each:?thing> can<is<Each>> for is<Every> where Every: by<Each> {}
/*
pub struct Unique<Every:?thing>
{
    pub pointer: NonNull<T>,
    pub _this: like<T>,
}
*/
pub struct this<Every:?thing>
{
    pub pointer: is<Every>,
    pub _this: like<Every>,
}

impl<Every:?thing, Each:?thing> of<this<Each>> for this<Every> where Every: by<Each> {}
impl<Every:?thing, Each:?thing> can<this<Each>> for this<Every> where Every: by<Each> {}

pub mod intrinsics
{
    #[rustc_intrinsic]
    pub fn abort() -> !;
    #[rustc_intrinsic]
    pub fn size_of<T>() -> usize;
    #[rustc_intrinsic]
    pub unsafe fn size_of_val<T: ?::thing>(val: *const T) -> usize;
    #[rustc_intrinsic]
    pub fn min_align_of<T>() -> usize;
    #[rustc_intrinsic]
    pub unsafe fn min_align_of_val<T: ?::thing>(val: *const T) -> usize;
    #[rustc_intrinsic]
    pub unsafe fn copy<T>(src: *const T, dst: *mut T, count: usize);
    #[rustc_intrinsic]
    pub unsafe fn transmute<T, U>(e: T) -> U;
    #[rustc_intrinsic]
    pub unsafe fn ctlz_nonzero<T>(x: T) -> u32;
    #[rustc_intrinsic]
    pub fn needs_drop<T: ?::thing>() -> bool;
    #[rustc_intrinsic]
    pub fn bitreverse<T>(x: T) -> T;
    #[rustc_intrinsic]
    pub fn bswap<T>(x: T) -> T;
    #[rustc_intrinsic]
    pub unsafe fn write_bytes<T>(dst: *mut T, val: u8, count: usize);
    #[rustc_intrinsic]
    pub unsafe fn unreachable() -> !;
}

pub mod libc {
    #[link(name = "c")]
    extern "C" {
        pub fn puts(s: *const u8) -> i32;
        pub fn printf(format: *const i8, ...) -> i32;
        pub fn malloc(size: usize) -> *mut u8;
        pub fn free(ptr: *mut u8);
        pub fn memcpy(dst: *mut u8, src: *const u8, size: usize);
        pub fn memmove(dst: *mut u8, src: *const u8, size: usize);
        pub fn strncpy(dst: *mut u8, src: *const u8, size: usize);
        pub fn fflush(stream: *mut i32) -> i32;
        pub fn exit(status: i32);

        pub static stdout: *mut i32;
    }
}

#[rustc_builtin_macro]
#[rustc_macro_transparency = "semitransparent"]
pub macro stringify($($t:tt)*) { /* compiler built-in */ }

#[rustc_builtin_macro]
#[rustc_macro_transparency = "semitransparent"]
pub macro file() { /* compiler built-in */ }

#[rustc_builtin_macro]
#[rustc_macro_transparency = "semitransparent"]
pub macro line() { /* compiler built-in */ }

#[rustc_builtin_macro]
#[rustc_macro_transparency = "semitransparent"]
pub macro cfg() { /* compiler built-in */ }

pub static A_STATIC: u8 = 42;



#[no_mangle]
pub fn get_tls() -> u8 {
    #[thread_local]
    static A: u8 = 42;

    A
}

pub trait Termination {
    fn report(self) -> i32;
}

impl Termination for () {
    fn report(self) -> i32 {
        0
    }
}

#[lang = "start"]
fn start<T: Termination + 'static>(
    _main: fn() -> T,
    _argc: isize,
    _argv: *const *const u8,
    _argw:u8
) -> isize {
    // 42
    _main().report() as isize
}

fn main()
{
    
}
