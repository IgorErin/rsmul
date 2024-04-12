use sprs::num_kinds::Pattern;

pub type TriMat = sprs::TriMatBase<Vec<u64>, Vec<Pattern>>;

pub type PatternMat = sprs::CsMatBase<Pattern, u64, Vec<u64>, Vec<u64>, Vec<Pattern>>;
pub type U8CSCMat = sprs::CsMatBase<u8, u64, Vec<u64>, Vec<u64>, Vec<u8>>;

pub fn muln(n: u32, mat: TriMat) -> PatternMat {
    let csc: U8CSCMat = mat.to_csc().map(|_| 1u8);
    let mut result: U8CSCMat = csc.clone();

    drop(mat);

    for _ in 0..n {
        result = &result * &csc;
    }

    result.map(|_| Pattern)
}
