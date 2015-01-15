use std::iter::repeat;
use std::iter::FromIterator;

fn zeros(len: usize) -> Vec<f32> {
    FromIterator::from_iter(repeat(0f32).take(len))
}

fn scale(a: f32, v: &Vec<f32>) -> Vec<f32> {
    let mut scaled_v = zeros(v.len());
    for i in (0..(*v).len()) {
        scaled_v[i] = a * (*v)[i];
    }
    scaled_v
}

fn sum(v1: &Vec<f32>, v2: &Vec<f32>) -> Vec<f32>{
    let mut asum: Vec<f32> = zeros((*v1).len());
    for i in (0..(*v1).len()) {
        asum[i] = (*v1)[i] + (*v2)[i];
    }
    asum
}

fn lincom(c: &Vec<f32>, v: &Vec<Vec<f32>>) -> Vec<f32> {
    let mut sum: Vec<f32> = zeros((*v)[0].len());
    let size = (*c).len();
    for i in (0..sum.len()) {
        let mut elem_sum: f32 = 0f32;
        for j in (0..size) {
            elem_sum += (*c)[j] * ((*v)[j])[i];
        }
        sum[i] = elem_sum;
    }
    sum
}

fn rk_iterate<F: Fn(f32, &Vec<f32>) -> Vec<f32>>(t: f32,
                                                 y: &Vec<f32>,
                                                 f: F,
                                                 h: f32,
                                                 a: &Vec<Vec<f32>>,
                                                 b: &Vec<f32>,
                                                 c: &Vec<f32>) -> Vec<f32> {
    
    let s: usize = (*c).len();
    let mut k: Vec<Vec<f32>> = FromIterator::from_iter((0..s).map(|&:x:usize| {zeros(y.len())}));

    for i in (0..s) {
        let kires = f(t + (*c)[i] * h, &sum(y, &scale(h, &lincom(&(a[i]), &k))));
        for j in (0..kires.len()) {
            (k[i])[j] = kires[j];
        }
    }
    sum(y,  &scale(h, &lincom(b, &k)))
}

fn myf(t: f32, y: &Vec<f32>) -> Vec<f32> {
    scale(-1.0f32, y)
}


#[test]
fn test_euler() {
    let y0 = vec![2f32, 2f32];
    let t0 = 0f32;
    let h = 0.1f32;
    let a = vec![vec![0f32]];
    let c = vec![0f32];
    let b = vec![1f32];

    let mut ynew = sum(&y0, &zeros(2));

    for i in (0..10) {
        ynew = rk_iterate(t0, &ynew, myf, h, &a, &b, &c);
    }
}
