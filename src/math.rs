use num_traits::int::PrimInt;

pub fn modulo<N: PrimInt>(a: N, b: N) -> N {
    ((a % b) + b) % b
}
