#![feature(test)]
#![allow(unused_imports)]

extern crate test;
#[macro_use(s)]
extern crate ndarray;

use ndarray::{
    Array,
    Axis,
    Ix,
};
use ndarray::{arr0, arr1, arr2};

use test::black_box;

#[bench]
fn iter_sum_1d_regular(bench: &mut test::Bencher)
{
    let a = Array::<i32, _>::zeros(64 * 64);
    let a = black_box(a);
    bench.iter(|| {
        let mut sum = 0;
        for &elt in a.iter() {
            sum += elt;
        }
        sum
    });
}

#[bench]
fn iter_sum_1d_raw(bench: &mut test::Bencher)
{
    // this is autovectorized to death (= great performance)
    let a = Array::<i32, _>::zeros(64 * 64);
    let a = black_box(a);
    bench.iter(|| {
        let mut sum = 0;
        for &elt in a.as_slice_memory_order().unwrap() {
            sum += elt;
        }
        sum
    });
}

#[bench]
fn iter_sum_2d_regular(bench: &mut test::Bencher)
{
    let a = Array::<i32, _>::zeros((64, 64));
    let a = black_box(a);
    bench.iter(|| {
        let mut sum = 0;
        for &elt in a.iter() {
            sum += elt;
        }
        sum
    });
}

#[bench]
fn iter_sum_2d_by_row(bench: &mut test::Bencher)
{
    let a = Array::<i32, _>::zeros((64, 64));
    let a = black_box(a);
    bench.iter(|| {
        let mut sum = 0;
        for row in a.inner_iter() {
            for &elt in row {
                sum += elt;
            }
        }
        sum
    });
}

#[bench]
fn iter_sum_2d_raw(bench: &mut test::Bencher)
{
    // this is autovectorized to death (= great performance)
    let a = Array::<i32, _>::zeros((64, 64));
    let a = black_box(a);
    bench.iter(|| {
        let mut sum = 0;
        for &elt in a.as_slice_memory_order().unwrap() {
            sum += elt;
        }
        sum
    });
}

#[bench]
fn iter_sum_2d_cutout(bench: &mut test::Bencher)
{
    let a = Array::<i32, _>::zeros((66, 66));
    let av = a.slice(s![1..-1, 1..-1]);
    let a = black_box(av);
    bench.iter(|| {
        let mut sum = 0;
        for &elt in a.iter() {
            sum += elt;
        }
        sum
    });
}

#[bench]
fn iter_sum_2d_cutout_by_row(bench: &mut test::Bencher)
{
    let a = Array::<i32, _>::zeros((66, 66));
    let av = a.slice(s![1..-1, 1..-1]);
    let a = black_box(av);
    bench.iter(|| {
        let mut sum = 0;
        for row in 0..a.shape()[0] {
            for &elt in a.row(row) {
                sum += elt;
            }
        }
        sum
    });
}

#[bench]
fn iter_sum_2d_cutout_outer_iter(bench: &mut test::Bencher)
{
    let a = Array::<i32, _>::zeros((66, 66));
    let av = a.slice(s![1..-1, 1..-1]);
    let a = black_box(av);
    bench.iter(|| {
        let mut sum = 0;
        for row in a.inner_iter() {
            for &elt in row {
                sum += elt;
            }
        }
        sum
    });
}

#[bench]
fn iter_sum_2d_transpose_regular(bench: &mut test::Bencher)
{
    let mut a = Array::<i32, _>::zeros((64, 64));
    a.swap_axes(0, 1);
    let a = black_box(a);
    bench.iter(|| {
        let mut sum = 0;
        for &elt in a.iter() {
            sum += elt;
        }
        sum
    });
}

#[bench]
fn iter_sum_2d_transpose_by_row(bench: &mut test::Bencher)
{
    let mut a = Array::<i32, _>::zeros((64, 64));
    a.swap_axes(0, 1);
    let a = black_box(a);
    bench.iter(|| {
        let mut sum = 0;
        for row in 0..a.shape()[0] {
            for &elt in a.row(row) {
                sum += elt;
            }
        }
        sum
    });
}

#[bench]
fn scalar_sum_2d_regular(bench: &mut test::Bencher)
{
    let a = Array::<i32, _>::zeros((64, 64));
    let a = black_box(a);
    bench.iter(|| {
        a.scalar_sum()
    });
}

#[bench]
fn scalar_sum_2d_cutout(bench: &mut test::Bencher)
{
    let a = Array::<i32, _>::zeros((66, 66));
    let av = a.slice(s![1..-1, 1..-1]);
    let a = black_box(av);
    bench.iter(|| {
        a.scalar_sum()
    });
}

#[bench]
fn scalar_sum_2d_float(bench: &mut test::Bencher)
{
    let a = Array::<f32, _>::zeros((64, 64));
    let a = black_box(a.view());
    bench.iter(|| {
        a.scalar_sum()
    });
}

#[bench]
fn scalar_sum_2d_float_cutout(bench: &mut test::Bencher)
{
    let a = Array::<f32, _>::zeros((66, 66));
    let av = a.slice(s![1..-1, 1..-1]);
    let a = black_box(av);
    bench.iter(|| {
        a.scalar_sum()
    });
}

#[bench]
fn fold_sum_i32_2d_regular(bench: &mut test::Bencher)
{
    let a = Array::<i32, _>::zeros((64, 64));
    bench.iter(|| {
        a.fold(0, |acc, &x| acc + x)
    });
}

#[bench]
fn fold_sum_i32_2d_cutout(bench: &mut test::Bencher)
{
    let a = Array::<i32, _>::zeros((66, 66));
    let av = a.slice(s![1..-1, 1..-1]);
    let a = black_box(av);
    bench.iter(|| {
        a.fold(0, |acc, &x| acc + x)
    });
}

#[bench]
fn fold_sum_i32_2d_stride(bench: &mut test::Bencher)
{
    let a = Array::<i32, _>::zeros((64, 128));
    let av = a.slice(s![.., ..;2]);
    let a = black_box(av);
    bench.iter(|| {
        a.fold(0, |acc, &x| acc + x)
    });
}

#[bench]
fn add_2d_regular(bench: &mut test::Bencher)
{
    let mut a = Array::<i32, _>::zeros((64, 64));
    let b = Array::<i32, _>::zeros((64, 64));
    let bv = b.view();
    bench.iter(|| {
        a += &bv;
    });
}

#[bench]
fn add_2d_assign_ops(bench: &mut test::Bencher)
{
    let mut a = Array::<i32, _>::zeros((64, 64));
    let b = Array::<i32, _>::zeros((64, 64));
    let bv = b.view();
    bench.iter(|| {
        let mut x = a.view_mut();
        x += &bv;
        black_box(x);
    });
}

#[bench]
fn add_2d_cutout(bench: &mut test::Bencher)
{
    let mut a = Array::<i32, _>::zeros((66, 66));
    let mut acut = a.slice_mut(s![1..-1, 1..-1]);
    let b = Array::<i32, _>::zeros((64, 64));
    let bv = b.view();
    bench.iter(|| {
        acut += &bv;
    });
}

#[bench]
fn add_2d_broadcast_1_to_2(bench: &mut test::Bencher)
{
    let mut a = Array::<i32, _>::zeros((64, 64));
    let b = Array::<i32, _>::zeros(64);
    let bv = b.view();
    bench.iter(|| {
        a += &bv;
    });
}

#[bench]
fn add_2d_broadcast_0_to_2(bench: &mut test::Bencher)
{
    let mut a = Array::<i32, _>::zeros((64, 64));
    let b = Array::<i32, _>::zeros(());
    let bv = b.view();
    bench.iter(|| {
        a += &bv;
    });
}

#[bench]
fn scalar_toowned(bench: &mut test::Bencher) {
    let a = Array::<f32, _>::zeros((64, 64));
    bench.iter(|| {
        a.to_owned()
    });
}

#[bench]
fn scalar_add_1(bench: &mut test::Bencher) {
    let a = Array::<f32, _>::zeros((64, 64));
    let n = 1.;
    bench.iter(|| {
        &a + n
    });
}

#[bench]
fn scalar_add_2(bench: &mut test::Bencher) {
    let a = Array::<f32, _>::zeros((64, 64));
    let n = 1.;
    bench.iter(|| {
        n + &a
    });
}

#[bench]
fn scalar_sub_1(bench: &mut test::Bencher) {
    let a = Array::<f32, _>::zeros((64, 64));
    let n = 1.;
    bench.iter(|| {
        &a - n
    });
}

#[bench]
fn scalar_sub_2(bench: &mut test::Bencher) {
    let a = Array::<f32, _>::zeros((64, 64));
    let n = 1.;
    bench.iter(|| {
        n - &a
    });
}


// This is for comparison with add_2d_broadcast_0_to_2
#[bench]
fn add_2d_0_to_2_iadd_scalar(bench: &mut test::Bencher)
{
    let mut a = Array::<i32, _>::zeros((64, 64));
    let n = black_box(0);
    bench.iter(|| {
        a += n;
    });
}

#[bench]
fn add_2d_strided(bench: &mut test::Bencher)
{
    let mut a = Array::<i32, _>::zeros((64, 128));
    let mut a = a.slice_mut(s![.., ..;2]);
    let b = Array::<i32, _>::zeros((64, 64));
    let bv = b.view();
    bench.iter(|| {
        a += &bv;
    });
}

#[bench]
fn add_2d_transposed(bench: &mut test::Bencher)
{
    let mut a = Array::<i32, _>::zeros((64, 64));
    a.swap_axes(0, 1);
    let b = Array::<i32, _>::zeros((64, 64));
    let bv = b.view();
    bench.iter(|| {
        a += &bv;
    });
}

#[bench]
fn add_2d_f32_regular(bench: &mut test::Bencher)
{
    let mut a = Array::<f32, _>::zeros((64, 64));
    let b = Array::<f32, _>::zeros((64, 64));
    let bv = b.view();
    bench.iter(|| {
        a += &bv;
    });
}

const ADD1D_SIZE: usize = 64 * 64;

#[bench]
fn add_1d_regular(bench: &mut test::Bencher)
{
    let mut a = Array::<f32, _>::zeros(ADD1D_SIZE);
    let b = Array::<f32, _>::zeros(a.dim());
    bench.iter(|| {
        a += &b;
    });
}

#[bench]
fn add_1d_strided(bench: &mut test::Bencher)
{
    let mut a = Array::<f32, _>::zeros(ADD1D_SIZE * 2);
    let mut av = a.slice_mut(s![..;2]);
    let b = Array::<f32, _>::zeros(av.dim());
    bench.iter(|| {
        av += &b;
    });
}

#[bench]
fn scaled_add_2d_f32_regular(bench: &mut test::Bencher)
{
    let mut av = Array::<f32, _>::zeros((64, 64));
    let bv = Array::<f32, _>::zeros((64, 64));
    let scalar = 3.1415926535;
    bench.iter(|| {
        av.scaled_add(scalar, &bv);
    });
}

#[bench]
fn assign_scalar_2d_corder(bench: &mut test::Bencher)
{
    let a = Array::zeros((64, 64));
    let mut a = black_box(a);
    let s = 3.;
    bench.iter(move || a.assign_scalar(&s))
}

#[bench]
fn assign_scalar_2d_cutout(bench: &mut test::Bencher)
{
    let mut a = Array::zeros((66, 66));
    let a = a.slice_mut(s![1..-1, 1..-1]);
    let mut a = black_box(a);
    let s = 3.;
    bench.iter(move || a.assign_scalar(&s))
}

#[bench]
fn assign_scalar_2d_forder(bench: &mut test::Bencher)
{
    let mut a = Array::zeros((64, 64));
    a.swap_axes(0, 1);
    let mut a = black_box(a);
    let s = 3.;
    bench.iter(move || a.assign_scalar(&s))
}

#[bench]
fn assign_zero_2d_corder(bench: &mut test::Bencher)
{
    let a = Array::zeros((64, 64));
    let mut a = black_box(a);
    bench.iter(|| a.assign_scalar(&0.))
}

#[bench]
fn assign_zero_2d_cutout(bench: &mut test::Bencher)
{
    let mut a = Array::zeros((66, 66));
    let a = a.slice_mut(s![1..-1, 1..-1]);
    let mut a = black_box(a);
    bench.iter(|| a.assign_scalar(&0.))
}

#[bench]
fn assign_zero_2d_forder(bench: &mut test::Bencher)
{
    let mut a = Array::zeros((64, 64));
    a.swap_axes(0, 1);
    let mut a = black_box(a);
    bench.iter(|| a.assign_scalar(&0.))
}

#[bench]
fn bench_iter_diag(bench: &mut test::Bencher)
{
    let a = Array::<f32, _>::zeros((1024, 1024));
    bench.iter(|| for elt in a.diag() { black_box(elt); })
}

#[bench]
fn bench_row_iter(bench: &mut test::Bencher)
{
    let a = Array::<f32, _>::zeros((1024, 1024));
    let it = a.row(17);
    bench.iter(|| for elt in it.clone() { black_box(elt); })
}

#[bench]
fn bench_col_iter(bench: &mut test::Bencher)
{
    let a = Array::<f32, _>::zeros((1024, 1024));
    let it = a.column(17);
    bench.iter(|| for elt in it.clone() { black_box(elt); })
}

macro_rules! mat_mul {
    ($modname:ident, $ty:ident, $(($name:ident, $m:expr, $n:expr, $k:expr))+) => {
        mod $modname {
            use test::{black_box, Bencher};
            use ndarray::Array;
            $(
            #[bench]
            fn $name(bench: &mut Bencher)
            {
                let a = Array::<$ty, _>::zeros(($m, $n));
                let b = Array::<$ty, _>::zeros(($n, $k));
                let a = black_box(a.view());
                let b = black_box(b.view());
                bench.iter(|| a.dot(&b));
            }
            )+
        }
    }
}

mat_mul!{mat_mul_f32, f32,
    (m004, 4, 4, 4)
    (m007, 7, 7, 7)
    (m008, 8, 8, 8)
    (m012, 12, 12, 12)
    (m016, 16, 16, 16)
    (m032, 32, 32, 32)
    (m064, 64, 64, 64)
    (m127, 127, 127, 127)
    (mix16x4, 32, 4, 32)
    (mix32x2, 32, 2, 32)
    (mix10000, 128, 10000, 128)
}

mat_mul!{mat_mul_f64, f64,
    (m004, 4, 4, 4)
    (m007, 7, 7, 7)
    (m008, 8, 8, 8)
    (m012, 12, 12, 12)
    (m016, 16, 16, 16)
    (m032, 32, 32, 32)
    (m064, 64, 64, 64)
    (m127, 127, 127, 127)
    (mix16x4, 32, 4, 32)
    (mix32x2, 32, 2, 32)
    (mix10000, 128, 10000, 128)
}

mat_mul!{mat_mul_i32, i32,
    (m004, 4, 4, 4)
    (m007, 7, 7, 7)
    (m008, 8, 8, 8)
    (m012, 12, 12, 12)
    (m016, 16, 16, 16)
    (m032, 32, 32, 32)
    (m064, 64, 64, 64)
    (m127, 127, 127, 127)
}

#[bench]
fn create_iter_4d(bench: &mut test::Bencher)
{
    let mut a = Array::from_elem((4, 5, 3, 2), 1.0);
    a.swap_axes(0, 1);
    a.swap_axes(2, 1);
    let v = black_box(a.view());

    bench.iter(|| {
        v.into_iter()
    });
}

#[bench]
fn bench_to_owned_n(bench: &mut test::Bencher)
{
    let a = Array::<f32, _>::zeros((32, 32));
    bench.iter(|| a.to_owned());
}

#[bench]
fn bench_to_owned_t(bench: &mut test::Bencher)
{
    let mut a = Array::<f32, _>::zeros((32, 32));
    a.swap_axes(0, 1);
    bench.iter(|| a.to_owned());
}

#[bench]
fn equality_i32(bench: &mut test::Bencher)
{
    let a = Array::<i32, _>::zeros((64, 64));
    let b = Array::<i32, _>::zeros((64, 64));
    bench.iter(|| a == b);
}

#[bench]
fn equality_f32(bench: &mut test::Bencher)
{
    let a = Array::<f32, _>::zeros((64, 64));
    let b = Array::<f32, _>::zeros((64, 64));
    bench.iter(|| a == b);
}

#[bench]
fn dot_f32_16(bench: &mut test::Bencher)
{
    let a = Array::<f32, _>::zeros(16);
    let b = Array::<f32, _>::zeros(16);
    bench.iter(|| a.dot(&b));
}

#[bench]
fn dot_f32_20(bench: &mut test::Bencher)
{
    let a = Array::<f32, _>::zeros(20);
    let b = Array::<f32, _>::zeros(20);
    bench.iter(|| a.dot(&b));
}

#[bench]
fn dot_f32_32(bench: &mut test::Bencher)
{
    let a = Array::<f32, _>::zeros(32);
    let b = Array::<f32, _>::zeros(32);
    bench.iter(|| a.dot(&b));
}

#[bench]
fn dot_f32_256(bench: &mut test::Bencher)
{
    let a = Array::<f32, _>::zeros(256);
    let b = Array::<f32, _>::zeros(256);
    bench.iter(|| a.dot(&b));
}

#[bench]
fn dot_f32_1024(bench: &mut test::Bencher)
{
    let av = Array::<f32, _>::zeros(1024);
    let bv = Array::<f32, _>::zeros(1024);
    bench.iter(|| {
        av.dot(&bv)
    });
}

#[bench]
fn dot_f32_10e6(bench: &mut test::Bencher)
{
    let n = 1_000_000;
    let av = Array::<f32, _>::zeros(n);
    let bv = Array::<f32, _>::zeros(n);
    bench.iter(|| {
        av.dot(&bv)
    });
}

#[bench]
fn dot_extended(bench: &mut test::Bencher) {
    let m = 10;
    let n = 33;
    let k = 10;
    let av = Array::<f32, _>::zeros((m, n));
    let bv = Array::<f32, _>::zeros((n, k));
    let mut res = Array::<f32, _>::zeros((m, k));
    // make a manual simple matrix multiply to test
    bench.iter(|| {
        for i in 0..m {
            for j in 0..k {
                unsafe {
                    *res.uget_mut((i, j)) = av.row(i).dot(&bv.column(j));
                }
            }
        }
    })
}

const MEAN_SUM_N: usize = 127;

fn range_mat(m: Ix, n: Ix) -> Array<f32, (Ix, Ix)> {
    assert!(m * n != 0);
    Array::linspace(0., (m * n - 1) as f32, m * n).into_shape((m, n)).unwrap()
}

#[bench]
fn mean_axis0(bench: &mut test::Bencher) {
    let a = range_mat(MEAN_SUM_N, MEAN_SUM_N);
    bench.iter(|| a.mean(Axis(0)));
}

#[bench]
fn mean_axis1(bench: &mut test::Bencher) {
    let a = range_mat(MEAN_SUM_N, MEAN_SUM_N);
    bench.iter(|| a.mean(Axis(1)));
}

#[bench]
fn sum_axis0(bench: &mut test::Bencher) {
    let a = range_mat(MEAN_SUM_N, MEAN_SUM_N);
    bench.iter(|| a.sum(Axis(0)));
}

#[bench]
fn sum_axis1(bench: &mut test::Bencher) {
    let a = range_mat(MEAN_SUM_N, MEAN_SUM_N);
    bench.iter(|| a.sum(Axis(1)));
}
