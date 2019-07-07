use std::ptr;

const REGSZ: usize = 4;

#[derive(Default)]
pub struct ascii_rawfilters {
    pub strings: Vec<String>,
    pub sources: Vec<usize>,
    pub region: Vec<Vec<u8>>,
    pub num_strings: i32,
}

pub fn decompose(predicates: Vec<String>) -> ascii_rawfilters {
    let mut num_ascii_rawfilters = 0;
    //let mut region_bytes = 0;

    for predicate in &predicates {
        let len = predicate.len();
        let possible_substrings = if len - REGSZ > 0 { len - REGSZ + 1 } else { 1 };
        num_ascii_rawfilters += possible_substrings + 1;
        //region_bytes += (possible_substrings * 5);
    }
    let mut sources = vec![];
    let mut i = 0;
    let mut region = vec![];
    let mut j = 0;
    while j < predicates.len() {
        let mut p = predicates.get(j).unwrap().clone();
        unsafe {
            let insert_data = p.as_bytes().clone();
            let mut result = Vec::with_capacity(insert_data.len());

            ptr::copy_nonoverlapping(insert_data.as_ptr(), result.as_mut_ptr(), insert_data.len());
            region.push(result);
            sources.push(j);
            i = i + 1;
        }
        let pred_length = predicates.get(j).unwrap().len();
        for start in 0..=pred_length - REGSZ {
            if pred_length == REGSZ && start == 0 {
                continue;
            }
            unsafe {
                let insert_data = p.as_bytes().get(start..start + REGSZ).unwrap();
                let mut result = Vec::with_capacity(insert_data.len());

                ptr::copy_nonoverlapping(
                    insert_data.as_ptr(),
                    result.as_mut_ptr(),
                    insert_data.len(),
                );
                region.push(result);
                sources.push(j);
                i = i + 1;
            }
        }
        j += 1;
    }
    return ascii_rawfilters {
        strings: predicates,
        sources: sources,
        region: region,
        num_strings: i,
    };
}
