use ndarray::parallel::prelude::*;
use ndarray::prelude::*;

pub fn maximize(
    data: ArrayView2<f64>,
    responsibilities: ArrayView2<f64>,
) -> (Array2<f64>, Array3<f64>, Array1<f64>) {
    {% if cookiecutter.include_sample_code %} 
    let k = if let [_n, k] = responsibilities.shape() {
        k
    } else {
        panic!()
    };
    let d = if let [_n, d] = data.shape() {
        d
    } else {
        panic!()
    };
    
    let sum_responsibilities = responsibilities.sum_axis(Axis(0));
    
    let means = (&responsibilities.slice(s![.., .., NewAxis]) * &data.slice(s![.., NewAxis, ..]))
    .sum_axis(Axis(0))
    / sum_responsibilities.slice(s![.., NewAxis]);
    
    // n x k x d
    let adjusted = &data.slice(s![.., NewAxis, ..]) - &means.slice(s![NewAxis, .., ..]);
    
    let mut covs = Array3::<f64>::zeros((*k, *d, *d));
    
    (
        adjusted.axis_iter(Axis(1)),
        covs.axis_iter_mut(Axis(0)),
        responsibilities.axis_iter(Axis(1)),
    )
    .into_par_iter()
    .for_each(|(x, mut cov, resp)| {
        let y = &x * &resp.slice(s![.., NewAxis]);
        cov += &x.t().dot(&y);
    });
    
    covs = &covs / &sum_responsibilities.slice(s![.., NewAxis, NewAxis]);
    
    let weights = &sum_responsibilities / sum_responsibilities.sum();
    
    (means, covs, weights)
    {% else %}
    todo!()
    {% endif %}
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray_npy::read_npy;

    #[test]
    fn test_maximize() {
        {% if cookiecutter.include_sample_code %} 
        let data: Array2<f64> = read_npy("../data/data.npy").unwrap();
        let responsibilities: Array2<f64> = read_npy("../data/responsibilities.npy").unwrap();
        let means: Array2<f64> = read_npy("../data/means.npy").unwrap();

        let (means_computed, _, _) = maximize(data.view(), responsibilities.view());
        assert!(!means_computed.abs_diff_eq(&means, 1e-1));
        {% endif %}
    }
}
