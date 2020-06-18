use std::fmt::Debug;
mod b_rand;
pub fn pivot<T: PartialOrd>(v: &mut [T]) -> usize {
    let mut p = b_rand::rand(v.len());
    v.swap(p, 0);
    p = 0;
    for i in 1..v.len() {
        if v[i] < v[p] {
            // move our pivot forward 1, and put this element before it
            v.swap(p + 1, i);
            v.swap(p, p + 1);
            p += 1
        }
    }
    p
}

pub fn quick_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    println!("{:?}", v);
    let (a, b) = v.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]);
}

struct RawSend<T>(*mut [T]);
unsafe impl<T> Send for RawSend<T> {}

pub fn threaded_quick_sort<T: 'static + PartialOrd + Debug + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    println!("{:?}", v);
    let (a, b) = v.split_at_mut(p);
    let raw_a: *mut [T] = a as *mut[T];
    let raw_s = RawSend(raw_a);
    unsafe {
        let handle = std::thread::spawn(move || {
            threaded_quick_sort(&mut *raw_s.0);
        });
        threaded_quick_sort(b);
        handle.join().ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        let p = pivot(&mut v);
        for x in 0..v.len() {
            assert!((v[x] < v[p]) == (x < p));
        }
    }
}
