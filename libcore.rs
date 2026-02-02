#![stable(feature = "core", since = "1.6.0")]
#![doc(rust_logo)]
#![doc(auto_cfg(hide(
    no_fp_fmt_parse,
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
    target_has_atomic = "8",
    target_has_atomic = "16",
    target_has_atomic = "32",
    target_has_atomic = "64",
    target_has_atomic_equal_alignment = "8",
    target_has_atomic_equal_alignment = "16",
    target_has_atomic_equal_alignment = "32",
    target_has_atomic_equal_alignment = "64",
    target_has_atomic_equal_alignment = "ptr",
    target_has_atomic_load_store = "8",
    target_has_atomic_load_store = "16",
    target_has_atomic_load_store = "32",
    target_has_atomic_load_store = "64",
    target_has_atomic_load_store = "ptr",
)))]
#![no_core]
#![rustc_coherence_is_core]
#![rustc_preserve_ub_checks]
//
// Lints:
#![deny(rust_2021_incompatible_or_patterns)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(fuzzy_provenance_casts)]
#![warn(deprecated_in_future)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![allow(explicit_outlives_requirements)]
#![allow(incomplete_features)]
#![warn(multiple_supertrait_upcastable)]
#![allow(internal_features)]
#![deny(ffi_unwind_calls)]
#![warn(unreachable_pub)]

#![allow(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::unescaped_backticks)]

#![feature(array_ptr_get)]
#![feature(asm_experimental_arch)]
#![feature(bigint_helper_methods)]
#![feature(bstr)]
#![feature(bstr_internals)]
#![feature(cfg_select)]
#![feature(cfg_target_has_reliable_f16_f128)]
#![feature(const_carrying_mul_add)]
#![feature(const_cmp)]
#![feature(const_destruct)]
#![feature(const_eval_select)]
#![feature(const_select_unpredictable)]
#![feature(core_intrinsics)]
#![feature(coverage_attribute)]
#![feature(disjoint_bitor)]
#![feature(internal_impls_macro)]
#![feature(ip)]
#![feature(is_ascii_octdigit)]
#![feature(lazy_get)]
#![feature(link_cfg)]
#![feature(offset_of_enum)]
#![feature(panic_internals)]
#![feature(pattern_type_macro)]
#![feature(ptr_alignment_type)]
#![feature(ptr_metadata)]
#![feature(set_ptr_value)]
#![feature(slice_ptr_get)]
#![feature(str_internals)]
#![feature(str_split_inclusive_remainder)]
#![feature(str_split_remainder)]
#![feature(ub_checks)]
#![feature(unsafe_pinned)]
#![feature(utf16_extra)]
#![feature(variant_count)]

#![feature(abi_unadjusted)]
#![feature(adt_const_params)]
#![feature(allow_internal_unsafe)]
#![feature(allow_internal_unstable)]
#![feature(auto_traits)]
#![feature(cfg_sanitize)]
#![feature(cfg_target_has_atomic)]
#![feature(cfg_target_has_atomic_equal_alignment)]
#![feature(cfg_ub_checks)]
#![feature(const_precise_live_drops)]
#![feature(const_trait_impl)]
#![feature(decl_macro)]
#![feature(deprecated_suggestion)]
#![feature(derive_const)]
#![feature(diagnostic_on_const)]
#![feature(doc_cfg)]
#![feature(doc_notable_trait)]
#![feature(extern_types)]
#![feature(f16)]
#![feature(f128)]
#![feature(freeze_impls)]
#![feature(fundamental)]
#![feature(funnel_shifts)]
#![feature(if_let_guard)]
#![feature(intra_doc_pointers)]
#![feature(intrinsics)]
#![feature(lang_items)]
#![feature(link_llvm_intrinsics)]
#![feature(macro_metavar_expr)]
#![feature(macro_metavar_expr_concat)]
#![feature(marker_trait_attr)]
#![feature(min_specialization)]
#![feature(multiple_supertrait_upcastable)]
#![feature(must_not_suspend)]
#![feature(negative_impls)]
#![feature(never_type)]
#![feature(no_core)]
#![feature(optimize_attribute)]
#![feature(pattern_types)]
#![feature(prelude_import)]
#![feature(reborrow)]
#![feature(repr_simd)]
#![feature(rustc_allow_const_fn_unstable)]
#![feature(rustc_attrs)]
#![feature(rustdoc_internals)]
#![feature(simd_ffi)]
#![feature(staged_api)]
#![feature(stmt_expr_attributes)]
#![feature(strict_provenance_lints)]
#![feature(trait_alias)]
#![feature(transparent_unions)]
#![feature(try_blocks)]
#![feature(unboxed_closures)]
#![feature(unsized_fn_params)]
#![feature(with_negative_coherence)]

#![feature(aarch64_unstable_target_feature)]
#![feature(arm_target_feature)]
#![feature(avx10_target_feature)]
#![feature(hexagon_target_feature)]
#![feature(loongarch_target_feature)]
#![feature(mips_target_feature)]
#![feature(nvptx_target_feature)]
#![feature(powerpc_target_feature)]
#![feature(riscv_target_feature)]
#![feature(rtm_target_feature)]
#![feature(s390x_target_feature)]
#![feature(wasm_target_feature)]
#![feature(x86_amx_intrinsics)]

#[allow(unused_extern_crates)]
extern crate self as core;

pub mod prelude
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::$1::{*};
    */

    // No formatting: this file is nothing but re-exports, and their order is worth preserving.
    #![cfg_attr(rustfmt, rustfmt::skip)]

    #![stable(feature = "core_prelude", since = "1.4.0")]

    pub mod v1
    {
        /*!
        */
        use ::
        {
            *
        };
        /*
        pub use std::v1::{*};
        */
        
    }

    /// The 2015 version of the core prelude.
    ///
    /// See the [module-level documentation](self) for more.
    #[stable(feature = "prelude_2015", since = "1.55.0")]
    pub mod rust_2015 {
        #[stable(feature = "prelude_2015", since = "1.55.0")]
        #[doc(no_inline)]
        pub use super::v1::*;
    }

    /// The 2018 version of the core prelude.
    ///
    /// See the [module-level documentation](self) for more.
    #[stable(feature = "prelude_2018", since = "1.55.0")]
    pub mod rust_2018 {
        #[stable(feature = "prelude_2018", since = "1.55.0")]
        #[doc(no_inline)]
        pub use super::v1::*;
    }

    /// The 2021 version of the core prelude.
    ///
    /// See the [module-level documentation](self) for more.
    #[stable(feature = "prelude_2021", since = "1.55.0")]
    pub mod rust_2021 {
        #[stable(feature = "prelude_2021", since = "1.55.0")]
        #[doc(no_inline)]
        pub use super::v1::*;

        #[stable(feature = "prelude_2021", since = "1.55.0")]
        #[doc(no_inline)]
        pub use crate::iter::FromIterator;

        #[stable(feature = "prelude_2021", since = "1.55.0")]
        #[doc(no_inline)]
        pub use crate::convert::{TryFrom, TryInto};
    }

    /// The 2024 version of the core prelude.
    ///
    /// See the [module-level documentation](self) for more.
    #[stable(feature = "prelude_2024", since = "1.85.0")]
    pub mod rust_2024 {
        #[stable(feature = "rust1", since = "1.0.0")]
        #[doc(no_inline)]
        pub use super::v1::*;

        #[stable(feature = "prelude_2021", since = "1.55.0")]
        #[doc(no_inline)]
        pub use crate::iter::FromIterator;

        #[stable(feature = "prelude_2021", since = "1.55.0")]
        #[doc(no_inline)]
        pub use crate::convert::{TryFrom, TryInto};

        #[stable(feature = "prelude_2024", since = "1.85.0")]
        #[doc(no_inline)]
        pub use crate::future::{Future, IntoFuture};
    }

    /// The Future version of the core prelude.
    ///
    /// See the [module-level documentation](self) for more.
    #[doc(hidden)]
    #[unstable(feature = "prelude_future", issue = "none")]
    pub mod rust_future {
        #[stable(feature = "rust1", since = "1.0.0")]
        #[doc(no_inline)]
        pub use super::v1::*;

        #[stable(feature = "prelude_2021", since = "1.55.0")]
        #[doc(no_inline)]
        pub use crate::iter::FromIterator;

        #[stable(feature = "prelude_2021", since = "1.55.0")]
        #[doc(no_inline)]
        pub use crate::convert::{TryFrom, TryInto};

        #[stable(feature = "prelude_2024", since = "1.85.0")]
        #[doc(no_inline)]
        pub use crate::future::{Future, IntoFuture};
    }
    
}

#[prelude_import]
#[allow(unused)]
use prelude::rust_2024::*;

#[macro_use]
pub mod macros
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::macros::{*};
    */
    #[macro_export]
    #[rustc_builtin_macro(core_panic)]
    #[allow_internal_unstable(edition_panic)]
    #[stable(feature = "core", since = "1.6.0")]
    #[rustc_diagnostic_item = "core_panic_macro"]
    macro_rules! panic {
        // Expands to either `$crate::panic::panic_2015` or `$crate::panic::panic_2021`
        // depending on the edition of the caller.
        ($($arg:tt)*) => {
            /* compiler built-in */
        };
    }

    /// Asserts that two expressions are equal to each other (using [`PartialEq`]).
    ///
    /// Assertions are always checked in both debug and release builds, and cannot
    /// be disabled. See [`debug_assert_eq!`] for assertions that are disabled in
    /// release builds by default.
    ///
    /// [`debug_assert_eq!`]: crate::debug_assert_eq
    ///
    /// On panic, this macro will print the values of the expressions with their
    /// debug representations.
    ///
    /// Like [`assert!`], this macro has a second form, where a custom
    /// panic message can be provided.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = 3;
    /// let b = 1 + 2;
    /// assert_eq!(a, b);
    ///
    /// assert_eq!(a, b, "we are testing addition with {} and {}", a, b);
    /// ```
    #[macro_export]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_diagnostic_item = "assert_eq_macro"]
    #[allow_internal_unstable(panic_internals)]
    macro_rules! assert_eq {
        ($left:expr, $right:expr $(,)?) => {
            match (&$left, &$right) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = $crate::panicking::AssertKind::Eq;
                        // The reborrows below are intentional. Without them, the stack slot for the
                        // borrow is initialized even before the values are compared, leading to a
                        // noticeable slow down.
                        $crate::panicking::assert_failed(kind, &*left_val, &*right_val, $crate::option::Option::None);
                    }
                }
            }
        };
        ($left:expr, $right:expr, $($arg:tt)+) => {
            match (&$left, &$right) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = $crate::panicking::AssertKind::Eq;
                        // The reborrows below are intentional. Without them, the stack slot for the
                        // borrow is initialized even before the values are compared, leading to a
                        // noticeable slow down.
                        $crate::panicking::assert_failed(kind, &*left_val, &*right_val, $crate::option::Option::Some($crate::format_args!($($arg)+)));
                    }
                }
            }
        };
    }

    /// Asserts that two expressions are not equal to each other (using [`PartialEq`]).
    ///
    /// Assertions are always checked in both debug and release builds, and cannot
    /// be disabled. See [`debug_assert_ne!`] for assertions that are disabled in
    /// release builds by default.
    ///
    /// [`debug_assert_ne!`]: crate::debug_assert_ne
    ///
    /// On panic, this macro will print the values of the expressions with their
    /// debug representations.
    ///
    /// Like [`assert!`], this macro has a second form, where a custom
    /// panic message can be provided.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = 3;
    /// let b = 2;
    /// assert_ne!(a, b);
    ///
    /// assert_ne!(a, b, "we are testing that the values are not equal");
    /// ```
    #[macro_export]
    #[stable(feature = "assert_ne", since = "1.13.0")]
    #[rustc_diagnostic_item = "assert_ne_macro"]
    #[allow_internal_unstable(panic_internals)]
    macro_rules! assert_ne {
        ($left:expr, $right:expr $(,)?) => {
            match (&$left, &$right) {
                (left_val, right_val) => {
                    if *left_val == *right_val {
                        let kind = $crate::panicking::AssertKind::Ne;
                        // The reborrows below are intentional. Without them, the stack slot for the
                        // borrow is initialized even before the values are compared, leading to a
                        // noticeable slow down.
                        $crate::panicking::assert_failed(kind, &*left_val, &*right_val, $crate::option::Option::None);
                    }
                }
            }
        };
        ($left:expr, $right:expr, $($arg:tt)+) => {
            match (&($left), &($right)) {
                (left_val, right_val) => {
                    if *left_val == *right_val {
                        let kind = $crate::panicking::AssertKind::Ne;
                        // The reborrows below are intentional. Without them, the stack slot for the
                        // borrow is initialized even before the values are compared, leading to a
                        // noticeable slow down.
                        $crate::panicking::assert_failed(kind, &*left_val, &*right_val, $crate::option::Option::Some($crate::format_args!($($arg)+)));
                    }
                }
            }
        };
    }

    /// Asserts that an expression matches the provided pattern.
    ///
    /// This macro is generally preferable to `assert!(matches!(value, pattern))`, because it can print
    /// the debug representation of the actual value shape that did not meet expectations. In contrast,
    /// using [`assert!`] will only print that expectations were not met, but not why.
    ///
    /// The pattern syntax is exactly the same as found in a match arm and the `matches!` macro. The
    /// optional if guard can be used to add additional checks that must be true for the matched value,
    /// otherwise this macro will panic.
    ///
    /// Assertions are always checked in both debug and release builds, and cannot
    /// be disabled. See [`debug_assert_matches!`] for assertions that are disabled in
    /// release builds by default.
    ///
    /// [`debug_assert_matches!`]: crate::assert_matches::debug_assert_matches
    ///
    /// On panic, this macro will print the value of the expression with its debug representation.
    ///
    /// Like [`assert!`], this macro has a second form, where a custom panic message can be provided.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(assert_matches)]
    ///
    /// use std::assert_matches::assert_matches;
    ///
    /// let a = Some(345);
    /// let b = Some(56);
    /// assert_matches!(a, Some(_));
    /// assert_matches!(b, Some(_));
    ///
    /// assert_matches!(a, Some(345));
    /// assert_matches!(a, Some(345) | None);
    ///
    /// // assert_matches!(a, None); // panics
    /// // assert_matches!(b, Some(345)); // panics
    /// // assert_matches!(b, Some(345) | None); // panics
    ///
    /// assert_matches!(a, Some(x) if x > 100);
    /// // assert_matches!(a, Some(x) if x < 100); // panics
    /// ```
    #[unstable(feature = "assert_matches", issue = "82775")]
    #[allow_internal_unstable(panic_internals)]
    #[rustc_macro_transparency = "semitransparent"]
    pub macro assert_matches {
        ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)?) => {
            match $left {
                $( $pattern )|+ $( if $guard )? => {}
                ref left_val => {
                    $crate::panicking::assert_matches_failed(
                        left_val,
                        $crate::stringify!($($pattern)|+ $(if $guard)?),
                        $crate::option::Option::None
                    );
                }
            }
        },
        ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )?, $($arg:tt)+) => {
            match $left {
                $( $pattern )|+ $( if $guard )? => {}
                ref left_val => {
                    $crate::panicking::assert_matches_failed(
                        left_val,
                        $crate::stringify!($($pattern)|+ $(if $guard)?),
                        $crate::option::Option::Some($crate::format_args!($($arg)+))
                    );
                }
            }
        },
    }

    /// Selects code at compile-time based on `cfg` predicates.
    ///
    /// This macro evaluates, at compile-time, a series of `cfg` predicates,
    /// selects the first that is true, and emits the code guarded by that
    /// predicate. The code guarded by other predicates is not emitted.
    ///
    /// An optional trailing `_` wildcard can be used to specify a fallback. If
    /// none of the predicates are true, a [`compile_error`] is emitted.
    ///
    /// # Example
    ///
    /// ```
    /// #![feature(cfg_select)]
    ///
    /// cfg_select! {
    ///     unix => {
    ///         fn foo() { /* unix specific functionality */ }
    ///     }
    ///     target_pointer_width = "32" => {
    ///         fn foo() { /* non-unix, 32-bit functionality */ }
    ///     }
    ///     _ => {
    ///         fn foo() { /* fallback implementation */ }
    ///     }
    /// }
    /// ```
    ///
    /// The `cfg_select!` macro can also be used in expression position, with or without braces on the
    /// right-hand side:
    ///
    /// ```
    /// #![feature(cfg_select)]
    ///
    /// let _some_string = cfg_select! {
    ///     unix => "With great power comes great electricity bills",
    ///     _ => { "Behind every successful diet is an unwatched pizza" }
    /// };
    /// ```
    #[unstable(feature = "cfg_select", issue = "115585")]
    #[rustc_diagnostic_item = "cfg_select"]
    #[rustc_builtin_macro]
    pub macro cfg_select($($tt:tt)*) {
        /* compiler built-in */
    }

    /// Asserts that a boolean expression is `true` at runtime.
    ///
    /// This will invoke the [`panic!`] macro if the provided expression cannot be
    /// evaluated to `true` at runtime.
    ///
    /// Like [`assert!`], this macro also has a second version, where a custom panic
    /// message can be provided.
    ///
    /// # Uses
    ///
    /// Unlike [`assert!`], `debug_assert!` statements are only enabled in non
    /// optimized builds by default. An optimized build will not execute
    /// `debug_assert!` statements unless `-C debug-assertions` is passed to the
    /// compiler. This makes `debug_assert!` useful for checks that are too
    /// expensive to be present in a release build but may be helpful during
    /// development. The result of expanding `debug_assert!` is always type checked.
    ///
    /// An unchecked assertion allows a program in an inconsistent state to keep
    /// running, which might have unexpected consequences but does not introduce
    /// unsafety as long as this only happens in safe code. The performance cost
    /// of assertions, however, is not measurable in general. Replacing [`assert!`]
    /// with `debug_assert!` is thus only encouraged after thorough profiling, and
    /// more importantly, only in safe code!
    ///
    /// # Examples
    ///
    /// ```
    /// // the panic message for these assertions is the stringified value of the
    /// // expression given.
    /// debug_assert!(true);
    ///
    /// fn some_expensive_computation() -> bool {
    ///     // Some expensive computation here
    ///     true
    /// }
    /// debug_assert!(some_expensive_computation());
    ///
    /// // assert with a custom message
    /// let x = true;
    /// debug_assert!(x, "x wasn't true!");
    ///
    /// let a = 3; let b = 27;
    /// debug_assert!(a + b == 30, "a = {}, b = {}", a, b);
    /// ```
    #[macro_export]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_diagnostic_item = "debug_assert_macro"]
    #[allow_internal_unstable(edition_panic)]
    macro_rules! debug_assert {
        ($($arg:tt)*) => {
            if $crate::cfg!(debug_assertions) {
                $crate::assert!($($arg)*);
            }
        };
    }

    /// Asserts that two expressions are equal to each other.
    ///
    /// On panic, this macro will print the values of the expressions with their
    /// debug representations.
    ///
    /// Unlike [`assert_eq!`], `debug_assert_eq!` statements are only enabled in non
    /// optimized builds by default. An optimized build will not execute
    /// `debug_assert_eq!` statements unless `-C debug-assertions` is passed to the
    /// compiler. This makes `debug_assert_eq!` useful for checks that are too
    /// expensive to be present in a release build but may be helpful during
    /// development. The result of expanding `debug_assert_eq!` is always type checked.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = 3;
    /// let b = 1 + 2;
    /// debug_assert_eq!(a, b);
    /// ```
    #[macro_export]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_diagnostic_item = "debug_assert_eq_macro"]
    macro_rules! debug_assert_eq {
        ($($arg:tt)*) => {
            if $crate::cfg!(debug_assertions) {
                $crate::assert_eq!($($arg)*);
            }
        };
    }

    /// Asserts that two expressions are not equal to each other.
    ///
    /// On panic, this macro will print the values of the expressions with their
    /// debug representations.
    ///
    /// Unlike [`assert_ne!`], `debug_assert_ne!` statements are only enabled in non
    /// optimized builds by default. An optimized build will not execute
    /// `debug_assert_ne!` statements unless `-C debug-assertions` is passed to the
    /// compiler. This makes `debug_assert_ne!` useful for checks that are too
    /// expensive to be present in a release build but may be helpful during
    /// development. The result of expanding `debug_assert_ne!` is always type checked.
    ///
    /// # Examples
    ///
    /// ```
    /// let a = 3;
    /// let b = 2;
    /// debug_assert_ne!(a, b);
    /// ```
    #[macro_export]
    #[stable(feature = "assert_ne", since = "1.13.0")]
    #[rustc_diagnostic_item = "debug_assert_ne_macro"]
    macro_rules! debug_assert_ne {
        ($($arg:tt)*) => {
            if $crate::cfg!(debug_assertions) {
                $crate::assert_ne!($($arg)*);
            }
        };
    }

    /// Asserts that an expression matches the provided pattern.
    ///
    /// This macro is generally preferable to `debug_assert!(matches!(value, pattern))`, because it can
    /// print the debug representation of the actual value shape that did not meet expectations. In
    /// contrast, using [`debug_assert!`] will only print that expectations were not met, but not why.
    ///
    /// The pattern syntax is exactly the same as found in a match arm and the `matches!` macro. The
    /// optional if guard can be used to add additional checks that must be true for the matched value,
    /// otherwise this macro will panic.
    ///
    /// On panic, this macro will print the value of the expression with its debug representation.
    ///
    /// Like [`assert!`], this macro has a second form, where a custom panic message can be provided.
    ///
    /// Unlike [`assert_matches!`], `debug_assert_matches!` statements are only enabled in non optimized
    /// builds by default. An optimized build will not execute `debug_assert_matches!` statements unless
    /// `-C debug-assertions` is passed to the compiler. This makes `debug_assert_matches!` useful for
    /// checks that are too expensive to be present in a release build but may be helpful during
    /// development. The result of expanding `debug_assert_matches!` is always type checked.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(assert_matches)]
    ///
    /// use std::assert_matches::debug_assert_matches;
    ///
    /// let a = Some(345);
    /// let b = Some(56);
    /// debug_assert_matches!(a, Some(_));
    /// debug_assert_matches!(b, Some(_));
    ///
    /// debug_assert_matches!(a, Some(345));
    /// debug_assert_matches!(a, Some(345) | None);
    ///
    /// // debug_assert_matches!(a, None); // panics
    /// // debug_assert_matches!(b, Some(345)); // panics
    /// // debug_assert_matches!(b, Some(345) | None); // panics
    ///
    /// debug_assert_matches!(a, Some(x) if x > 100);
    /// // debug_assert_matches!(a, Some(x) if x < 100); // panics
    /// ```
    #[unstable(feature = "assert_matches", issue = "82775")]
    #[allow_internal_unstable(assert_matches)]
    #[rustc_macro_transparency = "semitransparent"]
    pub macro debug_assert_matches($($arg:tt)*) {
        if $crate::cfg!(debug_assertions) {
            $crate::assert_matches::assert_matches!($($arg)*);
        }
    }

    /// Returns whether the given expression matches the provided pattern.
    ///
    /// The pattern syntax is exactly the same as found in a match arm. The optional if guard can be
    /// used to add additional checks that must be true for the matched value, otherwise this macro will
    /// return `false`.
    ///
    /// When testing that a value matches a pattern, it's generally preferable to use
    /// [`assert_matches!`] as it will print the debug representation of the value if the assertion
    /// fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let foo = 'f';
    /// assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));
    ///
    /// let bar = Some(4);
    /// assert!(matches!(bar, Some(x) if x > 2));
    /// ```
    #[macro_export]
    #[stable(feature = "matches_macro", since = "1.42.0")]
    #[rustc_diagnostic_item = "matches_macro"]
    #[allow_internal_unstable(non_exhaustive_omitted_patterns_lint, stmt_expr_attributes)]
    macro_rules! matches {
        ($expression:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {
            #[allow(non_exhaustive_omitted_patterns)]
            match $expression {
                $pattern $(if $guard)? => true,
                _ => false
            }
        };
    }

    /// Unwraps a result or propagates its error.
    ///
    /// The [`?` operator][propagating-errors] was added to replace `try!`
    /// and should be used instead. Furthermore, `try` is a reserved word
    /// in Rust 2018, so if you must use it, you will need to use the
    /// [raw-identifier syntax][ris]: `r#try`.
    ///
    /// [propagating-errors]: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
    /// [ris]: https://doc.rust-lang.org/nightly/rust-by-example/compatibility/raw_identifiers.html
    ///
    /// `try!` matches the given [`Result`]. In case of the `Ok` variant, the
    /// expression has the value of the wrapped value.
    ///
    /// In case of the `Err` variant, it retrieves the inner error. `try!` then
    /// performs conversion using `From`. This provides automatic conversion
    /// between specialized errors and more general ones. The resulting
    /// error is then immediately returned.
    ///
    /// Because of the early return, `try!` can only be used in functions that
    /// return [`Result`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io;
    /// use std::fs::File;
    /// use std::io::prelude::*;
    ///
    /// enum MyError {
    ///     FileWriteError
    /// }
    ///
    /// impl From<io::Error> for MyError {
    ///     fn from(e: io::Error) -> MyError {
    ///         MyError::FileWriteError
    ///     }
    /// }
    ///
    /// // The preferred method of quick returning Errors
    /// fn write_to_file_question() -> Result<(), MyError> {
    ///     let mut file = File::create("my_best_friends.txt")?;
    ///     file.write_all(b"This is a list of my best friends.")?;
    ///     Ok(())
    /// }
    ///
    /// // The previous method of quick returning Errors
    /// fn write_to_file_using_try() -> Result<(), MyError> {
    ///     let mut file = r#try!(File::create("my_best_friends.txt"));
    ///     r#try!(file.write_all(b"This is a list of my best friends."));
    ///     Ok(())
    /// }
    ///
    /// // This is equivalent to:
    /// fn write_to_file_using_match() -> Result<(), MyError> {
    ///     let mut file = r#try!(File::create("my_best_friends.txt"));
    ///     match file.write_all(b"This is a list of my best friends.") {
    ///         Ok(v) => v,
    ///         Err(e) => return Err(From::from(e)),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    #[macro_export]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[deprecated(since = "1.39.0", note = "use the `?` operator instead")]
    #[doc(alias = "?")]
    macro_rules! r#try {
        ($expr:expr $(,)?) => {
            match $expr {
                $crate::result::Result::Ok(val) => val,
                $crate::result::Result::Err(err) => {
                    return $crate::result::Result::Err($crate::convert::From::from(err));
                }
            }
        };
    }

    /// Writes formatted data into a buffer.
    ///
    /// This macro accepts a 'writer', a format string, and a list of arguments. Arguments will be
    /// formatted according to the specified format string and the result will be passed to the writer.
    /// The writer may be any value with a `write_fmt` method; generally this comes from an
    /// implementation of either the [`fmt::Write`] or the [`io::Write`] trait. The macro
    /// returns whatever the `write_fmt` method returns; commonly a [`fmt::Result`], or an
    /// [`io::Result`].
    ///
    /// See [`std::fmt`] for more information on the format string syntax.
    ///
    /// [`std::fmt`]: ../std/fmt/index.html
    /// [`fmt::Write`]: crate::fmt::Write
    /// [`io::Write`]: ../std/io/trait.Write.html
    /// [`fmt::Result`]: crate::fmt::Result
    /// [`io::Result`]: ../std/io/type.Result.html
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Write;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let mut w = Vec::new();
    ///     write!(&mut w, "test")?;
    ///     write!(&mut w, "formatted {}", "arguments")?;
    ///
    ///     assert_eq!(w, b"testformatted arguments");
    ///     Ok(())
    /// }
    /// ```
    ///
    /// A module can import both `std::fmt::Write` and `std::io::Write` and call `write!` on objects
    /// implementing either, as objects do not typically implement both. However, the module must
    /// avoid conflict between the trait names, such as by importing them as `_` or otherwise renaming
    /// them:
    ///
    /// ```
    /// use std::fmt::Write as _;
    /// use std::io::Write as _;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut s = String::new();
    ///     let mut v = Vec::new();
    ///
    ///     write!(&mut s, "{} {}", "abc", 123)?; // uses fmt::Write::write_fmt
    ///     write!(&mut v, "s = {:?}", s)?; // uses io::Write::write_fmt
    ///     assert_eq!(v, b"s = \"abc 123\"");
    ///     Ok(())
    /// }
    /// ```
    ///
    /// If you also need the trait names themselves, such as to implement one or both on your types,
    /// import the containing module and then name them with a prefix:
    ///
    /// ```
    /// # #![allow(unused_imports)]
    /// use std::fmt::{self, Write as _};
    /// use std::io::{self, Write as _};
    ///
    /// struct Example;
    ///
    /// impl fmt::Write for Example {
    ///     fn write_str(&mut self, _s: &str) -> core::fmt::Result {
    ///          unimplemented!();
    ///     }
    /// }
    /// ```
    ///
    /// Note: This macro can be used in `no_std` setups as well.
    /// In a `no_std` setup you are responsible for the implementation details of the components.
    ///
    /// ```no_run
    /// use core::fmt::Write;
    ///
    /// struct Example;
    ///
    /// impl Write for Example {
    ///     fn write_str(&mut self, _s: &str) -> core::fmt::Result {
    ///          unimplemented!();
    ///     }
    /// }
    ///
    /// let mut m = Example{};
    /// write!(&mut m, "Hello World").expect("Not written");
    /// ```
    #[macro_export]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_diagnostic_item = "write_macro"]
    macro_rules! write {
        ($dst:expr, $($arg:tt)*) => {
            $dst.write_fmt($crate::format_args!($($arg)*))
        };
    }

    /// Writes formatted data into a buffer, with a newline appended.
    ///
    /// On all platforms, the newline is the LINE FEED character (`\n`/`U+000A`) alone
    /// (no additional CARRIAGE RETURN (`\r`/`U+000D`).
    ///
    /// For more information, see [`write!`]. For information on the format string syntax, see
    /// [`std::fmt`].
    ///
    /// [`std::fmt`]: ../std/fmt/index.html
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::{Write, Result};
    ///
    /// fn main() -> Result<()> {
    ///     let mut w = Vec::new();
    ///     writeln!(&mut w)?;
    ///     writeln!(&mut w, "test")?;
    ///     writeln!(&mut w, "formatted {}", "arguments")?;
    ///
    ///     assert_eq!(&w[..], "\ntest\nformatted arguments\n".as_bytes());
    ///     Ok(())
    /// }
    /// ```
    #[macro_export]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_diagnostic_item = "writeln_macro"]
    #[allow_internal_unstable(format_args_nl)]
    macro_rules! writeln {
        ($dst:expr $(,)?) => {
            $crate::write!($dst, "\n")
        };
        ($dst:expr, $($arg:tt)*) => {
            $dst.write_fmt($crate::format_args_nl!($($arg)*))
        };
    }

    /// Indicates unreachable code.
    ///
    /// This is useful any time that the compiler can't determine that some code is unreachable. For
    /// example:
    ///
    /// * Match arms with guard conditions.
    /// * Loops that dynamically terminate.
    /// * Iterators that dynamically terminate.
    ///
    /// If the determination that the code is unreachable proves incorrect, the
    /// program immediately terminates with a [`panic!`].
    ///
    /// The unsafe counterpart of this macro is the [`unreachable_unchecked`] function, which
    /// will cause undefined behavior if the code is reached.
    ///
    /// [`unreachable_unchecked`]: crate::hint::unreachable_unchecked
    ///
    /// # Panics
    ///
    /// This will always [`panic!`] because `unreachable!` is just a shorthand for `panic!` with a
    /// fixed, specific message.
    ///
    /// Like `panic!`, this macro has a second form for displaying custom values.
    ///
    /// # Examples
    ///
    /// Match arms:
    ///
    /// ```
    /// # #[allow(dead_code)]
    /// fn foo(x: Option<i32>) {
    ///     match x {
    ///         Some(n) if n >= 0 => println!("Some(Non-negative)"),
    ///         Some(n) if n <  0 => println!("Some(Negative)"),
    ///         Some(_)           => unreachable!(), // compile error if commented out
    ///         None              => println!("None")
    ///     }
    /// }
    /// ```
    ///
    /// Iterators:
    ///
    /// ```
    /// # #[allow(dead_code)]
    /// fn divide_by_three(x: u32) -> u32 { // one of the poorest implementations of x/3
    ///     for i in 0.. {
    ///         if 3*i < i { panic!("u32 overflow"); }
    ///         if x < 3*i { return i-1; }
    ///     }
    ///     unreachable!("The loop should always return");
    /// }
    /// ```
    #[macro_export]
    #[rustc_builtin_macro(unreachable)]
    #[allow_internal_unstable(edition_panic)]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_diagnostic_item = "unreachable_macro"]
    macro_rules! unreachable {
        // Expands to either `$crate::panic::unreachable_2015` or `$crate::panic::unreachable_2021`
        // depending on the edition of the caller.
        ($($arg:tt)*) => {
            /* compiler built-in */
        };
    }

    /// Indicates unimplemented code by panicking with a message of "not implemented".
    ///
    /// This allows your code to type-check, which is useful if you are prototyping or
    /// implementing a trait that requires multiple methods which you don't plan to use all of.
    ///
    /// The difference between `unimplemented!` and [`todo!`] is that while `todo!`
    /// conveys an intent of implementing the functionality later and the message is "not yet
    /// implemented", `unimplemented!` makes no such claims. Its message is "not implemented".
    ///
    /// Also, some IDEs will mark `todo!`s.
    ///
    /// # Panics
    ///
    /// This will always [`panic!`] because `unimplemented!` is just a shorthand for `panic!` with a
    /// fixed, specific message.
    ///
    /// Like `panic!`, this macro has a second form for displaying custom values.
    ///
    /// [`todo!`]: crate::todo
    ///
    /// # Examples
    ///
    /// Say we have a trait `Foo`:
    ///
    /// ```
    /// trait Foo {
    ///     fn bar(&self) -> u8;
    ///     fn baz(&self);
    ///     fn qux(&self) -> Result<u64, ()>;
    /// }
    /// ```
    ///
    /// We want to implement `Foo` for 'MyStruct', but for some reason it only makes sense
    /// to implement the `bar()` function. `baz()` and `qux()` will still need to be defined
    /// in our implementation of `Foo`, but we can use `unimplemented!` in their definitions
    /// to allow our code to compile.
    ///
    /// We still want to have our program stop running if the unimplemented methods are
    /// reached.
    ///
    /// ```
    /// # trait Foo {
    /// #     fn bar(&self) -> u8;
    /// #     fn baz(&self);
    /// #     fn qux(&self) -> Result<u64, ()>;
    /// # }
    /// struct MyStruct;
    ///
    /// impl Foo for MyStruct {
    ///     fn bar(&self) -> u8 {
    ///         1 + 1
    ///     }
    ///
    ///     fn baz(&self) {
    ///         // It makes no sense to `baz` a `MyStruct`, so we have no logic here
    ///         // at all.
    ///         // This will display "thread 'main' panicked at 'not implemented'".
    ///         unimplemented!();
    ///     }
    ///
    ///     fn qux(&self) -> Result<u64, ()> {
    ///         // We have some logic here,
    ///         // We can add a message to unimplemented! to display our omission.
    ///         // This will display:
    ///         // "thread 'main' panicked at 'not implemented: MyStruct isn't quxable'".
    ///         unimplemented!("MyStruct isn't quxable");
    ///     }
    /// }
    ///
    /// fn main() {
    ///     let s = MyStruct;
    ///     s.bar();
    /// }
    /// ```
    #[macro_export]
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_diagnostic_item = "unimplemented_macro"]
    #[allow_internal_unstable(panic_internals)]
    macro_rules! unimplemented {
        () => {
            $crate::panicking::panic("not implemented")
        };
        ($($arg:tt)+) => {
            $crate::panic!("not implemented: {}", $crate::format_args!($($arg)+))
        };
    }

    /// Indicates unfinished code.
    ///
    /// This can be useful if you are prototyping and just
    /// want a placeholder to let your code pass type analysis.
    ///
    /// The difference between [`unimplemented!`] and `todo!` is that while `todo!` conveys
    /// an intent of implementing the functionality later and the message is "not yet
    /// implemented", `unimplemented!` makes no such claims. Its message is "not implemented".
    ///
    /// Also, some IDEs will mark `todo!`s.
    ///
    /// # Panics
    ///
    /// This will always [`panic!`] because `todo!` is just a shorthand for `panic!` with a
    /// fixed, specific message.
    ///
    /// Like `panic!`, this macro has a second form for displaying custom values.
    ///
    /// # Examples
    ///
    /// Here's an example of some in-progress code. We have a trait `Foo`:
    ///
    /// ```
    /// trait Foo {
    ///     fn bar(&self) -> u8;
    ///     fn baz(&self);
    ///     fn qux(&self) -> Result<u64, ()>;
    /// }
    /// ```
    ///
    /// We want to implement `Foo` on one of our types, but we also want to work on
    /// just `bar()` first. In order for our code to compile, we need to implement
    /// `baz()` and `qux()`, so we can use `todo!`:
    ///
    /// ```
    /// # trait Foo {
    /// #     fn bar(&self) -> u8;
    /// #     fn baz(&self);
    /// #     fn qux(&self) -> Result<u64, ()>;
    /// # }
    /// struct MyStruct;
    ///
    /// impl Foo for MyStruct {
    ///     fn bar(&self) -> u8 {
    ///         1 + 1
    ///     }
    ///
    ///     fn baz(&self) {
    ///         // Let's not worry about implementing baz() for now
    ///         todo!();
    ///     }
    ///
    ///     fn qux(&self) -> Result<u64, ()> {
    ///         // We can add a message to todo! to display our omission.
    ///         // This will display:
    ///         // "thread 'main' panicked at 'not yet implemented: MyStruct is not yet quxable'".
    ///         todo!("MyStruct is not yet quxable");
    ///     }
    /// }
    ///
    /// fn main() {
    ///     let s = MyStruct;
    ///     s.bar();
    ///
    ///     // We aren't even using baz() or qux(), so this is fine.
    /// }
    /// ```
    #[macro_export]
    #[stable(feature = "todo_macro", since = "1.40.0")]
    #[rustc_diagnostic_item = "todo_macro"]
    #[allow_internal_unstable(panic_internals)]
    macro_rules! todo {
        () => {
            $crate::panicking::panic("not yet implemented")
        };
        ($($arg:tt)+) => {
            $crate::panic!("not yet implemented: {}", $crate::format_args!($($arg)+))
        };
    }

    /// Definitions of built-in macros.
    ///
    /// Most of the macro properties (stability, visibility, etc.) are taken from the source code here,
    /// with exception of expansion functions transforming macro inputs into outputs,
    /// those functions are provided by the compiler.
    pub(crate) mod builtin {

        /// Causes compilation to fail with the given error message when encountered.
        ///
        /// This macro should be used when a crate uses a conditional compilation strategy to provide
        /// better error messages for erroneous conditions. It's the compiler-level form of [`panic!`],
        /// but emits an error during *compilation* rather than at *runtime*.
        ///
        /// # Examples
        ///
        /// Two such examples are macros and `#[cfg]` environments.
        ///
        /// Emit a better compiler error if a macro is passed invalid values. Without the final branch,
        /// the compiler would still emit an error, but the error's message would not mention the two
        /// valid values.
        ///
        /// ```compile_fail
        /// macro_rules! give_me_foo_or_bar {
        ///     (foo) => {};
        ///     (bar) => {};
        ///     ($x:ident) => {
        ///         compile_error!("This macro only accepts `foo` or `bar`");
        ///     }
        /// }
        ///
        /// give_me_foo_or_bar!(neither);
        /// // ^ will fail at compile time with message "This macro only accepts `foo` or `bar`"
        /// ```
        ///
        /// Emit a compiler error if one of a number of features isn't available.
        ///
        /// ```compile_fail
        /// #[cfg(not(any(feature = "foo", feature = "bar")))]
        /// compile_error!("Either feature \"foo\" or \"bar\" must be enabled for this crate.");
        /// ```
        #[stable(feature = "compile_error_macro", since = "1.20.0")]
        #[rustc_builtin_macro]
        #[macro_export]
        macro_rules! compile_error {
            ($msg:expr $(,)?) => {{ /* compiler built-in */ }};
        }

        /// Constructs parameters for the other string-formatting macros.
        ///
        /// This macro functions by taking a formatting string literal containing
        /// `{}` for each additional argument passed. `format_args!` prepares the
        /// additional parameters to ensure the output can be interpreted as a string
        /// and canonicalizes the arguments into a single type. Any value that implements
        /// the [`Display`] trait can be passed to `format_args!`, as can any
        /// [`Debug`] implementation be passed to a `{:?}` within the formatting string.
        ///
        /// This macro produces a value of type [`fmt::Arguments`]. This value can be
        /// passed to the macros within [`std::fmt`] for performing useful redirection.
        /// All other formatting macros ([`format!`], [`write!`], [`println!`], etc) are
        /// proxied through this one. `format_args!`, unlike its derived macros, avoids
        /// heap allocations.
        ///
        /// You can use the [`fmt::Arguments`] value that `format_args!` returns
        /// in `Debug` and `Display` contexts as seen below. The example also shows
        /// that `Debug` and `Display` format to the same thing: the interpolated
        /// format string in `format_args!`.
        ///
        /// ```rust
        /// let args = format_args!("{} foo {:?}", 1, 2);
        /// let debug = format!("{args:?}");
        /// let display = format!("{args}");
        /// assert_eq!("1 foo 2", display);
        /// assert_eq!(display, debug);
        /// ```
        ///
        /// See [the formatting documentation in `std::fmt`](../std/fmt/index.html)
        /// for details of the macro argument syntax, and further information.
        ///
        /// [`Display`]: crate::fmt::Display
        /// [`Debug`]: crate::fmt::Debug
        /// [`fmt::Arguments`]: crate::fmt::Arguments
        /// [`std::fmt`]: ../std/fmt/index.html
        /// [`format!`]: ../std/macro.format.html
        /// [`println!`]: ../std/macro.println.html
        ///
        /// # Examples
        ///
        /// ```
        /// use std::fmt;
        ///
        /// let s = fmt::format(format_args!("hello {}", "world"));
        /// assert_eq!(s, format!("hello {}", "world"));
        /// ```
        ///
        /// # Argument lifetimes
        ///
        /// Except when no formatting arguments are used,
        /// the produced `fmt::Arguments` value borrows temporary values.
        /// To allow it to be stored for later use, the arguments' lifetimes, as well as those of
        /// temporaries they borrow, may be [extended] when `format_args!` appears in the initializer
        /// expression of a `let` statement. The syntactic rules used to determine when temporaries'
        /// lifetimes are extended are documented in the [Reference].
        ///
        /// [extended]: ../reference/destructors.html#temporary-lifetime-extension
        /// [Reference]: ../reference/destructors.html#extending-based-on-expressions
        #[stable(feature = "rust1", since = "1.0.0")]
        #[rustc_diagnostic_item = "format_args_macro"]
        #[allow_internal_unsafe]
        #[allow_internal_unstable(fmt_internals, fmt_arguments_from_str)]
        #[rustc_builtin_macro]
        #[macro_export]
        macro_rules! format_args {
            ($fmt:expr) => {{ /* compiler built-in */ }};
            ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
        }

        /// Same as [`format_args`], but can be used in some const contexts.
        ///
        /// This macro is used by the panic macros for the `const_panic` feature.
        ///
        /// This macro will be removed once `format_args` is allowed in const contexts.
        #[unstable(feature = "const_format_args", issue = "none")]
        #[allow_internal_unstable(fmt_internals, fmt_arguments_from_str)]
        #[rustc_builtin_macro]
        #[macro_export]
        macro_rules! const_format_args {
            ($fmt:expr) => {{ /* compiler built-in */ }};
            ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
        }

        /// Same as [`format_args`], but adds a newline in the end.
        #[unstable(
            feature = "format_args_nl",
            issue = "none",
            reason = "`format_args_nl` is only for internal \
                    language use and is subject to change"
        )]
        #[allow_internal_unstable(fmt_internals, fmt_arguments_from_str)]
        #[rustc_builtin_macro]
        #[doc(hidden)]
        #[macro_export]
        macro_rules! format_args_nl {
            ($fmt:expr) => {{ /* compiler built-in */ }};
            ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};
        }

        /// Inspects an environment variable at compile time.
        ///
        /// This macro will expand to the value of the named environment variable at
        /// compile time, yielding an expression of type `&'static str`. Use
        /// [`std::env::var`] instead if you want to read the value at runtime.
        ///
        /// [`std::env::var`]: ../std/env/fn.var.html
        ///
        /// If the environment variable is not defined, then a compilation error
        /// will be emitted. To not emit a compile error, use the [`option_env!`]
        /// macro instead. A compilation error will also be emitted if the
        /// environment variable is not a valid Unicode string.
        ///
        /// # Examples
        ///
        /// ```
        /// let path: &'static str = env!("PATH");
        /// println!("the $PATH variable at the time of compiling was: {path}");
        /// ```
        ///
        /// You can customize the error message by passing a string as the second
        /// parameter:
        ///
        /// ```compile_fail
        /// let doc: &'static str = env!("documentation", "what's that?!");
        /// ```
        ///
        /// If the `documentation` environment variable is not defined, you'll get
        /// the following error:
        ///
        /// ```text
        /// error: what's that?!
        /// ```
        #[stable(feature = "rust1", since = "1.0.0")]
        #[rustc_builtin_macro]
        #[macro_export]
        #[rustc_diagnostic_item = "env_macro"] // useful for external lints
        macro_rules! env {
            ($name:expr $(,)?) => {{ /* compiler built-in */ }};
            ($name:expr, $error_msg:expr $(,)?) => {{ /* compiler built-in */ }};
        }

        /// Optionally inspects an environment variable at compile time.
        ///
        /// If the named environment variable is present at compile time, this will
        /// expand into an expression of type `Option<&'static str>` whose value is
        /// `Some` of the value of the environment variable (a compilation error
        /// will be emitted if the environment variable is not a valid Unicode
        /// string). If the environment variable is not present, then this will
        /// expand to `None`. See [`Option<T>`][Option] for more information on this
        /// type.  Use [`std::env::var`] instead if you want to read the value at
        /// runtime.
        ///
        /// [`std::env::var`]: ../std/env/fn.var.html
        ///
        /// A compile time error is only emitted when using this macro if the
        /// environment variable exists and is not a valid Unicode string. To also
        /// emit a compile error if the environment variable is not present, use the
        /// [`env!`] macro instead.
        ///
        /// # Examples
        ///
        /// ```
        /// let key: Option<&'static str> = option_env!("SECRET_KEY");
        /// println!("the secret key might be: {key:?}");
        /// ```
        #[stable(feature = "rust1", since = "1.0.0")]
        #[rustc_builtin_macro]
        #[macro_export]
        #[rustc_diagnostic_item = "option_env_macro"] // useful for external lints
        macro_rules! option_env {
            ($name:expr $(,)?) => {{ /* compiler built-in */ }};
        }

        /// Concatenates literals into a byte slice.
        ///
        /// This macro takes any number of comma-separated literals, and concatenates them all into
        /// one, yielding an expression of type `&[u8; _]`, which represents all of the literals
        /// concatenated left-to-right. The literals passed can be any combination of:
        ///
        /// - byte literals (`b'r'`)
        /// - byte strings (`b"Rust"`)
        /// - arrays of bytes/numbers (`[b'A', 66, b'C']`)
        ///
        /// # Examples
        ///
        /// ```
        /// #![feature(concat_bytes)]
        ///
        /// # fn main() {
        /// let s: &[u8; 6] = concat_bytes!(b'A', b"BC", [68, b'E', 70]);
        /// assert_eq!(s, b"ABCDEF");
        /// # }
        /// ```
        #[unstable(feature = "concat_bytes", issue = "87555")]
        #[rustc_builtin_macro]
        #[macro_export]
        macro_rules! concat_bytes {
            ($($e:literal),+ $(,)?) => {{ /* compiler built-in */ }};
        }

        /// Concatenates literals into a static string slice.
        ///
        /// This macro takes any number of comma-separated literals, yielding an
        /// expression of type `&'static str` which represents all of the literals
        /// concatenated left-to-right.
        ///
        /// Integer and floating point literals are [stringified](core::stringify) in order to be
        /// concatenated.
        ///
        /// # Examples
        ///
        /// ```
        /// let s = concat!("test", 10, 'b', true);
        /// assert_eq!(s, "test10btrue");
        /// ```
        #[stable(feature = "rust1", since = "1.0.0")]
        #[rustc_builtin_macro]
        #[rustc_diagnostic_item = "macro_concat"]
        #[macro_export]
        macro_rules! concat {
            ($($e:expr),* $(,)?) => {{ /* compiler built-in */ }};
        }

        /// Expands to the line number on which it was invoked.
        ///
        /// With [`column!`] and [`file!`], these macros provide debugging information for
        /// developers about the location within the source.
        ///
        /// The expanded expression has type `u32` and is 1-based, so the first line
        /// in each file evaluates to 1, the second to 2, etc. This is consistent
        /// with error messages by common compilers or popular editors.
        /// The returned line is *not necessarily* the line of the `line!` invocation itself,
        /// but rather the first macro invocation leading up to the invocation
        /// of the `line!` macro.
        ///
        /// # Examples
        ///
        /// ```
        /// let current_line = line!();
        /// println!("defined on line: {current_line}");
        /// ```
        #[stable(feature = "rust1", since = "1.0.0")]
        #[rustc_builtin_macro]
        #[macro_export]
        macro_rules! line {
            () => {
                /* compiler built-in */
            };
        }

        /// Expands to the column number at which it was invoked.
        ///
        /// With [`line!`] and [`file!`], these macros provide debugging information for
        /// developers about the location within the source.
        ///
        /// The expanded expression has type `u32` and is 1-based, so the first column
        /// in each line evaluates to 1, the second to 2, etc. This is consistent
        /// with error messages by common compilers or popular editors.
        /// The returned column is *not necessarily* the line of the `column!` invocation itself,
        /// but rather the first macro invocation leading up to the invocation
        /// of the `column!` macro.
        ///
        /// # Examples
        ///
        /// ```
        /// let current_col = column!();
        /// println!("defined on column: {current_col}");
        /// ```
        ///
        /// `column!` counts Unicode code points, not bytes or graphemes. As a result, the first two
        /// invocations return the same value, but the third does not.
        ///
        /// ```
        /// let a = ("foobar", column!()).1;
        /// let b = ("人之初性本善", column!()).1;
        /// let c = ("f̅o̅o̅b̅a̅r̅", column!()).1; // Uses combining overline (U+0305)
        ///
        /// assert_eq!(a, b);
        /// assert_ne!(b, c);
        /// ```
        #[stable(feature = "rust1", since = "1.0.0")]
        #[rustc_builtin_macro]
        #[macro_export]
        macro_rules! column {
            () => {
                /* compiler built-in */
            };
        }
    }
}


#[unstable(feature = "assert_matches", issue = "82775")]
/// Unstable module containing the unstable `assert_matches` macro.
pub mod assert_matches {
    #[unstable(feature = "assert_matches", issue = "82775")]
    pub use crate::macros::{assert_matches, debug_assert_matches};

}

#[unstable(feature = "derive_from", issue = "144889")]
/// Unstable module containing the unstable `From` derive macro.
pub mod from {
    #[unstable(feature = "derive_from", issue = "144889")]
    pub use crate::macros::builtin::From;
}

// We don't export this through #[macro_export] for now, to avoid breakage.
#[unstable(feature = "autodiff", issue = "124509")]
/// Unstable module containing the unstable `autodiff` macro.
pub mod autodiff {
    #[unstable(feature = "autodiff", issue = "124509")]
    pub use crate::macros::builtin::{autodiff_forward, autodiff_reverse};
}

#[unstable(feature = "contracts", issue = "128044")]
pub mod contracts
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::contracts::{*};
    */
    
}


#[unstable(feature = "cfg_select", issue = "115585")]
pub use crate::macros::cfg_select;

#[macro_use]
pub mod internal_macros
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::internal_macros::{*};
    */
    // implements the unary operator "op &T"
    // based on "op T" where T is expected to be `Copy`able
    macro_rules! forward_ref_unop {
        (impl $imp:ident, $method:ident for $t:ty, $(#[$attr:meta])+) => {
            $(#[$attr])+
            impl const $imp for &$t {
                type Output = <$t as $imp>::Output;

                #[inline]
                fn $method(self) -> <$t as $imp>::Output {
                    $imp::$method(*self)
                }
            }
        }
    }

    // implements binary operators "&T op U", "T op &U", "&T op &U"
    // based on "T op U" where T and U are expected to be `Copy`able
    macro_rules! forward_ref_binop {
        (impl $imp:ident, $method:ident for $t:ty, $u:ty, $(#[$attr:meta])+) => {
            $(#[$attr])+
            impl const $imp<$u> for &$t {
                type Output = <$t as $imp<$u>>::Output;

                #[inline]
                #[track_caller]
                fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
                    $imp::$method(*self, other)
                }
            }

            $(#[$attr])+
            impl const $imp<&$u> for $t {
                type Output = <$t as $imp<$u>>::Output;

                #[inline]
                #[track_caller]
                fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                    $imp::$method(self, *other)
                }
            }

            $(#[$attr])+
            impl const $imp<&$u> for &$t {
                type Output = <$t as $imp<$u>>::Output;

                #[inline]
                #[track_caller]
                fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                    $imp::$method(*self, *other)
                }
            }
        }
    }

    // implements "T op= &U", based on "T op= U"
    // where U is expected to be `Copy`able
    macro_rules! forward_ref_op_assign {
        (impl $imp:ident, $method:ident for $t:ty, $u:ty, $(#[$attr:meta])+) => {
            $(#[$attr])+
            impl const $imp<&$u> for $t {
                #[inline]
                #[track_caller]
                fn $method(&mut self, other: &$u) {
                    $imp::$method(self, *other);
                }
            }
        }
    }

    /// Creates a zero-size type similar to a closure type, but named.
    macro_rules! impl_fn_for_zst {
        ($(
            $( #[$attr: meta] )*
            struct $Name: ident impl$( <$( $lifetime : lifetime ),+> )? Fn =
                |$( $arg: ident: $ArgTy: ty ),*| -> $ReturnTy: ty
                $body: block;
        )+) => {
            $(
                $( #[$attr] )*
                struct $Name;

                impl $( <$( $lifetime ),+> )? Fn<($( $ArgTy, )*)> for $Name {
                    #[inline]
                    extern "rust-call" fn call(&self, ($( $arg, )*): ($( $ArgTy, )*)) -> $ReturnTy {
                        $body
                    }
                }

                impl $( <$( $lifetime ),+> )? FnMut<($( $ArgTy, )*)> for $Name {
                    #[inline]
                    extern "rust-call" fn call_mut(
                        &mut self,
                        ($( $arg, )*): ($( $ArgTy, )*)
                    ) -> $ReturnTy {
                        Fn::call(&*self, ($( $arg, )*))
                    }
                }

                impl $( <$( $lifetime ),+> )? FnOnce<($( $ArgTy, )*)> for $Name {
                    type Output = $ReturnTy;

                    #[inline]
                    extern "rust-call" fn call_once(self, ($( $arg, )*): ($( $ArgTy, )*)) -> $ReturnTy {
                        Fn::call(&self, ($( $arg, )*))
                    }
                }
            )+
        }
    }
}

pub mod legacy_int_modules
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::legacy_int_modules::{*};
    */
    
}

#[stable(feature = "rust1", since = "1.0.0")]
#[allow(clippy::useless_attribute)] // FIXME false positive (https://github.com/rust-lang/rust-clippy/issues/15636)
#[allow(deprecated_in_future)]
pub use legacy_int_modules::{i8, i16, i32, i64, isize, u8, u16, u32, u64, usize};
#[stable(feature = "i128", since = "1.26.0")]
#[allow(clippy::useless_attribute)] // FIXME false positive (https://github.com/rust-lang/rust-clippy/issues/15636)
#[allow(deprecated_in_future)]
pub use legacy_int_modules::{i128, u128};

pub mod f128
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::f128::{*};
    */
    
}

pub mod f16
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::f16::{*};
    */
    
}

pub mod f32
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::f32::{*};
    */
    
}

pub mod f64
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::f64::{*};
    */
    
}

#[macro_use]
pub mod num
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::num::{*};
    */
    
}

pub mod hint
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::hint::{*};
    */
    
}

pub mod intrinsics
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::intrinsics::{*};
    */
    
}

pub mod mem
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::mem::{*};
    */
    
}

#[unstable(feature = "profiling_marker_api", issue = "148197")]
pub mod profiling
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::profiling::{*};
    */
    
}

pub mod ptr
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::ptr::{*};
    */
    
}

#[unstable(feature = "ub_checks", issue = "none")]
pub mod ub_checks
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::ub_checks::{*};
    */
    
}

pub mod borrow
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::borrow::{*};
    */
    
}

pub mod clone
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::clone::{*};
    */
    
}

pub mod cmp
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::cmp::{*};
    */
    
}

pub mod convert
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::convert::{*};
    */
    
}

pub mod default
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::default::{*};
    */
    
}

pub mod error
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::error::{*};
    */
    
}

pub mod index
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::index::{*};
    */
    
}

pub mod marker
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::marker::{*};
    */
    
}

pub mod ops
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::ops::{*};
    */
    
}

pub mod any
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::any::{*};
    */
    
}

pub mod array
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::array::{*};
    */
    
}

pub mod ascii
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::ascii::{*};
    */
    
}

pub mod asserting
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::asserting::{*};
    */
    
}

#[unstable(feature = "async_iterator", issue = "79024")]
pub mod async_iter
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::async_iter::{*};
    */
    
}

#[unstable(feature = "bstr", issue = "134915")]
pub mod bstr
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::bstr::{*};
    */
    
}

pub mod cell
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::cell::{*};
    */
    
}

pub mod char
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::char::{*};
    */
    
}

pub mod ffi
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::ffi::{*};
    */
    
}

#[unstable(feature = "core_io_borrowed_buf", issue = "117693")]
pub mod io
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::io::{*};
    */
    
}

pub mod iter
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::iter::{*};
    */
    
}

pub mod net
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::net::{*};
    */
    
}

pub mod option
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::option::{*};
    */
    
}

pub mod os
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::os::{*};
    */
    
}

pub mod panic
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::panic::{*};
    */
    
}

pub mod panicking
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::panicking::{*};
    */
    
}

#[unstable(feature = "pattern_type_macro", issue = "123646")]
pub mod pat
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::pat::{*};
    */
    
}

pub mod pin
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::pin::{*};
    */
    
}

#[unstable(feature = "random", issue = "130703")]
pub mod random
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::random::{*};
    */
    
}

#[unstable(feature = "new_range_api", issue = "125687")]
pub mod range
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::range::{*};
    */
    
}

pub mod result
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::result::{*};
    */
    
}

pub mod sync
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::sync::{*};
    */
    
}

#[unstable(feature = "unsafe_binders", issue = "130516")]
pub mod unsafe_binder
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::unsafe_binder::{*};
    */
    
}


pub mod fmt
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::fmt::{*};
    */
    
}

pub mod hash
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::hash::{*};
    */
    
}

pub mod slice
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::slice::{*};
    */
    
}

pub mod str
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::str::{*};
    */
    
}

pub mod time
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::time::{*};
    */
    
}


pub mod wtf8
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::wtf8::{*};
    */
    
}


pub mod unicode
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::unicode::{*};
    */
    
}


/* Async */
pub mod future
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::future::{*};
    */
    
}

pub mod task
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::task::{*};
    */
    
}

#[allow(missing_docs)]
pub mod alloc
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::alloc::{*};
    */
    
}

pub mod bool
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::bool::{*};
    */
    
}

pub mod escape
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::escape::{*};
    */
    
}

pub mod tuple
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::tuple::{*};
    */
    
}

pub mod unit
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::unit::{*};
    */
    
}

#[stable(feature = "core_primitive", since = "1.43.0")]
pub mod primitive
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::primitive::{*};
    */
    
}

#[allow(
    missing_docs,
    missing_debug_implementations,
    dead_code,
    unused_imports,
    unsafe_op_in_unsafe_fn,
    ambiguous_glob_reexports,
    deprecated_in_future,
    unreachable_pub
)]
#[allow(rustdoc::bare_urls)]
pub mod core_arch
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::core_arch::{*};
    */
    
}


#[stable(feature = "simd_arch", since = "1.27.0")]
pub mod arch
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::arch::{*};
    */
    
}

#[allow(missing_debug_implementations, dead_code, unsafe_op_in_unsafe_fn)]
#[allow(rustdoc::bare_urls)]
#[unstable(feature = "portable_simd", issue = "86656")]
pub mod core_simd
{
    /*!
    */
    use ::
    {
        *
    };
    /*
    pub use std::core_simd::{*};
    */
    #[macro_use]
    pub mod swizzle
    {
        /*!
        */
        use ::
        {
            *
        };
        /*
        pub use std::swizzle::{*};
        */
        /// Constructs a new SIMD vector by copying elements from selected elements in other vectors.
        ///
        /// When swizzling one vector, elements are selected like [`Swizzle::swizzle`].
        ///
        /// When swizzling two vectors, elements are selected like [`Swizzle::concat_swizzle`].
        ///
        /// # Examples
        ///
        /// With a single SIMD vector, the const array specifies element indices in that vector:
        /// ```
        /// # #![feature(portable_simd)]
        /// # use core::simd::{u32x2, u32x4, simd_swizzle};
        /// let v = u32x4::from_array([10, 11, 12, 13]);
        ///
        /// // Keeping the same size
        /// let r: u32x4 = simd_swizzle!(v, [3, 0, 1, 2]);
        /// assert_eq!(r.to_array(), [13, 10, 11, 12]);
        ///
        /// // Changing the number of elements
        /// let r: u32x2 = simd_swizzle!(v, [3, 1]);
        /// assert_eq!(r.to_array(), [13, 11]);
        /// ```
        ///
        /// With two input SIMD vectors, the const array specifies element indices in the concatenation of
        /// those vectors:
        /// ```
        /// # #![feature(portable_simd)]
        /// # #[cfg(feature = "as_crate")] use core_simd::simd;
        /// # #[cfg(not(feature = "as_crate"))] use core::simd;
        /// # use simd::{u32x2, u32x4, simd_swizzle};
        /// let a = u32x4::from_array([0, 1, 2, 3]);
        /// let b = u32x4::from_array([4, 5, 6, 7]);
        ///
        /// // Keeping the same size
        /// let r: u32x4 = simd_swizzle!(a, b, [0, 1, 6, 7]);
        /// assert_eq!(r.to_array(), [0, 1, 6, 7]);
        ///
        /// // Changing the number of elements
        /// let r: u32x2 = simd_swizzle!(a, b, [0, 4]);
        /// assert_eq!(r.to_array(), [0, 4]);
        /// ```
        #[allow(unused_macros)]
        pub macro simd_swizzle {
            (
                $vector:expr, $index:expr $(,)?
            ) => {
                {
                    use $crate::simd::Swizzle;
                    struct Impl;
                    impl Swizzle<{$index.len()}> for Impl {
                        const INDEX: [usize; {$index.len()}] = $index;
                    }
                    Impl::swizzle($vector)
                }
            },
            (
                $first:expr, $second:expr, $index:expr $(,)?
            ) => {
                {
                    use $crate::simd::Swizzle;
                    struct Impl;
                    impl Swizzle<{$index.len()}> for Impl {
                        const INDEX: [usize; {$index.len()}] = $index;
                    }
                    Impl::concat_swizzle($first, $second)
                }
            }
        }

        /// Creates a vector from the elements of another vector.
        pub trait Swizzle<const N: usize> {
            /// Map from the elements of the input vector to the output vector.
            const INDEX: [usize; N];

            /// Creates a new vector from the elements of `vector`.
            ///
            /// Lane `i` of the output is `vector[Self::INDEX[i]]`.
            #[inline]
            #[must_use = "method returns a new vector and does not mutate the original inputs"]
            fn swizzle<T, const M: usize>(vector: Simd<T, M>) -> Simd<T, N>
            where
                T: SimdElement,
                LaneCount<N>: SupportedLaneCount,
                LaneCount<M>: SupportedLaneCount,
            {
                // Safety: `vector` is a vector, and the index is a const vector of u32.
                unsafe {
                    core::intrinsics::simd::simd_shuffle(
                        vector,
                        vector,
                        const {
                            let mut output = [0; N];
                            let mut i = 0;
                            while i < N {
                                let index = Self::INDEX[i];
                                assert!(index as u32 as usize == index);
                                assert!(
                                    index < M,
                                    "source element index exceeds input vector length"
                                );
                                output[i] = index as u32;
                                i += 1;
                            }

                            // The index list needs to be returned as a vector.
                            #[repr(simd)]
                            struct SimdShuffleIdx<const LEN: usize>([u32; LEN]);
                            SimdShuffleIdx(output)
                        },
                    )
                }
            }

            /// Creates a new vector from the elements of `first` and `second`.
            ///
            /// Lane `i` of the output is `concat[Self::INDEX[i]]`, where `concat` is the concatenation of
            /// `first` and `second`.
            #[inline]
            #[must_use = "method returns a new vector and does not mutate the original inputs"]
            fn concat_swizzle<T, const M: usize>(first: Simd<T, M>, second: Simd<T, M>) -> Simd<T, N>
            where
                T: SimdElement,
                LaneCount<N>: SupportedLaneCount,
                LaneCount<M>: SupportedLaneCount,
            {
                // Safety: `first` and `second` are vectors, and the index is a const vector of u32.
                unsafe {
                    core::intrinsics::simd::simd_shuffle(
                        first,
                        second,
                        const {
                            let mut output = [0; N];
                            let mut i = 0;
                            while i < N {
                                let index = Self::INDEX[i];
                                assert!(index as u32 as usize == index);
                                assert!(
                                    index < 2 * M,
                                    "source element index exceeds input vector length"
                                );
                                output[i] = index as u32;
                                i += 1;
                            }

                            // The index list needs to be returned as a vector.
                            #[repr(simd)]
                            struct SimdShuffleIdx<const LEN: usize>([u32; LEN]);
                            SimdShuffleIdx(output)
                        },
                    )
                }
            }

            /// Creates a new mask from the elements of `mask`.
            ///
            /// Element `i` of the output is `mask[Self::INDEX[i]]`.
            #[inline]
            #[must_use = "method returns a new mask and does not mutate the original inputs"]
            fn swizzle_mask<T, const M: usize>(mask: Mask<T, M>) -> Mask<T, N>
            where
                T: MaskElement,
                LaneCount<N>: SupportedLaneCount,
                LaneCount<M>: SupportedLaneCount,
            {
                // SAFETY: all elements of this mask come from another mask
                unsafe { Mask::from_int_unchecked(Self::swizzle(mask.to_int())) }
            }

            /// Creates a new mask from the elements of `first` and `second`.
            ///
            /// Element `i` of the output is `concat[Self::INDEX[i]]`, where `concat` is the concatenation of
            /// `first` and `second`.
            #[inline]
            #[must_use = "method returns a new mask and does not mutate the original inputs"]
            fn concat_swizzle_mask<T, const M: usize>(first: Mask<T, M>, second: Mask<T, M>) -> Mask<T, N>
            where
                T: MaskElement,
                LaneCount<N>: SupportedLaneCount,
                LaneCount<M>: SupportedLaneCount,
            {
                // SAFETY: all elements of this mask come from another mask
                unsafe { Mask::from_int_unchecked(Self::concat_swizzle(first.to_int(), second.to_int())) }
            }
        }

        impl<T, const N: usize> Simd<T, N>
        where
            T: SimdElement,
            LaneCount<N>: SupportedLaneCount,
        {
            /// Reverse the order of the elements in the vector.
            #[inline]
            #[must_use = "method returns a new vector and does not mutate the original inputs"]
            pub fn reverse(self) -> Self {
                struct Reverse;

                impl<const N: usize> Swizzle<N> for Reverse {
                    const INDEX: [usize; N] = const {
                        let mut index = [0; N];
                        let mut i = 0;
                        while i < N {
                            index[i] = N - i - 1;
                            i += 1;
                        }
                        index
                    };
                }

                Reverse::swizzle(self)
            }

            /// Rotates the vector such that the first `OFFSET` elements of the slice move to the end
            /// while the last `self.len() - OFFSET` elements move to the front. After calling `rotate_elements_left`,
            /// the element previously at index `OFFSET` will become the first element in the slice.
            /// ```
            /// # #![feature(portable_simd)]
            /// # #[cfg(feature = "as_crate")] use core_simd::simd::Simd;
            /// # #[cfg(not(feature = "as_crate"))] use core::simd::Simd;
            /// let a = Simd::from_array([0, 1, 2, 3]);
            /// let x = a.rotate_elements_left::<3>();
            /// assert_eq!(x.to_array(), [3, 0, 1, 2]);
            ///
            /// let y = a.rotate_elements_left::<7>();
            /// assert_eq!(y.to_array(), [3, 0, 1, 2]);
            /// ```
            #[inline]
            #[must_use = "method returns a new vector and does not mutate the original inputs"]
            pub fn rotate_elements_left<const OFFSET: usize>(self) -> Self {
                struct Rotate<const OFFSET: usize>;

                impl<const OFFSET: usize, const N: usize> Swizzle<N> for Rotate<OFFSET> {
                    const INDEX: [usize; N] = const {
                        let offset = OFFSET % N;
                        let mut index = [0; N];
                        let mut i = 0;
                        while i < N {
                            index[i] = (i + offset) % N;
                            i += 1;
                        }
                        index
                    };
                }

                Rotate::<OFFSET>::swizzle(self)
            }

            /// Rotates the vector such that the first `self.len() - OFFSET` elements of the vector move to
            /// the end while the last `OFFSET` elements move to the front. After calling `rotate_elements_right`,
            /// the element previously at index `self.len() - OFFSET` will become the first element in the slice.
            /// ```
            /// # #![feature(portable_simd)]
            /// # #[cfg(feature = "as_crate")] use core_simd::simd::Simd;
            /// # #[cfg(not(feature = "as_crate"))] use core::simd::Simd;
            /// let a = Simd::from_array([0, 1, 2, 3]);
            /// let x = a.rotate_elements_right::<3>();
            /// assert_eq!(x.to_array(), [1, 2, 3, 0]);
            ///
            /// let y = a.rotate_elements_right::<7>();
            /// assert_eq!(y.to_array(), [1, 2, 3, 0]);
            /// ```
            #[inline]
            #[must_use = "method returns a new vector and does not mutate the original inputs"]
            pub fn rotate_elements_right<const OFFSET: usize>(self) -> Self {
                struct Rotate<const OFFSET: usize>;

                impl<const OFFSET: usize, const N: usize> Swizzle<N> for Rotate<OFFSET> {
                    const INDEX: [usize; N] = const {
                        let offset = N - OFFSET % N;
                        let mut index = [0; N];
                        let mut i = 0;
                        while i < N {
                            index[i] = (i + offset) % N;
                            i += 1;
                        }
                        index
                    };
                }

                Rotate::<OFFSET>::swizzle(self)
            }

            /// Shifts the vector elements to the left by `OFFSET`, filling in with
            /// `padding` from the right.
            /// ```
            /// # #![feature(portable_simd)]
            /// # #[cfg(feature = "as_crate")] use core_simd::simd::Simd;
            /// # #[cfg(not(feature = "as_crate"))] use core::simd::Simd;
            /// let a = Simd::from_array([0, 1, 2, 3]);
            /// let x = a.shift_elements_left::<3>(255);
            /// assert_eq!(x.to_array(), [3, 255, 255, 255]);
            ///
            /// let y = a.shift_elements_left::<7>(255);
            /// assert_eq!(y.to_array(), [255, 255, 255, 255]);
            /// ```
            #[inline]
            #[must_use = "method returns a new vector and does not mutate the original inputs"]
            pub fn shift_elements_left<const OFFSET: usize>(self, padding: T) -> Self {
                struct Shift<const OFFSET: usize>;

                impl<const OFFSET: usize, const N: usize> Swizzle<N> for Shift<OFFSET> {
                    const INDEX: [usize; N] = const {
                        let mut index = [N; N];
                        let mut i = 0;
                        while i + OFFSET < N {
                            index[i] = i + OFFSET;
                            i += 1;
                        }
                        index
                    };
                }

                Shift::<OFFSET>::concat_swizzle(self, Simd::splat(padding))
            }

            /// Shifts the vector elements to the right by `OFFSET`, filling in with
            /// `padding` from the left.
            /// ```
            /// # #![feature(portable_simd)]
            /// # #[cfg(feature = "as_crate")] use core_simd::simd::Simd;
            /// # #[cfg(not(feature = "as_crate"))] use core::simd::Simd;
            /// let a = Simd::from_array([0, 1, 2, 3]);
            /// let x = a.shift_elements_right::<3>(255);
            /// assert_eq!(x.to_array(), [255, 255, 255, 0]);
            ///
            /// let y = a.shift_elements_right::<7>(255);
            /// assert_eq!(y.to_array(), [255, 255, 255, 255]);
            /// ```
            #[inline]
            #[must_use = "method returns a new vector and does not mutate the original inputs"]
            pub fn shift_elements_right<const OFFSET: usize>(self, padding: T) -> Self {
                struct Shift<const OFFSET: usize>;

                impl<const OFFSET: usize, const N: usize> Swizzle<N> for Shift<OFFSET> {
                    const INDEX: [usize; N] = const {
                        let mut index = [N; N];
                        let mut i = OFFSET;
                        while i < N {
                            index[i] = i - OFFSET;
                            i += 1;
                        }
                        index
                    };
                }

                Shift::<OFFSET>::concat_swizzle(self, Simd::splat(padding))
            }

            /// Interleave two vectors.
            ///
            /// The resulting vectors contain elements taken alternatively from `self` and `other`, first
            /// filling the first result, and then the second.
            ///
            /// The reverse of this operation is [`Simd::deinterleave`].
            ///
            /// ```
            /// # #![feature(portable_simd)]
            /// # use core::simd::Simd;
            /// let a = Simd::from_array([0, 1, 2, 3]);
            /// let b = Simd::from_array([4, 5, 6, 7]);
            /// let (x, y) = a.interleave(b);
            /// assert_eq!(x.to_array(), [0, 4, 1, 5]);
            /// assert_eq!(y.to_array(), [2, 6, 3, 7]);
            /// ```
            #[inline]
            #[must_use = "method returns a new vector and does not mutate the original inputs"]
            pub fn interleave(self, other: Self) -> (Self, Self) {
                const fn interleave<const N: usize>(high: bool) -> [usize; N] {
                    let mut idx = [0; N];
                    let mut i = 0;
                    while i < N {
                        let dst_index = if high { i + N } else { i };
                        let src_index = dst_index / 2 + (dst_index % 2) * N;
                        idx[i] = src_index;
                        i += 1;
                    }
                    idx
                }

                struct Lo;
                struct Hi;

                impl<const N: usize> Swizzle<N> for Lo {
                    const INDEX: [usize; N] = interleave::<N>(false);
                }

                impl<const N: usize> Swizzle<N> for Hi {
                    const INDEX: [usize; N] = interleave::<N>(true);
                }

                (
                    Lo::concat_swizzle(self, other),
                    Hi::concat_swizzle(self, other),
                )
            }

            /// Deinterleave two vectors.
            ///
            /// The first result takes every other element of `self` and then `other`, starting with
            /// the first element.
            ///
            /// The second result takes every other element of `self` and then `other`, starting with
            /// the second element.
            ///
            /// The reverse of this operation is [`Simd::interleave`].
            ///
            /// ```
            /// # #![feature(portable_simd)]
            /// # #[cfg(feature = "as_crate")] use core_simd::simd;
            /// # #[cfg(not(feature = "as_crate"))] use core::simd;
            /// # use simd::Simd;
            /// let a = Simd::from_array([0, 4, 1, 5]);
            /// let b = Simd::from_array([2, 6, 3, 7]);
            /// let (x, y) = a.deinterleave(b);
            /// assert_eq!(x.to_array(), [0, 1, 2, 3]);
            /// assert_eq!(y.to_array(), [4, 5, 6, 7]);
            /// ```
            #[inline]
            #[must_use = "method returns a new vector and does not mutate the original inputs"]
            pub fn deinterleave(self, other: Self) -> (Self, Self) {
                const fn deinterleave<const N: usize>(second: bool) -> [usize; N] {
                    let mut idx = [0; N];
                    let mut i = 0;
                    while i < N {
                        idx[i] = i * 2 + second as usize;
                        i += 1;
                    }
                    idx
                }

                struct Even;
                struct Odd;

                impl<const N: usize> Swizzle<N> for Even {
                    const INDEX: [usize; N] = deinterleave::<N>(false);
                }

                impl<const N: usize> Swizzle<N> for Odd {
                    const INDEX: [usize; N] = deinterleave::<N>(true);
                }

                (
                    Even::concat_swizzle(self, other),
                    Odd::concat_swizzle(self, other),
                )
            }

            /// Resize a vector.
            ///
            /// If `M` > `N`, extends the length of a vector, setting the new elements to `value`.
            /// If `M` < `N`, truncates the vector to the first `M` elements.
            ///
            /// ```
            /// # #![feature(portable_simd)]
            /// # #[cfg(feature = "as_crate")] use core_simd::simd;
            /// # #[cfg(not(feature = "as_crate"))] use core::simd;
            /// # use simd::u32x4;
            /// let x = u32x4::from_array([0, 1, 2, 3]);
            /// assert_eq!(x.resize::<8>(9).to_array(), [0, 1, 2, 3, 9, 9, 9, 9]);
            /// assert_eq!(x.resize::<2>(9).to_array(), [0, 1]);
            /// ```
            #[inline]
            #[must_use = "method returns a new vector and does not mutate the original inputs"]
            pub fn resize<const M: usize>(self, value: T) -> Simd<T, M>
            where
                LaneCount<M>: SupportedLaneCount,
            {
                struct Resize<const N: usize>;
                impl<const N: usize, const M: usize> Swizzle<M> for Resize<N> {
                    const INDEX: [usize; M] = const {
                        let mut index = [0; M];
                        let mut i = 0;
                        while i < M {
                            index[i] = if i < N { i } else { N };
                            i += 1;
                        }
                        index
                    };
                }
                Resize::<N>::concat_swizzle(self, Simd::splat(value))
            }

            /// Extract a vector from another vector.
            ///
            /// ```
            /// # #![feature(portable_simd)]
            /// # #[cfg(feature = "as_crate")] use core_simd::simd;
            /// # #[cfg(not(feature = "as_crate"))] use core::simd;
            /// # use simd::u32x4;
            /// let x = u32x4::from_array([0, 1, 2, 3]);
            /// assert_eq!(x.extract::<1, 2>().to_array(), [1, 2]);
            /// ```
            #[inline]
            #[must_use = "method returns a new vector and does not mutate the original inputs"]
            pub fn extract<const START: usize, const LEN: usize>(self) -> Simd<T, LEN>
            where
                LaneCount<LEN>: SupportedLaneCount,
            {
                struct Extract<const N: usize, const START: usize>;
                impl<const N: usize, const START: usize, const LEN: usize> Swizzle<LEN> for Extract<N, START> {
                    const INDEX: [usize; LEN] = const {
                        assert!(START + LEN <= N, "index out of bounds");
                        let mut index = [0; LEN];
                        let mut i = 0;
                        while i < LEN {
                            index[i] = START + i;
                            i += 1;
                        }
                        index
                    };
                }
                Extract::<N, START>::swizzle(self)
            }
        }

        impl<T, const N: usize> Mask<T, N>
        where
            T: MaskElement,
            LaneCount<N>: SupportedLaneCount,
        {
            /// Reverse the order of the elements in the mask.
            #[inline]
            #[must_use = "method returns a new vector and does not mutate the original inputs"]
            pub fn reverse(self) -> Self {
                // Safety: swizzles are safe for masks
                unsafe { Self::from_int_unchecked(self.to_int().reverse()) }
            }

            /// Rotates the mask such that the first `OFFSET` elements of the slice move to the end
            /// while the last `self.len() - OFFSET` elements move to the front. After calling `rotate_elements_left`,
            /// the element previously at index `OFFSET` will become the first element in the slice.
            #[inline]
            #[must_use = "method returns a new vector and does not mutate the original inputs"]
            pub fn rotate_elements_left<const OFFSET: usize>(self) -> Self {
                // Safety: swizzles are safe for masks
                unsafe { Self::from_int_unchecked(self.to_int().rotate_elements_left::<OFFSET>()) }
            }

            /// Rotates the mask such that the first `self.len() - OFFSET` elements of the mask move to
            /// the end while the last `OFFSET` elements move to the front. After calling `rotate_elements_right`,
            /// the element previously at index `self.len() - OFFSET` will become the first element in the slice.
            #[inline]
            #[must_use = "method returns a new vector and does not mutate the original inputs"]
            pub fn rotate_elements_right<const OFFSET: usize>(self) -> Self {
                // Safety: swizzles are safe for masks
                unsafe { Self::from_int_unchecked(self.to_int().rotate_elements_right::<OFFSET>()) }
            }

            /// Shifts the mask elements to the left by `OFFSET`, filling in with
            /// `padding` from the right.
            #[inline]
            #[must_use = "method returns a new mask and does not mutate the original inputs"]
            pub fn shift_elements_left<const OFFSET: usize>(self, padding: bool) -> Self {
                // Safety: swizzles are safe for masks
                unsafe {
                    Self::from_int_unchecked(self.to_int().shift_elements_left::<OFFSET>(if padding {
                        T::TRUE
                    } else {
                        T::FALSE
                    }))
                }
            }

            /// Shifts the mask elements to the right by `OFFSET`, filling in with
            /// `padding` from the left.
            #[inline]
            #[must_use = "method returns a new mask and does not mutate the original inputs"]
            pub fn shift_elements_right<const OFFSET: usize>(self, padding: bool) -> Self {
                // Safety: swizzles are safe for masks
                unsafe {
                    Self::from_int_unchecked(self.to_int().shift_elements_right::<OFFSET>(if padding {
                        T::TRUE
                    } else {
                        T::FALSE
                    }))
                }
            }

            /// Interleave two masks.
            ///
            /// The resulting masks contain elements taken alternatively from `self` and `other`, first
            /// filling the first result, and then the second.
            ///
            /// The reverse of this operation is [`Mask::deinterleave`].
            ///
            /// ```
            /// # #![feature(portable_simd)]
            /// # #[cfg(feature = "as_crate")] use core_simd::simd;
            /// # #[cfg(not(feature = "as_crate"))] use core::simd;
            /// # use simd::mask32x4;
            /// let a = mask32x4::from_array([false, true, false, true]);
            /// let b = mask32x4::from_array([false, false, true, true]);
            /// let (x, y) = a.interleave(b);
            /// assert_eq!(x.to_array(), [false, false, true, false]);
            /// assert_eq!(y.to_array(), [false, true, true, true]);
            /// ```
            #[inline]
            #[must_use = "method returns a new vector and does not mutate the original inputs"]
            pub fn interleave(self, other: Self) -> (Self, Self) {
                let (lo, hi) = self.to_int().interleave(other.to_int());
                // Safety: swizzles are safe for masks
                unsafe { (Self::from_int_unchecked(lo), Self::from_int_unchecked(hi)) }
            }

            /// Deinterleave two masks.
            ///
            /// The first result takes every other element of `self` and then `other`, starting with
            /// the first element.
            ///
            /// The second result takes every other element of `self` and then `other`, starting with
            /// the second element.
            ///
            /// The reverse of this operation is [`Mask::interleave`].
            ///
            /// ```
            /// # #![feature(portable_simd)]
            /// # #[cfg(feature = "as_crate")] use core_simd::simd;
            /// # #[cfg(not(feature = "as_crate"))] use core::simd;
            /// # use simd::mask32x4;
            /// let a = mask32x4::from_array([false, true, false, true]);
            /// let b = mask32x4::from_array([false, false, true, true]);
            /// let (x, y) = a.deinterleave(b);
            /// assert_eq!(x.to_array(), [false, false, false, true]);
            /// assert_eq!(y.to_array(), [true, true, false, true]);
            /// ```
            #[inline]
            #[must_use = "method returns a new vector and does not mutate the original inputs"]
            pub fn deinterleave(self, other: Self) -> (Self, Self) {
                let (even, odd) = self.to_int().deinterleave(other.to_int());
                // Safety: swizzles are safe for masks
                unsafe {
                    (
                        Self::from_int_unchecked(even),
                        Self::from_int_unchecked(odd),
                    )
                }
            }

            /// Resize a mask.
            ///
            /// If `M` > `N`, extends the length of a mask, setting the new elements to `value`.
            /// If `M` < `N`, truncates the mask to the first `M` elements.
            ///
            /// ```
            /// # #![feature(portable_simd)]
            /// # #[cfg(feature = "as_crate")] use core_simd::simd;
            /// # #[cfg(not(feature = "as_crate"))] use core::simd;
            /// # use simd::mask32x4;
            /// let x = mask32x4::from_array([false, true, true, false]);
            /// assert_eq!(x.resize::<8>(true).to_array(), [false, true, true, false, true, true, true, true]);
            /// assert_eq!(x.resize::<2>(true).to_array(), [false, true]);
            /// ```
            #[inline]
            #[must_use = "method returns a new vector and does not mutate the original inputs"]
            pub fn resize<const M: usize>(self, value: bool) -> Mask<T, M>
            where
                LaneCount<M>: SupportedLaneCount,
            {
                // Safety: swizzles are safe for masks
                unsafe {
                    Mask::<T, M>::from_int_unchecked(self.to_int().resize::<M>(if value {
                        T::TRUE
                    } else {
                        T::FALSE
                    }))
                }
            }

            /// Extract a vector from another vector.
            ///
            /// ```
            /// # #![feature(portable_simd)]
            /// # #[cfg(feature = "as_crate")] use core_simd::simd;
            /// # #[cfg(not(feature = "as_crate"))] use core::simd;
            /// # use simd::mask32x4;
            /// let x = mask32x4::from_array([false, true, true, false]);
            /// assert_eq!(x.extract::<1, 2>().to_array(), [true, true]);
            /// ```
            #[inline]
            #[must_use = "method returns a new vector and does not mutate the original inputs"]
            pub fn extract<const START: usize, const LEN: usize>(self) -> Mask<T, LEN>
            where
                LaneCount<LEN>: SupportedLaneCount,
            {
                // Safety: swizzles are safe for masks
                unsafe { Mask::<T, LEN>::from_int_unchecked(self.to_int().extract::<START, LEN>()) }
            }
        }
    }

    pub mod alias
    {
        /*!
        */
        use ::
        {
            *
        };
        /*
        pub use std::alias::{*};
        */
        
    }

    pub mod cast
    {
        /*!
        */
        use ::
        {
            *
        };
        /*
        pub use std::cast::{*};
        */
        
    }

    pub mod fmt
    {
        /*!
        */
        use ::
        {
            *
        };
        /*
        pub use std::fmt::{*};
        */
        
    }

    pub mod iter
    {
        /*!
        */
        use ::
        {
            *
        };
        /*
        pub use std::iter::{*};
        */
        
    }

    pub mod lane_count
    {
        /*!
        */
        use ::
        {
            *
        };
        /*
        pub use std::lane_count::{*};
        */
        
    }

    pub mod masks
    {
        /*!
        */
        use ::
        {
            *
        };
        /*
        pub use std::masks::{*};
        use crate::simd::{LaneCount, Simd, SimdCast, SimdElement, SupportedLaneCount};
        use core::cmp::Ordering;
        use core::{fmt, mem};
        */
        pub mod mask_impl
        {
            /*!
            */
            use ::
            {
                *
            };
            /*
            pub use std::lane_count::{*};
            use crate::simd::{LaneCount, MaskElement, Simd, SupportedLaneCount};

            */
            #[repr(transparent)]
            pub(crate) struct Mask<T, const N: usize>(Simd<T, N>)
            where
                T: MaskElement,
                LaneCount<N>: SupportedLaneCount;

            impl<T, const N: usize> Copy for Mask<T, N>
            where
                T: MaskElement,
                LaneCount<N>: SupportedLaneCount,
            {
            }

            impl<T, const N: usize> Clone for Mask<T, N>
            where
                T: MaskElement,
                LaneCount<N>: SupportedLaneCount,
            {
                #[inline]
                fn clone(&self) -> Self {
                    *self
                }
            }

            impl<T, const N: usize> PartialEq for Mask<T, N>
            where
                T: MaskElement + PartialEq,
                LaneCount<N>: SupportedLaneCount,
            {
                #[inline]
                fn eq(&self, other: &Self) -> bool {
                    self.0.eq(&other.0)
                }
            }

            impl<T, const N: usize> PartialOrd for Mask<T, N>
            where
                T: MaskElement + PartialOrd,
                LaneCount<N>: SupportedLaneCount,
            {
                #[inline]
                fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                    self.0.partial_cmp(&other.0)
                }
            }

            impl<T, const N: usize> Eq for Mask<T, N>
            where
                T: MaskElement + Eq,
                LaneCount<N>: SupportedLaneCount,
            {
            }

            impl<T, const N: usize> Ord for Mask<T, N>
            where
                T: MaskElement + Ord,
                LaneCount<N>: SupportedLaneCount,
            {
                #[inline]
                fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                    self.0.cmp(&other.0)
                }
            }

            // Used for bitmask bit order workaround
            pub(crate) trait ReverseBits {
                // Reverse the least significant `n` bits of `self`.
                // (Remaining bits must be 0.)
                fn reverse_bits(self, n: usize) -> Self;
            }

            macro_rules! impl_reverse_bits {
                { $($int:ty),* } => {
                    $(
                    impl ReverseBits for $int {
                        #[inline(always)]
                        fn reverse_bits(self, n: usize) -> Self {
                            let rev = <$int>::reverse_bits(self);
                            let bitsize = size_of::<$int>() * 8;
                            if n < bitsize {
                                // Shift things back to the right
                                rev >> (bitsize - n)
                            } else {
                                rev
                            }
                        }
                    }
                    )*
                }
            }

            impl_reverse_bits! { u8, u16, u32, u64 }

            impl<T, const N: usize> Mask<T, N>
            where
                T: MaskElement,
                LaneCount<N>: SupportedLaneCount,
            {
                #[inline]
                #[must_use = "method returns a new mask and does not mutate the original value"]
                pub(crate) fn splat(value: bool) -> Self {
                    Self(Simd::splat(if value { T::TRUE } else { T::FALSE }))
                }

                #[inline]
                #[must_use = "method returns a new bool and does not mutate the original value"]
                pub(crate) unsafe fn test_unchecked(&self, lane: usize) -> bool {
                    T::eq(self.0[lane], T::TRUE)
                }

                #[inline]
                pub(crate) unsafe fn set_unchecked(&mut self, lane: usize, value: bool) {
                    self.0[lane] = if value { T::TRUE } else { T::FALSE }
                }

                #[inline]
                #[must_use = "method returns a new vector and does not mutate the original value"]
                pub(crate) fn to_int(self) -> Simd<T, N> {
                    self.0
                }

                #[inline]
                #[must_use = "method returns a new mask and does not mutate the original value"]
                pub(crate) unsafe fn from_int_unchecked(value: Simd<T, N>) -> Self {
                    Self(value)
                }

                #[inline]
                #[must_use = "method returns a new mask and does not mutate the original value"]
                pub(crate) fn convert<U>(self) -> Mask<U, N>
                where
                    U: MaskElement,
                {
                    // Safety: masks are simply integer vectors of 0 and -1, and we can cast the element type.
                    unsafe { Mask(core::intrinsics::simd::simd_cast(self.0)) }
                }

                #[inline]
                unsafe fn to_bitmask_impl<U: ReverseBits, const M: usize>(self) -> U
                where
                    LaneCount<M>: SupportedLaneCount,
                {
                    let resized = self.to_int().resize::<M>(T::FALSE);

                    // Safety: `resized` is an integer vector with length M, which must match T
                    let bitmask: U = unsafe { core::intrinsics::simd::simd_bitmask(resized) };

                    // LLVM assumes bit order should match endianness
                    if cfg!(target_endian = "big") {
                        bitmask.reverse_bits(M)
                    } else {
                        bitmask
                    }
                }

                #[inline]
                unsafe fn from_bitmask_impl<U: ReverseBits, const M: usize>(bitmask: U) -> Self
                where
                    LaneCount<M>: SupportedLaneCount,
                {
                    // LLVM assumes bit order should match endianness
                    let bitmask = if cfg!(target_endian = "big") {
                        bitmask.reverse_bits(M)
                    } else {
                        bitmask
                    };

                    // SAFETY: `mask` is the correct bitmask type for a u64 bitmask
                    let mask: Simd<T, M> = unsafe {
                        core::intrinsics::simd::simd_select_bitmask(
                            bitmask,
                            Simd::<T, M>::splat(T::TRUE),
                            Simd::<T, M>::splat(T::FALSE),
                        )
                    };

                    // SAFETY: `mask` only contains `T::TRUE` or `T::FALSE`
                    unsafe { Self::from_int_unchecked(mask.resize::<N>(T::FALSE)) }
                }

                #[inline]
                pub(crate) fn to_bitmask_integer(self) -> u64 {
                    // TODO modify simd_bitmask to zero-extend output, making this unnecessary
                    if N <= 8 {
                        // Safety: bitmask matches length
                        unsafe { self.to_bitmask_impl::<u8, 8>() as u64 }
                    } else if N <= 16 {
                        // Safety: bitmask matches length
                        unsafe { self.to_bitmask_impl::<u16, 16>() as u64 }
                    } else if N <= 32 {
                        // Safety: bitmask matches length
                        unsafe { self.to_bitmask_impl::<u32, 32>() as u64 }
                    } else {
                        // Safety: bitmask matches length
                        unsafe { self.to_bitmask_impl::<u64, 64>() }
                    }
                }

                #[inline]
                pub(crate) fn from_bitmask_integer(bitmask: u64) -> Self {
                    // TODO modify simd_bitmask_select to truncate input, making this unnecessary
                    if N <= 8 {
                        // Safety: bitmask matches length
                        unsafe { Self::from_bitmask_impl::<u8, 8>(bitmask as u8) }
                    } else if N <= 16 {
                        // Safety: bitmask matches length
                        unsafe { Self::from_bitmask_impl::<u16, 16>(bitmask as u16) }
                    } else if N <= 32 {
                        // Safety: bitmask matches length
                        unsafe { Self::from_bitmask_impl::<u32, 32>(bitmask as u32) }
                    } else {
                        // Safety: bitmask matches length
                        unsafe { Self::from_bitmask_impl::<u64, 64>(bitmask) }
                    }
                }

                #[inline]
                #[must_use = "method returns a new bool and does not mutate the original value"]
                pub(crate) fn any(self) -> bool {
                    // Safety: use `self` as an integer vector
                    unsafe { core::intrinsics::simd::simd_reduce_any(self.to_int()) }
                }

                #[inline]
                #[must_use = "method returns a new bool and does not mutate the original value"]
                pub(crate) fn all(self) -> bool {
                    // Safety: use `self` as an integer vector
                    unsafe { core::intrinsics::simd::simd_reduce_all(self.to_int()) }
                }
            }

            impl<T, const N: usize> From<Mask<T, N>> for Simd<T, N>
            where
                T: MaskElement,
                LaneCount<N>: SupportedLaneCount,
            {
                #[inline]
                fn from(value: Mask<T, N>) -> Self {
                    value.0
                }
            }

            impl<T, const N: usize> core::ops::BitAnd for Mask<T, N>
            where
                T: MaskElement,
                LaneCount<N>: SupportedLaneCount,
            {
                type Output = Self;
                #[inline]
                fn bitand(self, rhs: Self) -> Self {
                    // Safety: `self` is an integer vector
                    unsafe { Self(core::intrinsics::simd::simd_and(self.0, rhs.0)) }
                }
            }

            impl<T, const N: usize> core::ops::BitOr for Mask<T, N>
            where
                T: MaskElement,
                LaneCount<N>: SupportedLaneCount,
            {
                type Output = Self;
                #[inline]
                fn bitor(self, rhs: Self) -> Self {
                    // Safety: `self` is an integer vector
                    unsafe { Self(core::intrinsics::simd::simd_or(self.0, rhs.0)) }
                }
            }

            impl<T, const N: usize> core::ops::BitXor for Mask<T, N>
            where
                T: MaskElement,
                LaneCount<N>: SupportedLaneCount,
            {
                type Output = Self;
                #[inline]
                fn bitxor(self, rhs: Self) -> Self {
                    // Safety: `self` is an integer vector
                    unsafe { Self(core::intrinsics::simd::simd_xor(self.0, rhs.0)) }
                }
            }

            impl<T, const N: usize> core::ops::Not for Mask<T, N>
            where
                T: MaskElement,
                LaneCount<N>: SupportedLaneCount,
            {
                type Output = Self;
                #[inline]
                fn not(self) -> Self::Output {
                    Self::splat(true) ^ self
                }
            }
        }
        mod sealed 
        {
            use super::*;

            /// Not only does this seal the `MaskElement` trait, but these functions prevent other traits
            /// from bleeding into the parent bounds.
            ///
            /// For example, `eq` could be provided by requiring `MaskElement: PartialEq`, but that would
            /// prevent us from ever removing that bound, or from implementing `MaskElement` on
            /// non-`PartialEq` types in the future.
            pub trait Sealed {
                fn valid<const N: usize>(values: Simd<Self, N>) -> bool
                where
                    LaneCount<N>: SupportedLaneCount,
                    Self: SimdElement;

                fn eq(self, other: Self) -> bool;

                fn to_usize(self) -> usize;
                fn max_unsigned() -> u64;

                type Unsigned: SimdElement;

                const TRUE: Self;

                const FALSE: Self;
            }
        }
        use sealed::Sealed;

        /// Marker trait for types that may be used as SIMD mask elements.
        ///
        /// # Safety
        /// Type must be a signed integer.
        pub unsafe trait MaskElement: SimdElement<Mask = Self> + SimdCast + Sealed {}

        macro_rules! impl_element {
            { $ty:ty, $unsigned:ty } => {
                impl Sealed for $ty {
                    #[inline]
                    fn valid<const N: usize>(value: Simd<Self, N>) -> bool
                    where
                        LaneCount<N>: SupportedLaneCount,
                    {
                        // We can't use `Simd` directly, because `Simd`'s functions call this function and
                        // we will end up with an infinite loop.
                        // Safety: `value` is an integer vector
                        unsafe {
                            use core::intrinsics::simd;
                            let falses: Simd<Self, N> = simd::simd_eq(value, Simd::splat(0 as _));
                            let trues: Simd<Self, N> = simd::simd_eq(value, Simd::splat(-1 as _));
                            let valid: Simd<Self, N> = simd::simd_or(falses, trues);
                            simd::simd_reduce_all(valid)
                        }
                    }

                    #[inline]
                    fn eq(self, other: Self) -> bool { self == other }

                    #[inline]
                    fn to_usize(self) -> usize {
                        self as usize
                    }

                    #[inline]
                    fn max_unsigned() -> u64 {
                        <$unsigned>::MAX as u64
                    }

                    type Unsigned = $unsigned;

                    const TRUE: Self = -1;
                    const FALSE: Self = 0;
                }

                // Safety: this is a valid mask element type
                unsafe impl MaskElement for $ty {}
            }
        }

        impl_element! { i8, u8 }
        impl_element! { i16, u16 }
        impl_element! { i32, u32 }
        impl_element! { i64, u64 }
        impl_element! { isize, usize }

        /// A SIMD vector mask for `N` elements of width specified by `Element`.
        ///
        /// Masks represent boolean inclusion/exclusion on a per-element basis.
        ///
        /// The layout of this type is unspecified, and may change between platforms
        /// and/or Rust versions, and code should not assume that it is equivalent to
        /// `[T; N]`.
        #[repr(transparent)]
        pub struct Mask<T, const N: usize>(mask_impl::Mask<T, N>)
        where
            T: MaskElement,
            LaneCount<N>: SupportedLaneCount;

        impl<T, const N: usize> Copy for Mask<T, N>
        where
            T: MaskElement,
            LaneCount<N>: SupportedLaneCount,
        {
        }

        impl<T, const N: usize> Clone for Mask<T, N>
        where
            T: MaskElement,
            LaneCount<N>: SupportedLaneCount,
        {
            #[inline]
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<T, const N: usize> Mask<T, N>
        where
            T: MaskElement,
            LaneCount<N>: SupportedLaneCount,
        {
            /// Constructs a mask by setting all elements to the given value.
            #[inline]
            pub fn splat(value: bool) -> Self {
                Self(mask_impl::Mask::splat(value))
            }

            /// Converts an array of bools to a SIMD mask.
            #[inline]
            pub fn from_array(array: [bool; N]) -> Self {
                // SAFETY: Rust's bool has a layout of 1 byte (u8) with a value of
                //     true:    0b_0000_0001
                //     false:   0b_0000_0000
                // Thus, an array of bools is also a valid array of bytes: [u8; N]
                // This would be hypothetically valid as an "in-place" transmute,
                // but these are "dependently-sized" types, so copy elision it is!
                unsafe {
                    let bytes: [u8; N] = mem::transmute_copy(&array);
                    let bools: Simd<i8, N> =
                        core::intrinsics::simd::simd_ne(Simd::from_array(bytes), Simd::splat(0u8));
                    Mask::from_int_unchecked(core::intrinsics::simd::simd_cast(bools))
                }
            }

            /// Converts a SIMD mask to an array of bools.
            #[inline]
            pub fn to_array(self) -> [bool; N] {
                // This follows mostly the same logic as from_array.
                // SAFETY: Rust's bool has a layout of 1 byte (u8) with a value of
                //     true:    0b_0000_0001
                //     false:   0b_0000_0000
                // Thus, an array of bools is also a valid array of bytes: [u8; N]
                // Since our masks are equal to integers where all bits are set,
                // we can simply convert them to i8s, and then bitand them by the
                // bitpattern for Rust's "true" bool.
                // This would be hypothetically valid as an "in-place" transmute,
                // but these are "dependently-sized" types, so copy elision it is!
                unsafe {
                    let mut bytes: Simd<i8, N> = core::intrinsics::simd::simd_cast(self.to_int());
                    bytes &= Simd::splat(1i8);
                    mem::transmute_copy(&bytes)
                }
            }

            /// Converts a vector of integers to a mask, where 0 represents `false` and -1
            /// represents `true`.
            ///
            /// # Safety
            /// All elements must be either 0 or -1.
            #[inline]
            #[must_use = "method returns a new mask and does not mutate the original value"]
            pub unsafe fn from_int_unchecked(value: Simd<T, N>) -> Self {
                // Safety: the caller must confirm this invariant
                unsafe {
                    core::intrinsics::assume(<T as Sealed>::valid(value));
                    Self(mask_impl::Mask::from_int_unchecked(value))
                }
            }

            /// Converts a vector of integers to a mask, where 0 represents `false` and -1
            /// represents `true`.
            ///
            /// # Panics
            /// Panics if any element is not 0 or -1.
            #[inline]
            #[must_use = "method returns a new mask and does not mutate the original value"]
            #[track_caller]
            pub fn from_int(value: Simd<T, N>) -> Self {
                assert!(T::valid(value), "all values must be either 0 or -1",);
                // Safety: the validity has been checked
                unsafe { Self::from_int_unchecked(value) }
            }

            /// Converts the mask to a vector of integers, where 0 represents `false` and -1
            /// represents `true`.
            #[inline]
            #[must_use = "method returns a new vector and does not mutate the original value"]
            pub fn to_int(self) -> Simd<T, N> {
                self.0.to_int()
            }

            /// Converts the mask to a mask of any other element size.
            #[inline]
            #[must_use = "method returns a new mask and does not mutate the original value"]
            pub fn cast<U: MaskElement>(self) -> Mask<U, N> {
                Mask(self.0.convert())
            }

            /// Tests the value of the specified element.
            ///
            /// # Safety
            /// `index` must be less than `self.len()`.
            #[inline]
            #[must_use = "method returns a new bool and does not mutate the original value"]
            pub unsafe fn test_unchecked(&self, index: usize) -> bool {
                // Safety: the caller must confirm this invariant
                unsafe { self.0.test_unchecked(index) }
            }

            /// Tests the value of the specified element.
            ///
            /// # Panics
            /// Panics if `index` is greater than or equal to the number of elements in the vector.
            #[inline]
            #[must_use = "method returns a new bool and does not mutate the original value"]
            #[track_caller]
            pub fn test(&self, index: usize) -> bool {
                assert!(index < N, "element index out of range");
                // Safety: the element index has been checked
                unsafe { self.test_unchecked(index) }
            }

            /// Sets the value of the specified element.
            ///
            /// # Safety
            /// `index` must be less than `self.len()`.
            #[inline]
            pub unsafe fn set_unchecked(&mut self, index: usize, value: bool) {
                // Safety: the caller must confirm this invariant
                unsafe {
                    self.0.set_unchecked(index, value);
                }
            }

            /// Sets the value of the specified element.
            ///
            /// # Panics
            /// Panics if `index` is greater than or equal to the number of elements in the vector.
            #[inline]
            #[track_caller]
            pub fn set(&mut self, index: usize, value: bool) {
                assert!(index < N, "element index out of range");
                // Safety: the element index has been checked
                unsafe {
                    self.set_unchecked(index, value);
                }
            }

            /// Returns true if any element is set, or false otherwise.
            #[inline]
            #[must_use = "method returns a new bool and does not mutate the original value"]
            pub fn any(self) -> bool {
                self.0.any()
            }

            /// Returns true if all elements are set, or false otherwise.
            #[inline]
            #[must_use = "method returns a new bool and does not mutate the original value"]
            pub fn all(self) -> bool {
                self.0.all()
            }

            /// Creates a bitmask from a mask.
            ///
            /// Each bit is set if the corresponding element in the mask is `true`.
            /// If the mask contains more than 64 elements, the bitmask is truncated to the first 64.
            #[inline]
            #[must_use = "method returns a new integer and does not mutate the original value"]
            pub fn to_bitmask(self) -> u64 {
                self.0.to_bitmask_integer()
            }

            /// Creates a mask from a bitmask.
            ///
            /// For each bit, if it is set, the corresponding element in the mask is set to `true`.
            /// If the mask contains more than 64 elements, the remainder are set to `false`.
            #[inline]
            #[must_use = "method returns a new mask and does not mutate the original value"]
            pub fn from_bitmask(bitmask: u64) -> Self {
                Self(mask_impl::Mask::from_bitmask_integer(bitmask))
            }

            /// Finds the index of the first set element.
            ///
            /// ```
            /// # #![feature(portable_simd)]
            /// # #[cfg(feature = "as_crate")] use core_simd::simd;
            /// # #[cfg(not(feature = "as_crate"))] use core::simd;
            /// # use simd::mask32x8;
            /// assert_eq!(mask32x8::splat(false).first_set(), None);
            /// assert_eq!(mask32x8::splat(true).first_set(), Some(0));
            ///
            /// let mask = mask32x8::from_array([false, true, false, false, true, false, false, true]);
            /// assert_eq!(mask.first_set(), Some(1));
            /// ```
            #[inline]
            #[must_use = "method returns the index and does not mutate the original value"]
            pub fn first_set(self) -> Option<usize> {
                // If bitmasks are efficient, using them is better
                if cfg!(target_feature = "sse") && N <= 64 {
                    let tz = self.to_bitmask().trailing_zeros();
                    return if tz == 64 { None } else { Some(tz as usize) };
                }

                // To find the first set index:
                // * create a vector 0..N
                // * replace unset mask elements in that vector with -1
                // * perform _unsigned_ reduce-min
                // * check if the result is -1 or an index

                let index = Simd::from_array(
                    const {
                        let mut index = [0; N];
                        let mut i = 0;
                        while i < N {
                            index[i] = i;
                            i += 1;
                        }
                        index
                    },
                );

                // Safety: the input and output are integer vectors
                let index: Simd<T, N> = unsafe { core::intrinsics::simd::simd_cast(index) };

                let masked_index = self.select(index, Self::splat(true).to_int());

                // Safety: the input and output are integer vectors
                let masked_index: Simd<T::Unsigned, N> =
                    unsafe { core::intrinsics::simd::simd_cast(masked_index) };

                // Safety: the input is an integer vector
                let min_index: T::Unsigned =
                    unsafe { core::intrinsics::simd::simd_reduce_min(masked_index) };

                // Safety: the return value is the unsigned version of T
                let min_index: T = unsafe { core::mem::transmute_copy(&min_index) };

                if min_index.eq(T::TRUE) {
                    None
                } else {
                    Some(min_index.to_usize())
                }
            }
        }

        // vector/array conversion
        impl<T, const N: usize> From<[bool; N]> for Mask<T, N>
        where
            T: MaskElement,
            LaneCount<N>: SupportedLaneCount,
        {
            #[inline]
            fn from(array: [bool; N]) -> Self {
                Self::from_array(array)
            }
        }

        impl<T, const N: usize> From<Mask<T, N>> for [bool; N]
        where
            T: MaskElement,
            LaneCount<N>: SupportedLaneCount,
        {
            #[inline]
            fn from(vector: Mask<T, N>) -> Self {
                vector.to_array()
            }
        }

        impl<T, const N: usize> Default for Mask<T, N>
        where
            T: MaskElement,
            LaneCount<N>: SupportedLaneCount,
        {
            #[inline]
            fn default() -> Self {
                Self::splat(false)
            }
        }

        impl<T, const N: usize> PartialEq for Mask<T, N>
        where
            T: MaskElement + PartialEq,
            LaneCount<N>: SupportedLaneCount,
        {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl<T, const N: usize> PartialOrd for Mask<T, N>
        where
            T: MaskElement + PartialOrd,
            LaneCount<N>: SupportedLaneCount,
        {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        impl<T, const N: usize> fmt::Debug for Mask<T, N>
        where
            T: MaskElement + fmt::Debug,
            LaneCount<N>: SupportedLaneCount,
        {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_list()
                    .entries((0..N).map(|i| self.test(i)))
                    .finish()
            }
        }

        impl<T, const N: usize> core::ops::BitAnd for Mask<T, N>
        where
            T: MaskElement,
            LaneCount<N>: SupportedLaneCount,
        {
            type Output = Self;
            #[inline]
            fn bitand(self, rhs: Self) -> Self {
                Self(self.0 & rhs.0)
            }
        }

        impl<T, const N: usize> core::ops::BitAnd<bool> for Mask<T, N>
        where
            T: MaskElement,
            LaneCount<N>: SupportedLaneCount,
        {
            type Output = Self;
            #[inline]
            fn bitand(self, rhs: bool) -> Self {
                self & Self::splat(rhs)
            }
        }

        impl<T, const N: usize> core::ops::BitAnd<Mask<T, N>> for bool
        where
            T: MaskElement,
            LaneCount<N>: SupportedLaneCount,
        {
            type Output = Mask<T, N>;
            #[inline]
            fn bitand(self, rhs: Mask<T, N>) -> Mask<T, N> {
                Mask::splat(self) & rhs
            }
        }

        impl<T, const N: usize> core::ops::BitOr for Mask<T, N>
        where
            T: MaskElement,
            LaneCount<N>: SupportedLaneCount,
        {
            type Output = Self;
            #[inline]
            fn bitor(self, rhs: Self) -> Self {
                Self(self.0 | rhs.0)
            }
        }

        impl<T, const N: usize> core::ops::BitOr<bool> for Mask<T, N>
        where
            T: MaskElement,
            LaneCount<N>: SupportedLaneCount,
        {
            type Output = Self;
            #[inline]
            fn bitor(self, rhs: bool) -> Self {
                self | Self::splat(rhs)
            }
        }

        impl<T, const N: usize> core::ops::BitOr<Mask<T, N>> for bool
        where
            T: MaskElement,
            LaneCount<N>: SupportedLaneCount,
        {
            type Output = Mask<T, N>;
            #[inline]
            fn bitor(self, rhs: Mask<T, N>) -> Mask<T, N> {
                Mask::splat(self) | rhs
            }
        }

        impl<T, const N: usize> core::ops::BitXor for Mask<T, N>
        where
            T: MaskElement,
            LaneCount<N>: SupportedLaneCount,
        {
            type Output = Self;
            #[inline]
            fn bitxor(self, rhs: Self) -> Self::Output {
                Self(self.0 ^ rhs.0)
            }
        }

        impl<T, const N: usize> core::ops::BitXor<bool> for Mask<T, N>
        where
            T: MaskElement,
            LaneCount<N>: SupportedLaneCount,
        {
            type Output = Self;
            #[inline]
            fn bitxor(self, rhs: bool) -> Self::Output {
                self ^ Self::splat(rhs)
            }
        }

        impl<T, const N: usize> core::ops::BitXor<Mask<T, N>> for bool
        where
            T: MaskElement,
            LaneCount<N>: SupportedLaneCount,
        {
            type Output = Mask<T, N>;
            #[inline]
            fn bitxor(self, rhs: Mask<T, N>) -> Self::Output {
                Mask::splat(self) ^ rhs
            }
        }

        impl<T, const N: usize> core::ops::Not for Mask<T, N>
        where
            T: MaskElement,
            LaneCount<N>: SupportedLaneCount,
        {
            type Output = Mask<T, N>;
            #[inline]
            fn not(self) -> Self::Output {
                Self(!self.0)
            }
        }

        impl<T, const N: usize> core::ops::BitAndAssign for Mask<T, N>
        where
            T: MaskElement,
            LaneCount<N>: SupportedLaneCount,
        {
            #[inline]
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 = self.0 & rhs.0;
            }
        }

        impl<T, const N: usize> core::ops::BitAndAssign<bool> for Mask<T, N>
        where
            T: MaskElement,
            LaneCount<N>: SupportedLaneCount,
        {
            #[inline]
            fn bitand_assign(&mut self, rhs: bool) {
                *self &= Self::splat(rhs);
            }
        }

        impl<T, const N: usize> core::ops::BitOrAssign for Mask<T, N>
        where
            T: MaskElement,
            LaneCount<N>: SupportedLaneCount,
        {
            #[inline]
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 = self.0 | rhs.0;
            }
        }

        impl<T, const N: usize> core::ops::BitOrAssign<bool> for Mask<T, N>
        where
            T: MaskElement,
            LaneCount<N>: SupportedLaneCount,
        {
            #[inline]
            fn bitor_assign(&mut self, rhs: bool) {
                *self |= Self::splat(rhs);
            }
        }

        impl<T, const N: usize> core::ops::BitXorAssign for Mask<T, N>
        where
            T: MaskElement,
            LaneCount<N>: SupportedLaneCount,
        {
            #[inline]
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 = self.0 ^ rhs.0;
            }
        }

        impl<T, const N: usize> core::ops::BitXorAssign<bool> for Mask<T, N>
        where
            T: MaskElement,
            LaneCount<N>: SupportedLaneCount,
        {
            #[inline]
            fn bitxor_assign(&mut self, rhs: bool) {
                *self ^= Self::splat(rhs);
            }
        }

        macro_rules! impl_from {
            { $from:ty  => $($to:ty),* } => {
                $(
                impl<const N: usize> From<Mask<$from, N>> for Mask<$to, N>
                where
                    LaneCount<N>: SupportedLaneCount,
                {
                    #[inline]
                    fn from(value: Mask<$from, N>) -> Self {
                        value.cast()
                    }
                }
                )*
            }
        }
        impl_from! { i8 => i16, i32, i64, isize }
        impl_from! { i16 => i32, i64, isize, i8 }
        impl_from! { i32 => i64, isize, i8, i16 }
        impl_from! { i64 => isize, i8, i16, i32 }
        impl_from! { isize => i8, i16, i32, i64 }

    }

    pub mod ops
    {
        /*!
        */
        use ::
        {
            *
        };
        /*
        pub use std::ops::{*};
        use crate::simd::{LaneCount, Simd, SimdElement, SupportedLaneCount, cmp::SimdPartialEq};
        use core::ops::{Add, Mul};
        use core::ops::{BitAnd, BitOr, BitXor};
        use core::ops::{Div, Rem, Sub};
        use core::ops::{Shl, Shr};
        */
        pub mod assign
        {
            /*!
            */
            use ::
            {
                *
            };
            /*
            pub use std::${1}::{*};
            use super::*;
            use core::ops::{AddAssign, MulAssign}; // commutative binary op-assignment
            use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign}; // commutative bit binary op-assignment
            use core::ops::{DivAssign, RemAssign, SubAssign}; // non-commutative binary op-assignment
            use core::ops::{ShlAssign, ShrAssign}; // non-commutative bit binary op-assignment
            */            
            macro_rules! assign_ops 
            {
                ($(impl<T, U, const N: usize> $assignTrait:ident<U> for Simd<T, N>
                    where
                        Self: $trait:ident,
                    {
                        fn $assign_call:ident(rhs: U) {
                            $call:ident
                        }
                    })*) => {
                    $(impl<T, U, const N: usize> $assignTrait<U> for Simd<T, N>
                    where
                        Self: $trait<U, Output = Self>,
                        T: SimdElement,
                        LaneCount<N>: SupportedLaneCount,
                    {
                        #[inline]
                        fn $assign_call(&mut self, rhs: U) {
                            *self = self.$call(rhs);
                        }
                    })*
                }
            }

            assign_ops! {
                // Arithmetic
                impl<T, U, const N: usize> AddAssign<U> for Simd<T, N>
                where
                    Self: Add,
                {
                    fn add_assign(rhs: U) {
                        add
                    }
                }

                impl<T, U, const N: usize> MulAssign<U> for Simd<T, N>
                where
                    Self: Mul,
                {
                    fn mul_assign(rhs: U) {
                        mul
                    }
                }

                impl<T, U, const N: usize> SubAssign<U> for Simd<T, N>
                where
                    Self: Sub,
                {
                    fn sub_assign(rhs: U) {
                        sub
                    }
                }

                impl<T, U, const N: usize> DivAssign<U> for Simd<T, N>
                where
                    Self: Div,
                {
                    fn div_assign(rhs: U) {
                        div
                    }
                }
                impl<T, U, const N: usize> RemAssign<U> for Simd<T, N>
                where
                    Self: Rem,
                {
                    fn rem_assign(rhs: U) {
                        rem
                    }
                }

                // Bitops
                impl<T, U, const N: usize> BitAndAssign<U> for Simd<T, N>
                where
                    Self: BitAnd,
                {
                    fn bitand_assign(rhs: U) {
                        bitand
                    }
                }

                impl<T, U, const N: usize> BitOrAssign<U> for Simd<T, N>
                where
                    Self: BitOr,
                {
                    fn bitor_assign(rhs: U) {
                        bitor
                    }
                }

                impl<T, U, const N: usize> BitXorAssign<U> for Simd<T, N>
                where
                    Self: BitXor,
                {
                    fn bitxor_assign(rhs: U) {
                        bitxor
                    }
                }

                impl<T, U, const N: usize> ShlAssign<U> for Simd<T, N>
                where
                    Self: Shl,
                {
                    fn shl_assign(rhs: U) {
                        shl
                    }
                }

                impl<T, U, const N: usize> ShrAssign<U> for Simd<T, N>
                where
                    Self: Shr,
                {
                    fn shr_assign(rhs: U) {
                        shr
                    }
                }
            }


        }

        pub mod deref
        {
            /*!
            */
            use ::
            {
                *
            };
            /*
            pub use std::${1}::{*};
            */
            use super::*;

            macro_rules! deref_lhs {
                (impl<T, const N: usize> $trait:ident for $simd:ty {
                        fn $call:ident
                    }) => {
                    impl<T, const N: usize> $trait<$simd> for &$simd
                    where
                        T: SimdElement,
                        $simd: $trait<$simd, Output = $simd>,
                        LaneCount<N>: SupportedLaneCount,
                    {
                        type Output = Simd<T, N>;

                        #[inline]
                        fn $call(self, rhs: $simd) -> Self::Output {
                            (*self).$call(rhs)
                        }
                    }
                };
            }

            macro_rules! deref_rhs {
                (impl<T, const N: usize> $trait:ident for $simd:ty {
                        fn $call:ident
                    }) => {
                    impl<T, const N: usize> $trait<&$simd> for $simd
                    where
                        T: SimdElement,
                        $simd: $trait<$simd, Output = $simd>,
                        LaneCount<N>: SupportedLaneCount,
                    {
                        type Output = Simd<T, N>;

                        #[inline]
                        fn $call(self, rhs: &$simd) -> Self::Output {
                            self.$call(*rhs)
                        }
                    }
                };
            }

            macro_rules! deref_ops {
                ($(impl<T, const N: usize> $trait:ident for $simd:ty {
                        fn $call:ident
                    })*) => {
                    $(
                        deref_rhs! {
                            impl<T, const N: usize> $trait for $simd {
                                fn $call
                            }
                        }
                        deref_lhs! {
                            impl<T, const N: usize> $trait for $simd {
                                fn $call
                            }
                        }
                        impl<'lhs, 'rhs, T, const N: usize> $trait<&'rhs $simd> for &'lhs $simd
                        where
                            T: SimdElement,
                            $simd: $trait<$simd, Output = $simd>,
                            LaneCount<N>: SupportedLaneCount,
                        {
                            type Output = $simd;

                            #[inline]
                            fn $call(self, rhs: &'rhs $simd) -> Self::Output {
                                (*self).$call(*rhs)
                            }
                        }
                    )*
                }
            }

            deref_ops! {
                // Arithmetic
                impl<T, const N: usize> Add for Simd<T, N> {
                    fn add
                }

                impl<T, const N: usize> Mul for Simd<T, N> {
                    fn mul
                }

                impl<T, const N: usize> Sub for Simd<T, N> {
                    fn sub
                }

                impl<T, const N: usize> Div for Simd<T, N> {
                    fn div
                }

                impl<T, const N: usize> Rem for Simd<T, N> {
                    fn rem
                }

                // Bitops
                impl<T, const N: usize> BitAnd for Simd<T, N> {
                    fn bitand
                }

                impl<T, const N: usize> BitOr for Simd<T, N> {
                    fn bitor
                }

                impl<T, const N: usize> BitXor for Simd<T, N> {
                    fn bitxor
                }

                impl<T, const N: usize> Shl for Simd<T, N> {
                    fn shl
                }

                impl<T, const N: usize> Shr for Simd<T, N> {
                    fn shr
                }
            }
        }

        pub mod shift_scalar
        {
            /*!
            */
            use ::
            {
                *
            };
            /*
            use crate::simd::{LaneCount, Simd, SupportedLaneCount};
            */
            macro_rules! impl_splatted_shifts {
                { impl $trait:ident :: $trait_fn:ident for $ty:ty } => {
                    impl<const N: usize> core::ops::$trait<$ty> for Simd<$ty, N>
                    where
                        LaneCount<N>: SupportedLaneCount,
                    {
                        type Output = Self;
                        #[inline]
                        fn $trait_fn(self, rhs: $ty) -> Self::Output {
                            self.$trait_fn(Simd::splat(rhs))
                        }
                    }

                    impl<const N: usize> core::ops::$trait<&$ty> for Simd<$ty, N>
                    where
                        LaneCount<N>: SupportedLaneCount,
                    {
                        type Output = Self;
                        #[inline]
                        fn $trait_fn(self, rhs: &$ty) -> Self::Output {
                            self.$trait_fn(Simd::splat(*rhs))
                        }
                    }

                    impl<'lhs, const N: usize> core::ops::$trait<$ty> for &'lhs Simd<$ty, N>
                    where
                        LaneCount<N>: SupportedLaneCount,
                    {
                        type Output = Simd<$ty, N>;
                        #[inline]
                        fn $trait_fn(self, rhs: $ty) -> Self::Output {
                            self.$trait_fn(Simd::splat(rhs))
                        }
                    }

                    impl<'lhs, const N: usize> core::ops::$trait<&$ty> for &'lhs Simd<$ty, N>
                    where
                        LaneCount<N>: SupportedLaneCount,
                    {
                        type Output = Simd<$ty, N>;
                        #[inline]
                        fn $trait_fn(self, rhs: &$ty) -> Self::Output {
                            self.$trait_fn(Simd::splat(*rhs))
                        }
                    }
                };
                { $($ty:ty),* } => {
                    $(
                    impl_splatted_shifts! { impl Shl::shl for $ty }
                    impl_splatted_shifts! { impl Shr::shr for $ty }
                    )*
                }
            }

            // In the past there were inference issues when generically splatting arguments.
            // Enumerate them instead.
            impl_splatted_shifts! { i8, i16, i32, i64, isize, u8, u16, u32, u64, usize }
        }

        pub mod unary
        {
            /*!
            */
            use ::
            {
                *
            };
            /*
            pub use std::${1}::{*};
            use crate::simd::{LaneCount, Simd, SimdElement, SupportedLaneCount};
            use core::ops::{Neg, Not}; // unary ops
            */
            macro_rules! neg {
                ($(impl<const N: usize> Neg for Simd<$scalar:ty, N>)*) => {
                    $(impl<const N: usize> Neg for Simd<$scalar, N>
                    where
                        $scalar: SimdElement,
                        LaneCount<N>: SupportedLaneCount,
                    {
                        type Output = Self;

                        #[inline]
                        fn neg(self) -> Self::Output {
                            // Safety: `self` is a signed vector
                            unsafe { core::intrinsics::simd::simd_neg(self) }
                        }
                    })*
                }
            }

            neg! {
                impl<const N: usize> Neg for Simd<f32, N>

                impl<const N: usize> Neg for Simd<f64, N>

                impl<const N: usize> Neg for Simd<i8, N>

                impl<const N: usize> Neg for Simd<i16, N>

                impl<const N: usize> Neg for Simd<i32, N>

                impl<const N: usize> Neg for Simd<i64, N>

                impl<const N: usize> Neg for Simd<isize, N>
            }

            macro_rules! not {
                ($(impl<const N: usize> Not for Simd<$scalar:ty, N>)*) => {
                    $(impl<const N: usize> Not for Simd<$scalar, N>
                    where
                        $scalar: SimdElement,
                        LaneCount<N>: SupportedLaneCount,
                    {
                        type Output = Self;

                        #[inline]
                        fn not(self) -> Self::Output {
                            self ^ (Simd::splat(!(0 as $scalar)))
                        }
                    })*
                }
            }

            not! {
                impl<const N: usize> Not for Simd<i8, N>

                impl<const N: usize> Not for Simd<i16, N>

                impl<const N: usize> Not for Simd<i32, N>

                impl<const N: usize> Not for Simd<i64, N>

                impl<const N: usize> Not for Simd<isize, N>

                impl<const N: usize> Not for Simd<u8, N>

                impl<const N: usize> Not for Simd<u16, N>

                impl<const N: usize> Not for Simd<u32, N>

                impl<const N: usize> Not for Simd<u64, N>

                impl<const N: usize> Not for Simd<usize, N>
            }
        }

        impl<I, T, const N: usize> core::ops::Index<I> for Simd<T, N> where
        T: SimdElement,
        LaneCount<N>: SupportedLaneCount,
        I: core::slice::SliceIndex<[T]>,
        {
            type Output = I::Output;
            #[inline]
            fn index(&self, index: I) -> &Self::Output {
                &self.as_array()[index]
            }
        }

        impl<I, T, const N: usize> core::ops::IndexMut<I> for Simd<T, N>
        where
            T: SimdElement,
            LaneCount<N>: SupportedLaneCount,
            I: core::slice::SliceIndex<[T]>,
        {
            #[inline]
            fn index_mut(&mut self, index: I) -> &mut Self::Output {
                &mut self.as_mut_array()[index]
            }
        }

        macro_rules! unsafe_base {
            ($lhs:ident, $rhs:ident, {$simd_call:ident}, $($_:tt)*) => {
                // Safety: $lhs and $rhs are vectors
                unsafe { core::intrinsics::simd::$simd_call($lhs, $rhs) }
            };
        }

        /// SAFETY: This macro should not be used for anything except Shl or Shr, and passed the appropriate shift intrinsic.
        /// It handles performing a bitand in addition to calling the shift operator, so that the result
        /// is well-defined: LLVM can return a poison value if you shl, lshr, or ashr if `rhs >= <Int>::BITS`
        /// At worst, this will maybe add another instruction and cycle,
        /// at best, it may open up more optimization opportunities,
        /// or simply be elided entirely, especially for SIMD ISAs which default to this.
        ///
        // FIXME: Consider implementing this in cg_llvm instead?
        // cg_clif defaults to this, and scalar MIR shifts also default to wrapping
        macro_rules! wrap_bitshift {
            ($lhs:ident, $rhs:ident, {$simd_call:ident}, $int:ident) => {
                #[allow(clippy::suspicious_arithmetic_impl)]
                // Safety: $lhs and the bitand result are vectors
                unsafe {
                    core::intrinsics::simd::$simd_call(
                        $lhs,
                        $rhs.bitand(Simd::splat(<$int>::BITS as $int - 1)),
                    )
                }
            };
        }

        /// SAFETY: This macro must only be used to impl Div or Rem and given the matching intrinsic.
        /// It guards against LLVM's UB conditions for integer div or rem using masks and selects,
        /// thus guaranteeing a Rust value returns instead.
        ///
        /// |                  | LLVM | Rust
        /// | :--------------: | :--- | :----------
        /// | N {/,%} 0        | UB   | panic!()
        /// | <$int>::MIN / -1 | UB   | <$int>::MIN
        /// | <$int>::MIN % -1 | UB   | 0
        ///
        macro_rules! int_divrem_guard {
            (   $lhs:ident,
                $rhs:ident,
                {   const PANIC_ZERO: &'static str = $zero:literal;
                    $simd_call:ident, $op:tt
                },
                $int:ident ) => {
                if $rhs.simd_eq(Simd::splat(0 as _)).any() {
                    panic!($zero);
                } else {
                    // Prevent otherwise-UB overflow on the MIN / -1 case.
                    let rhs = if <$int>::MIN != 0 {
                        // This should, at worst, optimize to a few branchless logical ops
                        // Ideally, this entire conditional should evaporate
                        // Fire LLVM and implement those manually if it doesn't get the hint
                        ($lhs.simd_eq(Simd::splat(<$int>::MIN))
                        // type inference can break here, so cut an SInt to size
                        & $rhs.simd_eq(Simd::splat(-1i64 as _)))
                        .select(Simd::splat(1 as _), $rhs)
                    } else {
                        // Nice base case to make it easy to const-fold away the other branch.
                        $rhs
                    };

                    // aarch64 div fails for arbitrary `v % 0`, mod fails when rhs is MIN, for non-powers-of-two
                    // these operations aren't vectorized on aarch64 anyway
                    #[cfg(target_arch = "aarch64")]
                    {
                        let mut out = Simd::splat(0 as _);
                        for i in 0..Self::LEN {
                            out[i] = $lhs[i] $op rhs[i];
                        }
                        out
                    }

                    #[cfg(not(target_arch = "aarch64"))]
                    {
                        // Safety: $lhs and rhs are vectors
                        unsafe { core::intrinsics::simd::$simd_call($lhs, rhs) }
                    }
                }
            };
        }

        macro_rules! for_base_types {
            (   T = ($($scalar:ident),*);
                type Lhs = Simd<T, N>;
                type Rhs = Simd<T, N>;
                type Output = $out:ty;

                impl $op:ident::$call:ident {
                    $macro_impl:ident $inner:tt
                }) => {
                    $(
                        impl<const N: usize> $op<Self> for Simd<$scalar, N>
                        where
                            $scalar: SimdElement,
                            LaneCount<N>: SupportedLaneCount,
                        {
                            type Output = $out;

                            #[inline]
                            // TODO: only useful for int Div::div, but we hope that this
                            // will essentially always get inlined anyway.
                            #[track_caller]
                            fn $call(self, rhs: Self) -> Self::Output {
                                $macro_impl!(self, rhs, $inner, $scalar)
                            }
                        }
                    )*
            }
        }

        // A "TokenTree muncher": takes a set of scalar types `T = {};`
        // type parameters for the ops it implements, `Op::fn` names,
        // and a macro that expands into an expr, substituting in an intrinsic.
        // It passes that to for_base_types, which expands an impl for the types,
        // using the expanded expr in the function, and recurses with itself.
        //
        // tl;dr impls a set of ops::{Traits} for a set of types
        macro_rules! for_base_ops {
            (
                T = $types:tt;
                type Lhs = Simd<T, N>;
                type Rhs = Simd<T, N>;
                type Output = $out:ident;
                impl $op:ident::$call:ident
                    $inner:tt
                $($rest:tt)*
            ) => {
                for_base_types! {
                    T = $types;
                    type Lhs = Simd<T, N>;
                    type Rhs = Simd<T, N>;
                    type Output = $out;
                    impl $op::$call
                        $inner
                }
                for_base_ops! {
                    T = $types;
                    type Lhs = Simd<T, N>;
                    type Rhs = Simd<T, N>;
                    type Output = $out;
                    $($rest)*
                }
            };
            ($($done:tt)*) => {
                // Done.
            }
        }

        // Integers can always accept add, mul, sub, bitand, bitor, and bitxor.
        // For all of these operations, simd_* intrinsics apply wrapping logic.
        for_base_ops! {
            T = (i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
            type Lhs = Simd<T, N>;
            type Rhs = Simd<T, N>;
            type Output = Self;

            impl Add::add {
                unsafe_base { simd_add }
            }

            impl Mul::mul {
                unsafe_base { simd_mul }
            }

            impl Sub::sub {
                unsafe_base { simd_sub }
            }

            impl BitAnd::bitand {
                unsafe_base { simd_and }
            }

            impl BitOr::bitor {
                unsafe_base { simd_or }
            }

            impl BitXor::bitxor {
                unsafe_base { simd_xor }
            }

            impl Div::div {
                int_divrem_guard {
                    const PANIC_ZERO: &'static str = "attempt to divide by zero";
                    simd_div, /
                }
            }

            impl Rem::rem {
                int_divrem_guard {
                    const PANIC_ZERO: &'static str = "attempt to calculate the remainder with a divisor of zero";
                    simd_rem, %
                }
            }

            // The only question is how to handle shifts >= <Int>::BITS?
            // Our current solution uses wrapping logic.
            impl Shl::shl {
                wrap_bitshift { simd_shl }
            }

            impl Shr::shr {
                wrap_bitshift {
                    // This automatically monomorphizes to lshr or ashr, depending,
                    // so it's fine to use it for both UInts and SInts.
                    simd_shr
                }
            }
        }

        // We don't need any special precautions here:
        // Floats always accept arithmetic ops, but may become NaN.
        for_base_ops! {
            T = (f32, f64);
            type Lhs = Simd<T, N>;
            type Rhs = Simd<T, N>;
            type Output = Self;

            impl Add::add {
                unsafe_base { simd_add }
            }

            impl Mul::mul {
                unsafe_base { simd_mul }
            }

            impl Sub::sub {
                unsafe_base { simd_sub }
            }

            impl Div::div {
                unsafe_base { simd_div }
            }

            impl Rem::rem {
                unsafe_base { simd_rem }
            }
        }
    }

    pub mod select
    {
        /*!
        */
        use ::
        {
            *
        };
        /*
        pub use std::select::{*};
        use crate::simd::{LaneCount, Mask, MaskElement, Simd, SimdElement, SupportedLaneCount};

        */
        impl<T, const N: usize> Mask<T, N>
        where
            T: MaskElement,
            LaneCount<N>: SupportedLaneCount,
        {
            /// Choose elements from two vectors.
            ///
            /// For each element in the mask, choose the corresponding element from `true_values` if
            /// that element mask is true, and `false_values` if that element mask is false.
            ///
            /// # Examples
            /// ```
            /// # #![feature(portable_simd)]
            /// # use core::simd::{Simd, Mask};
            /// let a = Simd::from_array([0, 1, 2, 3]);
            /// let b = Simd::from_array([4, 5, 6, 7]);
            /// let mask = Mask::from_array([true, false, false, true]);
            /// let c = mask.select(a, b);
            /// assert_eq!(c.to_array(), [0, 5, 6, 3]);
            /// ```
            #[inline]
            #[must_use = "method returns a new vector and does not mutate the original inputs"]
            pub fn select<U>(self, true_values: Simd<U, N>, false_values: Simd<U, N>) -> Simd<U, N>
            where
                U: SimdElement<Mask = T>,
            {
                // Safety: The mask has been cast to a vector of integers,
                // and the operands to select between are vectors of the same type and length.
                unsafe { core::intrinsics::simd::simd_select(self.to_int(), true_values, false_values) }
            }

            /// Choose elements from two masks.
            ///
            /// For each element in the mask, choose the corresponding element from `true_values` if
            /// that element mask is true, and `false_values` if that element mask is false.
            ///
            /// # Examples
            /// ```
            /// # #![feature(portable_simd)]
            /// # use core::simd::Mask;
            /// let a = Mask::<i32, 4>::from_array([true, true, false, false]);
            /// let b = Mask::<i32, 4>::from_array([false, false, true, true]);
            /// let mask = Mask::<i32, 4>::from_array([true, false, false, true]);
            /// let c = mask.select_mask(a, b);
            /// assert_eq!(c.to_array(), [true, false, true, false]);
            /// ```
            #[inline]
            #[must_use = "method returns a new mask and does not mutate the original inputs"]
            pub fn select_mask(self, true_values: Self, false_values: Self) -> Self {
                self & true_values | !self & false_values
            }
        }
    }

    pub mod swizzle_dyn
    {
        /*!
        */
        use ::
        {
            *
        };
        /*
        pub use std::swizzle_dyn::{*};
        use crate::simd::{LaneCount, Simd, SupportedLaneCount};
        use core::mem;
        */
        impl<const N: usize> Simd<u8, N>
        where
            LaneCount<N>: SupportedLaneCount,
        {
            /// Swizzle a vector of bytes according to the index vector.
            /// Indices within range select the appropriate byte.
            /// Indices "out of bounds" instead select 0.
            ///
            /// Note that the current implementation is selected during build-time
            /// of the standard library, so `cargo build -Zbuild-std` may be necessary
            /// to unlock better performance, especially for larger vectors.
            /// A planned compiler improvement will enable using `#[target_feature]` instead.
            #[inline]
            pub fn swizzle_dyn(self, idxs: Simd<u8, N>) -> Self {
                #![allow(unused_imports, unused_unsafe)]
                #[cfg(all(
                    any(target_arch = "aarch64", target_arch = "arm64ec"),
                    target_endian = "little"
                ))]
                use core::arch::aarch64::{uint8x8_t, vqtbl1q_u8, vtbl1_u8};
                #[cfg(all(
                    target_arch = "arm",
                    target_feature = "v7",
                    target_feature = "neon",
                    target_endian = "little"
                ))]
                use core::arch::arm::{uint8x8_t, vtbl1_u8};
                #[cfg(target_arch = "wasm32")]
                use core::arch::wasm32 as wasm;
                #[cfg(target_arch = "wasm64")]
                use core::arch::wasm64 as wasm;
                #[cfg(target_arch = "x86")]
                use core::arch::x86;
                #[cfg(target_arch = "x86_64")]
                use core::arch::x86_64 as x86;
                // SAFETY: Intrinsics covered by cfg
                unsafe {
                    match N {
                        #[cfg(all(
                            any(
                                target_arch = "aarch64",
                                target_arch = "arm64ec",
                                all(target_arch = "arm", target_feature = "v7")
                            ),
                            target_feature = "neon",
                            target_endian = "little"
                        ))]
                        8 => transize(vtbl1_u8, self, idxs),
                        #[cfg(target_feature = "ssse3")]
                        16 => transize(x86::_mm_shuffle_epi8, self, zeroing_idxs(idxs)),
                        #[cfg(target_feature = "simd128")]
                        16 => transize(wasm::i8x16_swizzle, self, idxs),
                        #[cfg(all(
                            any(target_arch = "aarch64", target_arch = "arm64ec"),
                            target_feature = "neon",
                            target_endian = "little"
                        ))]
                        16 => transize(vqtbl1q_u8, self, idxs),
                        #[cfg(all(
                            target_arch = "arm",
                            target_feature = "v7",
                            target_feature = "neon",
                            target_endian = "little"
                        ))]
                        16 => transize(armv7_neon_swizzle_u8x16, self, idxs),
                        #[cfg(all(target_feature = "avx2", not(target_feature = "avx512vbmi")))]
                        32 => transize(avx2_pshufb, self, idxs),
                        #[cfg(all(target_feature = "avx512vl", target_feature = "avx512vbmi"))]
                        32 => {
                            // Unlike vpshufb, vpermb doesn't zero out values in the result based on the index high bit
                            let swizzler = |bytes, idxs| {
                                let mask = x86::_mm256_cmp_epu8_mask::<{ x86::_MM_CMPINT_LT }>(
                                    idxs,
                                    Simd::<u8, 32>::splat(N as u8).into(),
                                );
                                x86::_mm256_maskz_permutexvar_epi8(mask, idxs, bytes)
                            };
                            transize(swizzler, self, idxs)
                        }
                        // Notable absence: avx512bw pshufb shuffle
                        #[cfg(all(target_feature = "avx512vl", target_feature = "avx512vbmi"))]
                        64 => {
                            // Unlike vpshufb, vpermb doesn't zero out values in the result based on the index high bit
                            let swizzler = |bytes, idxs| {
                                let mask = x86::_mm512_cmp_epu8_mask::<{ x86::_MM_CMPINT_LT }>(
                                    idxs,
                                    Simd::<u8, 64>::splat(N as u8).into(),
                                );
                                x86::_mm512_maskz_permutexvar_epi8(mask, idxs, bytes)
                            };
                            transize(swizzler, self, idxs)
                        }
                        _ => {
                            let mut array = [0; N];
                            for (i, k) in idxs.to_array().into_iter().enumerate() {
                                if (k as usize) < N {
                                    array[i] = self[k as usize];
                                };
                            }
                            array.into()
                        }
                    }
                }
            }
        }

        /// armv7 neon supports swizzling `u8x16` by swizzling two u8x8 blocks
        /// with a u8x8x2 lookup table.
        ///
        /// # Safety
        /// This requires armv7 neon to work
        #[cfg(all(
            target_arch = "arm",
            target_feature = "v7",
            target_feature = "neon",
            target_endian = "little"
        ))]
        unsafe fn armv7_neon_swizzle_u8x16(bytes: Simd<u8, 16>, idxs: Simd<u8, 16>) -> Simd<u8, 16> {
            use core::arch::arm::{uint8x8x2_t, vcombine_u8, vget_high_u8, vget_low_u8, vtbl2_u8};
            // SAFETY: Caller promised arm neon support
            unsafe {
                let bytes = uint8x8x2_t(vget_low_u8(bytes.into()), vget_high_u8(bytes.into()));
                let lo = vtbl2_u8(bytes, vget_low_u8(idxs.into()));
                let hi = vtbl2_u8(bytes, vget_high_u8(idxs.into()));
                vcombine_u8(lo, hi).into()
            }
        }

        /// "vpshufb like it was meant to be" on AVX2
        ///
        /// # Safety
        /// This requires AVX2 to work
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        #[target_feature(enable = "avx2")]
        #[allow(unused)]
        #[inline]
        #[allow(clippy::let_and_return)]
        unsafe fn avx2_pshufb(bytes: Simd<u8, 32>, idxs: Simd<u8, 32>) -> Simd<u8, 32> {
            use crate::simd::cmp::SimdPartialOrd;
            #[cfg(target_arch = "x86")]
            use core::arch::x86;
            #[cfg(target_arch = "x86_64")]
            use core::arch::x86_64 as x86;
            use x86::_mm256_permute2x128_si256 as avx2_cross_shuffle;
            use x86::_mm256_shuffle_epi8 as avx2_half_pshufb;
            let mid = Simd::splat(16u8);
            let high = mid + mid;
            // SAFETY: Caller promised AVX2
            unsafe {
                // This is ordering sensitive, and LLVM will order these how you put them.
                // Most AVX2 impls use ~5 "ports", and only 1 or 2 are capable of permutes.
                // But the "compose" step will lower to ops that can also use at least 1 other port.
                // So this tries to break up permutes so composition flows through "open" ports.
                // Comparative benches should be done on multiple AVX2 CPUs before reordering this

                let hihi = avx2_cross_shuffle::<0x11>(bytes.into(), bytes.into());
                let hi_shuf = Simd::from(avx2_half_pshufb(
                    hihi,        // duplicate the vector's top half
                    idxs.into(), // so that using only 4 bits of an index still picks bytes 16-31
                ));
                // A zero-fill during the compose step gives the "all-Neon-like" OOB-is-0 semantics
                let compose = idxs.simd_lt(high).select(hi_shuf, Simd::splat(0));
                let lolo = avx2_cross_shuffle::<0x00>(bytes.into(), bytes.into());
                let lo_shuf = Simd::from(avx2_half_pshufb(lolo, idxs.into()));
                // Repeat, then pick indices < 16, overwriting indices 0-15 from previous compose step
                let compose = idxs.simd_lt(mid).select(lo_shuf, compose);
                compose
            }
        }

        /// This sets up a call to an architecture-specific function, and in doing so
        /// it persuades rustc that everything is the correct size. Which it is.
        /// This would not be needed if one could convince Rust that, by matching on N,
        /// N is that value, and thus it would be valid to substitute e.g. 16.
        ///
        /// # Safety
        /// The correctness of this function hinges on the sizes agreeing in actuality.
        #[allow(dead_code)]
        #[inline(always)]
        unsafe fn transize<T, const N: usize>(
            f: unsafe fn(T, T) -> T,
            a: Simd<u8, N>,
            b: Simd<u8, N>,
        ) -> Simd<u8, N>
        where
            LaneCount<N>: SupportedLaneCount,
        {
            // SAFETY: Same obligation to use this function as to use mem::transmute_copy.
            unsafe { mem::transmute_copy(&f(mem::transmute_copy(&a), mem::transmute_copy(&b))) }
        }

        /// Make indices that yield 0 for x86
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        #[allow(unused)]
        #[inline(always)]
        fn zeroing_idxs<const N: usize>(idxs: Simd<u8, N>) -> Simd<u8, N>
        where
            LaneCount<N>: SupportedLaneCount,
        {
            use crate::simd::cmp::SimdPartialOrd;
            idxs.simd_lt(Simd::splat(N as u8))
                .select(idxs, Simd::splat(u8::MAX))
        }
    }

    pub mod to_bytes
    {
        /*!
        */
        use ::
        {
            *
        };
        /*
        pub use std::to_bytes::{*};
        use crate::simd::{
            LaneCount, Simd, SimdElement, SupportedLaneCount,
            num::{SimdFloat, SimdInt, SimdUint},
        };

        mod sealed {
            use super::*;
            pub trait Sealed {}
            impl<T: SimdElement, const N: usize> Sealed for Simd<T, N> where LaneCount<N>: SupportedLaneCount {}
        }
        use sealed::Sealed;
        */
        /// Converts SIMD vectors to vectors of bytes
        pub trait ToBytes: Sealed {
            /// This type, reinterpreted as bytes.
            type Bytes: Copy
                + Unpin
                + Send
                + Sync
                + AsRef<[u8]>
                + AsMut<[u8]>
                + SimdUint<Scalar = u8>
                + 'static;

            /// Returns the memory representation of this integer as a byte array in native byte
            /// order.
            fn to_ne_bytes(self) -> Self::Bytes;

            /// Returns the memory representation of this integer as a byte array in big-endian
            /// (network) byte order.
            fn to_be_bytes(self) -> Self::Bytes;

            /// Returns the memory representation of this integer as a byte array in little-endian
            /// byte order.
            fn to_le_bytes(self) -> Self::Bytes;

            /// Creates a native endian integer value from its memory representation as a byte array
            /// in native endianness.
            fn from_ne_bytes(bytes: Self::Bytes) -> Self;

            /// Creates an integer value from its representation as a byte array in big endian.
            fn from_be_bytes(bytes: Self::Bytes) -> Self;

            /// Creates an integer value from its representation as a byte array in little endian.
            fn from_le_bytes(bytes: Self::Bytes) -> Self;
        }

        macro_rules! swap_bytes {
            { f32, $x:expr } => { Simd::from_bits($x.to_bits().swap_bytes()) };
            { f64, $x:expr } => { Simd::from_bits($x.to_bits().swap_bytes()) };
            { $ty:ty, $x:expr } => { $x.swap_bytes() }
        }

        macro_rules! impl_to_bytes {
            { $ty:tt, 1  } => { impl_to_bytes! { $ty, 1  * [1, 2, 4, 8, 16, 32, 64] } };
            { $ty:tt, 2  } => { impl_to_bytes! { $ty, 2  * [1, 2, 4, 8, 16, 32] } };
            { $ty:tt, 4  } => { impl_to_bytes! { $ty, 4  * [1, 2, 4, 8, 16] } };
            { $ty:tt, 8  } => { impl_to_bytes! { $ty, 8  * [1, 2, 4, 8] } };
            { $ty:tt, 16 } => { impl_to_bytes! { $ty, 16 * [1, 2, 4] } };
            { $ty:tt, 32 } => { impl_to_bytes! { $ty, 32 * [1, 2] } };
            { $ty:tt, 64 } => { impl_to_bytes! { $ty, 64 * [1] } };

            { $ty:tt, $size:literal * [$($elems:literal),*] } => {
                $(
                impl ToBytes for Simd<$ty, $elems> {
                    type Bytes = Simd<u8, { $size * $elems }>;

                    #[inline]
                    fn to_ne_bytes(self) -> Self::Bytes {
                        // Safety: transmuting between vectors is safe
                        unsafe {
                            #![allow(clippy::useless_transmute)]
                            core::mem::transmute(self)
                        }
                    }

                    #[inline]
                    fn to_be_bytes(mut self) -> Self::Bytes {
                        if !cfg!(target_endian = "big") {
                            self = swap_bytes!($ty, self);
                        }
                        self.to_ne_bytes()
                    }

                    #[inline]
                    fn to_le_bytes(mut self) -> Self::Bytes {
                        if !cfg!(target_endian = "little") {
                            self = swap_bytes!($ty, self);
                        }
                        self.to_ne_bytes()
                    }

                    #[inline]
                    fn from_ne_bytes(bytes: Self::Bytes) -> Self {
                        // Safety: transmuting between vectors is safe
                        unsafe {
                            #![allow(clippy::useless_transmute)]
                            core::mem::transmute(bytes)
                        }
                    }

                    #[inline]
                    fn from_be_bytes(bytes: Self::Bytes) -> Self {
                        let ret = Self::from_ne_bytes(bytes);
                        if cfg!(target_endian = "big") {
                            ret
                        } else {
                            swap_bytes!($ty, ret)
                        }
                    }

                    #[inline]
                    fn from_le_bytes(bytes: Self::Bytes) -> Self {
                        let ret = Self::from_ne_bytes(bytes);
                        if cfg!(target_endian = "little") {
                            ret
                        } else {
                            swap_bytes!($ty, ret)
                        }
                    }
                }
                )*
            }
        }

        impl_to_bytes! { u8, 1 }
        impl_to_bytes! { u16, 2 }
        impl_to_bytes! { u32, 4 }
        impl_to_bytes! { u64, 8 }
        #[cfg(target_pointer_width = "32")]
        impl_to_bytes! { usize, 4 }
        #[cfg(target_pointer_width = "64")]
        impl_to_bytes! { usize, 8 }

        impl_to_bytes! { i8, 1 }
        impl_to_bytes! { i16, 2 }
        impl_to_bytes! { i32, 4 }
        impl_to_bytes! { i64, 8 }
        #[cfg(target_pointer_width = "32")]
        impl_to_bytes! { isize, 4 }
        #[cfg(target_pointer_width = "64")]
        impl_to_bytes! { isize, 8 }

        impl_to_bytes! { f32, 4 }
        impl_to_bytes! { f64, 8 }
    }

    pub mod vector
    {
        /*!
        */
        use ::
        {
            *
        };
        /*
        pub use std::vector::{*};

        use crate::simd::{
            LaneCount, Mask, MaskElement, SupportedLaneCount, Swizzle,
            cmp::SimdPartialOrd,
            num::SimdUint,
            ptr::{SimdConstPtr, SimdMutPtr},
        };
        */
        /// A SIMD vector with the shape of `[T; N]` but the operations of `T`.
        ///
        /// `Simd<T, N>` supports the operators (+, *, etc.) that `T` does in "elementwise" fashion.
        /// These take the element at each index from the left-hand side and right-hand side,
        /// perform the operation, then return the result in the same index in a vector of equal size.
        /// However, `Simd` differs from normal iteration and normal arrays:
        /// - `Simd<T, N>` executes `N` operations in a single step with no `break`s
        /// - `Simd<T, N>` can have an alignment greater than `T`, for better mechanical sympathy
        ///
        /// By always imposing these constraints on `Simd`, it is easier to compile elementwise operations
        /// into machine instructions that can themselves be executed in parallel.
        ///
        /// ```rust
        /// # #![feature(portable_simd)]
        /// # use core::simd::{Simd};
        /// # use core::array;
        /// let a: [i32; 4] = [-2, 0, 2, 4];
        /// let b = [10, 9, 8, 7];
        /// let sum = array::from_fn(|i| a[i] + b[i]);
        /// let prod = array::from_fn(|i| a[i] * b[i]);
        ///
        /// // `Simd<T, N>` implements `From<[T; N]>`
        /// let (v, w) = (Simd::from(a), Simd::from(b));
        /// // Which means arrays implement `Into<Simd<T, N>>`.
        /// assert_eq!(v + w, sum.into());
        /// assert_eq!(v * w, prod.into());
        /// ```
        ///
        ///
        /// `Simd` with integer elements treats operators as wrapping, as if `T` was [`Wrapping<T>`].
        /// Thus, `Simd` does not implement `wrapping_add`, because that is the default behavior.
        /// This means there is no warning on overflows, even in "debug" builds.
        /// For most applications where `Simd` is appropriate, it is "not a bug" to wrap,
        /// and even "debug builds" are unlikely to tolerate the loss of performance.
        /// You may want to consider using explicitly checked arithmetic if such is required.
        /// Division by zero on integers still causes a panic, so
        /// you may want to consider using `f32` or `f64` if that is unacceptable.
        ///
        /// [`Wrapping<T>`]: core::num::Wrapping
        ///
        /// # Layout
        /// `Simd<T, N>` has a layout similar to `[T; N]` (identical "shapes"), with a greater alignment.
        /// `[T; N]` is aligned to `T`, but `Simd<T, N>` will have an alignment based on both `T` and `N`.
        /// Thus it is sound to [`transmute`] `Simd<T, N>` to `[T; N]` and should optimize to "zero cost",
        /// but the reverse transmutation may require a copy the compiler cannot simply elide.
        ///
        /// # ABI "Features"
        /// Due to Rust's safety guarantees, `Simd<T, N>` is currently passed and returned via memory,
        /// not SIMD registers, except as an optimization. Using `#[inline]` on functions that accept
        /// `Simd<T, N>` or return it is recommended, at the cost of code generation time, as
        /// inlining SIMD-using functions can omit a large function prolog or epilog and thus
        /// improve both speed and code size. The need for this may be corrected in the future.
        ///
        /// Using `#[inline(always)]` still requires additional care.
        ///
        /// # Safe SIMD with Unsafe Rust
        ///
        /// Operations with `Simd` are typically safe, but there are many reasons to want to combine SIMD with `unsafe` code.
        /// Care must be taken to respect differences between `Simd` and other types it may be transformed into or derived from.
        /// In particular, the layout of `Simd<T, N>` may be similar to `[T; N]`, and may allow some transmutations,
        /// but references to `[T; N]` are not interchangeable with those to `Simd<T, N>`.
        /// Thus, when using `unsafe` Rust to read and write `Simd<T, N>` through [raw pointers], it is a good idea to first try with
        /// [`read_unaligned`] and [`write_unaligned`]. This is because:
        /// - [`read`] and [`write`] require full alignment (in this case, `Simd<T, N>`'s alignment)
        /// - `Simd<T, N>` is often read from or written to [`[T]`](slice) and other types aligned to `T`
        /// - combining these actions violates the `unsafe` contract and explodes the program into
        ///   a puff of **undefined behavior**
        /// - the compiler can implicitly adjust layouts to make unaligned reads or writes fully aligned
        ///   if it sees the optimization
        /// - most contemporary processors with "aligned" and "unaligned" read and write instructions
        ///   exhibit no performance difference if the "unaligned" variant is aligned at runtime
        ///
        /// Less obligations mean unaligned reads and writes are less likely to make the program unsound,
        /// and may be just as fast as stricter alternatives.
        /// When trying to guarantee alignment, [`[T]::as_simd`][as_simd] is an option for
        /// converting `[T]` to `[Simd<T, N>]`, and allows soundly operating on an aligned SIMD body,
        /// but it may cost more time when handling the scalar head and tail.
        /// If these are not enough, it is most ideal to design data structures to be already aligned
        /// to `align_of::<Simd<T, N>>()` before using `unsafe` Rust to read or write.
        /// Other ways to compensate for these facts, like materializing `Simd` to or from an array first,
        /// are handled by safe methods like [`Simd::from_array`] and [`Simd::from_slice`].
        ///
        /// [`transmute`]: core::mem::transmute
        /// [raw pointers]: pointer
        /// [`read_unaligned`]: pointer::read_unaligned
        /// [`write_unaligned`]: pointer::write_unaligned
        /// [`read`]: pointer::read
        /// [`write`]: pointer::write
        /// [as_simd]: slice::as_simd
        //
        // NOTE: Accessing the inner array directly in any way (e.g. by using the `.0` field syntax) or
        // directly constructing an instance of the type (i.e. `let vector = Simd(array)`) should be
        // avoided, as it will likely become illegal on `#[repr(simd)]` structs in the future. It also
        // causes rustc to emit illegal LLVM IR in some cases.
        #[repr(simd, packed)]
        pub struct Simd<T, const N: usize>([T; N])
        where
            LaneCount<N>: SupportedLaneCount,
            T: SimdElement;

        impl<T, const N: usize> Simd<T, N>
        where
            LaneCount<N>: SupportedLaneCount,
            T: SimdElement,
        {
            /// Number of elements in this vector.
            pub const LEN: usize = N;

            /// Returns the number of elements in this SIMD vector.
            ///
            /// # Examples
            ///
            /// ```
            /// # #![feature(portable_simd)]
            /// # #[cfg(feature = "as_crate")] use core_simd::simd;
            /// # #[cfg(not(feature = "as_crate"))] use core::simd;
            /// # use simd::u32x4;
            /// let v = u32x4::splat(0);
            /// assert_eq!(v.len(), 4);
            /// ```
            #[inline]
            #[allow(clippy::len_without_is_empty)]
            pub const fn len(&self) -> usize {
                Self::LEN
            }

            /// Constructs a new SIMD vector with all elements set to the given value.
            ///
            /// # Examples
            ///
            /// ```
            /// # #![feature(portable_simd)]
            /// # #[cfg(feature = "as_crate")] use core_simd::simd;
            /// # #[cfg(not(feature = "as_crate"))] use core::simd;
            /// # use simd::u32x4;
            /// let v = u32x4::splat(8);
            /// assert_eq!(v.as_array(), &[8, 8, 8, 8]);
            /// ```
            #[inline]
            #[rustc_const_unstable(feature = "portable_simd", issue = "86656")]
            pub const fn splat(value: T) -> Self {
                const fn splat_const<T, const N: usize>(value: T) -> Simd<T, N>
                where
                    T: SimdElement,
                    LaneCount<N>: SupportedLaneCount,
                {
                    Simd::from_array([value; N])
                }

                fn splat_rt<T, const N: usize>(value: T) -> Simd<T, N>
                where
                    T: SimdElement,
                    LaneCount<N>: SupportedLaneCount,
                {
                    // This is preferred over `[value; N]`, since it's explicitly a splat:
                    // https://github.com/rust-lang/rust/issues/97804
                    struct Splat;
                    impl<const N: usize> Swizzle<N> for Splat {
                        const INDEX: [usize; N] = [0; N];
                    }

                    Splat::swizzle::<T, 1>(Simd::<T, 1>::from([value]))
                }

                core::intrinsics::const_eval_select((value,), splat_const, splat_rt)
            }

            /// Returns an array reference containing the entire SIMD vector.
            ///
            /// # Examples
            ///
            /// ```
            /// # #![feature(portable_simd)]
            /// # use core::simd::{Simd, u64x4};
            /// let v: u64x4 = Simd::from_array([0, 1, 2, 3]);
            /// assert_eq!(v.as_array(), &[0, 1, 2, 3]);
            /// ```
            #[inline]
            pub const fn as_array(&self) -> &[T; N] {
                // SAFETY: `Simd<T, N>` is just an overaligned `[T; N]` with
                // potential padding at the end, so pointer casting to a
                // `&[T; N]` is safe.
                //
                // NOTE: This deliberately doesn't just use `&self.0`, see the comment
                // on the struct definition for details.
                unsafe { &*(self as *const Self as *const [T; N]) }
            }

            /// Returns a mutable array reference containing the entire SIMD vector.
            #[inline]
            pub fn as_mut_array(&mut self) -> &mut [T; N] {
                // SAFETY: `Simd<T, N>` is just an overaligned `[T; N]` with
                // potential padding at the end, so pointer casting to a
                // `&mut [T; N]` is safe.
                //
                // NOTE: This deliberately doesn't just use `&mut self.0`, see the comment
                // on the struct definition for details.
                unsafe { &mut *(self as *mut Self as *mut [T; N]) }
            }

            /// Loads a vector from an array of `T`.
            ///
            /// This function is necessary since `repr(simd)` has padding for non-power-of-2 vectors (at the time of writing).
            /// With padding, `read_unaligned` will read past the end of an array of N elements.
            ///
            /// # Safety
            /// Reading `ptr` must be safe, as if by `<*const [T; N]>::read`.
            #[inline]
            const unsafe fn load(ptr: *const [T; N]) -> Self {
                // There are potentially simpler ways to write this function, but this should result in
                // LLVM `load <N x T>`

                let mut tmp = core::mem::MaybeUninit::<Self>::uninit();
                // SAFETY: `Simd<T, N>` always contains `N` elements of type `T`.  It may have padding
                // which does not need to be initialized.  The safety of reading `ptr` is ensured by the
                // caller.
                unsafe {
                    core::ptr::copy_nonoverlapping(ptr, tmp.as_mut_ptr().cast(), 1);
                    tmp.assume_init()
                }
            }

            /// Store a vector to an array of `T`.
            ///
            /// See `load` as to why this function is necessary.
            ///
            /// # Safety
            /// Writing to `ptr` must be safe, as if by `<*mut [T; N]>::write`.
            #[inline]
            const unsafe fn store(self, ptr: *mut [T; N]) {
                // There are potentially simpler ways to write this function, but this should result in
                // LLVM `store <N x T>`

                // Creating a temporary helps LLVM turn the memcpy into a store.
                let tmp = self;
                // SAFETY: `Simd<T, N>` always contains `N` elements of type `T`.  The safety of writing
                // `ptr` is ensured by the caller.
                unsafe { core::ptr::copy_nonoverlapping(tmp.as_array(), ptr, 1) }
            }

            /// Converts an array to a SIMD vector.
            #[inline]
            pub const fn from_array(array: [T; N]) -> Self {
                // SAFETY: `&array` is safe to read.
                //
                // FIXME: We currently use a pointer load instead of `transmute_copy` because `repr(simd)`
                // results in padding for non-power-of-2 vectors (so vectors are larger than arrays).
                //
                // NOTE: This deliberately doesn't just use `Self(array)`, see the comment
                // on the struct definition for details.
                unsafe { Self::load(&array) }
            }

            /// Converts a SIMD vector to an array.
            #[inline]
            pub const fn to_array(self) -> [T; N] {
                let mut tmp = core::mem::MaybeUninit::uninit();
                // SAFETY: writing to `tmp` is safe and initializes it.
                //
                // FIXME: We currently use a pointer store instead of `transmute_copy` because `repr(simd)`
                // results in padding for non-power-of-2 vectors (so vectors are larger than arrays).
                //
                // NOTE: This deliberately doesn't just use `self.0`, see the comment
                // on the struct definition for details.
                unsafe {
                    self.store(tmp.as_mut_ptr());
                    tmp.assume_init()
                }
            }

            /// Converts a slice to a SIMD vector containing `slice[..N]`.
            ///
            /// # Panics
            ///
            /// Panics if the slice's length is less than the vector's `Simd::N`.
            /// Use `load_or_default` for an alternative that does not panic.
            ///
            /// # Example
            ///
            /// ```
            /// # #![feature(portable_simd)]
            /// # use core::simd::u32x4;
            /// let source = vec![1, 2, 3, 4, 5, 6];
            /// let v = u32x4::from_slice(&source);
            /// assert_eq!(v.as_array(), &[1, 2, 3, 4]);
            /// ```
            #[must_use]
            #[inline]
            #[track_caller]
            pub const fn from_slice(slice: &[T]) -> Self {
                assert!(
                    slice.len() >= Self::LEN,
                    "slice length must be at least the number of elements"
                );
                // SAFETY: We just checked that the slice contains
                // at least `N` elements.
                unsafe { Self::load(slice.as_ptr().cast()) }
            }

            /// Writes a SIMD vector to the first `N` elements of a slice.
            ///
            /// # Panics
            ///
            /// Panics if the slice's length is less than the vector's `Simd::N`.
            ///
            /// # Example
            ///
            /// ```
            /// # #![feature(portable_simd)]
            /// # #[cfg(feature = "as_crate")] use core_simd::simd;
            /// # #[cfg(not(feature = "as_crate"))] use core::simd;
            /// # use simd::u32x4;
            /// let mut dest = vec![0; 6];
            /// let v = u32x4::from_array([1, 2, 3, 4]);
            /// v.copy_to_slice(&mut dest);
            /// assert_eq!(&dest, &[1, 2, 3, 4, 0, 0]);
            /// ```
            #[inline]
            #[track_caller]
            pub fn copy_to_slice(self, slice: &mut [T]) {
                assert!(
                    slice.len() >= Self::LEN,
                    "slice length must be at least the number of elements"
                );
                // SAFETY: We just checked that the slice contains
                // at least `N` elements.
                unsafe { self.store(slice.as_mut_ptr().cast()) }
            }

            /// Reads contiguous elements from `slice`. Elements are read so long as they're in-bounds for
            /// the `slice`. Otherwise, the default value for the element type is returned.
            ///
            /// # Examples
            /// ```
            /// # #![feature(portable_simd)]
            /// # #[cfg(feature = "as_crate")] use core_simd::simd;
            /// # #[cfg(not(feature = "as_crate"))] use core::simd;
            /// # use simd::Simd;
            /// let vec: Vec<i32> = vec![10, 11];
            ///
            /// let result = Simd::<i32, 4>::load_or_default(&vec);
            /// assert_eq!(result, Simd::from_array([10, 11, 0, 0]));
            /// ```
            #[must_use]
            #[inline]
            pub fn load_or_default(slice: &[T]) -> Self
            where
                T: Default,
            {
                Self::load_or(slice, Default::default())
            }

            /// Reads contiguous elements from `slice`. Elements are read so long as they're in-bounds for
            /// the `slice`. Otherwise, the corresponding value from `or` is passed through.
            ///
            /// # Examples
            /// ```
            /// # #![feature(portable_simd)]
            /// # #[cfg(feature = "as_crate")] use core_simd::simd;
            /// # #[cfg(not(feature = "as_crate"))] use core::simd;
            /// # use simd::Simd;
            /// let vec: Vec<i32> = vec![10, 11];
            /// let or = Simd::from_array([-5, -4, -3, -2]);
            ///
            /// let result = Simd::load_or(&vec, or);
            /// assert_eq!(result, Simd::from_array([10, 11, -3, -2]));
            /// ```
            #[must_use]
            #[inline]
            pub fn load_or(slice: &[T], or: Self) -> Self {
                Self::load_select(slice, Mask::splat(true), or)
            }

            /// Reads contiguous elements from `slice`. Each element is read from memory if its
            /// corresponding element in `enable` is `true`.
            ///
            /// When the element is disabled or out of bounds for the slice, that memory location
            /// is not accessed and the corresponding value from `or` is passed through.
            ///
            /// # Examples
            /// ```
            /// # #![feature(portable_simd)]
            /// # #[cfg(feature = "as_crate")] use core_simd::simd;
            /// # #[cfg(not(feature = "as_crate"))] use core::simd;
            /// # use simd::{Simd, Mask};
            /// let vec: Vec<i32> = vec![10, 11, 12, 13, 14, 15, 16, 17, 18];
            /// let enable = Mask::from_array([true, true, false, true]);
            /// let or = Simd::from_array([-5, -4, -3, -2]);
            ///
            /// let result = Simd::load_select(&vec, enable, or);
            /// assert_eq!(result, Simd::from_array([10, 11, -3, 13]));
            /// ```
            #[must_use]
            #[inline]
            pub fn load_select_or_default(slice: &[T], enable: Mask<<T as SimdElement>::Mask, N>) -> Self
            where
                T: Default,
            {
                Self::load_select(slice, enable, Default::default())
            }

            /// Reads contiguous elements from `slice`. Each element is read from memory if its
            /// corresponding element in `enable` is `true`.
            ///
            /// When the element is disabled or out of bounds for the slice, that memory location
            /// is not accessed and the corresponding value from `or` is passed through.
            ///
            /// # Examples
            /// ```
            /// # #![feature(portable_simd)]
            /// # #[cfg(feature = "as_crate")] use core_simd::simd;
            /// # #[cfg(not(feature = "as_crate"))] use core::simd;
            /// # use simd::{Simd, Mask};
            /// let vec: Vec<i32> = vec![10, 11, 12, 13, 14, 15, 16, 17, 18];
            /// let enable = Mask::from_array([true, true, false, true]);
            /// let or = Simd::from_array([-5, -4, -3, -2]);
            ///
            /// let result = Simd::load_select(&vec, enable, or);
            /// assert_eq!(result, Simd::from_array([10, 11, -3, 13]));
            /// ```
            #[must_use]
            #[inline]
            pub fn load_select(
                slice: &[T],
                mut enable: Mask<<T as SimdElement>::Mask, N>,
                or: Self,
            ) -> Self {
                enable &= mask_up_to(slice.len());
                // SAFETY: We performed the bounds check by updating the mask. &[T] is properly aligned to
                // the element.
                unsafe { Self::load_select_ptr(slice.as_ptr(), enable, or) }
            }

            /// Reads contiguous elements from `slice`. Each element is read from memory if its
            /// corresponding element in `enable` is `true`.
            ///
            /// When the element is disabled, that memory location is not accessed and the corresponding
            /// value from `or` is passed through.
            ///
            /// # Safety
            /// Enabled loads must not exceed the length of `slice`.
            #[must_use]
            #[inline]
            pub unsafe fn load_select_unchecked(
                slice: &[T],
                enable: Mask<<T as SimdElement>::Mask, N>,
                or: Self,
            ) -> Self {
                let ptr = slice.as_ptr();
                // SAFETY: The safety of reading elements from `slice` is ensured by the caller.
                unsafe { Self::load_select_ptr(ptr, enable, or) }
            }

            /// Reads contiguous elements starting at `ptr`. Each element is read from memory if its
            /// corresponding element in `enable` is `true`.
            ///
            /// When the element is disabled, that memory location is not accessed and the corresponding
            /// value from `or` is passed through.
            ///
            /// # Safety
            /// Enabled `ptr` elements must be safe to read as if by `std::ptr::read`.
            #[must_use]
            #[inline]
            pub unsafe fn load_select_ptr(
                ptr: *const T,
                enable: Mask<<T as SimdElement>::Mask, N>,
                or: Self,
            ) -> Self {
                // SAFETY: The safety of reading elements through `ptr` is ensured by the caller.
                unsafe {
                    core::intrinsics::simd::simd_masked_load::<
                        _,
                        _,
                        _,
                        { core::intrinsics::simd::SimdAlign::Element },
                    >(enable.to_int(), ptr, or)
                }
            }

            /// Reads from potentially discontiguous indices in `slice` to construct a SIMD vector.
            /// If an index is out-of-bounds, the element is instead selected from the `or` vector.
            ///
            /// # Examples
            /// ```
            /// # #![feature(portable_simd)]
            /// # use core::simd::Simd;
            /// let vec: Vec<i32> = vec![10, 11, 12, 13, 14, 15, 16, 17, 18];
            /// let idxs = Simd::from_array([9, 3, 0, 5]);  // Note the index that is out-of-bounds
            /// let alt = Simd::from_array([-5, -4, -3, -2]);
            ///
            /// let result = Simd::gather_or(&vec, idxs, alt);
            /// assert_eq!(result, Simd::from_array([-5, 13, 10, 15]));
            /// ```
            #[must_use]
            #[inline]
            pub fn gather_or(slice: &[T], idxs: Simd<usize, N>, or: Self) -> Self {
                Self::gather_select(slice, Mask::splat(true), idxs, or)
            }

            /// Reads from indices in `slice` to construct a SIMD vector.
            /// If an index is out-of-bounds, the element is set to the default given by `T: Default`.
            ///
            /// # Examples
            /// ```
            /// # #![feature(portable_simd)]
            /// # use core::simd::Simd;
            /// let vec: Vec<i32> = vec![10, 11, 12, 13, 14, 15, 16, 17, 18];
            /// let idxs = Simd::from_array([9, 3, 0, 5]);  // Note the index that is out-of-bounds
            ///
            /// let result = Simd::gather_or_default(&vec, idxs);
            /// assert_eq!(result, Simd::from_array([0, 13, 10, 15]));
            /// ```
            #[must_use]
            #[inline]
            pub fn gather_or_default(slice: &[T], idxs: Simd<usize, N>) -> Self
            where
                T: Default,
            {
                Self::gather_or(slice, idxs, Self::splat(T::default()))
            }

            /// Reads from indices in `slice` to construct a SIMD vector.
            /// The mask `enable`s all `true` indices and disables all `false` indices.
            /// If an index is disabled or is out-of-bounds, the element is selected from the `or` vector.
            ///
            /// # Examples
            /// ```
            /// # #![feature(portable_simd)]
            /// # use core::simd::{Simd, Mask};
            /// let vec: Vec<i32> = vec![10, 11, 12, 13, 14, 15, 16, 17, 18];
            /// let idxs = Simd::from_array([9, 3, 0, 5]); // Includes an out-of-bounds index
            /// let alt = Simd::from_array([-5, -4, -3, -2]);
            /// let enable = Mask::from_array([true, true, true, false]); // Includes a masked element
            ///
            /// let result = Simd::gather_select(&vec, enable, idxs, alt);
            /// assert_eq!(result, Simd::from_array([-5, 13, 10, -2]));
            /// ```
            #[must_use]
            #[inline]
            pub fn gather_select(
                slice: &[T],
                enable: Mask<isize, N>,
                idxs: Simd<usize, N>,
                or: Self,
            ) -> Self {
                let enable: Mask<isize, N> = enable & idxs.simd_lt(Simd::splat(slice.len()));
                // Safety: We have masked-off out-of-bounds indices.
                unsafe { Self::gather_select_unchecked(slice, enable, idxs, or) }
            }

            /// Reads from indices in `slice` to construct a SIMD vector.
            /// The mask `enable`s all `true` indices and disables all `false` indices.
            /// If an index is disabled, the element is selected from the `or` vector.
            ///
            /// # Safety
            ///
            /// Calling this function with an `enable`d out-of-bounds index is *[undefined behavior]*
            /// even if the resulting value is not used.
            ///
            /// # Examples
            /// ```
            /// # #![feature(portable_simd)]
            /// # #[cfg(feature = "as_crate")] use core_simd::simd;
            /// # #[cfg(not(feature = "as_crate"))] use core::simd;
            /// # use simd::{Simd, cmp::SimdPartialOrd, Mask};
            /// let vec: Vec<i32> = vec![10, 11, 12, 13, 14, 15, 16, 17, 18];
            /// let idxs = Simd::from_array([9, 3, 0, 5]); // Includes an out-of-bounds index
            /// let alt = Simd::from_array([-5, -4, -3, -2]);
            /// let enable = Mask::from_array([true, true, true, false]); // Includes a masked element
            /// // If this mask was used to gather, it would be unsound. Let's fix that.
            /// let enable = enable & idxs.simd_lt(Simd::splat(vec.len()));
            ///
            /// // The out-of-bounds index has been masked, so it's safe to gather now.
            /// let result = unsafe { Simd::gather_select_unchecked(&vec, enable, idxs, alt) };
            /// assert_eq!(result, Simd::from_array([-5, 13, 10, -2]));
            /// ```
            /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
            #[must_use]
            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            pub unsafe fn gather_select_unchecked(
                slice: &[T],
                enable: Mask<isize, N>,
                idxs: Simd<usize, N>,
                or: Self,
            ) -> Self {
                let base_ptr = Simd::<*const T, N>::splat(slice.as_ptr());
                // Ferris forgive me, I have done pointer arithmetic here.
                let ptrs = base_ptr.wrapping_add(idxs);
                // Safety: The caller is responsible for determining the indices are okay to read
                unsafe { Self::gather_select_ptr(ptrs, enable, or) }
            }

            /// Reads elementwise from pointers into a SIMD vector.
            ///
            /// # Safety
            ///
            /// Each read must satisfy the same conditions as [`core::ptr::read`].
            ///
            /// # Example
            /// ```
            /// # #![feature(portable_simd)]
            /// # #[cfg(feature = "as_crate")] use core_simd::simd;
            /// # #[cfg(not(feature = "as_crate"))] use core::simd;
            /// # use simd::prelude::*;
            /// let values = [6, 2, 4, 9];
            /// let offsets = Simd::from_array([1, 0, 0, 3]);
            /// let source = Simd::splat(values.as_ptr()).wrapping_add(offsets);
            /// let gathered = unsafe { Simd::gather_ptr(source) };
            /// assert_eq!(gathered, Simd::from_array([2, 6, 6, 9]));
            /// ```
            #[must_use]
            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            pub unsafe fn gather_ptr(source: Simd<*const T, N>) -> Self
            where
                T: Default,
            {
                // TODO: add an intrinsic that doesn't use a passthru vector, and remove the T: Default bound
                // Safety: The caller is responsible for upholding all invariants
                unsafe { Self::gather_select_ptr(source, Mask::splat(true), Self::default()) }
            }

            /// Conditionally read elementwise from pointers into a SIMD vector.
            /// The mask `enable`s all `true` pointers and disables all `false` pointers.
            /// If a pointer is disabled, the element is selected from the `or` vector,
            /// and no read is performed.
            ///
            /// # Safety
            ///
            /// Enabled elements must satisfy the same conditions as [`core::ptr::read`].
            ///
            /// # Example
            /// ```
            /// # #![feature(portable_simd)]
            /// # #[cfg(feature = "as_crate")] use core_simd::simd;
            /// # #[cfg(not(feature = "as_crate"))] use core::simd;
            /// # use simd::prelude::*;
            /// let values = [6, 2, 4, 9];
            /// let enable = Mask::from_array([true, true, false, true]);
            /// let offsets = Simd::from_array([1, 0, 0, 3]);
            /// let source = Simd::splat(values.as_ptr()).wrapping_add(offsets);
            /// let gathered = unsafe { Simd::gather_select_ptr(source, enable, Simd::splat(0)) };
            /// assert_eq!(gathered, Simd::from_array([2, 6, 0, 9]));
            /// ```
            #[must_use]
            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            pub unsafe fn gather_select_ptr(
                source: Simd<*const T, N>,
                enable: Mask<isize, N>,
                or: Self,
            ) -> Self {
                // Safety: The caller is responsible for upholding all invariants
                unsafe { core::intrinsics::simd::simd_gather(or, source, enable.to_int()) }
            }

            /// Conditionally write contiguous elements to `slice`. The `enable` mask controls
            /// which elements are written, as long as they're in-bounds of the `slice`.
            /// If the element is disabled or out of bounds, no memory access to that location
            /// is made.
            ///
            /// # Examples
            /// ```
            /// # #![feature(portable_simd)]
            /// # #[cfg(feature = "as_crate")] use core_simd::simd;
            /// # #[cfg(not(feature = "as_crate"))] use core::simd;
            /// # use simd::{Simd, Mask};
            /// let mut arr = [0i32; 4];
            /// let write = Simd::from_array([-5, -4, -3, -2]);
            /// let enable = Mask::from_array([false, true, true, true]);
            ///
            /// write.store_select(&mut arr[..3], enable);
            /// assert_eq!(arr, [0, -4, -3, 0]);
            /// ```
            #[inline]
            pub fn store_select(self, slice: &mut [T], mut enable: Mask<<T as SimdElement>::Mask, N>) {
                enable &= mask_up_to(slice.len());
                // SAFETY: We performed the bounds check by updating the mask. &[T] is properly aligned to
                // the element.
                unsafe { self.store_select_ptr(slice.as_mut_ptr(), enable) }
            }

            /// Conditionally write contiguous elements to `slice`. The `enable` mask controls
            /// which elements are written.
            ///
            /// # Safety
            ///
            /// Every enabled element must be in bounds for the `slice`.
            ///
            /// # Examples
            /// ```
            /// # #![feature(portable_simd)]
            /// # #[cfg(feature = "as_crate")] use core_simd::simd;
            /// # #[cfg(not(feature = "as_crate"))] use core::simd;
            /// # use simd::{Simd, Mask};
            /// let mut arr = [0i32; 4];
            /// let write = Simd::from_array([-5, -4, -3, -2]);
            /// let enable = Mask::from_array([false, true, true, true]);
            ///
            /// unsafe { write.store_select_unchecked(&mut arr, enable) };
            /// assert_eq!(arr, [0, -4, -3, -2]);
            /// ```
            #[inline]
            pub unsafe fn store_select_unchecked(
                self,
                slice: &mut [T],
                enable: Mask<<T as SimdElement>::Mask, N>,
            ) {
                let ptr = slice.as_mut_ptr();
                // SAFETY: The safety of writing elements in `slice` is ensured by the caller.
                unsafe { self.store_select_ptr(ptr, enable) }
            }

            /// Conditionally write contiguous elements starting from `ptr`.
            /// The `enable` mask controls which elements are written.
            /// When disabled, the memory location corresponding to that element is not accessed.
            ///
            /// # Safety
            ///
            /// Memory addresses for element are calculated [`pointer::wrapping_offset`] and
            /// each enabled element must satisfy the same conditions as [`core::ptr::write`].
            #[inline]
            pub unsafe fn store_select_ptr(self, ptr: *mut T, enable: Mask<<T as SimdElement>::Mask, N>) {
                // SAFETY: The safety of writing elements through `ptr` is ensured by the caller.
                unsafe {
                    core::intrinsics::simd::simd_masked_store::<
                        _,
                        _,
                        _,
                        { core::intrinsics::simd::SimdAlign::Element },
                    >(enable.to_int(), ptr, self)
                }
            }

            /// Writes the values in a SIMD vector to potentially discontiguous indices in `slice`.
            /// If an index is out-of-bounds, the write is suppressed without panicking.
            /// If two elements in the scattered vector would write to the same index
            /// only the last element is guaranteed to actually be written.
            ///
            /// # Examples
            /// ```
            /// # #![feature(portable_simd)]
            /// # use core::simd::Simd;
            /// let mut vec: Vec<i32> = vec![10, 11, 12, 13, 14, 15, 16, 17, 18];
            /// let idxs = Simd::from_array([9, 3, 0, 0]); // Note the duplicate index.
            /// let vals = Simd::from_array([-27, 82, -41, 124]);
            ///
            /// vals.scatter(&mut vec, idxs); // two logical writes means the last wins.
            /// assert_eq!(vec, vec![124, 11, 12, 82, 14, 15, 16, 17, 18]);
            /// ```
            #[inline]
            pub fn scatter(self, slice: &mut [T], idxs: Simd<usize, N>) {
                self.scatter_select(slice, Mask::splat(true), idxs)
            }

            /// Writes values from a SIMD vector to multiple potentially discontiguous indices in `slice`.
            /// The mask `enable`s all `true` indices and disables all `false` indices.
            /// If an enabled index is out-of-bounds, the write is suppressed without panicking.
            /// If two enabled elements in the scattered vector would write to the same index,
            /// only the last element is guaranteed to actually be written.
            ///
            /// # Examples
            /// ```
            /// # #![feature(portable_simd)]
            /// # #[cfg(feature = "as_crate")] use core_simd::simd;
            /// # #[cfg(not(feature = "as_crate"))] use core::simd;
            /// # use simd::{Simd, Mask};
            /// let mut vec: Vec<i32> = vec![10, 11, 12, 13, 14, 15, 16, 17, 18];
            /// let idxs = Simd::from_array([9, 3, 0, 0]); // Includes an out-of-bounds index
            /// let vals = Simd::from_array([-27, 82, -41, 124]);
            /// let enable = Mask::from_array([true, true, true, false]); // Includes a masked element
            ///
            /// vals.scatter_select(&mut vec, enable, idxs); // The last write is masked, thus omitted.
            /// assert_eq!(vec, vec![-41, 11, 12, 82, 14, 15, 16, 17, 18]);
            /// ```
            #[inline]
            pub fn scatter_select(self, slice: &mut [T], enable: Mask<isize, N>, idxs: Simd<usize, N>) {
                let enable: Mask<isize, N> = enable & idxs.simd_lt(Simd::splat(slice.len()));
                // Safety: We have masked-off out-of-bounds indices.
                unsafe { self.scatter_select_unchecked(slice, enable, idxs) }
            }

            /// Writes values from a SIMD vector to multiple potentially discontiguous indices in `slice`.
            /// The mask `enable`s all `true` indices and disables all `false` indices.
            /// If two enabled elements in the scattered vector would write to the same index,
            /// only the last element is guaranteed to actually be written.
            ///
            /// # Safety
            ///
            /// Calling this function with an enabled out-of-bounds index is *[undefined behavior]*,
            /// and may lead to memory corruption.
            ///
            /// # Examples
            /// ```
            /// # #![feature(portable_simd)]
            /// # #[cfg(feature = "as_crate")] use core_simd::simd;
            /// # #[cfg(not(feature = "as_crate"))] use core::simd;
            /// # use simd::{Simd, cmp::SimdPartialOrd, Mask};
            /// let mut vec: Vec<i32> = vec![10, 11, 12, 13, 14, 15, 16, 17, 18];
            /// let idxs = Simd::from_array([9, 3, 0, 0]);
            /// let vals = Simd::from_array([-27, 82, -41, 124]);
            /// let enable = Mask::from_array([true, true, true, false]); // Masks the final index
            /// // If this mask was used to scatter, it would be unsound. Let's fix that.
            /// let enable = enable & idxs.simd_lt(Simd::splat(vec.len()));
            ///
            /// // We have masked the OOB index, so it's safe to scatter now.
            /// unsafe { vals.scatter_select_unchecked(&mut vec, enable, idxs); }
            /// // The second write to index 0 was masked, thus omitted.
            /// assert_eq!(vec, vec![-41, 11, 12, 82, 14, 15, 16, 17, 18]);
            /// ```
            /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            pub unsafe fn scatter_select_unchecked(
                self,
                slice: &mut [T],
                enable: Mask<isize, N>,
                idxs: Simd<usize, N>,
            ) {
                // Safety: This block works with *mut T derived from &mut 'a [T],
                // which means it is delicate in Rust's borrowing model, circa 2021:
                // &mut 'a [T] asserts uniqueness, so deriving &'a [T] invalidates live *mut Ts!
                // Even though this block is largely safe methods, it must be exactly this way
                // to prevent invalidating the raw ptrs while they're live.
                // Thus, entering this block requires all values to use being already ready:
                // 0. idxs we want to write to, which are used to construct the mask.
                // 1. enable, which depends on an initial &'a [T] and the idxs.
                // 2. actual values to scatter (self).
                // 3. &mut [T] which will become our base ptr.
                unsafe {
                    // Now Entering ☢️ *mut T Zone
                    let base_ptr = Simd::<*mut T, N>::splat(slice.as_mut_ptr());
                    // Ferris forgive me, I have done pointer arithmetic here.
                    let ptrs = base_ptr.wrapping_add(idxs);
                    // The ptrs have been bounds-masked to prevent memory-unsafe writes insha'allah
                    self.scatter_select_ptr(ptrs, enable);
                    // Cleared ☢️ *mut T Zone
                }
            }

            /// Writes pointers elementwise into a SIMD vector.
            ///
            /// # Safety
            ///
            /// Each write must satisfy the same conditions as [`core::ptr::write`].
            ///
            /// # Example
            /// ```
            /// # #![feature(portable_simd)]
            /// # #[cfg(feature = "as_crate")] use core_simd::simd;
            /// # #[cfg(not(feature = "as_crate"))] use core::simd;
            /// # use simd::{Simd, ptr::SimdMutPtr};
            /// let mut values = [0; 4];
            /// let offset = Simd::from_array([3, 2, 1, 0]);
            /// let ptrs = Simd::splat(values.as_mut_ptr()).wrapping_add(offset);
            /// unsafe { Simd::from_array([6, 3, 5, 7]).scatter_ptr(ptrs); }
            /// assert_eq!(values, [7, 5, 3, 6]);
            /// ```
            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            pub unsafe fn scatter_ptr(self, dest: Simd<*mut T, N>) {
                // Safety: The caller is responsible for upholding all invariants
                unsafe { self.scatter_select_ptr(dest, Mask::splat(true)) }
            }

            /// Conditionally write pointers elementwise into a SIMD vector.
            /// The mask `enable`s all `true` pointers and disables all `false` pointers.
            /// If a pointer is disabled, the write to its pointee is skipped.
            ///
            /// # Safety
            ///
            /// Enabled pointers must satisfy the same conditions as [`core::ptr::write`].
            ///
            /// # Example
            /// ```
            /// # #![feature(portable_simd)]
            /// # #[cfg(feature = "as_crate")] use core_simd::simd;
            /// # #[cfg(not(feature = "as_crate"))] use core::simd;
            /// # use simd::{Mask, Simd, ptr::SimdMutPtr};
            /// let mut values = [0; 4];
            /// let offset = Simd::from_array([3, 2, 1, 0]);
            /// let ptrs = Simd::splat(values.as_mut_ptr()).wrapping_add(offset);
            /// let enable = Mask::from_array([true, true, false, false]);
            /// unsafe { Simd::from_array([6, 3, 5, 7]).scatter_select_ptr(ptrs, enable); }
            /// assert_eq!(values, [0, 0, 3, 6]);
            /// ```
            #[inline]
            #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
            pub unsafe fn scatter_select_ptr(self, dest: Simd<*mut T, N>, enable: Mask<isize, N>) {
                // Safety: The caller is responsible for upholding all invariants
                unsafe { core::intrinsics::simd::simd_scatter(self, dest, enable.to_int()) }
            }
        }

        impl<T, const N: usize> Copy for Simd<T, N>
        where
            LaneCount<N>: SupportedLaneCount,
            T: SimdElement,
        {
        }

        impl<T, const N: usize> Clone for Simd<T, N>
        where
            LaneCount<N>: SupportedLaneCount,
            T: SimdElement,
        {
            #[inline]
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<T, const N: usize> Default for Simd<T, N>
        where
            LaneCount<N>: SupportedLaneCount,
            T: SimdElement + Default,
        {
            #[inline]
            fn default() -> Self {
                Self::splat(T::default())
            }
        }

        impl<T, const N: usize> PartialEq for Simd<T, N>
        where
            LaneCount<N>: SupportedLaneCount,
            T: SimdElement + PartialEq,
        {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                // Safety: All SIMD vectors are SimdPartialEq, and the comparison produces a valid mask.
                let mask = unsafe {
                    let tfvec: Simd<<T as SimdElement>::Mask, N> =
                        core::intrinsics::simd::simd_eq(*self, *other);
                    Mask::from_int_unchecked(tfvec)
                };

                // Two vectors are equal if all elements are equal when compared elementwise
                mask.all()
            }

            #[allow(clippy::partialeq_ne_impl)]
            #[inline]
            fn ne(&self, other: &Self) -> bool {
                // Safety: All SIMD vectors are SimdPartialEq, and the comparison produces a valid mask.
                let mask = unsafe {
                    let tfvec: Simd<<T as SimdElement>::Mask, N> =
                        core::intrinsics::simd::simd_ne(*self, *other);
                    Mask::from_int_unchecked(tfvec)
                };

                // Two vectors are non-equal if any elements are non-equal when compared elementwise
                mask.any()
            }
        }

        /// Lexicographic order. For the SIMD elementwise minimum and maximum, use simd_min and simd_max instead.
        impl<T, const N: usize> PartialOrd for Simd<T, N>
        where
            LaneCount<N>: SupportedLaneCount,
            T: SimdElement + PartialOrd,
        {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                // TODO use SIMD equality
                self.to_array().partial_cmp(other.as_ref())
            }
        }

        impl<T, const N: usize> Eq for Simd<T, N>
        where
            LaneCount<N>: SupportedLaneCount,
            T: SimdElement + Eq,
        {
        }

        /// Lexicographic order. For the SIMD elementwise minimum and maximum, use simd_min and simd_max instead.
        impl<T, const N: usize> Ord for Simd<T, N>
        where
            LaneCount<N>: SupportedLaneCount,
            T: SimdElement + Ord,
        {
            #[inline]
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                // TODO use SIMD equality
                self.to_array().cmp(other.as_ref())
            }
        }

        impl<T, const N: usize> core::hash::Hash for Simd<T, N>
        where
            LaneCount<N>: SupportedLaneCount,
            T: SimdElement + core::hash::Hash,
        {
            #[inline]
            fn hash<H>(&self, state: &mut H)
            where
                H: core::hash::Hasher,
            {
                self.as_array().hash(state)
            }
        }

        // array references
        impl<T, const N: usize> AsRef<[T; N]> for Simd<T, N>
        where
            LaneCount<N>: SupportedLaneCount,
            T: SimdElement,
        {
            #[inline]
            fn as_ref(&self) -> &[T; N] {
                self.as_array()
            }
        }

        impl<T, const N: usize> AsMut<[T; N]> for Simd<T, N>
        where
            LaneCount<N>: SupportedLaneCount,
            T: SimdElement,
        {
            #[inline]
            fn as_mut(&mut self) -> &mut [T; N] {
                self.as_mut_array()
            }
        }

        // slice references
        impl<T, const N: usize> AsRef<[T]> for Simd<T, N>
        where
            LaneCount<N>: SupportedLaneCount,
            T: SimdElement,
        {
            #[inline]
            fn as_ref(&self) -> &[T] {
                self.as_array()
            }
        }

        impl<T, const N: usize> AsMut<[T]> for Simd<T, N>
        where
            LaneCount<N>: SupportedLaneCount,
            T: SimdElement,
        {
            #[inline]
            fn as_mut(&mut self) -> &mut [T] {
                self.as_mut_array()
            }
        }

        // vector/array conversion
        impl<T, const N: usize> From<[T; N]> for Simd<T, N>
        where
            LaneCount<N>: SupportedLaneCount,
            T: SimdElement,
        {
            #[inline]
            fn from(array: [T; N]) -> Self {
                Self::from_array(array)
            }
        }

        impl<T, const N: usize> From<Simd<T, N>> for [T; N]
        where
            LaneCount<N>: SupportedLaneCount,
            T: SimdElement,
        {
            #[inline]
            fn from(vector: Simd<T, N>) -> Self {
                vector.to_array()
            }
        }

        impl<T, const N: usize> TryFrom<&[T]> for Simd<T, N>
        where
            LaneCount<N>: SupportedLaneCount,
            T: SimdElement,
        {
            type Error = core::array::TryFromSliceError;

            #[inline]
            fn try_from(slice: &[T]) -> Result<Self, core::array::TryFromSliceError> {
                Ok(Self::from_array(slice.try_into()?))
            }
        }

        impl<T, const N: usize> TryFrom<&mut [T]> for Simd<T, N>
        where
            LaneCount<N>: SupportedLaneCount,
            T: SimdElement,
        {
            type Error = core::array::TryFromSliceError;

            #[inline]
            fn try_from(slice: &mut [T]) -> Result<Self, core::array::TryFromSliceError> {
                Ok(Self::from_array(slice.try_into()?))
            }
        }

        mod sealed {
            pub trait Sealed {}
        }
        use sealed::Sealed;

        /// Marker trait for types that may be used as SIMD vector elements.
        ///
        /// # Safety
        /// This trait, when implemented, asserts the compiler can monomorphize
        /// `#[repr(simd)]` structs with the marked type as an element.
        /// Strictly, it is valid to impl if the vector will not be miscompiled.
        /// Practically, it is user-unfriendly to impl it if the vector won't compile,
        /// even when no soundness guarantees are broken by allowing the user to try.
        pub unsafe trait SimdElement: Sealed + Copy {
            /// The mask element type corresponding to this element type.
            type Mask: MaskElement;
        }

        impl Sealed for u8 {}

        // Safety: u8 is a valid SIMD element type, and is supported by this API
        unsafe impl SimdElement for u8 {
            type Mask = i8;
        }

        impl Sealed for u16 {}

        // Safety: u16 is a valid SIMD element type, and is supported by this API
        unsafe impl SimdElement for u16 {
            type Mask = i16;
        }

        impl Sealed for u32 {}

        // Safety: u32 is a valid SIMD element type, and is supported by this API
        unsafe impl SimdElement for u32 {
            type Mask = i32;
        }

        impl Sealed for u64 {}

        // Safety: u64 is a valid SIMD element type, and is supported by this API
        unsafe impl SimdElement for u64 {
            type Mask = i64;
        }

        impl Sealed for usize {}

        // Safety: usize is a valid SIMD element type, and is supported by this API
        unsafe impl SimdElement for usize {
            type Mask = isize;
        }

        impl Sealed for i8 {}

        // Safety: i8 is a valid SIMD element type, and is supported by this API
        unsafe impl SimdElement for i8 {
            type Mask = i8;
        }

        impl Sealed for i16 {}

        // Safety: i16 is a valid SIMD element type, and is supported by this API
        unsafe impl SimdElement for i16 {
            type Mask = i16;
        }

        impl Sealed for i32 {}

        // Safety: i32 is a valid SIMD element type, and is supported by this API
        unsafe impl SimdElement for i32 {
            type Mask = i32;
        }

        impl Sealed for i64 {}

        // Safety: i64 is a valid SIMD element type, and is supported by this API
        unsafe impl SimdElement for i64 {
            type Mask = i64;
        }

        impl Sealed for isize {}

        // Safety: isize is a valid SIMD element type, and is supported by this API
        unsafe impl SimdElement for isize {
            type Mask = isize;
        }

        impl Sealed for f32 {}

        // Safety: f32 is a valid SIMD element type, and is supported by this API
        unsafe impl SimdElement for f32 {
            type Mask = i32;
        }

        impl Sealed for f64 {}

        // Safety: f64 is a valid SIMD element type, and is supported by this API
        unsafe impl SimdElement for f64 {
            type Mask = i64;
        }

        impl<T> Sealed for *const T {}

        // Safety: (thin) const pointers are valid SIMD element types, and are supported by this API
        //
        // Fat pointers may be supported in the future.
        unsafe impl<T> SimdElement for *const T
        where
            T: core::ptr::Pointee<Metadata = ()>,
        {
            type Mask = isize;
        }

        impl<T> Sealed for *mut T {}

        // Safety: (thin) mut pointers are valid SIMD element types, and are supported by this API
        //
        // Fat pointers may be supported in the future.
        unsafe impl<T> SimdElement for *mut T
        where
            T: core::ptr::Pointee<Metadata = ()>,
        {
            type Mask = isize;
        }

        #[inline]
        fn lane_indices<const N: usize>() -> Simd<usize, N>
        where
            LaneCount<N>: SupportedLaneCount,
        {
            #![allow(clippy::needless_range_loop)]
            let mut index = [0; N];
            for i in 0..N {
                index[i] = i;
            }
            Simd::from_array(index)
        }

        #[inline]
        fn mask_up_to<M, const N: usize>(len: usize) -> Mask<M, N>
        where
            LaneCount<N>: SupportedLaneCount,
            M: MaskElement,
        {
            let index = lane_indices::<N>();
            let max_value: u64 = M::max_unsigned();
            macro_rules! case {
                ($ty:ty) => {
                    if N < <$ty>::MAX as usize && max_value as $ty as u64 == max_value {
                        return index.cast().simd_lt(Simd::splat(len.min(N) as $ty)).cast();
                    }
                };
            }
            case!(u8);
            case!(u16);
            case!(u32);
            case!(u64);
            index.simd_lt(Simd::splat(len)).cast()
        }
    }

    pub mod vendor
    {
        /*!
        */
        use ::
        {
            *
        };
        /*
        pub use std::vendor::{*};
        */
        /// Provides implementations of `From<$a> for $b` and `From<$b> for $a` that transmutes the value.
        #[allow(unused)]
        macro_rules! from_transmute {
            { unsafe $a:ty => $b:ty } => {
                from_transmute!{ @impl $a => $b }
                from_transmute!{ @impl $b => $a }
            };
            { @impl $from:ty => $to:ty } => {
                impl core::convert::From<$from> for $to {
                    #[inline]
                    fn from(value: $from) -> $to {
                        // Safety: transmuting between vectors is safe, but the caller of this macro
                        // checks the invariants
                        unsafe { core::mem::transmute(value) }
                    }
                }
            };
        }

        /// Conversions to x86's SIMD types.
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        pub mod x86
        {
            /*!
            */
            use ::
            {
                *
            };
            /*
            pub use std::${1}::{*};
            */
            use crate::simd::*;

            #[cfg(target_arch = "x86")]
            use crate::arch::x86::*;

            #[cfg(target_arch = "x86_64")]
            use crate::arch::x86_64::*;

            from_transmute! { unsafe u8x16 => __m128i }
            from_transmute! { unsafe u8x32 => __m256i }
            from_transmute! { unsafe u8x64 => __m512i }
            from_transmute! { unsafe i8x16 => __m128i }
            from_transmute! { unsafe i8x32 => __m256i }
            from_transmute! { unsafe i8x64 => __m512i }

            from_transmute! { unsafe u16x8 => __m128i }
            from_transmute! { unsafe u16x16 => __m256i }
            from_transmute! { unsafe u16x32 => __m512i }
            from_transmute! { unsafe i16x8 => __m128i }
            from_transmute! { unsafe i16x16 => __m256i }
            from_transmute! { unsafe i16x32 => __m512i }

            from_transmute! { unsafe u32x4 => __m128i }
            from_transmute! { unsafe u32x8 => __m256i }
            from_transmute! { unsafe u32x16 => __m512i }
            from_transmute! { unsafe i32x4 => __m128i }
            from_transmute! { unsafe i32x8 => __m256i }
            from_transmute! { unsafe i32x16 => __m512i }
            from_transmute! { unsafe f32x4 => __m128 }
            from_transmute! { unsafe f32x8 => __m256 }
            from_transmute! { unsafe f32x16 => __m512 }

            from_transmute! { unsafe u64x2 => __m128i }
            from_transmute! { unsafe u64x4 => __m256i }
            from_transmute! { unsafe u64x8 => __m512i }
            from_transmute! { unsafe i64x2 => __m128i }
            from_transmute! { unsafe i64x4 => __m256i }
            from_transmute! { unsafe i64x8 => __m512i }
            from_transmute! { unsafe f64x2 => __m128d }
            from_transmute! { unsafe f64x4 => __m256d }
            from_transmute! { unsafe f64x8 => __m512d }

            #[cfg(target_pointer_width = "32")]
            mod p32 {
                use super::*;
                from_transmute! { unsafe usizex4 => __m128i }
                from_transmute! { unsafe usizex8 => __m256i }
                from_transmute! { unsafe Simd<usize, 16> => __m512i }
                from_transmute! { unsafe isizex4 => __m128i }
                from_transmute! { unsafe isizex8 => __m256i }
                from_transmute! { unsafe Simd<isize, 16> => __m512i }
            }

            #[cfg(target_pointer_width = "64")]
            mod p64 {
                use super::*;
                from_transmute! { unsafe usizex2 => __m128i }
                from_transmute! { unsafe usizex4 => __m256i }
                from_transmute! { unsafe usizex8 => __m512i }
                from_transmute! { unsafe isizex2 => __m128i }
                from_transmute! { unsafe isizex4 => __m256i }
                from_transmute! { unsafe isizex8 => __m512i }
            }
        }

        #[cfg(target_arch = "wasm32")]
        pub mod wasm32
        {
            /*!
            */
            use ::
            {
                *
            };
            /*
            pub use std::${1}::{*};
            */
            
        }

        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "arm",))]
        pub mod arm
        {
            /*!
            */
            use ::
            {
                *
            };
            /*
            pub use std::${1}::{*};
            */
            
        }

        #[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
        pub mod powerpc
        {
            /*!
            */
            use ::
            {
                *
            };
            /*
            pub use std::${1}::{*};
            */
            
        }

        #[cfg(target_arch = "loongarch64")]
        pub mod loongarch64
        {
            /*!
            */
            use ::
            {
                *
            };
            /*
            pub use std::${1}::{*};
            */
            
        }
    }

    pub mod simd 
    {
        pub mod prelude
        {
            /*!
            */
            use ::
            {
                *
            };
            /*
            pub use std::prelude::{*};
            */
            //! The portable SIMD prelude.
            //!
            //! Includes important traits and types to be imported with a glob:
            //! ```ignore
            //! use std::simd::prelude::*;
            //! ```

            #[doc(no_inline)]
            pub use super::{
                Mask, Simd,
                cmp::{SimdOrd, SimdPartialEq, SimdPartialOrd},
                num::{SimdFloat, SimdInt, SimdUint},
                ptr::{SimdConstPtr, SimdMutPtr},
                simd_swizzle,
            };

            #[rustfmt::skip]
            #[doc(no_inline)]
            pub use super::{f32x1, f32x2, f32x4, f32x8, f32x16, f32x32, f32x64};

            #[rustfmt::skip]
            #[doc(no_inline)]
            pub use super::{f64x1, f64x2, f64x4, f64x8, f64x16, f64x32, f64x64};

            #[rustfmt::skip]
            #[doc(no_inline)]
            pub use super::{i8x1, i8x2, i8x4, i8x8, i8x16, i8x32, i8x64};

            #[rustfmt::skip]
            #[doc(no_inline)]
            pub use super::{i16x1, i16x2, i16x4, i16x8, i16x16, i16x32, i16x64};

            #[rustfmt::skip]
            #[doc(no_inline)]
            pub use super::{i32x1, i32x2, i32x4, i32x8, i32x16, i32x32, i32x64};

            #[rustfmt::skip]
            #[doc(no_inline)]
            pub use super::{i64x1, i64x2, i64x4, i64x8, i64x16, i64x32, i64x64};

            #[rustfmt::skip]
            #[doc(no_inline)]
            pub use super::{isizex1, isizex2, isizex4, isizex8, isizex16, isizex32, isizex64};

            #[rustfmt::skip]
            #[doc(no_inline)]
            pub use super::{u8x1, u8x2, u8x4, u8x8, u8x16, u8x32, u8x64};

            #[rustfmt::skip]
            #[doc(no_inline)]
            pub use super::{u16x1, u16x2, u16x4, u16x8, u16x16, u16x32, u16x64};

            #[rustfmt::skip]
            #[doc(no_inline)]
            pub use super::{u32x1, u32x2, u32x4, u32x8, u32x16, u32x32, u32x64};

            #[rustfmt::skip]
            #[doc(no_inline)]
            pub use super::{u64x1, u64x2, u64x4, u64x8, u64x16, u64x32, u64x64};

            #[rustfmt::skip]
            #[doc(no_inline)]
            pub use super::{usizex1, usizex2, usizex4, usizex8, usizex16, usizex32, usizex64};

            #[rustfmt::skip]
            #[doc(no_inline)]
            pub use super::{mask8x1, mask8x2, mask8x4, mask8x8, mask8x16, mask8x32, mask8x64};

            #[rustfmt::skip]
            #[doc(no_inline)]
            pub use super::{mask16x1, mask16x2, mask16x4, mask16x8, mask16x16, mask16x32, mask16x64};

            #[rustfmt::skip]
            #[doc(no_inline)]
            pub use super::{mask32x1, mask32x2, mask32x4, mask32x8, mask32x16, mask32x32, mask32x64};

            #[rustfmt::skip]
            #[doc(no_inline)]
            pub use super::{mask64x1, mask64x2, mask64x4, mask64x8, mask64x16, mask64x32, mask64x64};

            #[rustfmt::skip]
            #[doc(no_inline)]
            pub use super::{masksizex1, masksizex2, masksizex4, masksizex8, masksizex16, masksizex32, masksizex64};
        }

        pub mod num
        {
            /*!
            */
            use ::
            {
                *
            };
            /*
            pub use std::num::{*};

            use super::sealed::Sealed;
            use crate::simd::{
                LaneCount, Mask, Simd, SimdCast, SimdElement, SupportedLaneCount,
                cmp::{SimdPartialEq, SimdPartialOrd},
            };

            use super::sealed::Sealed;
            use crate::simd::{
                LaneCount, Mask, Simd, SimdCast, SimdElement, SupportedLaneCount, cmp::SimdOrd,
                cmp::SimdPartialOrd, num::SimdUint,
            };
            
            use super::sealed::Sealed;
            use crate::simd::{LaneCount, Simd, SimdCast, SimdElement, SupportedLaneCount, cmp::SimdOrd};
            */
            mod sealed {
                pub trait Sealed {}
            }

            /// Operations on SIMD vectors of unsigned integers.
            pub trait SimdUint: Copy + Sealed {
                /// Scalar type contained by this SIMD vector type.
                type Scalar;

                /// A SIMD vector with a different element type.
                type Cast<T: SimdElement>;

                /// Performs elementwise conversion of this vector's elements to another SIMD-valid type.
                ///
                /// This follows the semantics of Rust's `as` conversion for casting integers (wrapping to
                /// other integer types, and saturating to float types).
                #[must_use]
                fn cast<T: SimdCast>(self) -> Self::Cast<T>;

                /// Wrapping negation.
                ///
                /// Like [`u32::wrapping_neg`], all applications of this function will wrap, with the exception
                /// of `-0`.
                fn wrapping_neg(self) -> Self;

                /// Lanewise saturating add.
                ///
                /// # Examples
                /// ```
                /// # #![feature(portable_simd)]
                /// # #[cfg(feature = "as_crate")] use core_simd::simd;
                /// # #[cfg(not(feature = "as_crate"))] use core::simd;
                /// # use simd::prelude::*;
                /// use core::u32::MAX;
                /// let x = Simd::from_array([2, 1, 0, MAX]);
                /// let max = Simd::splat(MAX);
                /// let unsat = x + max;
                /// let sat = x.saturating_add(max);
                /// assert_eq!(unsat, Simd::from_array([1, 0, MAX, MAX - 1]));
                /// assert_eq!(sat, max);
                /// ```
                fn saturating_add(self, second: Self) -> Self;

                /// Lanewise saturating subtract.
                ///
                /// # Examples
                /// ```
                /// # #![feature(portable_simd)]
                /// # #[cfg(feature = "as_crate")] use core_simd::simd;
                /// # #[cfg(not(feature = "as_crate"))] use core::simd;
                /// # use simd::prelude::*;
                /// use core::u32::MAX;
                /// let x = Simd::from_array([2, 1, 0, MAX]);
                /// let max = Simd::splat(MAX);
                /// let unsat = x - max;
                /// let sat = x.saturating_sub(max);
                /// assert_eq!(unsat, Simd::from_array([3, 2, 1, 0]));
                /// assert_eq!(sat, Simd::splat(0));
                /// ```
                fn saturating_sub(self, second: Self) -> Self;

                /// Lanewise absolute difference.
                /// Every element becomes the absolute difference of `self` and `second`.
                ///
                /// # Examples
                /// ```
                /// # #![feature(portable_simd)]
                /// # #[cfg(feature = "as_crate")] use core_simd::simd;
                /// # #[cfg(not(feature = "as_crate"))] use core::simd;
                /// # use simd::prelude::*;
                /// use core::u32::MAX;
                /// let a = Simd::from_array([0, MAX, 100, 20]);
                /// let b = Simd::from_array([MAX, 0, 80, 200]);
                /// assert_eq!(a.abs_diff(b), Simd::from_array([MAX, MAX, 20, 180]));
                /// ```
                fn abs_diff(self, second: Self) -> Self;

                /// Returns the sum of the elements of the vector, with wrapping addition.
                fn reduce_sum(self) -> Self::Scalar;

                /// Returns the product of the elements of the vector, with wrapping multiplication.
                fn reduce_product(self) -> Self::Scalar;

                /// Returns the maximum element in the vector.
                fn reduce_max(self) -> Self::Scalar;

                /// Returns the minimum element in the vector.
                fn reduce_min(self) -> Self::Scalar;

                /// Returns the cumulative bitwise "and" across the elements of the vector.
                fn reduce_and(self) -> Self::Scalar;

                /// Returns the cumulative bitwise "or" across the elements of the vector.
                fn reduce_or(self) -> Self::Scalar;

                /// Returns the cumulative bitwise "xor" across the elements of the vector.
                fn reduce_xor(self) -> Self::Scalar;

                /// Reverses the byte order of each element.
                fn swap_bytes(self) -> Self;

                /// Reverses the order of bits in each elemnent.
                /// The least significant bit becomes the most significant bit, second least-significant bit becomes second most-significant bit, etc.
                fn reverse_bits(self) -> Self;

                /// Returns the number of ones in the binary representation of each element.
                fn count_ones(self) -> Self;

                /// Returns the number of zeros in the binary representation of each element.
                fn count_zeros(self) -> Self;

                /// Returns the number of leading zeros in the binary representation of each element.
                fn leading_zeros(self) -> Self;

                /// Returns the number of trailing zeros in the binary representation of each element.
                fn trailing_zeros(self) -> Self;

                /// Returns the number of leading ones in the binary representation of each element.
                fn leading_ones(self) -> Self;

                /// Returns the number of trailing ones in the binary representation of each element.
                fn trailing_ones(self) -> Self;
            }

            macro_rules! impl_trait {
                { $($ty:ident ($signed:ident)),* } => {
                    $(
                    impl<const N: usize> Sealed for Simd<$ty, N>
                    where
                        LaneCount<N>: SupportedLaneCount,
                    {
                    }

                    impl<const N: usize> SimdUint for Simd<$ty, N>
                    where
                        LaneCount<N>: SupportedLaneCount,
                    {
                        type Scalar = $ty;
                        type Cast<T: SimdElement> = Simd<T, N>;

                        #[inline]
                        fn cast<T: SimdCast>(self) -> Self::Cast<T> {
                            // Safety: supported types are guaranteed by SimdCast
                            unsafe { core::intrinsics::simd::simd_as(self) }
                        }

                        #[inline]
                        fn wrapping_neg(self) -> Self {
                            use crate::simd::num::SimdInt;
                            (-self.cast::<$signed>()).cast()
                        }

                        #[inline]
                        fn saturating_add(self, second: Self) -> Self {
                            // Safety: `self` is a vector
                            unsafe { core::intrinsics::simd::simd_saturating_add(self, second) }
                        }

                        #[inline]
                        fn saturating_sub(self, second: Self) -> Self {
                            // Safety: `self` is a vector
                            unsafe { core::intrinsics::simd::simd_saturating_sub(self, second) }
                        }

                        #[inline]
                        fn abs_diff(self, second: Self) -> Self {
                            let max = self.simd_max(second);
                            let min = self.simd_min(second);
                            max - min
                        }

                        #[inline]
                        fn reduce_sum(self) -> Self::Scalar {
                            // Safety: `self` is an integer vector
                            unsafe { core::intrinsics::simd::simd_reduce_add_ordered(self, 0) }
                        }

                        #[inline]
                        fn reduce_product(self) -> Self::Scalar {
                            // Safety: `self` is an integer vector
                            unsafe { core::intrinsics::simd::simd_reduce_mul_ordered(self, 1) }
                        }

                        #[inline]
                        fn reduce_max(self) -> Self::Scalar {
                            // Safety: `self` is an integer vector
                            unsafe { core::intrinsics::simd::simd_reduce_max(self) }
                        }

                        #[inline]
                        fn reduce_min(self) -> Self::Scalar {
                            // Safety: `self` is an integer vector
                            unsafe { core::intrinsics::simd::simd_reduce_min(self) }
                        }

                        #[inline]
                        fn reduce_and(self) -> Self::Scalar {
                            // Safety: `self` is an integer vector
                            unsafe { core::intrinsics::simd::simd_reduce_and(self) }
                        }

                        #[inline]
                        fn reduce_or(self) -> Self::Scalar {
                            // Safety: `self` is an integer vector
                            unsafe { core::intrinsics::simd::simd_reduce_or(self) }
                        }

                        #[inline]
                        fn reduce_xor(self) -> Self::Scalar {
                            // Safety: `self` is an integer vector
                            unsafe { core::intrinsics::simd::simd_reduce_xor(self) }
                        }

                        #[inline]
                        fn swap_bytes(self) -> Self {
                            // Safety: `self` is an integer vector
                            unsafe { core::intrinsics::simd::simd_bswap(self) }
                        }

                        #[inline]
                        fn reverse_bits(self) -> Self {
                            // Safety: `self` is an integer vector
                            unsafe { core::intrinsics::simd::simd_bitreverse(self) }
                        }

                        #[inline]
                        fn count_ones(self) -> Self {
                            // Safety: `self` is an integer vector
                            unsafe { core::intrinsics::simd::simd_ctpop(self) }
                        }

                        #[inline]
                        fn count_zeros(self) -> Self {
                            (!self).count_ones()
                        }

                        #[inline]
                        fn leading_zeros(self) -> Self {
                            // Safety: `self` is an integer vector
                            unsafe { core::intrinsics::simd::simd_ctlz(self) }
                        }

                        #[inline]
                        fn trailing_zeros(self) -> Self {
                            // Safety: `self` is an integer vector
                            unsafe { core::intrinsics::simd::simd_cttz(self) }
                        }

                        #[inline]
                        fn leading_ones(self) -> Self {
                            (!self).leading_zeros()
                        }

                        #[inline]
                        fn trailing_ones(self) -> Self {
                            (!self).trailing_zeros()
                        }
                    }
                    )*
                }
            }

            impl_trait! { u8 (i8), u16 (i16), u32 (i32), u64 (i64), usize (isize) }

            /// Operations on SIMD vectors of signed integers.
            pub trait SimdInt: Copy + Sealed {
                /// Mask type used for manipulating this SIMD vector type.
                type Mask;

                /// Scalar type contained by this SIMD vector type.
                type Scalar;

                /// A SIMD vector of unsigned integers with the same element size.
                type Unsigned;

                /// A SIMD vector with a different element type.
                type Cast<T: SimdElement>;

                /// Performs elementwise conversion of this vector's elements to another SIMD-valid type.
                ///
                /// This follows the semantics of Rust's `as` conversion for casting integers (wrapping to
                /// other integer types, and saturating to float types).
                #[must_use]
                fn cast<T: SimdCast>(self) -> Self::Cast<T>;

                /// Lanewise saturating add.
                ///
                /// # Examples
                /// ```
                /// # #![feature(portable_simd)]
                /// # #[cfg(feature = "as_crate")] use core_simd::simd;
                /// # #[cfg(not(feature = "as_crate"))] use core::simd;
                /// # use simd::prelude::*;
                /// use core::i32::{MIN, MAX};
                /// let x = Simd::from_array([MIN, 0, 1, MAX]);
                /// let max = Simd::splat(MAX);
                /// let unsat = x + max;
                /// let sat = x.saturating_add(max);
                /// assert_eq!(unsat, Simd::from_array([-1, MAX, MIN, -2]));
                /// assert_eq!(sat, Simd::from_array([-1, MAX, MAX, MAX]));
                /// ```
                fn saturating_add(self, second: Self) -> Self;

                /// Lanewise saturating subtract.
                ///
                /// # Examples
                /// ```
                /// # #![feature(portable_simd)]
                /// # #[cfg(feature = "as_crate")] use core_simd::simd;
                /// # #[cfg(not(feature = "as_crate"))] use core::simd;
                /// # use simd::prelude::*;
                /// use core::i32::{MIN, MAX};
                /// let x = Simd::from_array([MIN, -2, -1, MAX]);
                /// let max = Simd::splat(MAX);
                /// let unsat = x - max;
                /// let sat = x.saturating_sub(max);
                /// assert_eq!(unsat, Simd::from_array([1, MAX, MIN, 0]));
                /// assert_eq!(sat, Simd::from_array([MIN, MIN, MIN, 0]));
                /// ```
                fn saturating_sub(self, second: Self) -> Self;

                /// Lanewise absolute value, implemented in Rust.
                /// Every element becomes its absolute value.
                ///
                /// # Examples
                /// ```
                /// # #![feature(portable_simd)]
                /// # #[cfg(feature = "as_crate")] use core_simd::simd;
                /// # #[cfg(not(feature = "as_crate"))] use core::simd;
                /// # use simd::prelude::*;
                /// use core::i32::{MIN, MAX};
                /// let xs = Simd::from_array([MIN, MIN + 1, -5, 0]);
                /// assert_eq!(xs.abs(), Simd::from_array([MIN, MAX, 5, 0]));
                /// ```
                fn abs(self) -> Self;

                /// Lanewise absolute difference.
                /// Every element becomes the absolute difference of `self` and `second`.
                ///
                /// # Examples
                /// ```
                /// # #![feature(portable_simd)]
                /// # #[cfg(feature = "as_crate")] use core_simd::simd;
                /// # #[cfg(not(feature = "as_crate"))] use core::simd;
                /// # use simd::prelude::*;
                /// use core::i32::{MIN, MAX};
                /// let a = Simd::from_array([MIN, MAX, 100, -100]);
                /// let b = Simd::from_array([MAX, MIN, -80, -120]);
                /// assert_eq!(a.abs_diff(b), Simd::from_array([u32::MAX, u32::MAX, 180, 20]));
                /// ```
                fn abs_diff(self, second: Self) -> Self::Unsigned;

                /// Lanewise saturating absolute value, implemented in Rust.
                /// As abs(), except the MIN value becomes MAX instead of itself.
                ///
                /// # Examples
                /// ```
                /// # #![feature(portable_simd)]
                /// # #[cfg(feature = "as_crate")] use core_simd::simd;
                /// # #[cfg(not(feature = "as_crate"))] use core::simd;
                /// # use simd::prelude::*;
                /// use core::i32::{MIN, MAX};
                /// let xs = Simd::from_array([MIN, -2, 0, 3]);
                /// let unsat = xs.abs();
                /// let sat = xs.saturating_abs();
                /// assert_eq!(unsat, Simd::from_array([MIN, 2, 0, 3]));
                /// assert_eq!(sat, Simd::from_array([MAX, 2, 0, 3]));
                /// ```
                fn saturating_abs(self) -> Self;

                /// Lanewise saturating negation, implemented in Rust.
                /// As neg(), except the MIN value becomes MAX instead of itself.
                ///
                /// # Examples
                /// ```
                /// # #![feature(portable_simd)]
                /// # #[cfg(feature = "as_crate")] use core_simd::simd;
                /// # #[cfg(not(feature = "as_crate"))] use core::simd;
                /// # use simd::prelude::*;
                /// use core::i32::{MIN, MAX};
                /// let x = Simd::from_array([MIN, -2, 3, MAX]);
                /// let unsat = -x;
                /// let sat = x.saturating_neg();
                /// assert_eq!(unsat, Simd::from_array([MIN, 2, -3, MIN + 1]));
                /// assert_eq!(sat, Simd::from_array([MAX, 2, -3, MIN + 1]));
                /// ```
                fn saturating_neg(self) -> Self;

                /// Returns true for each positive element and false if it is zero or negative.
                fn is_positive(self) -> Self::Mask;

                /// Returns true for each negative element and false if it is zero or positive.
                fn is_negative(self) -> Self::Mask;

                /// Returns numbers representing the sign of each element.
                /// * `0` if the number is zero
                /// * `1` if the number is positive
                /// * `-1` if the number is negative
                fn signum(self) -> Self;

                /// Returns the sum of the elements of the vector, with wrapping addition.
                ///
                /// # Examples
                ///
                /// ```
                /// # #![feature(portable_simd)]
                /// # #[cfg(feature = "as_crate")] use core_simd::simd;
                /// # #[cfg(not(feature = "as_crate"))] use core::simd;
                /// # use simd::prelude::*;
                /// let v = i32x4::from_array([1, 2, 3, 4]);
                /// assert_eq!(v.reduce_sum(), 10);
                ///
                /// // SIMD integer addition is always wrapping
                /// let v = i32x4::from_array([i32::MAX, 1, 0, 0]);
                /// assert_eq!(v.reduce_sum(), i32::MIN);
                /// ```
                fn reduce_sum(self) -> Self::Scalar;

                /// Returns the product of the elements of the vector, with wrapping multiplication.
                ///
                /// # Examples
                ///
                /// ```
                /// # #![feature(portable_simd)]
                /// # #[cfg(feature = "as_crate")] use core_simd::simd;
                /// # #[cfg(not(feature = "as_crate"))] use core::simd;
                /// # use simd::prelude::*;
                /// let v = i32x4::from_array([1, 2, 3, 4]);
                /// assert_eq!(v.reduce_product(), 24);
                ///
                /// // SIMD integer multiplication is always wrapping
                /// let v = i32x4::from_array([i32::MAX, 2, 1, 1]);
                /// assert!(v.reduce_product() < i32::MAX);
                /// ```
                fn reduce_product(self) -> Self::Scalar;

                /// Returns the maximum element in the vector.
                ///
                /// # Examples
                ///
                /// ```
                /// # #![feature(portable_simd)]
                /// # #[cfg(feature = "as_crate")] use core_simd::simd;
                /// # #[cfg(not(feature = "as_crate"))] use core::simd;
                /// # use simd::prelude::*;
                /// let v = i32x4::from_array([1, 2, 3, 4]);
                /// assert_eq!(v.reduce_max(), 4);
                /// ```
                fn reduce_max(self) -> Self::Scalar;

                /// Returns the minimum element in the vector.
                ///
                /// # Examples
                ///
                /// ```
                /// # #![feature(portable_simd)]
                /// # #[cfg(feature = "as_crate")] use core_simd::simd;
                /// # #[cfg(not(feature = "as_crate"))] use core::simd;
                /// # use simd::prelude::*;
                /// let v = i32x4::from_array([1, 2, 3, 4]);
                /// assert_eq!(v.reduce_min(), 1);
                /// ```
                fn reduce_min(self) -> Self::Scalar;

                /// Returns the cumulative bitwise "and" across the elements of the vector.
                fn reduce_and(self) -> Self::Scalar;

                /// Returns the cumulative bitwise "or" across the elements of the vector.
                fn reduce_or(self) -> Self::Scalar;

                /// Returns the cumulative bitwise "xor" across the elements of the vector.
                fn reduce_xor(self) -> Self::Scalar;

                /// Reverses the byte order of each element.
                fn swap_bytes(self) -> Self;

                /// Reverses the order of bits in each elemnent.
                /// The least significant bit becomes the most significant bit, second least-significant bit becomes second most-significant bit, etc.
                fn reverse_bits(self) -> Self;

                /// Returns the number of ones in the binary representation of each element.
                fn count_ones(self) -> Self::Unsigned;

                /// Returns the number of zeros in the binary representation of each element.
                fn count_zeros(self) -> Self::Unsigned;

                /// Returns the number of leading zeros in the binary representation of each element.
                fn leading_zeros(self) -> Self::Unsigned;

                /// Returns the number of trailing zeros in the binary representation of each element.
                fn trailing_zeros(self) -> Self::Unsigned;

                /// Returns the number of leading ones in the binary representation of each element.
                fn leading_ones(self) -> Self::Unsigned;

                /// Returns the number of trailing ones in the binary representation of each element.
                fn trailing_ones(self) -> Self::Unsigned;
            }

            macro_rules! impl_trait {
                { $($ty:ident ($unsigned:ident)),* } => {
                    $(
                    impl<const N: usize> Sealed for Simd<$ty, N>
                    where
                        LaneCount<N>: SupportedLaneCount,
                    {
                    }

                    impl<const N: usize> SimdInt for Simd<$ty, N>
                    where
                        LaneCount<N>: SupportedLaneCount,
                    {
                        type Mask = Mask<<$ty as SimdElement>::Mask, N>;
                        type Scalar = $ty;
                        type Unsigned = Simd<$unsigned, N>;
                        type Cast<T: SimdElement> = Simd<T, N>;

                        #[inline]
                        fn cast<T: SimdCast>(self) -> Self::Cast<T> {
                            // Safety: supported types are guaranteed by SimdCast
                            unsafe { core::intrinsics::simd::simd_as(self) }
                        }

                        #[inline]
                        fn saturating_add(self, second: Self) -> Self {
                            // Safety: `self` is a vector
                            unsafe { core::intrinsics::simd::simd_saturating_add(self, second) }
                        }

                        #[inline]
                        fn saturating_sub(self, second: Self) -> Self {
                            // Safety: `self` is a vector
                            unsafe { core::intrinsics::simd::simd_saturating_sub(self, second) }
                        }

                        #[inline]
                        fn abs(self) -> Self {
                            const SHR: $ty = <$ty>::BITS as $ty - 1;
                            let m = self >> Simd::splat(SHR);
                            (self^m) - m
                        }

                        #[inline]
                        fn abs_diff(self, second: Self) -> Self::Unsigned {
                            let max = self.simd_max(second);
                            let min = self.simd_min(second);
                            (max - min).cast()
                        }

                        #[inline]
                        fn saturating_abs(self) -> Self {
                            // arith shift for -1 or 0 mask based on sign bit, giving 2s complement
                            const SHR: $ty = <$ty>::BITS as $ty - 1;
                            let m = self >> Simd::splat(SHR);
                            (self^m).saturating_sub(m)
                        }

                        #[inline]
                        fn saturating_neg(self) -> Self {
                            Self::splat(0).saturating_sub(self)
                        }

                        #[inline]
                        fn is_positive(self) -> Self::Mask {
                            self.simd_gt(Self::splat(0))
                        }

                        #[inline]
                        fn is_negative(self) -> Self::Mask {
                            self.simd_lt(Self::splat(0))
                        }

                        #[inline]
                        fn signum(self) -> Self {
                            self.is_positive().select(
                                Self::splat(1),
                                self.is_negative().select(Self::splat(-1), Self::splat(0))
                            )
                        }

                        #[inline]
                        fn reduce_sum(self) -> Self::Scalar {
                            // Safety: `self` is an integer vector
                            unsafe { core::intrinsics::simd::simd_reduce_add_ordered(self, 0) }
                        }

                        #[inline]
                        fn reduce_product(self) -> Self::Scalar {
                            // Safety: `self` is an integer vector
                            unsafe { core::intrinsics::simd::simd_reduce_mul_ordered(self, 1) }
                        }

                        #[inline]
                        fn reduce_max(self) -> Self::Scalar {
                            // Safety: `self` is an integer vector
                            unsafe { core::intrinsics::simd::simd_reduce_max(self) }
                        }

                        #[inline]
                        fn reduce_min(self) -> Self::Scalar {
                            // Safety: `self` is an integer vector
                            unsafe { core::intrinsics::simd::simd_reduce_min(self) }
                        }

                        #[inline]
                        fn reduce_and(self) -> Self::Scalar {
                            // Safety: `self` is an integer vector
                            unsafe { core::intrinsics::simd::simd_reduce_and(self) }
                        }

                        #[inline]
                        fn reduce_or(self) -> Self::Scalar {
                            // Safety: `self` is an integer vector
                            unsafe { core::intrinsics::simd::simd_reduce_or(self) }
                        }

                        #[inline]
                        fn reduce_xor(self) -> Self::Scalar {
                            // Safety: `self` is an integer vector
                            unsafe { core::intrinsics::simd::simd_reduce_xor(self) }
                        }

                        #[inline]
                        fn swap_bytes(self) -> Self {
                            // Safety: `self` is an integer vector
                            unsafe { core::intrinsics::simd::simd_bswap(self) }
                        }

                        #[inline]
                        fn reverse_bits(self) -> Self {
                            // Safety: `self` is an integer vector
                            unsafe { core::intrinsics::simd::simd_bitreverse(self) }
                        }

                        #[inline]
                        fn count_ones(self) -> Self::Unsigned {
                            self.cast::<$unsigned>().count_ones()
                        }

                        #[inline]
                        fn count_zeros(self) -> Self::Unsigned {
                            self.cast::<$unsigned>().count_zeros()
                        }

                        #[inline]
                        fn leading_zeros(self) -> Self::Unsigned {
                            self.cast::<$unsigned>().leading_zeros()
                        }

                        #[inline]
                        fn trailing_zeros(self) -> Self::Unsigned {
                            self.cast::<$unsigned>().trailing_zeros()
                        }

                        #[inline]
                        fn leading_ones(self) -> Self::Unsigned {
                            self.cast::<$unsigned>().leading_ones()
                        }

                        #[inline]
                        fn trailing_ones(self) -> Self::Unsigned {
                            self.cast::<$unsigned>().trailing_ones()
                        }
                    }
                    )*
                }
            }

            impl_trait! { i8 (u8), i16 (u16), i32 (u32), i64 (u64), isize (usize) }

            

            /// Operations on SIMD vectors of floats.
            pub trait SimdFloat: Copy + Sealed {
                /// Mask type used for manipulating this SIMD vector type.
                type Mask;

                /// Scalar type contained by this SIMD vector type.
                type Scalar;

                /// Bit representation of this SIMD vector type.
                type Bits;

                /// A SIMD vector with a different element type.
                type Cast<T: SimdElement>;

                /// Performs elementwise conversion of this vector's elements to another SIMD-valid type.
                ///
                /// This follows the semantics of Rust's `as` conversion for floats (truncating or saturating
                /// at the limits) for each element.
                ///
                /// # Example
                /// ```
                /// # #![feature(portable_simd)]
                /// # #[cfg(feature = "as_crate")] use core_simd::simd;
                /// # #[cfg(not(feature = "as_crate"))] use core::simd;
                /// # use simd::prelude::*;
                /// let floats: Simd<f32, 4> = Simd::from_array([1.9, -4.5, f32::INFINITY, f32::NAN]);
                /// let ints = floats.cast::<i32>();
                /// assert_eq!(ints, Simd::from_array([1, -4, i32::MAX, 0]));
                ///
                /// // Formally equivalent, but `Simd::cast` can optimize better.
                /// assert_eq!(ints, Simd::from_array(floats.to_array().map(|x| x as i32)));
                ///
                /// // The float conversion does not round-trip.
                /// let floats_again = ints.cast();
                /// assert_ne!(floats, floats_again);
                /// assert_eq!(floats_again, Simd::from_array([1.0, -4.0, 2147483647.0, 0.0]));
                /// ```
                #[must_use]
                fn cast<T: SimdCast>(self) -> Self::Cast<T>;

                /// Rounds toward zero and converts to the same-width integer type, assuming that
                /// the value is finite and fits in that type.
                ///
                /// # Safety
                /// The value must:
                ///
                /// * Not be NaN
                /// * Not be infinite
                /// * Be representable in the return type, after truncating off its fractional part
                ///
                /// If these requirements are infeasible or costly, consider using the safe function [cast],
                /// which saturates on conversion.
                ///
                /// [cast]: Simd::cast
                unsafe fn to_int_unchecked<I: SimdCast>(self) -> Self::Cast<I>
                where
                    Self::Scalar: core::convert::FloatToInt<I>;

                /// Raw transmutation to an unsigned integer vector type with the
                /// same size and number of elements.
                #[must_use = "method returns a new vector and does not mutate the original value"]
                fn to_bits(self) -> Self::Bits;

                /// Raw transmutation from an unsigned integer vector type with the
                /// same size and number of elements.
                #[must_use = "method returns a new vector and does not mutate the original value"]
                fn from_bits(bits: Self::Bits) -> Self;

                /// Produces a vector where every element has the absolute value of the
                /// equivalently-indexed element in `self`.
                #[must_use = "method returns a new vector and does not mutate the original value"]
                fn abs(self) -> Self;

                /// Takes the reciprocal (inverse) of each element, `1/x`.
                #[must_use = "method returns a new vector and does not mutate the original value"]
                fn recip(self) -> Self;

                /// Converts each element from radians to degrees.
                #[must_use = "method returns a new vector and does not mutate the original value"]
                fn to_degrees(self) -> Self;

                /// Converts each element from degrees to radians.
                #[must_use = "method returns a new vector and does not mutate the original value"]
                fn to_radians(self) -> Self;

                /// Returns true for each element if it has a positive sign, including
                /// `+0.0`, `NaN`s with positive sign bit and positive infinity.
                #[must_use = "method returns a new mask and does not mutate the original value"]
                fn is_sign_positive(self) -> Self::Mask;

                /// Returns true for each element if it has a negative sign, including
                /// `-0.0`, `NaN`s with negative sign bit and negative infinity.
                #[must_use = "method returns a new mask and does not mutate the original value"]
                fn is_sign_negative(self) -> Self::Mask;

                /// Returns true for each element if its value is `NaN`.
                #[must_use = "method returns a new mask and does not mutate the original value"]
                fn is_nan(self) -> Self::Mask;

                /// Returns true for each element if its value is positive infinity or negative infinity.
                #[must_use = "method returns a new mask and does not mutate the original value"]
                fn is_infinite(self) -> Self::Mask;

                /// Returns true for each element if its value is neither infinite nor `NaN`.
                #[must_use = "method returns a new mask and does not mutate the original value"]
                fn is_finite(self) -> Self::Mask;

                /// Returns true for each element if its value is subnormal.
                #[must_use = "method returns a new mask and does not mutate the original value"]
                fn is_subnormal(self) -> Self::Mask;

                /// Returns true for each element if its value is neither zero, infinite,
                /// subnormal, nor `NaN`.
                #[must_use = "method returns a new mask and does not mutate the original value"]
                fn is_normal(self) -> Self::Mask;

                /// Replaces each element with a number that represents its sign.
                ///
                /// * `1.0` if the number is positive, `+0.0`, or `INFINITY`
                /// * `-1.0` if the number is negative, `-0.0`, or `NEG_INFINITY`
                /// * `NAN` if the number is `NAN`
                #[must_use = "method returns a new vector and does not mutate the original value"]
                fn signum(self) -> Self;

                /// Returns each element with the magnitude of `self` and the sign of `sign`.
                ///
                /// For any element containing a `NAN`, a `NAN` with the sign of `sign` is returned.
                #[must_use = "method returns a new vector and does not mutate the original value"]
                fn copysign(self, sign: Self) -> Self;

                /// Returns the minimum of each element.
                ///
                /// If one of the values is `NAN`, then the other value is returned.
                #[must_use = "method returns a new vector and does not mutate the original value"]
                fn simd_min(self, other: Self) -> Self;

                /// Returns the maximum of each element.
                ///
                /// If one of the values is `NAN`, then the other value is returned.
                #[must_use = "method returns a new vector and does not mutate the original value"]
                fn simd_max(self, other: Self) -> Self;

                /// Restrict each element to a certain interval unless it is NaN.
                ///
                /// For each element in `self`, returns the corresponding element in `max` if the element is
                /// greater than `max`, and the corresponding element in `min` if the element is less
                /// than `min`.  Otherwise returns the element in `self`.
                #[must_use = "method returns a new vector and does not mutate the original value"]
                fn simd_clamp(self, min: Self, max: Self) -> Self;

                /// Returns the sum of the elements of the vector.
                ///
                /// # Examples
                ///
                /// ```
                /// # #![feature(portable_simd)]
                /// # #[cfg(feature = "as_crate")] use core_simd::simd;
                /// # #[cfg(not(feature = "as_crate"))] use core::simd;
                /// # use simd::prelude::*;
                /// let v = f32x2::from_array([1., 2.]);
                /// assert_eq!(v.reduce_sum(), 3.);
                /// ```
                fn reduce_sum(self) -> Self::Scalar;

                /// Reducing multiply.  Returns the product of the elements of the vector.
                ///
                /// # Examples
                ///
                /// ```
                /// # #![feature(portable_simd)]
                /// # #[cfg(feature = "as_crate")] use core_simd::simd;
                /// # #[cfg(not(feature = "as_crate"))] use core::simd;
                /// # use simd::prelude::*;
                /// let v = f32x2::from_array([3., 4.]);
                /// assert_eq!(v.reduce_product(), 12.);
                /// ```
                fn reduce_product(self) -> Self::Scalar;

                /// Returns the maximum element in the vector.
                ///
                /// Returns values based on equality, so a vector containing both `0.` and `-0.` may
                /// return either.
                ///
                /// This function will not return `NaN` unless all elements are `NaN`.
                ///
                /// # Examples
                ///
                /// ```
                /// # #![feature(portable_simd)]
                /// # #[cfg(feature = "as_crate")] use core_simd::simd;
                /// # #[cfg(not(feature = "as_crate"))] use core::simd;
                /// # use simd::prelude::*;
                /// let v = f32x2::from_array([1., 2.]);
                /// assert_eq!(v.reduce_max(), 2.);
                ///
                /// // NaN values are skipped...
                /// let v = f32x2::from_array([1., f32::NAN]);
                /// assert_eq!(v.reduce_max(), 1.);
                ///
                /// // ...unless all values are NaN
                /// let v = f32x2::from_array([f32::NAN, f32::NAN]);
                /// assert!(v.reduce_max().is_nan());
                /// ```
                fn reduce_max(self) -> Self::Scalar;

                /// Returns the minimum element in the vector.
                ///
                /// Returns values based on equality, so a vector containing both `0.` and `-0.` may
                /// return either.
                ///
                /// This function will not return `NaN` unless all elements are `NaN`.
                ///
                /// # Examples
                ///
                /// ```
                /// # #![feature(portable_simd)]
                /// # #[cfg(feature = "as_crate")] use core_simd::simd;
                /// # #[cfg(not(feature = "as_crate"))] use core::simd;
                /// # use simd::prelude::*;
                /// let v = f32x2::from_array([3., 7.]);
                /// assert_eq!(v.reduce_min(), 3.);
                ///
                /// // NaN values are skipped...
                /// let v = f32x2::from_array([1., f32::NAN]);
                /// assert_eq!(v.reduce_min(), 1.);
                ///
                /// // ...unless all values are NaN
                /// let v = f32x2::from_array([f32::NAN, f32::NAN]);
                /// assert!(v.reduce_min().is_nan());
                /// ```
                fn reduce_min(self) -> Self::Scalar;
            }

            macro_rules! impl_trait {
                { $($ty:ty { bits: $bits_ty:ty, mask: $mask_ty:ty }),* } => {
                    $(
                    impl<const N: usize> Sealed for Simd<$ty, N>
                    where
                        LaneCount<N>: SupportedLaneCount,
                    {
                    }

                    impl<const N: usize> SimdFloat for Simd<$ty, N>
                    where
                        LaneCount<N>: SupportedLaneCount,
                    {
                        type Mask = Mask<<$mask_ty as SimdElement>::Mask, N>;
                        type Scalar = $ty;
                        type Bits = Simd<$bits_ty, N>;
                        type Cast<T: SimdElement> = Simd<T, N>;

                        #[cfg(not(target_arch = "aarch64"))]
                        #[inline]
                        fn cast<T: SimdCast>(self) -> Self::Cast<T>
                        {
                            // Safety: supported types are guaranteed by SimdCast
                            unsafe { core::intrinsics::simd::simd_as(self) }
                        }

                        // workaround for https://github.com/llvm/llvm-project/issues/94694 (fixed in LLVM 20)
                        // tracked in: https://github.com/rust-lang/rust/issues/135982
                        #[cfg(target_arch = "aarch64")]
                        #[inline]
                        fn cast<T: SimdCast>(self) -> Self::Cast<T>
                        {
                            const { assert!(N <= 64) };
                            if N <= 2 || N == 4 || N == 8 || N == 16 || N == 32 || N == 64 {
                                // Safety: supported types are guaranteed by SimdCast
                                unsafe { core::intrinsics::simd::simd_as(self) }
                            } else if N < 4 {
                                let x = self.resize::<4>(Default::default()).cast();
                                x.resize::<N>(x[0])
                            } else if N < 8 {
                                let x = self.resize::<8>(Default::default()).cast();
                                x.resize::<N>(x[0])
                            } else if N < 16 {
                                let x = self.resize::<16>(Default::default()).cast();
                                x.resize::<N>(x[0])
                            } else if N < 32 {
                                let x = self.resize::<32>(Default::default()).cast();
                                x.resize::<N>(x[0])
                            } else {
                                let x = self.resize::<64>(Default::default()).cast();
                                x.resize::<N>(x[0])
                            }
                        }

                        #[inline]
                        #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
                        unsafe fn to_int_unchecked<I: SimdCast>(self) -> Self::Cast<I>
                        where
                            Self::Scalar: core::convert::FloatToInt<I>,
                        {
                            // Safety: supported types are guaranteed by SimdCast, the caller is responsible for the extra invariants
                            unsafe { core::intrinsics::simd::simd_cast(self) }
                        }

                        #[inline]
                        fn to_bits(self) -> Simd<$bits_ty, N> {
                            assert_eq!(size_of::<Self>(), size_of::<Self::Bits>());
                            // Safety: transmuting between vector types is safe
                            unsafe { core::mem::transmute_copy(&self) }
                        }

                        #[inline]
                        fn from_bits(bits: Simd<$bits_ty, N>) -> Self {
                            assert_eq!(size_of::<Self>(), size_of::<Self::Bits>());
                            // Safety: transmuting between vector types is safe
                            unsafe { core::mem::transmute_copy(&bits) }
                        }

                        #[inline]
                        fn abs(self) -> Self {
                            // Safety: `self` is a float vector
                            unsafe { core::intrinsics::simd::simd_fabs(self) }
                        }

                        #[inline]
                        fn recip(self) -> Self {
                            Self::splat(1.0) / self
                        }

                        #[inline]
                        fn to_degrees(self) -> Self {
                            // to_degrees uses a special constant for better precision, so extract that constant
                            self * Self::splat(Self::Scalar::to_degrees(1.))
                        }

                        #[inline]
                        fn to_radians(self) -> Self {
                            self * Self::splat(Self::Scalar::to_radians(1.))
                        }

                        #[inline]
                        fn is_sign_positive(self) -> Self::Mask {
                            !self.is_sign_negative()
                        }

                        #[inline]
                        fn is_sign_negative(self) -> Self::Mask {
                            let sign_bits = self.to_bits() & Simd::splat((!0 >> 1) + 1);
                            sign_bits.simd_gt(Simd::splat(0))
                        }

                        #[inline]
                        fn is_nan(self) -> Self::Mask {
                            self.simd_ne(self)
                        }

                        #[inline]
                        fn is_infinite(self) -> Self::Mask {
                            self.abs().simd_eq(Self::splat(Self::Scalar::INFINITY))
                        }

                        #[inline]
                        fn is_finite(self) -> Self::Mask {
                            self.abs().simd_lt(Self::splat(Self::Scalar::INFINITY))
                        }

                        #[inline]
                        fn is_subnormal(self) -> Self::Mask {
                            // On some architectures (e.g. armv7 and some ppc) subnormals are flushed to zero,
                            // so this comparison must be done with integers.
                            let not_zero = self.abs().to_bits().simd_ne(Self::splat(0.0).to_bits());
                            not_zero & (self.to_bits() & Self::splat(Self::Scalar::INFINITY).to_bits()).simd_eq(Simd::splat(0))
                        }

                        #[inline]
                        fn is_normal(self) -> Self::Mask {
                            !(self.abs().simd_eq(Self::splat(0.0)) | self.is_nan() | self.is_subnormal() | self.is_infinite())
                        }

                        #[inline]
                        fn signum(self) -> Self {
                            self.is_nan().select(Self::splat(Self::Scalar::NAN), Self::splat(1.0).copysign(self))
                        }

                        #[inline]
                        fn copysign(self, sign: Self) -> Self {
                            let sign_bit = sign.to_bits() & Self::splat(-0.).to_bits();
                            let magnitude = self.to_bits() & !Self::splat(-0.).to_bits();
                            Self::from_bits(sign_bit | magnitude)
                        }

                        #[inline]
                        fn simd_min(self, other: Self) -> Self {
                            // Safety: `self` and `other` are float vectors
                            unsafe { core::intrinsics::simd::simd_fmin(self, other) }
                        }

                        #[inline]
                        fn simd_max(self, other: Self) -> Self {
                            // Safety: `self` and `other` are floating point vectors
                            unsafe { core::intrinsics::simd::simd_fmax(self, other) }
                        }

                        #[inline]
                        fn simd_clamp(self, min: Self, max: Self) -> Self {
                            assert!(
                                min.simd_le(max).all(),
                                "each element in `min` must be less than or equal to the corresponding element in `max`",
                            );
                            let mut x = self;
                            x = x.simd_lt(min).select(min, x);
                            x = x.simd_gt(max).select(max, x);
                            x
                        }

                        #[inline]
                        fn reduce_sum(self) -> Self::Scalar {
                            // LLVM sum is inaccurate on i586
                            if cfg!(all(target_arch = "x86", not(target_feature = "sse2"))) {
                                self.as_array().iter().sum()
                            } else {
                                // Safety: `self` is a float vector
                                unsafe { core::intrinsics::simd::simd_reduce_add_ordered(self, -0.) }
                            }
                        }

                        #[inline]
                        fn reduce_product(self) -> Self::Scalar {
                            // LLVM product is inaccurate on i586
                            if cfg!(all(target_arch = "x86", not(target_feature = "sse2"))) {
                                self.as_array().iter().product()
                            } else {
                                // Safety: `self` is a float vector
                                unsafe { core::intrinsics::simd::simd_reduce_mul_ordered(self, 1.) }
                            }
                        }

                        #[inline]
                        fn reduce_max(self) -> Self::Scalar {
                            // Safety: `self` is a float vector
                            unsafe { core::intrinsics::simd::simd_reduce_max(self) }
                        }

                        #[inline]
                        fn reduce_min(self) -> Self::Scalar {
                            // Safety: `self` is a float vector
                            unsafe { core::intrinsics::simd::simd_reduce_min(self) }
                        }
                    }
                    )*
                }
            }

            impl_trait! { f32 { bits: u32, mask: i32 }, f64 { bits: u64, mask: i64 } }
            
        }

        pub mod ptr
        {
            /*!
            */
            use ::
            {
                *
            };
            /*
            pub use std::ptr::{*};
            */
            mod sealed {
                pub trait Sealed {}
            }

            use super::sealed::Sealed;
            use crate::simd::{LaneCount, Mask, Simd, SupportedLaneCount, cmp::SimdPartialEq, num::SimdUint};

            /// Operations on SIMD vectors of constant pointers.
            pub trait SimdConstPtr: Copy + Sealed 
            {
                /// Vector of `usize` with the same number of elements.
                type Usize;

                /// Vector of `isize` with the same number of elements.
                type Isize;

                /// Vector of const pointers with the same number of elements.
                type CastPtr<T>;

                /// Vector of mutable pointers to the same type.
                type MutPtr;

                /// Mask type used for manipulating this SIMD vector type.
                type Mask;

                /// Returns `true` for each element that is null.
                fn is_null(self) -> Self::Mask;

                /// Casts to a pointer of another type.
                ///
                /// Equivalent to calling [`pointer::cast`] on each element.
                fn cast<T>(self) -> Self::CastPtr<T>;

                /// Changes constness without changing the type.
                ///
                /// Equivalent to calling [`pointer::cast_mut`] on each element.
                fn cast_mut(self) -> Self::MutPtr;

                /// Gets the "address" portion of the pointer.
                ///
                /// This method discards pointer semantic metadata, so the result cannot be
                /// directly cast into a valid pointer.
                ///
                /// This method semantically discards *provenance* and
                /// *address-space* information. To properly restore that information, use [`Self::with_addr`].
                ///
                /// Equivalent to calling [`pointer::addr`] on each element.
                fn addr(self) -> Self::Usize;

                /// Converts an address to a pointer without giving it any provenance.
                ///
                /// Without provenance, this pointer is not associated with any actual allocation. Such a
                /// no-provenance pointer may be used for zero-sized memory accesses (if suitably aligned), but
                /// non-zero-sized memory accesses with a no-provenance pointer are UB. No-provenance pointers
                /// are little more than a usize address in disguise.
                ///
                /// This is different from [`Self::with_exposed_provenance`], which creates a pointer that picks up a
                /// previously exposed provenance.
                ///
                /// Equivalent to calling [`core::ptr::without_provenance`] on each element.
                fn without_provenance(addr: Self::Usize) -> Self;

                /// Creates a new pointer with the given address.
                ///
                /// This performs the same operation as a cast, but copies the *address-space* and
                /// *provenance* of `self` to the new pointer.
                ///
                /// Equivalent to calling [`pointer::with_addr`] on each element.
                fn with_addr(self, addr: Self::Usize) -> Self;

                /// Exposes the "provenance" part of the pointer for future use in
                /// [`Self::with_exposed_provenance`] and returns the "address" portion.
                fn expose_provenance(self) -> Self::Usize;

                /// Converts an address back to a pointer, picking up a previously "exposed" provenance.
                ///
                /// Equivalent to calling [`core::ptr::with_exposed_provenance`] on each element.
                fn with_exposed_provenance(addr: Self::Usize) -> Self;

                /// Calculates the offset from a pointer using wrapping arithmetic.
                ///
                /// Equivalent to calling [`pointer::wrapping_offset`] on each element.
                fn wrapping_offset(self, offset: Self::Isize) -> Self;

                /// Calculates the offset from a pointer using wrapping arithmetic.
                ///
                /// Equivalent to calling [`pointer::wrapping_add`] on each element.
                fn wrapping_add(self, count: Self::Usize) -> Self;

                /// Calculates the offset from a pointer using wrapping arithmetic.
                ///
                /// Equivalent to calling [`pointer::wrapping_sub`] on each element.
                fn wrapping_sub(self, count: Self::Usize) -> Self;
            }

            impl<T, const N: usize> Sealed for Simd<*const T, N> where LaneCount<N>: SupportedLaneCount {}

            impl<T, const N: usize> SimdConstPtr for Simd<*const T, N>
            where
                LaneCount<N>: SupportedLaneCount,
            {
                type Usize = Simd<usize, N>;
                type Isize = Simd<isize, N>;
                type CastPtr<U> = Simd<*const U, N>;
                type MutPtr = Simd<*mut T, N>;
                type Mask = Mask<isize, N>;

                #[inline]
                fn is_null(self) -> Self::Mask {
                    Simd::splat(core::ptr::null()).simd_eq(self)
                }

                #[inline]
                fn cast<U>(self) -> Self::CastPtr<U> {
                    // SimdElement currently requires zero-sized metadata, so this should never fail.
                    // If this ever changes, `simd_cast_ptr` should produce a post-mono error.
                    use core::ptr::Pointee;
                    assert_eq!(size_of::<<T as Pointee>::Metadata>(), 0);
                    assert_eq!(size_of::<<U as Pointee>::Metadata>(), 0);

                    // Safety: pointers can be cast
                    unsafe { core::intrinsics::simd::simd_cast_ptr(self) }
                }

                #[inline]
                fn cast_mut(self) -> Self::MutPtr {
                    // Safety: pointers can be cast
                    unsafe { core::intrinsics::simd::simd_cast_ptr(self) }
                }

                #[inline]
                fn addr(self) -> Self::Usize {
                    // FIXME(strict_provenance_magic): I am magic and should be a compiler intrinsic.
                    // SAFETY: Pointer-to-integer transmutes are valid (if you are okay with losing the
                    // provenance).
                    unsafe { core::mem::transmute_copy(&self) }
                }

                #[inline]
                fn without_provenance(addr: Self::Usize) -> Self {
                    // FIXME(strict_provenance_magic): I am magic and should be a compiler intrinsic.
                    // SAFETY: Integer-to-pointer transmutes are valid (if you are okay with not getting any
                    // provenance).
                    unsafe { core::mem::transmute_copy(&addr) }
                }

                #[inline]
                fn with_addr(self, addr: Self::Usize) -> Self {
                    // FIXME(strict_provenance_magic): I am magic and should be a compiler intrinsic.
                    //
                    // In the mean-time, this operation is defined to be "as if" it was
                    // a wrapping_offset, so we can emulate it as such. This should properly
                    // restore pointer provenance even under today's compiler.
                    self.cast::<u8>()
                        .wrapping_offset(addr.cast::<isize>() - self.addr().cast::<isize>())
                        .cast()
                }

                #[inline]
                fn expose_provenance(self) -> Self::Usize {
                    // Safety: `self` is a pointer vector
                    unsafe { core::intrinsics::simd::simd_expose_provenance(self) }
                }

                #[inline]
                fn with_exposed_provenance(addr: Self::Usize) -> Self {
                    // Safety: `self` is a pointer vector
                    unsafe { core::intrinsics::simd::simd_with_exposed_provenance(addr) }
                }

                #[inline]
                fn wrapping_offset(self, count: Self::Isize) -> Self {
                    // Safety: simd_arith_offset takes a vector of pointers and a vector of offsets
                    unsafe { core::intrinsics::simd::simd_arith_offset(self, count) }
                }

                #[inline]
                fn wrapping_add(self, count: Self::Usize) -> Self {
                    self.wrapping_offset(count.cast())
                }

                #[inline]
                fn wrapping_sub(self, count: Self::Usize) -> Self {
                    self.wrapping_offset(-count.cast::<isize>())
                }
            }
        }

        pub mod cmp
        {
            /*!
            */
            use ::
            {
                *
            };
            /*
            pub use std::cmp::{*};
            */
            use crate::simd::
            {
                LaneCount, Mask, Simd, SupportedLaneCount,
                cmp::SimdPartialEq,
                ptr::{SimdConstPtr, SimdMutPtr},
            };

            /// Parallel `PartialOrd`.
            pub trait SimdPartialOrd: SimdPartialEq 
            {
                /// Test if each element is less than the corresponding element in `other`.
                #[must_use = "method returns a new mask and does not mutate the original value"]
                fn simd_lt(self, other: Self) -> Self::Mask;

                /// Test if each element is less than or equal to the corresponding element in `other`.
                #[must_use = "method returns a new mask and does not mutate the original value"]
                fn simd_le(self, other: Self) -> Self::Mask;

                /// Test if each element is greater than the corresponding element in `other`.
                #[must_use = "method returns a new mask and does not mutate the original value"]
                fn simd_gt(self, other: Self) -> Self::Mask;

                /// Test if each element is greater than or equal to the corresponding element in `other`.
                #[must_use = "method returns a new mask and does not mutate the original value"]
                fn simd_ge(self, other: Self) -> Self::Mask;
            }

            /// Parallel `Ord`.
            pub trait SimdOrd: SimdPartialOrd 
            {
                /// Returns the element-wise maximum with `other`.
                #[must_use = "method returns a new vector and does not mutate the original value"]
                fn simd_max(self, other: Self) -> Self;

                /// Returns the element-wise minimum with `other`.
                #[must_use = "method returns a new vector and does not mutate the original value"]
                fn simd_min(self, other: Self) -> Self;

                /// Restrict each element to a certain interval.
                ///
                /// For each element, returns `max` if `self` is greater than `max`, and `min` if `self` is
                /// less than `min`. Otherwise returns `self`.
                ///
                /// # Panics
                ///
                /// Panics if `min > max` on any element.
                #[must_use = "method returns a new vector and does not mutate the original value"]
                fn simd_clamp(self, min: Self, max: Self) -> Self;
            }

            macro_rules! impl_integer 
            {
                { $($integer:ty),* } => {
                    $(
                    impl<const N: usize> SimdPartialOrd for Simd<$integer, N>
                    where
                        LaneCount<N>: SupportedLaneCount,
                    {
                        #[inline]
                        fn simd_lt(self, other: Self) -> Self::Mask {
                            // Safety: `self` is a vector, and the result of the comparison
                            // is always a valid mask.
                            unsafe { Mask::from_int_unchecked(core::intrinsics::simd::simd_lt(self, other)) }
                        }

                        #[inline]
                        fn simd_le(self, other: Self) -> Self::Mask {
                            // Safety: `self` is a vector, and the result of the comparison
                            // is always a valid mask.
                            unsafe { Mask::from_int_unchecked(core::intrinsics::simd::simd_le(self, other)) }
                        }

                        #[inline]
                        fn simd_gt(self, other: Self) -> Self::Mask {
                            // Safety: `self` is a vector, and the result of the comparison
                            // is always a valid mask.
                            unsafe { Mask::from_int_unchecked(core::intrinsics::simd::simd_gt(self, other)) }
                        }

                        #[inline]
                        fn simd_ge(self, other: Self) -> Self::Mask {
                            // Safety: `self` is a vector, and the result of the comparison
                            // is always a valid mask.
                            unsafe { Mask::from_int_unchecked(core::intrinsics::simd::simd_ge(self, other)) }
                        }
                    }

                    impl<const N: usize> SimdOrd for Simd<$integer, N>
                    where
                        LaneCount<N>: SupportedLaneCount,
                    {
                        #[inline]
                        fn simd_max(self, other: Self) -> Self {
                            self.simd_lt(other).select(other, self)
                        }

                        #[inline]
                        fn simd_min(self, other: Self) -> Self {
                            self.simd_gt(other).select(other, self)
                        }

                        #[inline]
                        #[track_caller]
                        fn simd_clamp(self, min: Self, max: Self) -> Self {
                            assert!(
                                min.simd_le(max).all(),
                                "each element in `min` must be less than or equal to the corresponding element in `max`",
                            );
                            self.simd_max(min).simd_min(max)
                        }
                    }
                    )*
                }
            }

            impl_integer! { u8, u16, u32, u64, usize, i8, i16, i32, i64, isize }

            macro_rules! impl_float 
            {
                { $($float:ty),* } => {
                    $(
                    impl<const N: usize> SimdPartialOrd for Simd<$float, N>
                    where
                        LaneCount<N>: SupportedLaneCount,
                    {
                        #[inline]
                        fn simd_lt(self, other: Self) -> Self::Mask {
                            // Safety: `self` is a vector, and the result of the comparison
                            // is always a valid mask.
                            unsafe { Mask::from_int_unchecked(core::intrinsics::simd::simd_lt(self, other)) }
                        }

                        #[inline]
                        fn simd_le(self, other: Self) -> Self::Mask {
                            // Safety: `self` is a vector, and the result of the comparison
                            // is always a valid mask.
                            unsafe { Mask::from_int_unchecked(core::intrinsics::simd::simd_le(self, other)) }
                        }

                        #[inline]
                        fn simd_gt(self, other: Self) -> Self::Mask {
                            // Safety: `self` is a vector, and the result of the comparison
                            // is always a valid mask.
                            unsafe { Mask::from_int_unchecked(core::intrinsics::simd::simd_gt(self, other)) }
                        }

                        #[inline]
                        fn simd_ge(self, other: Self) -> Self::Mask {
                            // Safety: `self` is a vector, and the result of the comparison
                            // is always a valid mask.
                            unsafe { Mask::from_int_unchecked(core::intrinsics::simd::simd_ge(self, other)) }
                        }
                    }
                    )*
                }
            }

            impl_float! { f32, f64 }

            macro_rules! impl_mask 
            {
                { $($integer:ty),* } => {
                    $(
                    impl<const N: usize> SimdPartialOrd for Mask<$integer, N>
                    where
                        LaneCount<N>: SupportedLaneCount,
                    {
                        #[inline]
                        fn simd_lt(self, other: Self) -> Self::Mask {
                            // Safety: `self` is a vector, and the result of the comparison
                            // is always a valid mask.
                            unsafe { Self::from_int_unchecked(core::intrinsics::simd::simd_lt(self.to_int(), other.to_int())) }
                        }

                        #[inline]
                        fn simd_le(self, other: Self) -> Self::Mask {
                            // Safety: `self` is a vector, and the result of the comparison
                            // is always a valid mask.
                            unsafe { Self::from_int_unchecked(core::intrinsics::simd::simd_le(self.to_int(), other.to_int())) }
                        }

                        #[inline]
                        fn simd_gt(self, other: Self) -> Self::Mask {
                            // Safety: `self` is a vector, and the result of the comparison
                            // is always a valid mask.
                            unsafe { Self::from_int_unchecked(core::intrinsics::simd::simd_gt(self.to_int(), other.to_int())) }
                        }

                        #[inline]
                        fn simd_ge(self, other: Self) -> Self::Mask {
                            // Safety: `self` is a vector, and the result of the comparison
                            // is always a valid mask.
                            unsafe { Self::from_int_unchecked(core::intrinsics::simd::simd_ge(self.to_int(), other.to_int())) }
                        }
                    }

                    impl<const N: usize> SimdOrd for Mask<$integer, N>
                    where
                        LaneCount<N>: SupportedLaneCount,
                    {
                        #[inline]
                        fn simd_max(self, other: Self) -> Self {
                            self.simd_gt(other).select_mask(other, self)
                        }

                        #[inline]
                        fn simd_min(self, other: Self) -> Self {
                            self.simd_lt(other).select_mask(other, self)
                        }

                        #[inline]
                        #[track_caller]
                        fn simd_clamp(self, min: Self, max: Self) -> Self {
                            assert!(
                                min.simd_le(max).all(),
                                "each element in `min` must be less than or equal to the corresponding element in `max`",
                            );
                            self.simd_max(min).simd_min(max)
                        }
                    }
                    )*
                }
            }

            impl_mask! { i8, i16, i32, i64, isize }

            impl<T, const N: usize> SimdPartialOrd for Simd<*const T, N>
            where
                LaneCount<N>: SupportedLaneCount,
            {
                #[inline]
                fn simd_lt(self, other: Self) -> Self::Mask {
                    self.addr().simd_lt(other.addr())
                }

                #[inline]
                fn simd_le(self, other: Self) -> Self::Mask {
                    self.addr().simd_le(other.addr())
                }

                #[inline]
                fn simd_gt(self, other: Self) -> Self::Mask {
                    self.addr().simd_gt(other.addr())
                }

                #[inline]
                fn simd_ge(self, other: Self) -> Self::Mask {
                    self.addr().simd_ge(other.addr())
                }
            }

            impl<T, const N: usize> SimdOrd for Simd<*const T, N>
            where
                LaneCount<N>: SupportedLaneCount,
            {
                #[inline]
                fn simd_max(self, other: Self) -> Self {
                    self.simd_lt(other).select(other, self)
                }

                #[inline]
                fn simd_min(self, other: Self) -> Self {
                    self.simd_gt(other).select(other, self)
                }

                #[inline]
                #[track_caller]
                fn simd_clamp(self, min: Self, max: Self) -> Self {
                    assert!(
                        min.simd_le(max).all(),
                        "each element in `min` must be less than or equal to the corresponding element in `max`",
                    );
                    self.simd_max(min).simd_min(max)
                }
            }

            impl<T, const N: usize> SimdPartialOrd for Simd<*mut T, N>
            where
                LaneCount<N>: SupportedLaneCount,
            {
                #[inline]
                fn simd_lt(self, other: Self) -> Self::Mask {
                    self.addr().simd_lt(other.addr())
                }

                #[inline]
                fn simd_le(self, other: Self) -> Self::Mask {
                    self.addr().simd_le(other.addr())
                }

                #[inline]
                fn simd_gt(self, other: Self) -> Self::Mask {
                    self.addr().simd_gt(other.addr())
                }

                #[inline]
                fn simd_ge(self, other: Self) -> Self::Mask {
                    self.addr().simd_ge(other.addr())
                }
            }

            impl<T, const N: usize> SimdOrd for Simd<*mut T, N>
            where
                LaneCount<N>: SupportedLaneCount,
            {
                #[inline]
                fn simd_max(self, other: Self) -> Self {
                    self.simd_lt(other).select(other, self)
                }

                #[inline]
                fn simd_min(self, other: Self) -> Self {
                    self.simd_gt(other).select(other, self)
                }

                #[inline]
                #[track_caller]
                fn simd_clamp(self, min: Self, max: Self) -> Self {
                    assert!(
                        min.simd_le(max).all(),
                        "each element in `min` must be less than or equal to the corresponding element in `max`",
                    );
                    self.simd_max(min).simd_min(max)
                }
            }

            use crate::simd::{
                LaneCount, Mask, Simd, SimdElement, SupportedLaneCount,
                ptr::{SimdConstPtr, SimdMutPtr},
            };

            /// Parallel `PartialEq`.
            pub trait SimdPartialEq {
                /// The mask type returned by each comparison.
                type Mask;

                /// Test if each element is equal to the corresponding element in `other`.
                #[must_use = "method returns a new mask and does not mutate the original value"]
                fn simd_eq(self, other: Self) -> Self::Mask;

                /// Test if each element is not equal to the corresponding element in `other`.
                #[must_use = "method returns a new mask and does not mutate the original value"]
                fn simd_ne(self, other: Self) -> Self::Mask;
            }

            macro_rules! impl_number {
                { $($number:ty),* } => {
                    $(
                    impl<const N: usize> SimdPartialEq for Simd<$number, N>
                    where
                        LaneCount<N>: SupportedLaneCount,
                    {
                        type Mask = Mask<<$number as SimdElement>::Mask, N>;

                        #[inline]
                        fn simd_eq(self, other: Self) -> Self::Mask {
                            // Safety: `self` is a vector, and the result of the comparison
                            // is always a valid mask.
                            unsafe { Mask::from_int_unchecked(core::intrinsics::simd::simd_eq(self, other)) }
                        }

                        #[inline]
                        fn simd_ne(self, other: Self) -> Self::Mask {
                            // Safety: `self` is a vector, and the result of the comparison
                            // is always a valid mask.
                            unsafe { Mask::from_int_unchecked(core::intrinsics::simd::simd_ne(self, other)) }
                        }
                    }
                    )*
                }
            }

            impl_number! { f32, f64, u8, u16, u32, u64, usize, i8, i16, i32, i64, isize }

            macro_rules! impl_mask {
                { $($integer:ty),* } => {
                    $(
                    impl<const N: usize> SimdPartialEq for Mask<$integer, N>
                    where
                        LaneCount<N>: SupportedLaneCount,
                    {
                        type Mask = Self;

                        #[inline]
                        fn simd_eq(self, other: Self) -> Self::Mask {
                            // Safety: `self` is a vector, and the result of the comparison
                            // is always a valid mask.
                            unsafe { Self::from_int_unchecked(core::intrinsics::simd::simd_eq(self.to_int(), other.to_int())) }
                        }

                        #[inline]
                        fn simd_ne(self, other: Self) -> Self::Mask {
                            // Safety: `self` is a vector, and the result of the comparison
                            // is always a valid mask.
                            unsafe { Self::from_int_unchecked(core::intrinsics::simd::simd_ne(self.to_int(), other.to_int())) }
                        }
                    }
                    )*
                }
            }

            impl_mask! { i8, i16, i32, i64, isize }

            impl<T, const N: usize> SimdPartialEq for Simd<*const T, N>
            where
                LaneCount<N>: SupportedLaneCount,
            {
                type Mask = Mask<isize, N>;

                #[inline]
                fn simd_eq(self, other: Self) -> Self::Mask {
                    self.addr().simd_eq(other.addr())
                }

                #[inline]
                fn simd_ne(self, other: Self) -> Self::Mask {
                    self.addr().simd_ne(other.addr())
                }
            }

            impl<T, const N: usize> SimdPartialEq for Simd<*mut T, N>
            where
                LaneCount<N>: SupportedLaneCount,
            {
                type Mask = Mask<isize, N>;

                #[inline]
                fn simd_eq(self, other: Self) -> Self::Mask {
                    self.addr().simd_eq(other.addr())
                }

                #[inline]
                fn simd_ne(self, other: Self) -> Self::Mask {
                    self.addr().simd_ne(other.addr())
                }
            }
        }

        pub use crate::core_simd::alias::*;
        pub use crate::core_simd::cast::*;
        pub use crate::core_simd::lane_count::{LaneCount, SupportedLaneCount};
        pub use crate::core_simd::masks::*;
        pub use crate::core_simd::swizzle::*;
        pub use crate::core_simd::to_bytes::ToBytes;
        pub use crate::core_simd::vector::*;
    }
}


#[unstable(feature = "portable_simd", issue = "86656")]
pub mod simd 
{
    #[unstable(feature = "portable_simd", issue = "86656")]
    pub use crate::core_simd::simd::*;
}
