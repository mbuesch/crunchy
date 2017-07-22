#[macro_export]
macro_rules! unroll {
    (for $v:ident in 0..0 $c:block) => {};
    (for $v:ident in 0..$b:tt {$($c:tt)*}) => {
        #[allow(non_upper_case_globals)] { unroll!(@$v, 0, $b, {$($c)*}); }
    };
    (@$v:ident, $a:expr, 1, $c:block) => {
        { const $v: usize = $a + 0; $c }
    };
    (@$v:ident, $a:expr, 2, $c:block) => {
        unroll!(@$v, $a, 1, $c);
        unroll!(@$v, $a + 1, 1, $c);
    };
    (@$v:ident, $a:expr, 3, $c:block) => {
        { const $v: usize = $a + 2; $c }
    };
    (@$v:ident, $a:expr, 4, $c:block) => {
        unroll!(@$v, $a, 2, $c);
        unroll!(@$v, $a + 2, 2, $c);
    };
    (@$v:ident, $a:expr, 5, $c:block) => {
        unroll!(@$v, $a, 4, $c);
        { const $v: usize = $a + 4; $c }
    };
    (@$v:ident, $a:expr, 6, $c:block) => {
        unroll!(@$v, $a, 3, $c);
        unroll!(@$v, $a + 3, 3, $c);
    };
    (@$v:ident, $a:expr, 7, $c:block) => {
        unroll!(@$v, $a, 6, $c);
        { const $v: usize = $a + 6; $c }
    };
    (@$v:ident, $a:expr, 8, $c:block) => {
        unroll!(@$v, $a, 4, $c);
        unroll!(@$v, $a + 4, 4, $c);
    };
    (@$v:ident, $a:expr, 9, $c:block) => {
        unroll!(@$v, $a, 8, $c);
        { const $v: usize = $a + 8; $c }
    };
    (@$v:ident, $a:expr, 10, $c:block) => {
        unroll!(@$v, $a, 5, $c);
        unroll!(@$v, $a + 5, 5, $c);
    };
    (@$v:ident, $a:expr, 11, $c:block) => {
        unroll!(@$v, $a, 10, $c);
        { const $v: usize = $a + 10; $c }
    };
    (@$v:ident, $a:expr, 12, $c:block) => {
        unroll!(@$v, $a, 6, $c);
        unroll!(@$v, $a + 6, 6, $c);
    };
    (@$v:ident, $a:expr, 13, $c:block) => {
        unroll!(@$v, $a, 12, $c);
        { const $v: usize = $a + 12; $c }
    };
    (@$v:ident, $a:expr, 14, $c:block) => {
        unroll!(@$v, $a, 7, $c);
        unroll!(@$v, $a + 7, 7, $c);
    };
    (@$v:ident, $a:expr, 15, $c:block) => {
        unroll!(@$v, $a, 14, $c);
        { const $v: usize = $a + 14; $c }
    };
    (@$v:ident, $a:expr, 16, $c:block) => {
        unroll!(@$v, $a, 8, $c);
        unroll!(@$v, $a + 8, 8, $c);
    };
    (@$v:ident, $a:expr, 17, $c:block) => {
        unroll!(@$v, $a, 16, $c);
        { const $v: usize = $a + 16; $c }
    };
    (@$v:ident, $a:expr, 18, $c:block) => {
        unroll!(@$v, $a, 9, $c);
        unroll!(@$v, $a + 9, 9, $c);
    };
    (@$v:ident, $a:expr, 19, $c:block) => {
        unroll!(@$v, $a, 18, $c);
        { const $v: usize = $a + 18; $c }
    };
    (@$v:ident, $a:expr, 20, $c:block) => {
        unroll!(@$v, $a, 10, $c);
        unroll!(@$v, $a + 10, 10, $c);
    };
    (@$v:ident, $a:expr, 21, $c:block) => {
        unroll!(@$v, $a, 20, $c);
        { const $v: usize = $a + 20; $c }
    };
    (@$v:ident, $a:expr, 22, $c:block) => {
        unroll!(@$v, $a, 11, $c);
        unroll!(@$v, $a + 11, 11, $c);
    };
    (@$v:ident, $a:expr, 23, $c:block) => {
        unroll!(@$v, $a, 22, $c);
        { const $v: usize = $a + 22; $c }
    };
    (@$v:ident, $a:expr, 24, $c:block) => {
        unroll!(@$v, $a, 12, $c);
        unroll!(@$v, $a + 12, 12, $c);
    };
    (@$v:ident, $a:expr, 25, $c:block) => {
        unroll!(@$v, $a, 24, $c);
        { const $v: usize = $a + 24; $c }
    };
    (@$v:ident, $a:expr, 26, $c:block) => {
        unroll!(@$v, $a, 13, $c);
        unroll!(@$v, $a + 13, 13, $c);
    };
    (@$v:ident, $a:expr, 27, $c:block) => {
        unroll!(@$v, $a, 26, $c);
        { const $v: usize = $a + 26; $c }
    };
    (@$v:ident, $a:expr, 28, $c:block) => {
        unroll!(@$v, $a, 14, $c);
        unroll!(@$v, $a + 14, 14, $c);
    };
    (@$v:ident, $a:expr, 29, $c:block) => {
        unroll!(@$v, $a, 28, $c);
        { const $v: usize = $a + 28; $c }
    };
    (@$v:ident, $a:expr, 30, $c:block) => {
        unroll!(@$v, $a, 15, $c);
        unroll!(@$v, $a + 15, 15, $c);
    };
    (@$v:ident, $a:expr, 31, $c:block) => {
        unroll!(@$v, $a, 30, $c);
        { const $v: usize = $a + 30; $c }
    };
    (@$v:ident, $a:expr, 32, $c:block) => {
        unroll!(@$v, $a, 16, $c);
        unroll!(@$v, $a + 16, 16, $c);
    };
    (@$v:ident, $a:expr, 33, $c:block) => {
        unroll!(@$v, $a, 32, $c);
        { const $v: usize = $a + 32; $c }
    };
    (@$v:ident, $a:expr, 34, $c:block) => {
        unroll!(@$v, $a, 17, $c);
        unroll!(@$v, $a + 17, 17, $c);
    };
    (@$v:ident, $a:expr, 35, $c:block) => {
        unroll!(@$v, $a, 34, $c);
        { const $v: usize = $a + 34; $c }
    };
    (@$v:ident, $a:expr, 36, $c:block) => {
        unroll!(@$v, $a, 18, $c);
        unroll!(@$v, $a + 18, 18, $c);
    };
    (@$v:ident, $a:expr, 37, $c:block) => {
        unroll!(@$v, $a, 36, $c);
        { const $v: usize = $a + 36; $c }
    };
    (@$v:ident, $a:expr, 38, $c:block) => {
        unroll!(@$v, $a, 19, $c);
        unroll!(@$v, $a + 19, 19, $c);
    };
    (@$v:ident, $a:expr, 39, $c:block) => {
        unroll!(@$v, $a, 38, $c);
        { const $v: usize = $a + 38; $c }
    };
    (@$v:ident, $a:expr, 40, $c:block) => {
        unroll!(@$v, $a, 20, $c);
        unroll!(@$v, $a + 20, 20, $c);
    };
    (@$v:ident, $a:expr, 41, $c:block) => {
        unroll!(@$v, $a, 40, $c);
        { const $v: usize = $a + 40; $c }
    };
    (@$v:ident, $a:expr, 42, $c:block) => {
        unroll!(@$v, $a, 21, $c);
        unroll!(@$v, $a + 21, 21, $c);
    };
    (@$v:ident, $a:expr, 43, $c:block) => {
        unroll!(@$v, $a, 42, $c);
        { const $v: usize = $a + 42; $c }
    };
    (@$v:ident, $a:expr, 44, $c:block) => {
        unroll!(@$v, $a, 22, $c);
        unroll!(@$v, $a + 22, 22, $c);
    };
    (@$v:ident, $a:expr, 45, $c:block) => {
        unroll!(@$v, $a, 44, $c);
        { const $v: usize = $a + 44; $c }
    };
    (@$v:ident, $a:expr, 46, $c:block) => {
        unroll!(@$v, $a, 23, $c);
        unroll!(@$v, $a + 23, 23, $c);
    };
    (@$v:ident, $a:expr, 47, $c:block) => {
        unroll!(@$v, $a, 46, $c);
        { const $v: usize = $a + 46; $c }
    };
    (@$v:ident, $a:expr, 48, $c:block) => {
        unroll!(@$v, $a, 24, $c);
        unroll!(@$v, $a + 24, 24, $c);
    };
    (@$v:ident, $a:expr, 49, $c:block) => {
        unroll!(@$v, $a, 48, $c);
        { const $v: usize = $a + 48; $c }
    };
    (@$v:ident, $a:expr, 50, $c:block) => {
        unroll!(@$v, $a, 25, $c);
        unroll!(@$v, $a + 25, 25, $c);
    };
    (@$v:ident, $a:expr, 51, $c:block) => {
        unroll!(@$v, $a, 50, $c);
        { const $v: usize = $a + 50; $c }
    };
    (@$v:ident, $a:expr, 52, $c:block) => {
        unroll!(@$v, $a, 26, $c);
        unroll!(@$v, $a + 26, 26, $c);
    };
    (@$v:ident, $a:expr, 53, $c:block) => {
        unroll!(@$v, $a, 52, $c);
        { const $v: usize = $a + 52; $c }
    };
    (@$v:ident, $a:expr, 54, $c:block) => {
        unroll!(@$v, $a, 27, $c);
        unroll!(@$v, $a + 27, 27, $c);
    };
    (@$v:ident, $a:expr, 55, $c:block) => {
        unroll!(@$v, $a, 54, $c);
        { const $v: usize = $a + 54; $c }
    };
    (@$v:ident, $a:expr, 56, $c:block) => {
        unroll!(@$v, $a, 28, $c);
        unroll!(@$v, $a + 28, 28, $c);
    };
    (@$v:ident, $a:expr, 57, $c:block) => {
        unroll!(@$v, $a, 56, $c);
        { const $v: usize = $a + 56; $c }
    };
    (@$v:ident, $a:expr, 58, $c:block) => {
        unroll!(@$v, $a, 29, $c);
        unroll!(@$v, $a + 29, 29, $c);
    };
    (@$v:ident, $a:expr, 59, $c:block) => {
        unroll!(@$v, $a, 58, $c);
        { const $v: usize = $a + 58; $c }
    };
    (@$v:ident, $a:expr, 60, $c:block) => {
        unroll!(@$v, $a, 30, $c);
        unroll!(@$v, $a + 30, 30, $c);
    };
    (@$v:ident, $a:expr, 61, $c:block) => {
        unroll!(@$v, $a, 60, $c);
        { const $v: usize = $a + 60; $c }
    };
    (@$v:ident, $a:expr, 62, $c:block) => {
        unroll!(@$v, $a, 31, $c);
        unroll!(@$v, $a + 31, 31, $c);
    };
    (@$v:ident, $a:expr, 63, $c:block) => {
        unroll!(@$v, $a, 62, $c);
        { const $v: usize = $a + 62; $c }
    };
    (@$v:ident, $a:expr, 64, $c:block) => {
        unroll!(@$v, $a, 32, $c);
        unroll!(@$v, $a + 32, 32, $c);
    };
    (@$v:ident, $a:expr, 65, $c:block) => {
        unroll!(@$v, $a, 64, $c);
        { const $v: usize = $a + 64; $c }
    };
    (@$v:ident, $a:expr, 66, $c:block) => {
        unroll!(@$v, $a, 33, $c);
        unroll!(@$v, $a + 33, 33, $c);
    };
    (@$v:ident, $a:expr, 67, $c:block) => {
        unroll!(@$v, $a, 66, $c);
        { const $v: usize = $a + 66; $c }
    };
    (@$v:ident, $a:expr, 68, $c:block) => {
        unroll!(@$v, $a, 34, $c);
        unroll!(@$v, $a + 34, 34, $c);
    };
    (@$v:ident, $a:expr, 69, $c:block) => {
        unroll!(@$v, $a, 68, $c);
        { const $v: usize = $a + 68; $c }
    };
    (@$v:ident, $a:expr, 70, $c:block) => {
        unroll!(@$v, $a, 35, $c);
        unroll!(@$v, $a + 35, 35, $c);
    };
    (@$v:ident, $a:expr, 71, $c:block) => {
        unroll!(@$v, $a, 70, $c);
        { const $v: usize = $a + 70; $c }
    };
    (@$v:ident, $a:expr, 72, $c:block) => {
        unroll!(@$v, $a, 36, $c);
        unroll!(@$v, $a + 36, 36, $c);
    };
    (@$v:ident, $a:expr, 73, $c:block) => {
        unroll!(@$v, $a, 72, $c);
        { const $v: usize = $a + 72; $c }
    };
    (@$v:ident, $a:expr, 74, $c:block) => {
        unroll!(@$v, $a, 37, $c);
        unroll!(@$v, $a + 37, 37, $c);
    };
    (@$v:ident, $a:expr, 75, $c:block) => {
        unroll!(@$v, $a, 74, $c);
        { const $v: usize = $a + 74; $c }
    };
    (@$v:ident, $a:expr, 76, $c:block) => {
        unroll!(@$v, $a, 38, $c);
        unroll!(@$v, $a + 38, 38, $c);
    };
    (@$v:ident, $a:expr, 77, $c:block) => {
        unroll!(@$v, $a, 76, $c);
        { const $v: usize = $a + 76; $c }
    };
    (@$v:ident, $a:expr, 78, $c:block) => {
        unroll!(@$v, $a, 39, $c);
        unroll!(@$v, $a + 39, 39, $c);
    };
    (@$v:ident, $a:expr, 79, $c:block) => {
        unroll!(@$v, $a, 78, $c);
        { const $v: usize = $a + 78; $c }
    };
    (@$v:ident, $a:expr, 80, $c:block) => {
        unroll!(@$v, $a, 40, $c);
        unroll!(@$v, $a + 40, 40, $c);
    };
    (@$v:ident, $a:expr, 81, $c:block) => {
        unroll!(@$v, $a, 80, $c);
        { const $v: usize = $a + 80; $c }
    };
    (@$v:ident, $a:expr, 82, $c:block) => {
        unroll!(@$v, $a, 41, $c);
        unroll!(@$v, $a + 41, 41, $c);
    };
    (@$v:ident, $a:expr, 83, $c:block) => {
        unroll!(@$v, $a, 82, $c);
        { const $v: usize = $a + 82; $c }
    };
    (@$v:ident, $a:expr, 84, $c:block) => {
        unroll!(@$v, $a, 42, $c);
        unroll!(@$v, $a + 42, 42, $c);
    };
    (@$v:ident, $a:expr, 85, $c:block) => {
        unroll!(@$v, $a, 84, $c);
        { const $v: usize = $a + 84; $c }
    };
    (@$v:ident, $a:expr, 86, $c:block) => {
        unroll!(@$v, $a, 43, $c);
        unroll!(@$v, $a + 43, 43, $c);
    };
    (@$v:ident, $a:expr, 87, $c:block) => {
        unroll!(@$v, $a, 86, $c);
        { const $v: usize = $a + 86; $c }
    };
    (@$v:ident, $a:expr, 88, $c:block) => {
        unroll!(@$v, $a, 44, $c);
        unroll!(@$v, $a + 44, 44, $c);
    };
    (@$v:ident, $a:expr, 89, $c:block) => {
        unroll!(@$v, $a, 88, $c);
        { const $v: usize = $a + 88; $c }
    };
    (@$v:ident, $a:expr, 90, $c:block) => {
        unroll!(@$v, $a, 45, $c);
        unroll!(@$v, $a + 45, 45, $c);
    };
    (@$v:ident, $a:expr, 91, $c:block) => {
        unroll!(@$v, $a, 90, $c);
        { const $v: usize = $a + 90; $c }
    };
    (@$v:ident, $a:expr, 92, $c:block) => {
        unroll!(@$v, $a, 46, $c);
        unroll!(@$v, $a + 46, 46, $c);
    };
    (@$v:ident, $a:expr, 93, $c:block) => {
        unroll!(@$v, $a, 92, $c);
        { const $v: usize = $a + 92; $c }
    };
    (@$v:ident, $a:expr, 94, $c:block) => {
        unroll!(@$v, $a, 47, $c);
        unroll!(@$v, $a + 47, 47, $c);
    };
    (@$v:ident, $a:expr, 95, $c:block) => {
        unroll!(@$v, $a, 94, $c);
        { const $v: usize = $a + 94; $c }
    };
    (@$v:ident, $a:expr, 96, $c:block) => {
        unroll!(@$v, $a, 48, $c);
        unroll!(@$v, $a + 48, 48, $c);
    };
    (@$v:ident, $a:expr, 97, $c:block) => {
        unroll!(@$v, $a, 96, $c);
        { const $v: usize = $a + 96; $c }
    };
    (@$v:ident, $a:expr, 98, $c:block) => {
        unroll!(@$v, $a, 49, $c);
        unroll!(@$v, $a + 49, 49, $c);
    };
    (@$v:ident, $a:expr, 99, $c:block) => {
        unroll!(@$v, $a, 98, $c);
        { const $v: usize = $a + 98; $c }
    };
    (@$v:ident, $a:expr, 100, $c:block) => {
        unroll!(@$v, $a, 50, $c);
        unroll!(@$v, $a + 50, 50, $c);
    };
    (@$v:ident, $a:expr, 101, $c:block) => {
        unroll!(@$v, $a, 100, $c);
        { const $v: usize = $a + 100; $c }
    };
    (@$v:ident, $a:expr, 102, $c:block) => {
        unroll!(@$v, $a, 51, $c);
        unroll!(@$v, $a + 51, 51, $c);
    };
    (@$v:ident, $a:expr, 103, $c:block) => {
        unroll!(@$v, $a, 102, $c);
        { const $v: usize = $a + 102; $c }
    };
    (@$v:ident, $a:expr, 104, $c:block) => {
        unroll!(@$v, $a, 52, $c);
        unroll!(@$v, $a + 52, 52, $c);
    };
    (@$v:ident, $a:expr, 105, $c:block) => {
        unroll!(@$v, $a, 104, $c);
        { const $v: usize = $a + 104; $c }
    };
    (@$v:ident, $a:expr, 106, $c:block) => {
        unroll!(@$v, $a, 53, $c);
        unroll!(@$v, $a + 53, 53, $c);
    };
    (@$v:ident, $a:expr, 107, $c:block) => {
        unroll!(@$v, $a, 106, $c);
        { const $v: usize = $a + 106; $c }
    };
    (@$v:ident, $a:expr, 108, $c:block) => {
        unroll!(@$v, $a, 54, $c);
        unroll!(@$v, $a + 54, 54, $c);
    };
    (@$v:ident, $a:expr, 109, $c:block) => {
        unroll!(@$v, $a, 108, $c);
        { const $v: usize = $a + 108; $c }
    };
    (@$v:ident, $a:expr, 110, $c:block) => {
        unroll!(@$v, $a, 55, $c);
        unroll!(@$v, $a + 55, 55, $c);
    };
    (@$v:ident, $a:expr, 111, $c:block) => {
        unroll!(@$v, $a, 110, $c);
        { const $v: usize = $a + 110; $c }
    };
    (@$v:ident, $a:expr, 112, $c:block) => {
        unroll!(@$v, $a, 56, $c);
        unroll!(@$v, $a + 56, 56, $c);
    };
    (@$v:ident, $a:expr, 113, $c:block) => {
        unroll!(@$v, $a, 112, $c);
        { const $v: usize = $a + 112; $c }
    };
    (@$v:ident, $a:expr, 114, $c:block) => {
        unroll!(@$v, $a, 57, $c);
        unroll!(@$v, $a + 57, 57, $c);
    };
    (@$v:ident, $a:expr, 115, $c:block) => {
        unroll!(@$v, $a, 114, $c);
        { const $v: usize = $a + 114; $c }
    };
    (@$v:ident, $a:expr, 116, $c:block) => {
        unroll!(@$v, $a, 58, $c);
        unroll!(@$v, $a + 58, 58, $c);
    };
    (@$v:ident, $a:expr, 117, $c:block) => {
        unroll!(@$v, $a, 116, $c);
        { const $v: usize = $a + 116; $c }
    };
    (@$v:ident, $a:expr, 118, $c:block) => {
        unroll!(@$v, $a, 59, $c);
        unroll!(@$v, $a + 59, 59, $c);
    };
    (@$v:ident, $a:expr, 119, $c:block) => {
        unroll!(@$v, $a, 118, $c);
        { const $v: usize = $a + 118; $c }
    };
    (@$v:ident, $a:expr, 120, $c:block) => {
        unroll!(@$v, $a, 60, $c);
        unroll!(@$v, $a + 60, 60, $c);
    };
    (@$v:ident, $a:expr, 121, $c:block) => {
        unroll!(@$v, $a, 120, $c);
        { const $v: usize = $a + 120; $c }
    };
    (@$v:ident, $a:expr, 122, $c:block) => {
        unroll!(@$v, $a, 61, $c);
        unroll!(@$v, $a + 61, 61, $c);
    };
    (@$v:ident, $a:expr, 123, $c:block) => {
        unroll!(@$v, $a, 122, $c);
        { const $v: usize = $a + 122; $c }
    };
    (@$v:ident, $a:expr, 124, $c:block) => {
        unroll!(@$v, $a, 62, $c);
        unroll!(@$v, $a + 62, 62, $c);
    };
    (@$v:ident, $a:expr, 125, $c:block) => {
        unroll!(@$v, $a, 124, $c);
        { const $v: usize = $a + 124; $c }
    };
    (@$v:ident, $a:expr, 126, $c:block) => {
        unroll!(@$v, $a, 63, $c);
        unroll!(@$v, $a + 63, 63, $c);
    };
    (@$v:ident, $a:expr, 127, $c:block) => {
        unroll!(@$v, $a, 126, $c);
        { const $v: usize = $a + 126; $c }
    };
    (@$v:ident, $a:expr, 128, $c:block) => {
        unroll!(@$v, $a, 64, $c);
        unroll!(@$v, $a + 64, 64, $c);
    };
    (@$v:ident, $a:expr, 129, $c:block) => {
        unroll!(@$v, $a, 128, $c);
        { const $v: usize = $a + 128; $c }
    };
    (@$v:ident, $a:expr, 130, $c:block) => {
        unroll!(@$v, $a, 65, $c);
        unroll!(@$v, $a + 65, 65, $c);
    };
    (@$v:ident, $a:expr, 131, $c:block) => {
        unroll!(@$v, $a, 130, $c);
        { const $v: usize = $a + 130; $c }
    };
    (@$v:ident, $a:expr, 132, $c:block) => {
        unroll!(@$v, $a, 66, $c);
        unroll!(@$v, $a + 66, 66, $c);
    };
    (@$v:ident, $a:expr, 133, $c:block) => {
        unroll!(@$v, $a, 132, $c);
        { const $v: usize = $a + 132; $c }
    };
    (@$v:ident, $a:expr, 134, $c:block) => {
        unroll!(@$v, $a, 67, $c);
        unroll!(@$v, $a + 67, 67, $c);
    };
    (@$v:ident, $a:expr, 135, $c:block) => {
        unroll!(@$v, $a, 134, $c);
        { const $v: usize = $a + 134; $c }
    };
    (@$v:ident, $a:expr, 136, $c:block) => {
        unroll!(@$v, $a, 68, $c);
        unroll!(@$v, $a + 68, 68, $c);
    };
    (@$v:ident, $a:expr, 137, $c:block) => {
        unroll!(@$v, $a, 136, $c);
        { const $v: usize = $a + 136; $c }
    };
    (@$v:ident, $a:expr, 138, $c:block) => {
        unroll!(@$v, $a, 69, $c);
        unroll!(@$v, $a + 69, 69, $c);
    };
    (@$v:ident, $a:expr, 139, $c:block) => {
        unroll!(@$v, $a, 138, $c);
        { const $v: usize = $a + 138; $c }
    };
    (@$v:ident, $a:expr, 140, $c:block) => {
        unroll!(@$v, $a, 70, $c);
        unroll!(@$v, $a + 70, 70, $c);
    };
    (@$v:ident, $a:expr, 141, $c:block) => {
        unroll!(@$v, $a, 140, $c);
        { const $v: usize = $a + 140; $c }
    };
    (@$v:ident, $a:expr, 142, $c:block) => {
        unroll!(@$v, $a, 71, $c);
        unroll!(@$v, $a + 71, 71, $c);
    };
    (@$v:ident, $a:expr, 143, $c:block) => {
        unroll!(@$v, $a, 142, $c);
        { const $v: usize = $a + 142; $c }
    };
    (@$v:ident, $a:expr, 144, $c:block) => {
        unroll!(@$v, $a, 72, $c);
        unroll!(@$v, $a + 72, 72, $c);
    };
    (@$v:ident, $a:expr, 145, $c:block) => {
        unroll!(@$v, $a, 144, $c);
        { const $v: usize = $a + 144; $c }
    };
    (@$v:ident, $a:expr, 146, $c:block) => {
        unroll!(@$v, $a, 73, $c);
        unroll!(@$v, $a + 73, 73, $c);
    };
    (@$v:ident, $a:expr, 147, $c:block) => {
        unroll!(@$v, $a, 146, $c);
        { const $v: usize = $a + 146; $c }
    };
    (@$v:ident, $a:expr, 148, $c:block) => {
        unroll!(@$v, $a, 74, $c);
        unroll!(@$v, $a + 74, 74, $c);
    };
    (@$v:ident, $a:expr, 149, $c:block) => {
        unroll!(@$v, $a, 148, $c);
        { const $v: usize = $a + 148; $c }
    };
    (@$v:ident, $a:expr, 150, $c:block) => {
        unroll!(@$v, $a, 75, $c);
        unroll!(@$v, $a + 75, 75, $c);
    };
    (@$v:ident, $a:expr, 151, $c:block) => {
        unroll!(@$v, $a, 150, $c);
        { const $v: usize = $a + 150; $c }
    };
    (@$v:ident, $a:expr, 152, $c:block) => {
        unroll!(@$v, $a, 76, $c);
        unroll!(@$v, $a + 76, 76, $c);
    };
    (@$v:ident, $a:expr, 153, $c:block) => {
        unroll!(@$v, $a, 152, $c);
        { const $v: usize = $a + 152; $c }
    };
    (@$v:ident, $a:expr, 154, $c:block) => {
        unroll!(@$v, $a, 77, $c);
        unroll!(@$v, $a + 77, 77, $c);
    };
    (@$v:ident, $a:expr, 155, $c:block) => {
        unroll!(@$v, $a, 154, $c);
        { const $v: usize = $a + 154; $c }
    };
    (@$v:ident, $a:expr, 156, $c:block) => {
        unroll!(@$v, $a, 78, $c);
        unroll!(@$v, $a + 78, 78, $c);
    };
    (@$v:ident, $a:expr, 157, $c:block) => {
        unroll!(@$v, $a, 156, $c);
        { const $v: usize = $a + 156; $c }
    };
    (@$v:ident, $a:expr, 158, $c:block) => {
        unroll!(@$v, $a, 79, $c);
        unroll!(@$v, $a + 79, 79, $c);
    };
    (@$v:ident, $a:expr, 159, $c:block) => {
        unroll!(@$v, $a, 158, $c);
        { const $v: usize = $a + 158; $c }
    };
    (@$v:ident, $a:expr, 160, $c:block) => {
        unroll!(@$v, $a, 80, $c);
        unroll!(@$v, $a + 80, 80, $c);
    };
    (@$v:ident, $a:expr, 161, $c:block) => {
        unroll!(@$v, $a, 160, $c);
        { const $v: usize = $a + 160; $c }
    };
    (@$v:ident, $a:expr, 162, $c:block) => {
        unroll!(@$v, $a, 81, $c);
        unroll!(@$v, $a + 81, 81, $c);
    };
    (@$v:ident, $a:expr, 163, $c:block) => {
        unroll!(@$v, $a, 162, $c);
        { const $v: usize = $a + 162; $c }
    };
    (@$v:ident, $a:expr, 164, $c:block) => {
        unroll!(@$v, $a, 82, $c);
        unroll!(@$v, $a + 82, 82, $c);
    };
    (@$v:ident, $a:expr, 165, $c:block) => {
        unroll!(@$v, $a, 164, $c);
        { const $v: usize = $a + 164; $c }
    };
    (@$v:ident, $a:expr, 166, $c:block) => {
        unroll!(@$v, $a, 83, $c);
        unroll!(@$v, $a + 83, 83, $c);
    };
    (@$v:ident, $a:expr, 167, $c:block) => {
        unroll!(@$v, $a, 166, $c);
        { const $v: usize = $a + 166; $c }
    };
    (@$v:ident, $a:expr, 168, $c:block) => {
        unroll!(@$v, $a, 84, $c);
        unroll!(@$v, $a + 84, 84, $c);
    };
    (@$v:ident, $a:expr, 169, $c:block) => {
        unroll!(@$v, $a, 168, $c);
        { const $v: usize = $a + 168; $c }
    };
    (@$v:ident, $a:expr, 170, $c:block) => {
        unroll!(@$v, $a, 85, $c);
        unroll!(@$v, $a + 85, 85, $c);
    };
    (@$v:ident, $a:expr, 171, $c:block) => {
        unroll!(@$v, $a, 170, $c);
        { const $v: usize = $a + 170; $c }
    };
    (@$v:ident, $a:expr, 172, $c:block) => {
        unroll!(@$v, $a, 86, $c);
        unroll!(@$v, $a + 86, 86, $c);
    };
    (@$v:ident, $a:expr, 173, $c:block) => {
        unroll!(@$v, $a, 172, $c);
        { const $v: usize = $a + 172; $c }
    };
    (@$v:ident, $a:expr, 174, $c:block) => {
        unroll!(@$v, $a, 87, $c);
        unroll!(@$v, $a + 87, 87, $c);
    };
    (@$v:ident, $a:expr, 175, $c:block) => {
        unroll!(@$v, $a, 174, $c);
        { const $v: usize = $a + 174; $c }
    };
    (@$v:ident, $a:expr, 176, $c:block) => {
        unroll!(@$v, $a, 88, $c);
        unroll!(@$v, $a + 88, 88, $c);
    };
    (@$v:ident, $a:expr, 177, $c:block) => {
        unroll!(@$v, $a, 176, $c);
        { const $v: usize = $a + 176; $c }
    };
    (@$v:ident, $a:expr, 178, $c:block) => {
        unroll!(@$v, $a, 89, $c);
        unroll!(@$v, $a + 89, 89, $c);
    };
    (@$v:ident, $a:expr, 179, $c:block) => {
        unroll!(@$v, $a, 178, $c);
        { const $v: usize = $a + 178; $c }
    };
    (@$v:ident, $a:expr, 180, $c:block) => {
        unroll!(@$v, $a, 90, $c);
        unroll!(@$v, $a + 90, 90, $c);
    };
    (@$v:ident, $a:expr, 181, $c:block) => {
        unroll!(@$v, $a, 180, $c);
        { const $v: usize = $a + 180; $c }
    };
    (@$v:ident, $a:expr, 182, $c:block) => {
        unroll!(@$v, $a, 91, $c);
        unroll!(@$v, $a + 91, 91, $c);
    };
    (@$v:ident, $a:expr, 183, $c:block) => {
        unroll!(@$v, $a, 182, $c);
        { const $v: usize = $a + 182; $c }
    };
    (@$v:ident, $a:expr, 184, $c:block) => {
        unroll!(@$v, $a, 92, $c);
        unroll!(@$v, $a + 92, 92, $c);
    };
    (@$v:ident, $a:expr, 185, $c:block) => {
        unroll!(@$v, $a, 184, $c);
        { const $v: usize = $a + 184; $c }
    };
    (@$v:ident, $a:expr, 186, $c:block) => {
        unroll!(@$v, $a, 93, $c);
        unroll!(@$v, $a + 93, 93, $c);
    };
    (@$v:ident, $a:expr, 187, $c:block) => {
        unroll!(@$v, $a, 186, $c);
        { const $v: usize = $a + 186; $c }
    };
    (@$v:ident, $a:expr, 188, $c:block) => {
        unroll!(@$v, $a, 94, $c);
        unroll!(@$v, $a + 94, 94, $c);
    };
    (@$v:ident, $a:expr, 189, $c:block) => {
        unroll!(@$v, $a, 188, $c);
        { const $v: usize = $a + 188; $c }
    };
    (@$v:ident, $a:expr, 190, $c:block) => {
        unroll!(@$v, $a, 95, $c);
        unroll!(@$v, $a + 95, 95, $c);
    };
    (@$v:ident, $a:expr, 191, $c:block) => {
        unroll!(@$v, $a, 190, $c);
        { const $v: usize = $a + 190; $c }
    };
    (@$v:ident, $a:expr, 192, $c:block) => {
        unroll!(@$v, $a, 96, $c);
        unroll!(@$v, $a + 96, 96, $c);
    };
    (@$v:ident, $a:expr, 193, $c:block) => {
        unroll!(@$v, $a, 192, $c);
        { const $v: usize = $a + 192; $c }
    };
    (@$v:ident, $a:expr, 194, $c:block) => {
        unroll!(@$v, $a, 97, $c);
        unroll!(@$v, $a + 97, 97, $c);
    };
    (@$v:ident, $a:expr, 195, $c:block) => {
        unroll!(@$v, $a, 194, $c);
        { const $v: usize = $a + 194; $c }
    };
    (@$v:ident, $a:expr, 196, $c:block) => {
        unroll!(@$v, $a, 98, $c);
        unroll!(@$v, $a + 98, 98, $c);
    };
    (@$v:ident, $a:expr, 197, $c:block) => {
        unroll!(@$v, $a, 196, $c);
        { const $v: usize = $a + 196; $c }
    };
    (@$v:ident, $a:expr, 198, $c:block) => {
        unroll!(@$v, $a, 99, $c);
        unroll!(@$v, $a + 99, 99, $c);
    };
    (@$v:ident, $a:expr, 199, $c:block) => {
        unroll!(@$v, $a, 198, $c);
        { const $v: usize = $a + 198; $c }
    };
    (@$v:ident, $a:expr, 200, $c:block) => {
        unroll!(@$v, $a, 100, $c);
        unroll!(@$v, $a + 100, 100, $c);
    };
    (@$v:ident, $a:expr, 201, $c:block) => {
        unroll!(@$v, $a, 200, $c);
        { const $v: usize = $a + 200; $c }
    };
    (@$v:ident, $a:expr, 202, $c:block) => {
        unroll!(@$v, $a, 101, $c);
        unroll!(@$v, $a + 101, 101, $c);
    };
    (@$v:ident, $a:expr, 203, $c:block) => {
        unroll!(@$v, $a, 202, $c);
        { const $v: usize = $a + 202; $c }
    };
    (@$v:ident, $a:expr, 204, $c:block) => {
        unroll!(@$v, $a, 102, $c);
        unroll!(@$v, $a + 102, 102, $c);
    };
    (@$v:ident, $a:expr, 205, $c:block) => {
        unroll!(@$v, $a, 204, $c);
        { const $v: usize = $a + 204; $c }
    };
    (@$v:ident, $a:expr, 206, $c:block) => {
        unroll!(@$v, $a, 103, $c);
        unroll!(@$v, $a + 103, 103, $c);
    };
    (@$v:ident, $a:expr, 207, $c:block) => {
        unroll!(@$v, $a, 206, $c);
        { const $v: usize = $a + 206; $c }
    };
    (@$v:ident, $a:expr, 208, $c:block) => {
        unroll!(@$v, $a, 104, $c);
        unroll!(@$v, $a + 104, 104, $c);
    };
    (@$v:ident, $a:expr, 209, $c:block) => {
        unroll!(@$v, $a, 208, $c);
        { const $v: usize = $a + 208; $c }
    };
    (@$v:ident, $a:expr, 210, $c:block) => {
        unroll!(@$v, $a, 105, $c);
        unroll!(@$v, $a + 105, 105, $c);
    };
    (@$v:ident, $a:expr, 211, $c:block) => {
        unroll!(@$v, $a, 210, $c);
        { const $v: usize = $a + 210; $c }
    };
    (@$v:ident, $a:expr, 212, $c:block) => {
        unroll!(@$v, $a, 106, $c);
        unroll!(@$v, $a + 106, 106, $c);
    };
    (@$v:ident, $a:expr, 213, $c:block) => {
        unroll!(@$v, $a, 212, $c);
        { const $v: usize = $a + 212; $c }
    };
    (@$v:ident, $a:expr, 214, $c:block) => {
        unroll!(@$v, $a, 107, $c);
        unroll!(@$v, $a + 107, 107, $c);
    };
    (@$v:ident, $a:expr, 215, $c:block) => {
        unroll!(@$v, $a, 214, $c);
        { const $v: usize = $a + 214; $c }
    };
    (@$v:ident, $a:expr, 216, $c:block) => {
        unroll!(@$v, $a, 108, $c);
        unroll!(@$v, $a + 108, 108, $c);
    };
    (@$v:ident, $a:expr, 217, $c:block) => {
        unroll!(@$v, $a, 216, $c);
        { const $v: usize = $a + 216; $c }
    };
    (@$v:ident, $a:expr, 218, $c:block) => {
        unroll!(@$v, $a, 109, $c);
        unroll!(@$v, $a + 109, 109, $c);
    };
    (@$v:ident, $a:expr, 219, $c:block) => {
        unroll!(@$v, $a, 218, $c);
        { const $v: usize = $a + 218; $c }
    };
    (@$v:ident, $a:expr, 220, $c:block) => {
        unroll!(@$v, $a, 110, $c);
        unroll!(@$v, $a + 110, 110, $c);
    };
    (@$v:ident, $a:expr, 221, $c:block) => {
        unroll!(@$v, $a, 220, $c);
        { const $v: usize = $a + 220; $c }
    };
    (@$v:ident, $a:expr, 222, $c:block) => {
        unroll!(@$v, $a, 111, $c);
        unroll!(@$v, $a + 111, 111, $c);
    };
    (@$v:ident, $a:expr, 223, $c:block) => {
        unroll!(@$v, $a, 222, $c);
        { const $v: usize = $a + 222; $c }
    };
    (@$v:ident, $a:expr, 224, $c:block) => {
        unroll!(@$v, $a, 112, $c);
        unroll!(@$v, $a + 112, 112, $c);
    };
    (@$v:ident, $a:expr, 225, $c:block) => {
        unroll!(@$v, $a, 224, $c);
        { const $v: usize = $a + 224; $c }
    };
    (@$v:ident, $a:expr, 226, $c:block) => {
        unroll!(@$v, $a, 113, $c);
        unroll!(@$v, $a + 113, 113, $c);
    };
    (@$v:ident, $a:expr, 227, $c:block) => {
        unroll!(@$v, $a, 226, $c);
        { const $v: usize = $a + 226; $c }
    };
    (@$v:ident, $a:expr, 228, $c:block) => {
        unroll!(@$v, $a, 114, $c);
        unroll!(@$v, $a + 114, 114, $c);
    };
    (@$v:ident, $a:expr, 229, $c:block) => {
        unroll!(@$v, $a, 228, $c);
        { const $v: usize = $a + 228; $c }
    };
    (@$v:ident, $a:expr, 230, $c:block) => {
        unroll!(@$v, $a, 115, $c);
        unroll!(@$v, $a + 115, 115, $c);
    };
    (@$v:ident, $a:expr, 231, $c:block) => {
        unroll!(@$v, $a, 230, $c);
        { const $v: usize = $a + 230; $c }
    };
    (@$v:ident, $a:expr, 232, $c:block) => {
        unroll!(@$v, $a, 116, $c);
        unroll!(@$v, $a + 116, 116, $c);
    };
    (@$v:ident, $a:expr, 233, $c:block) => {
        unroll!(@$v, $a, 232, $c);
        { const $v: usize = $a + 232; $c }
    };
    (@$v:ident, $a:expr, 234, $c:block) => {
        unroll!(@$v, $a, 117, $c);
        unroll!(@$v, $a + 117, 117, $c);
    };
    (@$v:ident, $a:expr, 235, $c:block) => {
        unroll!(@$v, $a, 234, $c);
        { const $v: usize = $a + 234; $c }
    };
    (@$v:ident, $a:expr, 236, $c:block) => {
        unroll!(@$v, $a, 118, $c);
        unroll!(@$v, $a + 118, 118, $c);
    };
    (@$v:ident, $a:expr, 237, $c:block) => {
        unroll!(@$v, $a, 236, $c);
        { const $v: usize = $a + 236; $c }
    };
    (@$v:ident, $a:expr, 238, $c:block) => {
        unroll!(@$v, $a, 119, $c);
        unroll!(@$v, $a + 119, 119, $c);
    };
    (@$v:ident, $a:expr, 239, $c:block) => {
        unroll!(@$v, $a, 238, $c);
        { const $v: usize = $a + 238; $c }
    };
    (@$v:ident, $a:expr, 240, $c:block) => {
        unroll!(@$v, $a, 120, $c);
        unroll!(@$v, $a + 120, 120, $c);
    };
    (@$v:ident, $a:expr, 241, $c:block) => {
        unroll!(@$v, $a, 240, $c);
        { const $v: usize = $a + 240; $c }
    };
    (@$v:ident, $a:expr, 242, $c:block) => {
        unroll!(@$v, $a, 121, $c);
        unroll!(@$v, $a + 121, 121, $c);
    };
    (@$v:ident, $a:expr, 243, $c:block) => {
        unroll!(@$v, $a, 242, $c);
        { const $v: usize = $a + 242; $c }
    };
    (@$v:ident, $a:expr, 244, $c:block) => {
        unroll!(@$v, $a, 122, $c);
        unroll!(@$v, $a + 122, 122, $c);
    };
    (@$v:ident, $a:expr, 245, $c:block) => {
        unroll!(@$v, $a, 244, $c);
        { const $v: usize = $a + 244; $c }
    };
    (@$v:ident, $a:expr, 246, $c:block) => {
        unroll!(@$v, $a, 123, $c);
        unroll!(@$v, $a + 123, 123, $c);
    };
    (@$v:ident, $a:expr, 247, $c:block) => {
        unroll!(@$v, $a, 246, $c);
        { const $v: usize = $a + 246; $c }
    };
    (@$v:ident, $a:expr, 248, $c:block) => {
        unroll!(@$v, $a, 124, $c);
        unroll!(@$v, $a + 124, 124, $c);
    };
    (@$v:ident, $a:expr, 249, $c:block) => {
        unroll!(@$v, $a, 248, $c);
        { const $v: usize = $a + 248; $c }
    };
    (@$v:ident, $a:expr, 250, $c:block) => {
        unroll!(@$v, $a, 125, $c);
        unroll!(@$v, $a + 125, 125, $c);
    };
    (@$v:ident, $a:expr, 251, $c:block) => {
        unroll!(@$v, $a, 250, $c);
        { const $v: usize = $a + 250; $c }
    };
    (@$v:ident, $a:expr, 252, $c:block) => {
        unroll!(@$v, $a, 126, $c);
        unroll!(@$v, $a + 126, 126, $c);
    };
    (@$v:ident, $a:expr, 253, $c:block) => {
        unroll!(@$v, $a, 252, $c);
        { const $v: usize = $a + 252; $c }
    };
    (@$v:ident, $a:expr, 254, $c:block) => {
        unroll!(@$v, $a, 127, $c);
        unroll!(@$v, $a + 127, 127, $c);
    };
    (@$v:ident, $a:expr, 255, $c:block) => {
        unroll!(@$v, $a, 254, $c);
        { const $v: usize = $a + 254; $c }
    };
    (@$v:ident, $a:expr, 256, $c:block) => {
        unroll!(@$v, $a, 128, $c);
        unroll!(@$v, $a + 128, 128, $c);
    };
    (@$v:ident, $a:expr, 257, $c:block) => {
        unroll!(@$v, $a, 256, $c);
        { const $v: usize = $a + 256; $c }
    };
    (@$v:ident, $a:expr, 258, $c:block) => {
        unroll!(@$v, $a, 129, $c);
        unroll!(@$v, $a + 129, 129, $c);
    };
    (@$v:ident, $a:expr, 259, $c:block) => {
        unroll!(@$v, $a, 258, $c);
        { const $v: usize = $a + 258; $c }
    };
    (@$v:ident, $a:expr, 260, $c:block) => {
        unroll!(@$v, $a, 130, $c);
        unroll!(@$v, $a + 130, 130, $c);
    };
    (@$v:ident, $a:expr, 261, $c:block) => {
        unroll!(@$v, $a, 260, $c);
        { const $v: usize = $a + 260; $c }
    };
    (@$v:ident, $a:expr, 262, $c:block) => {
        unroll!(@$v, $a, 131, $c);
        unroll!(@$v, $a + 131, 131, $c);
    };
    (@$v:ident, $a:expr, 263, $c:block) => {
        unroll!(@$v, $a, 262, $c);
        { const $v: usize = $a + 262; $c }
    };
    (@$v:ident, $a:expr, 264, $c:block) => {
        unroll!(@$v, $a, 132, $c);
        unroll!(@$v, $a + 132, 132, $c);
    };
    (@$v:ident, $a:expr, 265, $c:block) => {
        unroll!(@$v, $a, 264, $c);
        { const $v: usize = $a + 264; $c }
    };
    (@$v:ident, $a:expr, 266, $c:block) => {
        unroll!(@$v, $a, 133, $c);
        unroll!(@$v, $a + 133, 133, $c);
    };
    (@$v:ident, $a:expr, 267, $c:block) => {
        unroll!(@$v, $a, 266, $c);
        { const $v: usize = $a + 266; $c }
    };
    (@$v:ident, $a:expr, 268, $c:block) => {
        unroll!(@$v, $a, 134, $c);
        unroll!(@$v, $a + 134, 134, $c);
    };
    (@$v:ident, $a:expr, 269, $c:block) => {
        unroll!(@$v, $a, 268, $c);
        { const $v: usize = $a + 268; $c }
    };
    (@$v:ident, $a:expr, 270, $c:block) => {
        unroll!(@$v, $a, 135, $c);
        unroll!(@$v, $a + 135, 135, $c);
    };
    (@$v:ident, $a:expr, 271, $c:block) => {
        unroll!(@$v, $a, 270, $c);
        { const $v: usize = $a + 270; $c }
    };
    (@$v:ident, $a:expr, 272, $c:block) => {
        unroll!(@$v, $a, 136, $c);
        unroll!(@$v, $a + 136, 136, $c);
    };
    (@$v:ident, $a:expr, 273, $c:block) => {
        unroll!(@$v, $a, 272, $c);
        { const $v: usize = $a + 272; $c }
    };
    (@$v:ident, $a:expr, 274, $c:block) => {
        unroll!(@$v, $a, 137, $c);
        unroll!(@$v, $a + 137, 137, $c);
    };
    (@$v:ident, $a:expr, 275, $c:block) => {
        unroll!(@$v, $a, 274, $c);
        { const $v: usize = $a + 274; $c }
    };
    (@$v:ident, $a:expr, 276, $c:block) => {
        unroll!(@$v, $a, 138, $c);
        unroll!(@$v, $a + 138, 138, $c);
    };
    (@$v:ident, $a:expr, 277, $c:block) => {
        unroll!(@$v, $a, 276, $c);
        { const $v: usize = $a + 276; $c }
    };
    (@$v:ident, $a:expr, 278, $c:block) => {
        unroll!(@$v, $a, 139, $c);
        unroll!(@$v, $a + 139, 139, $c);
    };
    (@$v:ident, $a:expr, 279, $c:block) => {
        unroll!(@$v, $a, 278, $c);
        { const $v: usize = $a + 278; $c }
    };
    (@$v:ident, $a:expr, 280, $c:block) => {
        unroll!(@$v, $a, 140, $c);
        unroll!(@$v, $a + 140, 140, $c);
    };
    (@$v:ident, $a:expr, 281, $c:block) => {
        unroll!(@$v, $a, 280, $c);
        { const $v: usize = $a + 280; $c }
    };
    (@$v:ident, $a:expr, 282, $c:block) => {
        unroll!(@$v, $a, 141, $c);
        unroll!(@$v, $a + 141, 141, $c);
    };
    (@$v:ident, $a:expr, 283, $c:block) => {
        unroll!(@$v, $a, 282, $c);
        { const $v: usize = $a + 282; $c }
    };
    (@$v:ident, $a:expr, 284, $c:block) => {
        unroll!(@$v, $a, 142, $c);
        unroll!(@$v, $a + 142, 142, $c);
    };
    (@$v:ident, $a:expr, 285, $c:block) => {
        unroll!(@$v, $a, 284, $c);
        { const $v: usize = $a + 284; $c }
    };
    (@$v:ident, $a:expr, 286, $c:block) => {
        unroll!(@$v, $a, 143, $c);
        unroll!(@$v, $a + 143, 143, $c);
    };
    (@$v:ident, $a:expr, 287, $c:block) => {
        unroll!(@$v, $a, 286, $c);
        { const $v: usize = $a + 286; $c }
    };
    (@$v:ident, $a:expr, 288, $c:block) => {
        unroll!(@$v, $a, 144, $c);
        unroll!(@$v, $a + 144, 144, $c);
    };
    (@$v:ident, $a:expr, 289, $c:block) => {
        unroll!(@$v, $a, 288, $c);
        { const $v: usize = $a + 288; $c }
    };
    (@$v:ident, $a:expr, 290, $c:block) => {
        unroll!(@$v, $a, 145, $c);
        unroll!(@$v, $a + 145, 145, $c);
    };
    (@$v:ident, $a:expr, 291, $c:block) => {
        unroll!(@$v, $a, 290, $c);
        { const $v: usize = $a + 290; $c }
    };
    (@$v:ident, $a:expr, 292, $c:block) => {
        unroll!(@$v, $a, 146, $c);
        unroll!(@$v, $a + 146, 146, $c);
    };
    (@$v:ident, $a:expr, 293, $c:block) => {
        unroll!(@$v, $a, 292, $c);
        { const $v: usize = $a + 292; $c }
    };
    (@$v:ident, $a:expr, 294, $c:block) => {
        unroll!(@$v, $a, 147, $c);
        unroll!(@$v, $a + 147, 147, $c);
    };
    (@$v:ident, $a:expr, 295, $c:block) => {
        unroll!(@$v, $a, 294, $c);
        { const $v: usize = $a + 294; $c }
    };
    (@$v:ident, $a:expr, 296, $c:block) => {
        unroll!(@$v, $a, 148, $c);
        unroll!(@$v, $a + 148, 148, $c);
    };
    (@$v:ident, $a:expr, 297, $c:block) => {
        unroll!(@$v, $a, 296, $c);
        { const $v: usize = $a + 296; $c }
    };
    (@$v:ident, $a:expr, 298, $c:block) => {
        unroll!(@$v, $a, 149, $c);
        unroll!(@$v, $a + 149, 149, $c);
    };
    (@$v:ident, $a:expr, 299, $c:block) => {
        unroll!(@$v, $a, 298, $c);
        { const $v: usize = $a + 298; $c }
    };
    (@$v:ident, $a:expr, 300, $c:block) => {
        unroll!(@$v, $a, 150, $c);
        unroll!(@$v, $a + 150, 150, $c);
    };
    (@$v:ident, $a:expr, 301, $c:block) => {
        unroll!(@$v, $a, 300, $c);
        { const $v: usize = $a + 300; $c }
    };
    (@$v:ident, $a:expr, 302, $c:block) => {
        unroll!(@$v, $a, 151, $c);
        unroll!(@$v, $a + 151, 151, $c);
    };
    (@$v:ident, $a:expr, 303, $c:block) => {
        unroll!(@$v, $a, 302, $c);
        { const $v: usize = $a + 302; $c }
    };
    (@$v:ident, $a:expr, 304, $c:block) => {
        unroll!(@$v, $a, 152, $c);
        unroll!(@$v, $a + 152, 152, $c);
    };
    (@$v:ident, $a:expr, 305, $c:block) => {
        unroll!(@$v, $a, 304, $c);
        { const $v: usize = $a + 304; $c }
    };
    (@$v:ident, $a:expr, 306, $c:block) => {
        unroll!(@$v, $a, 153, $c);
        unroll!(@$v, $a + 153, 153, $c);
    };
    (@$v:ident, $a:expr, 307, $c:block) => {
        unroll!(@$v, $a, 306, $c);
        { const $v: usize = $a + 306; $c }
    };
    (@$v:ident, $a:expr, 308, $c:block) => {
        unroll!(@$v, $a, 154, $c);
        unroll!(@$v, $a + 154, 154, $c);
    };
    (@$v:ident, $a:expr, 309, $c:block) => {
        unroll!(@$v, $a, 308, $c);
        { const $v: usize = $a + 308; $c }
    };
    (@$v:ident, $a:expr, 310, $c:block) => {
        unroll!(@$v, $a, 155, $c);
        unroll!(@$v, $a + 155, 155, $c);
    };
    (@$v:ident, $a:expr, 311, $c:block) => {
        unroll!(@$v, $a, 310, $c);
        { const $v: usize = $a + 310; $c }
    };
    (@$v:ident, $a:expr, 312, $c:block) => {
        unroll!(@$v, $a, 156, $c);
        unroll!(@$v, $a + 156, 156, $c);
    };
    (@$v:ident, $a:expr, 313, $c:block) => {
        unroll!(@$v, $a, 312, $c);
        { const $v: usize = $a + 312; $c }
    };
    (@$v:ident, $a:expr, 314, $c:block) => {
        unroll!(@$v, $a, 157, $c);
        unroll!(@$v, $a + 157, 157, $c);
    };
    (@$v:ident, $a:expr, 315, $c:block) => {
        unroll!(@$v, $a, 314, $c);
        { const $v: usize = $a + 314; $c }
    };
    (@$v:ident, $a:expr, 316, $c:block) => {
        unroll!(@$v, $a, 158, $c);
        unroll!(@$v, $a + 158, 158, $c);
    };
    (@$v:ident, $a:expr, 317, $c:block) => {
        unroll!(@$v, $a, 316, $c);
        { const $v: usize = $a + 316; $c }
    };
    (@$v:ident, $a:expr, 318, $c:block) => {
        unroll!(@$v, $a, 159, $c);
        unroll!(@$v, $a + 159, 159, $c);
    };
    (@$v:ident, $a:expr, 319, $c:block) => {
        unroll!(@$v, $a, 318, $c);
        { const $v: usize = $a + 318; $c }
    };
    (@$v:ident, $a:expr, 320, $c:block) => {
        unroll!(@$v, $a, 160, $c);
        unroll!(@$v, $a + 160, 160, $c);
    };
    (@$v:ident, $a:expr, 321, $c:block) => {
        unroll!(@$v, $a, 320, $c);
        { const $v: usize = $a + 320; $c }
    };
    (@$v:ident, $a:expr, 322, $c:block) => {
        unroll!(@$v, $a, 161, $c);
        unroll!(@$v, $a + 161, 161, $c);
    };
    (@$v:ident, $a:expr, 323, $c:block) => {
        unroll!(@$v, $a, 322, $c);
        { const $v: usize = $a + 322; $c }
    };
    (@$v:ident, $a:expr, 324, $c:block) => {
        unroll!(@$v, $a, 162, $c);
        unroll!(@$v, $a + 162, 162, $c);
    };
    (@$v:ident, $a:expr, 325, $c:block) => {
        unroll!(@$v, $a, 324, $c);
        { const $v: usize = $a + 324; $c }
    };
    (@$v:ident, $a:expr, 326, $c:block) => {
        unroll!(@$v, $a, 163, $c);
        unroll!(@$v, $a + 163, 163, $c);
    };
    (@$v:ident, $a:expr, 327, $c:block) => {
        unroll!(@$v, $a, 326, $c);
        { const $v: usize = $a + 326; $c }
    };
    (@$v:ident, $a:expr, 328, $c:block) => {
        unroll!(@$v, $a, 164, $c);
        unroll!(@$v, $a + 164, 164, $c);
    };
    (@$v:ident, $a:expr, 329, $c:block) => {
        unroll!(@$v, $a, 328, $c);
        { const $v: usize = $a + 328; $c }
    };
    (@$v:ident, $a:expr, 330, $c:block) => {
        unroll!(@$v, $a, 165, $c);
        unroll!(@$v, $a + 165, 165, $c);
    };
    (@$v:ident, $a:expr, 331, $c:block) => {
        unroll!(@$v, $a, 330, $c);
        { const $v: usize = $a + 330; $c }
    };
    (@$v:ident, $a:expr, 332, $c:block) => {
        unroll!(@$v, $a, 166, $c);
        unroll!(@$v, $a + 166, 166, $c);
    };
    (@$v:ident, $a:expr, 333, $c:block) => {
        unroll!(@$v, $a, 332, $c);
        { const $v: usize = $a + 332; $c }
    };
    (@$v:ident, $a:expr, 334, $c:block) => {
        unroll!(@$v, $a, 167, $c);
        unroll!(@$v, $a + 167, 167, $c);
    };
    (@$v:ident, $a:expr, 335, $c:block) => {
        unroll!(@$v, $a, 334, $c);
        { const $v: usize = $a + 334; $c }
    };
    (@$v:ident, $a:expr, 336, $c:block) => {
        unroll!(@$v, $a, 168, $c);
        unroll!(@$v, $a + 168, 168, $c);
    };
    (@$v:ident, $a:expr, 337, $c:block) => {
        unroll!(@$v, $a, 336, $c);
        { const $v: usize = $a + 336; $c }
    };
    (@$v:ident, $a:expr, 338, $c:block) => {
        unroll!(@$v, $a, 169, $c);
        unroll!(@$v, $a + 169, 169, $c);
    };
    (@$v:ident, $a:expr, 339, $c:block) => {
        unroll!(@$v, $a, 338, $c);
        { const $v: usize = $a + 338; $c }
    };
    (@$v:ident, $a:expr, 340, $c:block) => {
        unroll!(@$v, $a, 170, $c);
        unroll!(@$v, $a + 170, 170, $c);
    };
    (@$v:ident, $a:expr, 341, $c:block) => {
        unroll!(@$v, $a, 340, $c);
        { const $v: usize = $a + 340; $c }
    };
    (@$v:ident, $a:expr, 342, $c:block) => {
        unroll!(@$v, $a, 171, $c);
        unroll!(@$v, $a + 171, 171, $c);
    };
    (@$v:ident, $a:expr, 343, $c:block) => {
        unroll!(@$v, $a, 342, $c);
        { const $v: usize = $a + 342; $c }
    };
    (@$v:ident, $a:expr, 344, $c:block) => {
        unroll!(@$v, $a, 172, $c);
        unroll!(@$v, $a + 172, 172, $c);
    };
    (@$v:ident, $a:expr, 345, $c:block) => {
        unroll!(@$v, $a, 344, $c);
        { const $v: usize = $a + 344; $c }
    };
    (@$v:ident, $a:expr, 346, $c:block) => {
        unroll!(@$v, $a, 173, $c);
        unroll!(@$v, $a + 173, 173, $c);
    };
    (@$v:ident, $a:expr, 347, $c:block) => {
        unroll!(@$v, $a, 346, $c);
        { const $v: usize = $a + 346; $c }
    };
    (@$v:ident, $a:expr, 348, $c:block) => {
        unroll!(@$v, $a, 174, $c);
        unroll!(@$v, $a + 174, 174, $c);
    };
    (@$v:ident, $a:expr, 349, $c:block) => {
        unroll!(@$v, $a, 348, $c);
        { const $v: usize = $a + 348; $c }
    };
    (@$v:ident, $a:expr, 350, $c:block) => {
        unroll!(@$v, $a, 175, $c);
        unroll!(@$v, $a + 175, 175, $c);
    };
    (@$v:ident, $a:expr, 351, $c:block) => {
        unroll!(@$v, $a, 350, $c);
        { const $v: usize = $a + 350; $c }
    };
    (@$v:ident, $a:expr, 352, $c:block) => {
        unroll!(@$v, $a, 176, $c);
        unroll!(@$v, $a + 176, 176, $c);
    };
    (@$v:ident, $a:expr, 353, $c:block) => {
        unroll!(@$v, $a, 352, $c);
        { const $v: usize = $a + 352; $c }
    };
    (@$v:ident, $a:expr, 354, $c:block) => {
        unroll!(@$v, $a, 177, $c);
        unroll!(@$v, $a + 177, 177, $c);
    };
    (@$v:ident, $a:expr, 355, $c:block) => {
        unroll!(@$v, $a, 354, $c);
        { const $v: usize = $a + 354; $c }
    };
    (@$v:ident, $a:expr, 356, $c:block) => {
        unroll!(@$v, $a, 178, $c);
        unroll!(@$v, $a + 178, 178, $c);
    };
    (@$v:ident, $a:expr, 357, $c:block) => {
        unroll!(@$v, $a, 356, $c);
        { const $v: usize = $a + 356; $c }
    };
    (@$v:ident, $a:expr, 358, $c:block) => {
        unroll!(@$v, $a, 179, $c);
        unroll!(@$v, $a + 179, 179, $c);
    };
    (@$v:ident, $a:expr, 359, $c:block) => {
        unroll!(@$v, $a, 358, $c);
        { const $v: usize = $a + 358; $c }
    };
    (@$v:ident, $a:expr, 360, $c:block) => {
        unroll!(@$v, $a, 180, $c);
        unroll!(@$v, $a + 180, 180, $c);
    };
    (@$v:ident, $a:expr, 361, $c:block) => {
        unroll!(@$v, $a, 360, $c);
        { const $v: usize = $a + 360; $c }
    };
    (@$v:ident, $a:expr, 362, $c:block) => {
        unroll!(@$v, $a, 181, $c);
        unroll!(@$v, $a + 181, 181, $c);
    };
    (@$v:ident, $a:expr, 363, $c:block) => {
        unroll!(@$v, $a, 362, $c);
        { const $v: usize = $a + 362; $c }
    };
    (@$v:ident, $a:expr, 364, $c:block) => {
        unroll!(@$v, $a, 182, $c);
        unroll!(@$v, $a + 182, 182, $c);
    };
    (@$v:ident, $a:expr, 365, $c:block) => {
        unroll!(@$v, $a, 364, $c);
        { const $v: usize = $a + 364; $c }
    };
    (@$v:ident, $a:expr, 366, $c:block) => {
        unroll!(@$v, $a, 183, $c);
        unroll!(@$v, $a + 183, 183, $c);
    };
    (@$v:ident, $a:expr, 367, $c:block) => {
        unroll!(@$v, $a, 366, $c);
        { const $v: usize = $a + 366; $c }
    };
    (@$v:ident, $a:expr, 368, $c:block) => {
        unroll!(@$v, $a, 184, $c);
        unroll!(@$v, $a + 184, 184, $c);
    };
    (@$v:ident, $a:expr, 369, $c:block) => {
        unroll!(@$v, $a, 368, $c);
        { const $v: usize = $a + 368; $c }
    };
    (@$v:ident, $a:expr, 370, $c:block) => {
        unroll!(@$v, $a, 185, $c);
        unroll!(@$v, $a + 185, 185, $c);
    };
    (@$v:ident, $a:expr, 371, $c:block) => {
        unroll!(@$v, $a, 370, $c);
        { const $v: usize = $a + 370; $c }
    };
    (@$v:ident, $a:expr, 372, $c:block) => {
        unroll!(@$v, $a, 186, $c);
        unroll!(@$v, $a + 186, 186, $c);
    };
    (@$v:ident, $a:expr, 373, $c:block) => {
        unroll!(@$v, $a, 372, $c);
        { const $v: usize = $a + 372; $c }
    };
    (@$v:ident, $a:expr, 374, $c:block) => {
        unroll!(@$v, $a, 187, $c);
        unroll!(@$v, $a + 187, 187, $c);
    };
    (@$v:ident, $a:expr, 375, $c:block) => {
        unroll!(@$v, $a, 374, $c);
        { const $v: usize = $a + 374; $c }
    };
    (@$v:ident, $a:expr, 376, $c:block) => {
        unroll!(@$v, $a, 188, $c);
        unroll!(@$v, $a + 188, 188, $c);
    };
    (@$v:ident, $a:expr, 377, $c:block) => {
        unroll!(@$v, $a, 376, $c);
        { const $v: usize = $a + 376; $c }
    };
    (@$v:ident, $a:expr, 378, $c:block) => {
        unroll!(@$v, $a, 189, $c);
        unroll!(@$v, $a + 189, 189, $c);
    };
    (@$v:ident, $a:expr, 379, $c:block) => {
        unroll!(@$v, $a, 378, $c);
        { const $v: usize = $a + 378; $c }
    };
    (@$v:ident, $a:expr, 380, $c:block) => {
        unroll!(@$v, $a, 190, $c);
        unroll!(@$v, $a + 190, 190, $c);
    };
    (@$v:ident, $a:expr, 381, $c:block) => {
        unroll!(@$v, $a, 380, $c);
        { const $v: usize = $a + 380; $c }
    };
    (@$v:ident, $a:expr, 382, $c:block) => {
        unroll!(@$v, $a, 191, $c);
        unroll!(@$v, $a + 191, 191, $c);
    };
    (@$v:ident, $a:expr, 383, $c:block) => {
        unroll!(@$v, $a, 382, $c);
        { const $v: usize = $a + 382; $c }
    };
    (@$v:ident, $a:expr, 384, $c:block) => {
        unroll!(@$v, $a, 192, $c);
        unroll!(@$v, $a + 192, 192, $c);
    };
    (@$v:ident, $a:expr, 385, $c:block) => {
        unroll!(@$v, $a, 384, $c);
        { const $v: usize = $a + 384; $c }
    };
    (@$v:ident, $a:expr, 386, $c:block) => {
        unroll!(@$v, $a, 193, $c);
        unroll!(@$v, $a + 193, 193, $c);
    };
    (@$v:ident, $a:expr, 387, $c:block) => {
        unroll!(@$v, $a, 386, $c);
        { const $v: usize = $a + 386; $c }
    };
    (@$v:ident, $a:expr, 388, $c:block) => {
        unroll!(@$v, $a, 194, $c);
        unroll!(@$v, $a + 194, 194, $c);
    };
    (@$v:ident, $a:expr, 389, $c:block) => {
        unroll!(@$v, $a, 388, $c);
        { const $v: usize = $a + 388; $c }
    };
    (@$v:ident, $a:expr, 390, $c:block) => {
        unroll!(@$v, $a, 195, $c);
        unroll!(@$v, $a + 195, 195, $c);
    };
    (@$v:ident, $a:expr, 391, $c:block) => {
        unroll!(@$v, $a, 390, $c);
        { const $v: usize = $a + 390; $c }
    };
    (@$v:ident, $a:expr, 392, $c:block) => {
        unroll!(@$v, $a, 196, $c);
        unroll!(@$v, $a + 196, 196, $c);
    };
    (@$v:ident, $a:expr, 393, $c:block) => {
        unroll!(@$v, $a, 392, $c);
        { const $v: usize = $a + 392; $c }
    };
    (@$v:ident, $a:expr, 394, $c:block) => {
        unroll!(@$v, $a, 197, $c);
        unroll!(@$v, $a + 197, 197, $c);
    };
    (@$v:ident, $a:expr, 395, $c:block) => {
        unroll!(@$v, $a, 394, $c);
        { const $v: usize = $a + 394; $c }
    };
    (@$v:ident, $a:expr, 396, $c:block) => {
        unroll!(@$v, $a, 198, $c);
        unroll!(@$v, $a + 198, 198, $c);
    };
    (@$v:ident, $a:expr, 397, $c:block) => {
        unroll!(@$v, $a, 396, $c);
        { const $v: usize = $a + 396; $c }
    };
    (@$v:ident, $a:expr, 398, $c:block) => {
        unroll!(@$v, $a, 199, $c);
        unroll!(@$v, $a + 199, 199, $c);
    };
    (@$v:ident, $a:expr, 399, $c:block) => {
        unroll!(@$v, $a, 398, $c);
        { const $v: usize = $a + 398; $c }
    };
    (@$v:ident, $a:expr, 400, $c:block) => {
        unroll!(@$v, $a, 200, $c);
        unroll!(@$v, $a + 200, 200, $c);
    };
    (@$v:ident, $a:expr, 401, $c:block) => {
        unroll!(@$v, $a, 400, $c);
        { const $v: usize = $a + 400; $c }
    };
    (@$v:ident, $a:expr, 402, $c:block) => {
        unroll!(@$v, $a, 201, $c);
        unroll!(@$v, $a + 201, 201, $c);
    };
    (@$v:ident, $a:expr, 403, $c:block) => {
        unroll!(@$v, $a, 402, $c);
        { const $v: usize = $a + 402; $c }
    };
    (@$v:ident, $a:expr, 404, $c:block) => {
        unroll!(@$v, $a, 202, $c);
        unroll!(@$v, $a + 202, 202, $c);
    };
    (@$v:ident, $a:expr, 405, $c:block) => {
        unroll!(@$v, $a, 404, $c);
        { const $v: usize = $a + 404; $c }
    };
    (@$v:ident, $a:expr, 406, $c:block) => {
        unroll!(@$v, $a, 203, $c);
        unroll!(@$v, $a + 203, 203, $c);
    };
    (@$v:ident, $a:expr, 407, $c:block) => {
        unroll!(@$v, $a, 406, $c);
        { const $v: usize = $a + 406; $c }
    };
    (@$v:ident, $a:expr, 408, $c:block) => {
        unroll!(@$v, $a, 204, $c);
        unroll!(@$v, $a + 204, 204, $c);
    };
    (@$v:ident, $a:expr, 409, $c:block) => {
        unroll!(@$v, $a, 408, $c);
        { const $v: usize = $a + 408; $c }
    };
    (@$v:ident, $a:expr, 410, $c:block) => {
        unroll!(@$v, $a, 205, $c);
        unroll!(@$v, $a + 205, 205, $c);
    };
    (@$v:ident, $a:expr, 411, $c:block) => {
        unroll!(@$v, $a, 410, $c);
        { const $v: usize = $a + 410; $c }
    };
    (@$v:ident, $a:expr, 412, $c:block) => {
        unroll!(@$v, $a, 206, $c);
        unroll!(@$v, $a + 206, 206, $c);
    };
    (@$v:ident, $a:expr, 413, $c:block) => {
        unroll!(@$v, $a, 412, $c);
        { const $v: usize = $a + 412; $c }
    };
    (@$v:ident, $a:expr, 414, $c:block) => {
        unroll!(@$v, $a, 207, $c);
        unroll!(@$v, $a + 207, 207, $c);
    };
    (@$v:ident, $a:expr, 415, $c:block) => {
        unroll!(@$v, $a, 414, $c);
        { const $v: usize = $a + 414; $c }
    };
    (@$v:ident, $a:expr, 416, $c:block) => {
        unroll!(@$v, $a, 208, $c);
        unroll!(@$v, $a + 208, 208, $c);
    };
    (@$v:ident, $a:expr, 417, $c:block) => {
        unroll!(@$v, $a, 416, $c);
        { const $v: usize = $a + 416; $c }
    };
    (@$v:ident, $a:expr, 418, $c:block) => {
        unroll!(@$v, $a, 209, $c);
        unroll!(@$v, $a + 209, 209, $c);
    };
    (@$v:ident, $a:expr, 419, $c:block) => {
        unroll!(@$v, $a, 418, $c);
        { const $v: usize = $a + 418; $c }
    };
    (@$v:ident, $a:expr, 420, $c:block) => {
        unroll!(@$v, $a, 210, $c);
        unroll!(@$v, $a + 210, 210, $c);
    };
    (@$v:ident, $a:expr, 421, $c:block) => {
        unroll!(@$v, $a, 420, $c);
        { const $v: usize = $a + 420; $c }
    };
    (@$v:ident, $a:expr, 422, $c:block) => {
        unroll!(@$v, $a, 211, $c);
        unroll!(@$v, $a + 211, 211, $c);
    };
    (@$v:ident, $a:expr, 423, $c:block) => {
        unroll!(@$v, $a, 422, $c);
        { const $v: usize = $a + 422; $c }
    };
    (@$v:ident, $a:expr, 424, $c:block) => {
        unroll!(@$v, $a, 212, $c);
        unroll!(@$v, $a + 212, 212, $c);
    };
    (@$v:ident, $a:expr, 425, $c:block) => {
        unroll!(@$v, $a, 424, $c);
        { const $v: usize = $a + 424; $c }
    };
    (@$v:ident, $a:expr, 426, $c:block) => {
        unroll!(@$v, $a, 213, $c);
        unroll!(@$v, $a + 213, 213, $c);
    };
    (@$v:ident, $a:expr, 427, $c:block) => {
        unroll!(@$v, $a, 426, $c);
        { const $v: usize = $a + 426; $c }
    };
    (@$v:ident, $a:expr, 428, $c:block) => {
        unroll!(@$v, $a, 214, $c);
        unroll!(@$v, $a + 214, 214, $c);
    };
    (@$v:ident, $a:expr, 429, $c:block) => {
        unroll!(@$v, $a, 428, $c);
        { const $v: usize = $a + 428; $c }
    };
    (@$v:ident, $a:expr, 430, $c:block) => {
        unroll!(@$v, $a, 215, $c);
        unroll!(@$v, $a + 215, 215, $c);
    };
    (@$v:ident, $a:expr, 431, $c:block) => {
        unroll!(@$v, $a, 430, $c);
        { const $v: usize = $a + 430; $c }
    };
    (@$v:ident, $a:expr, 432, $c:block) => {
        unroll!(@$v, $a, 216, $c);
        unroll!(@$v, $a + 216, 216, $c);
    };
    (@$v:ident, $a:expr, 433, $c:block) => {
        unroll!(@$v, $a, 432, $c);
        { const $v: usize = $a + 432; $c }
    };
    (@$v:ident, $a:expr, 434, $c:block) => {
        unroll!(@$v, $a, 217, $c);
        unroll!(@$v, $a + 217, 217, $c);
    };
    (@$v:ident, $a:expr, 435, $c:block) => {
        unroll!(@$v, $a, 434, $c);
        { const $v: usize = $a + 434; $c }
    };
    (@$v:ident, $a:expr, 436, $c:block) => {
        unroll!(@$v, $a, 218, $c);
        unroll!(@$v, $a + 218, 218, $c);
    };
    (@$v:ident, $a:expr, 437, $c:block) => {
        unroll!(@$v, $a, 436, $c);
        { const $v: usize = $a + 436; $c }
    };
    (@$v:ident, $a:expr, 438, $c:block) => {
        unroll!(@$v, $a, 219, $c);
        unroll!(@$v, $a + 219, 219, $c);
    };
    (@$v:ident, $a:expr, 439, $c:block) => {
        unroll!(@$v, $a, 438, $c);
        { const $v: usize = $a + 438; $c }
    };
    (@$v:ident, $a:expr, 440, $c:block) => {
        unroll!(@$v, $a, 220, $c);
        unroll!(@$v, $a + 220, 220, $c);
    };
    (@$v:ident, $a:expr, 441, $c:block) => {
        unroll!(@$v, $a, 440, $c);
        { const $v: usize = $a + 440; $c }
    };
    (@$v:ident, $a:expr, 442, $c:block) => {
        unroll!(@$v, $a, 221, $c);
        unroll!(@$v, $a + 221, 221, $c);
    };
    (@$v:ident, $a:expr, 443, $c:block) => {
        unroll!(@$v, $a, 442, $c);
        { const $v: usize = $a + 442; $c }
    };
    (@$v:ident, $a:expr, 444, $c:block) => {
        unroll!(@$v, $a, 222, $c);
        unroll!(@$v, $a + 222, 222, $c);
    };
    (@$v:ident, $a:expr, 445, $c:block) => {
        unroll!(@$v, $a, 444, $c);
        { const $v: usize = $a + 444; $c }
    };
    (@$v:ident, $a:expr, 446, $c:block) => {
        unroll!(@$v, $a, 223, $c);
        unroll!(@$v, $a + 223, 223, $c);
    };
    (@$v:ident, $a:expr, 447, $c:block) => {
        unroll!(@$v, $a, 446, $c);
        { const $v: usize = $a + 446; $c }
    };
    (@$v:ident, $a:expr, 448, $c:block) => {
        unroll!(@$v, $a, 224, $c);
        unroll!(@$v, $a + 224, 224, $c);
    };
    (@$v:ident, $a:expr, 449, $c:block) => {
        unroll!(@$v, $a, 448, $c);
        { const $v: usize = $a + 448; $c }
    };
    (@$v:ident, $a:expr, 450, $c:block) => {
        unroll!(@$v, $a, 225, $c);
        unroll!(@$v, $a + 225, 225, $c);
    };
    (@$v:ident, $a:expr, 451, $c:block) => {
        unroll!(@$v, $a, 450, $c);
        { const $v: usize = $a + 450; $c }
    };
    (@$v:ident, $a:expr, 452, $c:block) => {
        unroll!(@$v, $a, 226, $c);
        unroll!(@$v, $a + 226, 226, $c);
    };
    (@$v:ident, $a:expr, 453, $c:block) => {
        unroll!(@$v, $a, 452, $c);
        { const $v: usize = $a + 452; $c }
    };
    (@$v:ident, $a:expr, 454, $c:block) => {
        unroll!(@$v, $a, 227, $c);
        unroll!(@$v, $a + 227, 227, $c);
    };
    (@$v:ident, $a:expr, 455, $c:block) => {
        unroll!(@$v, $a, 454, $c);
        { const $v: usize = $a + 454; $c }
    };
    (@$v:ident, $a:expr, 456, $c:block) => {
        unroll!(@$v, $a, 228, $c);
        unroll!(@$v, $a + 228, 228, $c);
    };
    (@$v:ident, $a:expr, 457, $c:block) => {
        unroll!(@$v, $a, 456, $c);
        { const $v: usize = $a + 456; $c }
    };
    (@$v:ident, $a:expr, 458, $c:block) => {
        unroll!(@$v, $a, 229, $c);
        unroll!(@$v, $a + 229, 229, $c);
    };
    (@$v:ident, $a:expr, 459, $c:block) => {
        unroll!(@$v, $a, 458, $c);
        { const $v: usize = $a + 458; $c }
    };
    (@$v:ident, $a:expr, 460, $c:block) => {
        unroll!(@$v, $a, 230, $c);
        unroll!(@$v, $a + 230, 230, $c);
    };
    (@$v:ident, $a:expr, 461, $c:block) => {
        unroll!(@$v, $a, 460, $c);
        { const $v: usize = $a + 460; $c }
    };
    (@$v:ident, $a:expr, 462, $c:block) => {
        unroll!(@$v, $a, 231, $c);
        unroll!(@$v, $a + 231, 231, $c);
    };
    (@$v:ident, $a:expr, 463, $c:block) => {
        unroll!(@$v, $a, 462, $c);
        { const $v: usize = $a + 462; $c }
    };
    (@$v:ident, $a:expr, 464, $c:block) => {
        unroll!(@$v, $a, 232, $c);
        unroll!(@$v, $a + 232, 232, $c);
    };
    (@$v:ident, $a:expr, 465, $c:block) => {
        unroll!(@$v, $a, 464, $c);
        { const $v: usize = $a + 464; $c }
    };
    (@$v:ident, $a:expr, 466, $c:block) => {
        unroll!(@$v, $a, 233, $c);
        unroll!(@$v, $a + 233, 233, $c);
    };
    (@$v:ident, $a:expr, 467, $c:block) => {
        unroll!(@$v, $a, 466, $c);
        { const $v: usize = $a + 466; $c }
    };
    (@$v:ident, $a:expr, 468, $c:block) => {
        unroll!(@$v, $a, 234, $c);
        unroll!(@$v, $a + 234, 234, $c);
    };
    (@$v:ident, $a:expr, 469, $c:block) => {
        unroll!(@$v, $a, 468, $c);
        { const $v: usize = $a + 468; $c }
    };
    (@$v:ident, $a:expr, 470, $c:block) => {
        unroll!(@$v, $a, 235, $c);
        unroll!(@$v, $a + 235, 235, $c);
    };
    (@$v:ident, $a:expr, 471, $c:block) => {
        unroll!(@$v, $a, 470, $c);
        { const $v: usize = $a + 470; $c }
    };
    (@$v:ident, $a:expr, 472, $c:block) => {
        unroll!(@$v, $a, 236, $c);
        unroll!(@$v, $a + 236, 236, $c);
    };
    (@$v:ident, $a:expr, 473, $c:block) => {
        unroll!(@$v, $a, 472, $c);
        { const $v: usize = $a + 472; $c }
    };
    (@$v:ident, $a:expr, 474, $c:block) => {
        unroll!(@$v, $a, 237, $c);
        unroll!(@$v, $a + 237, 237, $c);
    };
    (@$v:ident, $a:expr, 475, $c:block) => {
        unroll!(@$v, $a, 474, $c);
        { const $v: usize = $a + 474; $c }
    };
    (@$v:ident, $a:expr, 476, $c:block) => {
        unroll!(@$v, $a, 238, $c);
        unroll!(@$v, $a + 238, 238, $c);
    };
    (@$v:ident, $a:expr, 477, $c:block) => {
        unroll!(@$v, $a, 476, $c);
        { const $v: usize = $a + 476; $c }
    };
    (@$v:ident, $a:expr, 478, $c:block) => {
        unroll!(@$v, $a, 239, $c);
        unroll!(@$v, $a + 239, 239, $c);
    };
    (@$v:ident, $a:expr, 479, $c:block) => {
        unroll!(@$v, $a, 478, $c);
        { const $v: usize = $a + 478; $c }
    };
    (@$v:ident, $a:expr, 480, $c:block) => {
        unroll!(@$v, $a, 240, $c);
        unroll!(@$v, $a + 240, 240, $c);
    };
    (@$v:ident, $a:expr, 481, $c:block) => {
        unroll!(@$v, $a, 480, $c);
        { const $v: usize = $a + 480; $c }
    };
    (@$v:ident, $a:expr, 482, $c:block) => {
        unroll!(@$v, $a, 241, $c);
        unroll!(@$v, $a + 241, 241, $c);
    };
    (@$v:ident, $a:expr, 483, $c:block) => {
        unroll!(@$v, $a, 482, $c);
        { const $v: usize = $a + 482; $c }
    };
    (@$v:ident, $a:expr, 484, $c:block) => {
        unroll!(@$v, $a, 242, $c);
        unroll!(@$v, $a + 242, 242, $c);
    };
    (@$v:ident, $a:expr, 485, $c:block) => {
        unroll!(@$v, $a, 484, $c);
        { const $v: usize = $a + 484; $c }
    };
    (@$v:ident, $a:expr, 486, $c:block) => {
        unroll!(@$v, $a, 243, $c);
        unroll!(@$v, $a + 243, 243, $c);
    };
    (@$v:ident, $a:expr, 487, $c:block) => {
        unroll!(@$v, $a, 486, $c);
        { const $v: usize = $a + 486; $c }
    };
    (@$v:ident, $a:expr, 488, $c:block) => {
        unroll!(@$v, $a, 244, $c);
        unroll!(@$v, $a + 244, 244, $c);
    };
    (@$v:ident, $a:expr, 489, $c:block) => {
        unroll!(@$v, $a, 488, $c);
        { const $v: usize = $a + 488; $c }
    };
    (@$v:ident, $a:expr, 490, $c:block) => {
        unroll!(@$v, $a, 245, $c);
        unroll!(@$v, $a + 245, 245, $c);
    };
    (@$v:ident, $a:expr, 491, $c:block) => {
        unroll!(@$v, $a, 490, $c);
        { const $v: usize = $a + 490; $c }
    };
    (@$v:ident, $a:expr, 492, $c:block) => {
        unroll!(@$v, $a, 246, $c);
        unroll!(@$v, $a + 246, 246, $c);
    };
    (@$v:ident, $a:expr, 493, $c:block) => {
        unroll!(@$v, $a, 492, $c);
        { const $v: usize = $a + 492; $c }
    };
    (@$v:ident, $a:expr, 494, $c:block) => {
        unroll!(@$v, $a, 247, $c);
        unroll!(@$v, $a + 247, 247, $c);
    };
    (@$v:ident, $a:expr, 495, $c:block) => {
        unroll!(@$v, $a, 494, $c);
        { const $v: usize = $a + 494; $c }
    };
    (@$v:ident, $a:expr, 496, $c:block) => {
        unroll!(@$v, $a, 248, $c);
        unroll!(@$v, $a + 248, 248, $c);
    };
    (@$v:ident, $a:expr, 497, $c:block) => {
        unroll!(@$v, $a, 496, $c);
        { const $v: usize = $a + 496; $c }
    };
    (@$v:ident, $a:expr, 498, $c:block) => {
        unroll!(@$v, $a, 249, $c);
        unroll!(@$v, $a + 249, 249, $c);
    };
    (@$v:ident, $a:expr, 499, $c:block) => {
        unroll!(@$v, $a, 498, $c);
        { const $v: usize = $a + 498; $c }
    };
    (@$v:ident, $a:expr, 500, $c:block) => {
        unroll!(@$v, $a, 250, $c);
        unroll!(@$v, $a + 250, 250, $c);
    };
    (@$v:ident, $a:expr, 501, $c:block) => {
        unroll!(@$v, $a, 500, $c);
        { const $v: usize = $a + 500; $c }
    };
    (@$v:ident, $a:expr, 502, $c:block) => {
        unroll!(@$v, $a, 251, $c);
        unroll!(@$v, $a + 251, 251, $c);
    };
    (@$v:ident, $a:expr, 503, $c:block) => {
        unroll!(@$v, $a, 502, $c);
        { const $v: usize = $a + 502; $c }
    };
    (@$v:ident, $a:expr, 504, $c:block) => {
        unroll!(@$v, $a, 252, $c);
        unroll!(@$v, $a + 252, 252, $c);
    };
    (@$v:ident, $a:expr, 505, $c:block) => {
        unroll!(@$v, $a, 504, $c);
        { const $v: usize = $a + 504; $c }
    };
    (@$v:ident, $a:expr, 506, $c:block) => {
        unroll!(@$v, $a, 253, $c);
        unroll!(@$v, $a + 253, 253, $c);
    };
    (@$v:ident, $a:expr, 507, $c:block) => {
        unroll!(@$v, $a, 506, $c);
        { const $v: usize = $a + 506; $c }
    };
    (@$v:ident, $a:expr, 508, $c:block) => {
        unroll!(@$v, $a, 254, $c);
        unroll!(@$v, $a + 254, 254, $c);
    };
    (@$v:ident, $a:expr, 509, $c:block) => {
        unroll!(@$v, $a, 508, $c);
        { const $v: usize = $a + 508; $c }
    };
    (@$v:ident, $a:expr, 510, $c:block) => {
        unroll!(@$v, $a, 255, $c);
        unroll!(@$v, $a + 255, 255, $c);
    };
    (@$v:ident, $a:expr, 511, $c:block) => {
        unroll!(@$v, $a, 510, $c);
        { const $v: usize = $a + 510; $c }
    };
    (@$v:ident, $a:expr, 512, $c:block) => {
        unroll!(@$v, $a, 256, $c);
        unroll!(@$v, $a + 256, 256, $c);
    };
}

#[test]
fn test() {
    {
        let a: Vec<usize> = vec![];
        unroll! {
            for i in 0..0 {
                a.push(i);
            }
        }
        assert_eq!(a, (0..0).collect::<Vec<usize>>());
    }
    {
        let mut a: Vec<usize> = vec![];
        unroll! {
            for i in 0..1 {
                a.push(i);
            }
        }
        assert_eq!(a, (0..1).collect::<Vec<usize>>());
    }
    {
        let mut a: Vec<usize> = vec![];
        unroll! {
            for i in 0..512 {
                a.push(i);
            }
        }
        assert_eq!(a, (0..512).collect::<Vec<usize>>());
    }
}

