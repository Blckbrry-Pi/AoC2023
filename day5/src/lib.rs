use range::{RangeMap, IdRange, union_list};

pub mod range;

#[derive(Debug, Clone)]
pub struct Mapper {
    seed_to_soil: Vec<RangeMap>,
    soil_to_fert: Vec<RangeMap>,
    fert_to_watr: Vec<RangeMap>,
    watr_to_lite: Vec<RangeMap>,
    lite_to_temp: Vec<RangeMap>,
    temp_to_humd: Vec<RangeMap>,
    humd_to_loc8: Vec<RangeMap>,
}

impl Mapper {
    fn parse_chunk(lines: &[impl AsRef<str>], starting_idx: usize) -> (usize, Vec<RangeMap>) {
        let mut maps = vec![];
        let mut idx = starting_idx;
        while idx < lines.len() && !lines[idx].as_ref().is_empty() {
            if lines[idx].as_ref().chars().next().unwrap().is_ascii_digit() {
                maps.push(RangeMap::parse(lines[idx].as_ref()));
            }
            idx += 1;
        }
        (idx+1, maps)
    }
    pub fn parse(lines: &[impl AsRef<str>]) -> Self {
        let idx = 0;
        let (idx, seed_to_soil) = Self::parse_chunk(lines, idx);
        let (idx, soil_to_fert) = Self::parse_chunk(lines, idx);
        let (idx, fert_to_watr) = Self::parse_chunk(lines, idx);
        let (idx, watr_to_lite) = Self::parse_chunk(lines, idx);
        let (idx, lite_to_temp) = Self::parse_chunk(lines, idx);
        let (idx, temp_to_humd) = Self::parse_chunk(lines, idx);
        let (_, humd_to_loc8) = Self::parse_chunk(lines, idx);

        Self {
            seed_to_soil,
            soil_to_fert,
            fert_to_watr,
            watr_to_lite,
            lite_to_temp,
            temp_to_humd,
            humd_to_loc8,
        }
    }

    pub fn get_val(&self, val: usize) -> usize {
        let mut val = val;
        for map in &self.seed_to_soil {
            if map.contains(val) {
                val = map.map_default(val);
                break;
            }
        }
        for map in &self.soil_to_fert {
            if map.contains(val) {
                val = map.map_default(val);
                break;
            }
        }
        for map in &self.fert_to_watr {
            if map.contains(val) {
                val = map.map_default(val);
                break;
            }
        }
        for map in &self.watr_to_lite {
            if map.contains(val) {
                val = map.map_default(val);
                break;
            }
        }
        for map in &self.lite_to_temp {
            if map.contains(val) {
                val = map.map_default(val);
                break;
            }
        }
        for map in &self.temp_to_humd {
            if map.contains(val) {
                val = map.map_default(val);
                break;
            }
        }
        for map in &self.humd_to_loc8 {
            if map.contains(val) {
                val = map.map_default(val);
                break;
            }
        }
        val
    }

    fn get_ranges_one_step(&self, range: &[IdRange], mappings: &[RangeMap]) -> Vec<IdRange> {
        let mut unmapped = range.to_vec();
        let mut mapped = vec![];
        
        for map in mappings {
            mapped.extend(unmapped.iter().copied().map(|unmapped_range| map.map_range(unmapped_range)));
            unmapped = unmapped.into_iter().flat_map(|unmapped_range| map.unmapped_range(unmapped_range)).collect::<Vec<_>>();

            mapped = union_list(&mapped);
            unmapped = union_list(&unmapped);
        }

        let output = unmapped.into_iter().chain(mapped).collect::<Vec<_>>();

        union_list(&output)
    }

    pub fn possibilities(&self, ranges: &[IdRange]) -> Vec<IdRange> {
        let ranges = ranges.to_vec();
        println!("seed: {ranges:#?}");
        let ranges = self.get_ranges_one_step(&ranges, &self.seed_to_soil);
        println!("soil: {ranges:#?}");
        let ranges = self.get_ranges_one_step(&ranges, &self.soil_to_fert);
        println!("fert: {ranges:#?}");
        let ranges = self.get_ranges_one_step(&ranges, &self.fert_to_watr);
        println!("watr: {ranges:#?}");
        let ranges = self.get_ranges_one_step(&ranges, &self.watr_to_lite);
        println!("lite: {ranges:#?}");
        let ranges = self.get_ranges_one_step(&ranges, &self.lite_to_temp);
        println!("temp: {ranges:#?}");
        let ranges = self.get_ranges_one_step(&ranges, &self.temp_to_humd);
        println!("humd: {ranges:#?}");
        let ranges = self.get_ranges_one_step(&ranges, &self.humd_to_loc8);
        println!("loc8: {ranges:#?}");

        ranges
    }
}

