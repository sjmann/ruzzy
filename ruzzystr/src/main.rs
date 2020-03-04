use ruzzystr;
use suffix_tree;

fn main() {
    suffix_tree::hello_tree();
    ruzzystr::levenshtein_distance("kitten", "sitting");
}
