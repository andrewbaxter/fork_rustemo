use rustemo::rustemo_mod;
use rustemo_tools::output_cmp;

use self::generic_tree::GenericTreeParser;

// Only parser, no actions are generated for generic builder.
rustemo_mod!(generic_tree, "/src/builder/generic_tree");

// ANCHOR: generic_tree
#[test]
fn generic_tree() {
    let result = GenericTreeParser::parse("a 42 a 3 b");
    output_cmp!(
        "src/builder/generic_tree/generic_tree.ast",
        format!("{:#?}", result)
    );
}
// ANCHOR_END: generic_tree