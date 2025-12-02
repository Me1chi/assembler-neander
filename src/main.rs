use std::{
    env,
    fs
};

use assembler_neander::{
    encoder::assemble,
    metadata::{
        labelinfo::{
            immediatetrick::ImmediateAddressing,
            label::Label,
            LabelInfo
        },
        segment::SegInfo,
        to_lower_chop_comment,
        Metadata
    },
    utils::pipeline::Pipeline
};

fn main() -> std::io::Result<()> {

    let source_filename: String;
    let output_filename: String;

    //TODO: AQUI eh onde eu vou detectar primitivamente
    // as flags, DE MANEIRA BEM PRIMITIVA MESMO
    // MAS A ENTREGA DO TRABALHO EH NESSA QUINTA...

    let mut args = env::args();
    args.next(); // Descarta o nome do binario

    // First argument
    if let Some(filename) = args.next() {
        source_filename = filename;

    } else {
        source_filename = String::from("src.nad");
    }

    // Second argument
    if let Some(filename) = args.next() {
        output_filename = filename;

    } else {
        output_filename = String::from("output.mem");
    }

    // Third argument
    let target_sim = if let Some(target) = args.next() {
        target == "simulation"
    } else {
        false
    };

    // Fourth argument
    let extended = if let Some(ext) = args.next() {
        ext != "no-extension"
    } else {
        true
    };

    // Fifth argument
    let no_hex = if let Some(hex_mode) = args.next() {
        hex_mode == "no-hex"
    } else {
        false
    };


    // TODO: AQUI TEM QUE ARRUMAR O TARGET SIM PRA DETECTAR FLAG
    let mut metadata = Metadata::new(target_sim);
    let mut frontend = Pipeline::new();

    frontend.add(      to_lower_chop_comment);
    frontend.add(      SegInfo::resolve_seginfo);
    frontend.add(      Label::resolve_label_defs);
    frontend.add(      ImmediateAddressing::resolve_immediates);
    frontend.add(      LabelInfo::apply_labels);

    metadata.text = fs::read_to_string(source_filename)?;

    metadata = frontend.run(metadata)?;

    // Later I will create a flag so the user can see
    // the intermediate build file
    // fs::write("build", &metadata.text)?;

    let mem = assemble(metadata, extended);

    // Output file writing
    if no_hex {
        mem.to_output_file(&output_filename)?;

    } else {
        mem.to_altera_mif(&output_filename)?;
    }

    Ok(())
}
