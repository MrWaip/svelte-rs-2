#[macro_export]
macro_rules! compiler_case {
    ($name:ident) => {
        $crate::compiler_case!(@build $name, stringify!($name), [prod, dev]);
    };
    ($name:ident, $case:literal) => {
        $crate::compiler_case!(@build $name, $case, [prod, dev]);
    };
    ($name:ident, [$($variant:ident),+ $(,)?]) => {
        $crate::compiler_case!(@build $name, stringify!($name), [$($variant),+]);
    };
    ($name:ident, $case:literal, [$($variant:ident),+ $(,)?]) => {
        $crate::compiler_case!(@build $name, $case, [$($variant),+]);
    };
    ($name:ident, $case:literal, ignore = $reason:literal) => {
        mod $name {
            use super::*;
            #[test]
            #[ignore = $reason]
            fn prod() {
                assert_compiler_prod($case);
            }
            #[test]
            #[ignore = $reason]
            fn dev() {
                assert_compiler_dev($case);
            }
        }
    };
    (@build $name:ident, $case:expr, [$($variant:ident),+]) => {
        mod $name {
            use super::*;
            $( $crate::compiler_case!(@variant $variant, $case); )+
        }
    };
    (@variant prod, $case:expr) => {
        #[test]
        fn prod() {
            assert_compiler_prod($case);
        }
    };
    (@variant dev, $case:expr) => {
        #[test]
        fn dev() {
            assert_compiler_dev($case);
        }
    };
    (@variant dev_todo, $case:expr) => {
        #[test]
        #[ignore = "dev parity not yet reached"]
        fn dev() {
            assert_compiler_dev($case);
        }
    };
}

#[macro_export]
macro_rules! compiler_case_no_dev {
    ($name:ident) => {
        $crate::compiler_case!($name, [prod]);
    };
    ($name:ident, $case:literal) => {
        $crate::compiler_case!($name, $case, [prod]);
    };
}

#[macro_export]
macro_rules! compiler_module_case {
    ($name:ident, $path:literal) => {
        mod $name {
            use super::*;
            #[test]
            fn prod() {
                assert_compiler_module_prod($path);
            }
            #[test]
            fn dev() {
                assert_compiler_module_dev($path);
            }
        }
    };
    ($name:ident, $path:literal, ignore = $reason:literal) => {
        mod $name {
            use super::*;
            #[test]
            #[ignore = $reason]
            fn prod() {
                assert_compiler_module_prod($path);
            }
            #[test]
            #[ignore = $reason]
            fn dev() {
                assert_compiler_module_dev($path);
            }
        }
    };
}
