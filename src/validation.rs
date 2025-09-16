use validatrix::Accumulator;

use crate::MaybeNDim;

pub(crate) fn validate_ndims<'a, T: MaybeNDim + 'a>(
    accum: &mut Accumulator,
    expected: Option<usize>,
    dimensionals: impl IntoIterator<Item = &'a T> + 'a,
) -> usize {
    // iterator of idx, ndims
    let mut it = dimensionals
        .into_iter()
        .enumerate()
        .filter_map(|(idx, d)| d.maybe_ndim().map(|n| (idx, n)));
    let mut total = 0;
    let exp = match expected {
        Some(e) => e,
        None => {
            if let Some((_idx, n)) = it.next() {
                n
            } else {
                return total;
            }
        }
    };
    for (idx, n) in it {
        if n != exp {
            accum.add_failure(
                format!("inconsistent dimensionalty: got {n}D, expected {exp}D").into(),
                &[idx.into()],
            );
            total += 1;
        }
    }
    total
}
