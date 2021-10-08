#[macro_export]
macro_rules! test_case {
    (
        $($f: expr => {
            $($s: expr => $e: expr$(,)?)*
            $(         => $e2: expr$(,)?)*
        },)*
    ) => {
        $(
            $(assert_eq!($e, $f($s));)*
            $(assert_eq!($e2, $f());)*
        )+
    };

    (
        $($f: expr => {
            $($s1: expr, $s2: expr => $e: expr$(,)?)*
        },)*
    ) => {
        $(
            $(assert_eq!($e, $f($s1, $s2));)*
        )+
    };

    (
        $($f: expr => {
            $($s1: expr, $s2: expr, $s3: expr => $e: expr$(,)?)*
        },)*
    ) => {
        $(
            $(assert_eq!($e, $f($s1, $s2, $s3));)*
        )+
    };

    (
        $($f: expr => {
            $($s1: expr, $s2: expr, $s3: expr, $s4: expr => $e: expr$(,)?)*
        },)*
    ) => {
        $(
            $(assert_eq!($e, $f($s1, $s2, $s3, $s4));)*
        )+
    };
}
