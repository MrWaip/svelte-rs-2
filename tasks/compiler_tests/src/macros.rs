#[macro_export]
macro_rules! compiler_case {
    ($name:ident) => {
        $crate::compiler_case!(@build $name, stringify!($name), [prod, dev, ssr, ssr_dev]);
    };
    ($name:ident, $case:literal) => {
        $crate::compiler_case!(@build $name, $case, [prod, dev, ssr, ssr_dev]);
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
            #[test]
            #[ignore = $reason]
            fn ssr() {
                assert_compiler_ssr($case);
            }
            #[test]
            #[ignore = $reason]
            fn ssr_dev() {
                assert_compiler_ssr_dev($case);
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
    (@variant prod_todo, $case:expr) => {
        #[test]
        #[ignore = "prod parity not yet reached"]
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
    (@variant ssr, $case:expr) => {
        #[test]
        fn ssr() {
            assert_compiler_ssr($case);
        }
    };
    (@variant ssr_todo, $case:expr) => {
        #[test]
        #[ignore = "ssr parity not yet reached"]
        fn ssr() {
            assert_compiler_ssr($case);
        }
    };
    (@variant ssr_dev, $case:expr) => {
        #[test]
        fn ssr_dev() {
            assert_compiler_ssr_dev($case);
        }
    };
    (@variant ssr_dev_todo, $case:expr) => {
        #[test]
        #[ignore = "ssr dev parity not yet reached"]
        fn ssr_dev() {
            assert_compiler_ssr_dev($case);
        }
    };
}

#[macro_export]
macro_rules! compiler_case_no_dev {
    ($name:ident) => {
        $crate::compiler_case!($name, [prod, ssr_todo, ssr_dev_todo]);
    };
    ($name:ident, $case:literal) => {
        $crate::compiler_case!($name, $case, [prod, ssr_todo, ssr_dev_todo]);
    };
}

#[macro_export]
macro_rules! compiler_module_case {
    ($name:ident, $path:literal) => {
        $crate::compiler_module_case!(@build $name, $path, [prod, dev]);
    };
    ($name:ident, $path:literal, [$($variant:ident),+ $(,)?]) => {
        $crate::compiler_module_case!(@build $name, $path, [$($variant),+]);
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
    (@build $name:ident, $path:expr, [$($variant:ident),+]) => {
        mod $name {
            use super::*;
            $( $crate::compiler_module_case!(@variant $variant, $path); )+
        }
    };
    (@variant prod, $path:expr) => {
        #[test]
        fn prod() {
            assert_compiler_module_prod($path);
        }
    };
    (@variant dev, $path:expr) => {
        #[test]
        fn dev() {
            assert_compiler_module_dev($path);
        }
    };
    (@variant ssr, $path:expr) => {
        #[test]
        fn ssr() {
            assert_compiler_module_ssr($path);
        }
    };
    (@variant ssr_todo, $path:expr) => {
        #[test]
        #[ignore = "ssr parity not yet reached"]
        fn ssr() {
            assert_compiler_module_ssr($path);
        }
    };
    (@variant ssr_dev, $path:expr) => {
        #[test]
        fn ssr_dev() {
            assert_compiler_module_ssr_dev($path);
        }
    };
    (@variant ssr_dev_todo, $path:expr) => {
        #[test]
        #[ignore = "ssr dev parity not yet reached"]
        fn ssr_dev() {
            assert_compiler_module_ssr_dev($path);
        }
    };
}

#[macro_export]
macro_rules! preprocess_case {
    ($name:ident) => {
        $crate::preprocess_case!(@build $name, stringify!($name));
    };
    ($name:ident, $case:literal) => {
        $crate::preprocess_case!(@build $name, $case);
    };
    (@build $name:ident, $case:expr) => {
        #[test]
        fn $name() {
            assert_preprocess_case($case);
        }
    };
}
