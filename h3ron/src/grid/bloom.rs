use std::collections::HashMap;

use crate::index::ToIndex;
use crate::{FromH3Index, HasH3Index, Index};

pub struct BloomOutput<T> {
    pub h3indexes: Vec<T>,
    pub values: Vec<f32>,
}

// https://www.sciencedirect.com/topics/mathematics/logarithmic-curve
// https://en.wikipedia.org/wiki/Exponential_growth
// http://rovdownloads.com/blog/quick-overview-of-probability-distributions/
pub enum BloomGradientType {
    Linear,
    Exponential2,
}

/// how to aggregate values on indexes covered in multiple radius's
pub enum BloomAggregationMethod {
    Min,
    Max,
    Sum,
}

/// create a bloom effect (https://en.wikipedia.org/wiki/Bloom_(shader_effect)) around h3indexes
pub fn bloom<'a, I, T>(
    index_iter: I,
    radius: u32,
    gradient: BloomGradientType,
    aggregation_method: BloomAggregationMethod,
) -> BloomOutput<T>
where
    I: IntoIterator<Item = &'a T>,
    T: ToIndex + FromH3Index + 'a,
{
    let mut bloom_indexes: HashMap<Index, f32> = HashMap::new();

    for index_t in index_iter {
        let index: Index = index_t.to_index();
        for (ring_index_distance, ring_index) in index.k_ring_distances(0, radius) {
            let mut bloom_value = ring_index_distance as f32 / radius as f32; // TODO: gradient
            bloom_indexes
                .entry(ring_index)
                .and_modify(|value| match aggregation_method {
                    BloomAggregationMethod::Min => {
                        if value < &mut bloom_value {
                            *value = bloom_value;
                        }
                    }
                    BloomAggregationMethod::Max => {
                        if value > &mut bloom_value {
                            *value = bloom_value
                        }
                    }
                    BloomAggregationMethod::Sum => *value += bloom_value,
                })
                .or_insert(bloom_value);
        }
    }

    let mut out_indexes = Vec::with_capacity(bloom_indexes.len());
    let mut out_values = Vec::with_capacity(bloom_indexes.len());
    for (index, value) in bloom_indexes.drain() {
        out_indexes.push(T::from_h3index(index.h3index()));
        out_values.push(value);
    }
    BloomOutput {
        h3indexes: out_indexes,
        values: out_values,
    }
}

pub trait Bloom<T>
where
    T: ToIndex + FromH3Index,
{
    fn bloom(
        self,
        radius: u32,
        gradient: BloomGradientType,
        aggregation_method: BloomAggregationMethod,
    ) -> BloomOutput<T>;
}

impl<'a, I, T> Bloom<T> for I
where
    I: IntoIterator<Item = &'a T>,
    T: ToIndex + FromH3Index + 'a,
{
    fn bloom(
        self,
        radius: u32,
        gradient: BloomGradientType,
        aggregation_method: BloomAggregationMethod,
    ) -> BloomOutput<T> {
        bloom(self, radius, gradient, aggregation_method)
    }
}

/*
impl<T> Bloom<T> for &[T] where T: ToIndex + FromH3Index {
    fn bloom(&self, radius: u32, gradient: BloomGradientType, aggregation_method: BloomAggregationMethod) -> BloomOutput<T> {
        bloom(self, radius, gradient, aggregation_method)
    }
}

impl<T> Bloom<T> for Vec<T> where T: ToIndex + FromH3Index {
    fn bloom(&self, radius: u32, gradient: BloomGradientType, aggregation_method: BloomAggregationMethod) -> BloomOutput<T> {
        bloom(self, radius, gradient, aggregation_method)
    }
}
*/
#[cfg(test)]
mod tests {
    use crate::grid::bloom::{Bloom, BloomAggregationMethod, BloomGradientType};
    use crate::Index;

    #[test]
    fn vec_iter_boom() {
        let indexes: Vec<Index> = vec![0x89283080ddbffff_u64.into()];

        let out = indexes
            .iter()
            .bloom(10, BloomGradientType::Linear, BloomAggregationMethod::Max);
    }
}
