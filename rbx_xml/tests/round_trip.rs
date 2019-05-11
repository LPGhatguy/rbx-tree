use std::io::Cursor;

use log::info;

static TEST_MODELS: &[&str] = &[
    include_str!("../test-files/baseplate.rbxlx"),
    include_str!("../test-files/decals.rbxmx"),
    include_str!("../test-files/effects.rbxmx"),
    include_str!("../test-files/gui.rbxmx"),
    include_str!("../test-files/inf-and-nan.rbxmx"),
    include_str!("../test-files/numbers.rbxmx"),
    include_str!("../test-files/part-referent.rbxmx"),
    include_str!("../test-files/parts.rbxmx"),
    include_str!("../test-files/terrain.rbxmx"),
];

#[test]
fn round_trip() {
    let _ = env_logger::try_init();

    for (index, model_source) in TEST_MODELS.iter().enumerate() {
        info!("Decoding #{}...", index);
        let tree = rbx_xml::from_str_default(model_source).unwrap();
        let root_id = tree.get_root_id();

        info!("Encoding #{}...", index);
        let mut buffer = Vec::new();
        rbx_xml::to_writer_default(Cursor::new(&mut buffer), &tree, &[root_id]).unwrap();

        info!("Re-Decoding #{}...", index);
        rbx_xml::from_reader_default(buffer.as_slice()).unwrap();
    }
}