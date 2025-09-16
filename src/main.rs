use std::fs;

use assembler_neander::{metadata::{labelinfo::{immediatetrick::ImmediateAddressing, label::Label, LabelInfo}, segment::SegInfo, to_lower_chop_comment, Metadata}, utils::pipeline::Pipeline};

fn main() -> std::io::Result<()> {

    // NEW TESTING
    let mut metadata = Metadata::new();
    let mut frontend = Pipeline::new();
    frontend.add(      to_lower_chop_comment);
    frontend.add(      SegInfo::resolve_seginfo);
    frontend.add(      Label::resolve_label_defs);
    frontend.add(      ImmediateAddressing::resolve_immediates);
    frontend.add(      LabelInfo::apply_labels);

    metadata.text = fs::read_to_string("arquivo_ajeitado.txt")?;

    metadata = frontend.run(metadata)?;

    fs::write("build", metadata.text)?;

    println!("{:?}", metadata.label_info); 
    println!("{:?}", metadata.seg_info);

    // END NEW TESTING


    // Output file writing
    metadata.mem_layout.to_output_file("output.mem")?;

    Ok(())
}
