/*!
# MSBWT v2
This library provides access to a rust-based implementation of Multi-String BWT (MSBWT) queries.
Currently, the BWT is assumed to be stored in the same numpy format as expected by the original `msbwt` tool.
It could have been generated by the built-in `msbwt2-build` tool or externally.

## Current BWT Types
* RleBWT - 
This is short for the Run-length encoded BWT, which stores the data in a compressed format identical to the `msbwt` numpy format.
FM-indices are built from the data after loading.
This structure is not dynamic, and must currently be loaded from disk before usage.
However, if your data is fixed, this is generally a faster structure.
* DynamicBWT -
This format is intended to allow for dynamic addition of strings to the structure at run-time.
This means it can be used for BWT construction (`msbwt2-build` uses this under the hood) or simply for adding more data to an existing BWT on the fly.
However, the dynamic ability comes at a run-time cost for queries
If you only need to query the BWT, it is recommended you use one of the fixed data structures.

## Examples
### Basic load and query
```
use msbwt2::msbwt_core::BWT;
use msbwt2::rle_bwt::RleBWT;
use msbwt2::string_util;
let mut bwt = RleBWT::new();
let filename: String = "test_data/two_string.npy".to_string();
bwt.load_numpy_file(&filename);
assert_eq!(bwt.count_kmer(&string_util::convert_stoi(&"ACGT")), 1);
```
### Creation from FASTX files and adding string dynamically
```
use msbwt2::dynamic_bwt::{create_from_fastx,DynamicBWT};
use msbwt2::msbwt_core::BWT;
use msbwt2::string_util;
let single_file = vec!["./test_data/two_string.fa"];
let mut bwt = create_from_fastx(&single_file, true).unwrap();
assert_eq!(bwt.count_kmer(&string_util::convert_stoi(&"$")), 2);
assert_eq!(bwt.count_kmer(&string_util::convert_stoi(&"ACGT")), 1);
//adds an identical sorted string
bwt.insert_string(&"ACGT", true);
assert_eq!(bwt.count_kmer(&string_util::convert_stoi(&"$")), 3);
assert_eq!(bwt.count_kmer(&string_util::convert_stoi(&"ACGT")), 2);
```
*/

/// TODO
pub mod binary_block;
/// TODO
pub mod binary_bplus_tree;
/// Contains the function for reformating a BWT string into the expected run-length format or numpy file
pub mod bwt_converter;
/// Contains helper functions related to BWT construction, primarily for testing purposes
pub mod bwt_util;
/// Contains the implementation of a dynamic BWT structure. Reads can be added to this BWT during run-time, so it is useful for construction of a new BWT.
pub mod dynamic_bwt;
/// Includes the trait for a multi-string BWT
pub mod msbwt_core;
/// This is the classic RLE implementation from the original msbwt package
pub mod rle_bwt;
/// Contains inline functions for converting between strings and integer formats
pub mod string_util;
/// Contains the implementation of a run-length encoded B+ tree
pub mod rle_bplus_tree;
/// Contains a block structure for capturing runs of data in a more succinct but dynamic format
pub mod run_block_av_flat;