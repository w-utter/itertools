#![feature(test)]

extern crate test;
extern crate itertools;

use test::{black_box};
use itertools::Stride;
use itertools::Itertools;

#[cfg(feature = "unstable")]
use itertools::{ZipTrusted};

use itertools::ZipSlices;

use std::iter::repeat;
use std::cmp;
use std::cmp::Ordering;

#[bench]
fn slice_iter(b: &mut test::Bencher)
{
    let xs: Vec<_> = repeat(1i32).take(20).collect();
    b.iter(|| for elt in xs.iter() {
        test::black_box(elt);
    })
}

#[bench]
fn slice_iter_rev(b: &mut test::Bencher)
{
    let xs: Vec<_> = repeat(1i32).take(20).collect();
    b.iter(|| for elt in xs.iter().rev() {
        test::black_box(elt);
    })
}

#[bench]
fn stride_iter(b: &mut test::Bencher)
{
    let xs: Vec<_> = repeat(1i32).take(20).collect();
    b.iter(|| for elt in Stride::from_slice(&xs, 1) {
        test::black_box(elt);
    })
}

#[bench]
fn stride_iter_rev(b: &mut test::Bencher)
{
    let xs: Vec<_> = repeat(1i32).take(20).collect();
    b.iter(|| for elt in Stride::from_slice(&xs, 1).rev() {
        test::black_box(elt);
    })
}

#[bench]
fn zip_default_zip(b: &mut test::Bencher)
{
    let xs = vec![0; 1024];
    let ys = vec![0; 768];
    let xs = black_box(xs);
    let ys = black_box(ys);

    b.iter(|| {
        for (&x, &y) in xs.iter().zip(&ys) {
            test::black_box(x);
            test::black_box(y);
        }
    })
}

#[bench]
fn zip_default_zip3(b: &mut test::Bencher)
{
    let xs = vec![0; 1024];
    let ys = vec![0; 768];
    let zs = vec![0; 766];
    let xs = black_box(xs);
    let ys = black_box(ys);
    let zs = black_box(zs);

    b.iter(|| {
        for ((&x, &y), &z) in xs.iter().zip(&ys).zip(&zs) {
            test::black_box(x);
            test::black_box(y);
            test::black_box(z);
        }
    })
}

/*
#[bench]
fn zip_slices_ziptuple(b: &mut test::Bencher)
{
    let xs = vec![0; 1024];
    let ys = vec![0; 768];

    b.iter(|| {
        let xs = black_box(&xs);
        let ys = black_box(&ys);
        for (&x, &y) in Zip::new((xs, ys)) {
            test::black_box(x);
            test::black_box(y);
        }
    })
}
*/

#[bench]
fn zip_slices(b: &mut test::Bencher)
{
    let xs = vec![0; 1024];
    let ys = vec![0; 768];
    let xs = black_box(xs);
    let ys = black_box(ys);

    b.iter(|| {
        for (&x, &y) in ZipSlices::new(&xs, &ys) {
            test::black_box(x);
            test::black_box(y);
        }
    })
}

#[bench]
fn zip_slices_mut(b: &mut test::Bencher)
{
    let xs = vec![0; 1024];
    let ys = vec![0; 768];
    let xs = black_box(xs);
    let mut ys = black_box(ys);

    b.iter(|| {
        for (&x, &mut y) in ZipSlices::from_slices(&xs[..], &mut ys[..]) {
            test::black_box(x);
            test::black_box(y);
        }
    })
}

#[cfg(feature = "unstable")]
#[bench]
fn ziptrusted(b: &mut test::Bencher)
{
    let xs = vec![0; 1024];
    let ys = vec![0; 768];
    let xs = black_box(xs);
    let ys = black_box(ys);

    b.iter(|| {
        for (&x, &y) in ZipTrusted::new((xs.iter(), ys.iter())) {
            test::black_box(x);
            test::black_box(y);
        }
    })
}

#[cfg(feature = "unstable")]
#[bench]
fn ziptrusted3(b: &mut test::Bencher)
{
    let xs = vec![0; 1024];
    let ys = vec![0; 768];
    let zs = vec![0; 766];
    let xs = black_box(xs);
    let ys = black_box(ys);
    let zs = black_box(zs);

    b.iter(|| {
        for (&x, &y, &z) in ZipTrusted::new((xs.iter(), ys.iter(), zs.iter())) {
            test::black_box(x);
            test::black_box(y);
            test::black_box(z);
        }
    })
}

#[bench]
fn zip_checked_counted_loop(b: &mut test::Bencher)
{
    let xs = vec![0; 1024];
    let ys = vec![0; 768];
    let xs = black_box(xs);
    let ys = black_box(ys);

    b.iter(|| {
        // Must slice to equal lengths, and then bounds checks are eliminated!
        let len = cmp::min(xs.len(), ys.len());
        let xs = &xs[..len];
        let ys = &ys[..len];

        for i in 0..len {
            let x = xs[i];
            let y = ys[i];
            test::black_box(x);
            test::black_box(y);
        }
    })
}

#[bench]
fn zip_unchecked_counted_loop(b: &mut test::Bencher)
{
    let xs = vec![0; 1024];
    let ys = vec![0; 768];
    let xs = black_box(xs);
    let ys = black_box(ys);

    b.iter(|| {
        let len = cmp::min(xs.len(), ys.len());
        for i in 0..len {
            unsafe {
            let x = *xs.get_unchecked(i);
            let y = *ys.get_unchecked(i);
            test::black_box(x);
            test::black_box(y);
            }
        }
    })
}

#[bench]
fn zip_unchecked_counted_loop3(b: &mut test::Bencher)
{
    let xs = vec![0; 1024];
    let ys = vec![0; 768];
    let zs = vec![0; 766];
    let xs = black_box(xs);
    let ys = black_box(ys);
    let zs = black_box(zs);

    b.iter(|| {
        let len = cmp::min(xs.len(), cmp::min(ys.len(), zs.len()));
        for i in 0..len {
            unsafe {
            let x = *xs.get_unchecked(i);
            let y = *ys.get_unchecked(i);
            let z = *zs.get_unchecked(i);
            test::black_box(x);
            test::black_box(y);
            test::black_box(z);
            }
        }
    })
}

#[bench]
fn group_by_lazy_1(b: &mut test::Bencher) {
    let mut data = vec![0; 1024];
    for (index, elt) in data.iter_mut().enumerate() {
        *elt = index / 10;
    }

    let data = test::black_box(data);

    b.iter(|| {
        for (_key, group) in &data.iter().group_by_lazy(|elt| **elt) {
            for elt in group {
                test::black_box(elt);
            }
        }
    })
}

#[bench]
fn group_by_lazy_2(b: &mut test::Bencher) {
    let mut data = vec![0; 1024];
    for (index, elt) in data.iter_mut().enumerate() {
        *elt = index / 2;
    }

    let data = test::black_box(data);

    b.iter(|| {
        for (_key, group) in &data.iter().group_by_lazy(|elt| **elt) {
            for elt in group {
                test::black_box(elt);
            }
        }
    })
}

#[bench]
fn slice_chunks(b: &mut test::Bencher) {
    let data = vec![0; 1024];

    let data = test::black_box(data);
    let sz = test::black_box(10);

    b.iter(|| {
        for group in data.chunks(sz) {
            for elt in group {
                test::black_box(elt);
            }
        }
    })
}

#[bench]
fn chunks_lazy_1(b: &mut test::Bencher) {
    let data = vec![0; 1024];

    let data = test::black_box(data);
    let sz = test::black_box(10);

    b.iter(|| {
        for group in &data.iter().chunks_lazy(sz) {
            for elt in group {
                test::black_box(elt);
            }
        }
    })
}

#[bench]
fn equal(b: &mut test::Bencher) {
    let data = vec![7; 1024];
    let l = data.len();
    let alpha = test::black_box(&data[1..]);
    let beta = test::black_box(&data[..l - 1]);
    b.iter(|| {
        itertools::equal(alpha, beta)
    })
}

#[bench]
fn merge_default(b: &mut test::Bencher) {
    let mut data1 = vec![0; 1024];
    let mut data2 = vec![0; 800];
    let mut x = 0;
    for (_, elt) in data1.iter_mut().enumerate() {
        *elt = x;
        x += 1;
    }

    let mut y = 0;
    for (i, elt) in data2.iter_mut().enumerate() {
        *elt += y;
        if i % 3 == 0 {
            y += 3;
        } else {
            y += 0;
        }
    }
    let data1 = test::black_box(data1);
    let data2 = test::black_box(data2);
    b.iter(|| {
        data1.iter().merge(&data2).count()
    })
}

#[bench]
fn merge_by_cmp(b: &mut test::Bencher) {
    let mut data1 = vec![0; 1024];
    let mut data2 = vec![0; 800];
    let mut x = 0;
    for (_, elt) in data1.iter_mut().enumerate() {
        *elt = x;
        x += 1;
    }

    let mut y = 0;
    for (i, elt) in data2.iter_mut().enumerate() {
        *elt += y;
        if i % 3 == 0 {
            y += 3;
        } else {
            y += 0;
        }
    }
    let data1 = test::black_box(data1);
    let data2 = test::black_box(data2);
    b.iter(|| {
        data1.iter().merge_by(&data2, Ord::cmp).count()
    })
}

#[bench]
fn merge_by_lt(b: &mut test::Bencher) {
    let mut data1 = vec![0; 1024];
    let mut data2 = vec![0; 800];
    let mut x = 0;
    for (_, elt) in data1.iter_mut().enumerate() {
        *elt = x;
        x += 1;
    }

    let mut y = 0;
    for (i, elt) in data2.iter_mut().enumerate() {
        *elt += y;
        if i % 3 == 0 {
            y += 3;
        } else {
            y += 0;
        }
    }
    let data1 = test::black_box(data1);
    let data2 = test::black_box(data2);
    b.iter(|| {
        data1.iter().merge_by(&data2, |a, b| if a <= b { Ordering::Less } else { Ordering::Greater }).count()
    })
}
