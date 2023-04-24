use crate::jbeam;

pub fn write_beam(beam: &jbeam::JBeam) -> String {

    let mut export_string = "[".to_string();
    let mut modifier_string = ",{".to_string();

    // keep track of the number of modifiers

    let mut modifier_count = 0;
    
    // id1 and id2 are required arguments

    export_string = format!(r#"{} "{}", "{}" "#, export_string, beam.id1, beam.id2);


    // the rest of the fields are optional modifiers. In some cases it is best to not export anything.

    // beam type (|NORMAL)
    println!("{}", beam.beam_type);
    if beam.beam_type != "|NORMAL".to_string() {
        
        if beam.beam_type == "|SUPPORT".to_string() {
            modifier_string = format!(r#"{}, "beamType":"{}" "#, modifier_string, beam.beam_type);
            modifier_count += 1;
        }
    }

    // beam spring (4300000)

    if beam.beam_spring != 4300000.0 {
        modifier_string = format!(r#"{}, "beamSpring":{} "#, modifier_string, beam.beam_spring);
        modifier_count += 1;
    }

    // beam damp (580)

    if beam.beam_damp != 580.0 {
        modifier_string = format!(r#"{}, "beamDamp":{} "#, modifier_string, beam.beam_damp);
        modifier_count += 1;
    }

    // beam strength (FLT_MAX)

    if beam.beam_strength != "FLT_MAX".to_string() {
        // check if strength is a number

        let is_number = beam.beam_strength.parse::<f32>();

        if let Ok(number) = is_number {
            modifier_string = format!(r#"{}, "beamStrength":{} "#, modifier_string, number);
            modifier_count += 1;
        }
    }

    // beam deform (220000)

    if beam.beam_deform != 220000.0 {
        modifier_string = format!(r#"{}, "beamDeform":{} "#, modifier_string, beam.beam_deform);
        modifier_count += 1;
    }

    // beam precompression (1.0)

    if beam.beam_compression != 1.0 {
        modifier_string = format!(r#"{}, "beamPrecompression":{} "#, modifier_string, beam.beam_compression);
        modifier_count += 1;
    }

    // beam precompression range (0.0?)

    if beam.beam_compression_range != 0.0 {
        modifier_string = format!(r#"{}, "beamPrecompressionRange":{} "#, modifier_string, beam.beam_compression_range);
        modifier_count += 1;
    }

    // beam precompression time (0.0?)

    if beam.beam_compression_time != 0.0 {
        modifier_string = format!(r#"{}, "beamPrecompressionTime":{} "#, modifier_string, beam.beam_compression_time);
        modifier_count += 1;
    }

    // beam break group (empty string?)

    if !beam.break_group.is_empty() {
        modifier_string = format!(r#"{}, "breakGroup":"{}" "#, modifier_string, beam.break_group);
        modifier_count += 1;
    }

    // beam break group type (0?)

    if beam.break_group_type != "0".to_string() {
        // check if strength is a number

        let is_number = beam.break_group_type.parse::<u32>();

        if let Ok(number) = is_number {
            modifier_string = format!(r#"{}, "breakGroupType":{} "#, modifier_string, number);
            modifier_count += 1;
        }
    }

    // beam name (empty string?)

    if !beam.name.is_empty() {
        modifier_string = format!(r#"{}, "name":"{}" "#, modifier_string, beam.name);
        modifier_count += 1;
    }

    // beam damp cutoff hz (0.0?)

    if beam.damp_cutoff_hz != 0.0 {
        modifier_string = format!(r#"{}, "dampCutoffHz":{} "#, modifier_string, beam.damp_cutoff_hz);
        modifier_count += 1;
    }

    // beam deform limit (0.0?)

    if beam.deform_limit != 0.0 {
        modifier_string = format!(r#"{}, "deformLimit":{} "#, modifier_string, beam.deform_limit);
        modifier_count += 1;

    }

    // beam deform limit expansion (0.0?)

    if beam.deform_limt_expansion != 0.0 {
        modifier_string = format!(r#"{}, "deformLimitExpansion":{} "#, modifier_string, beam.deform_limt_expansion);
        modifier_count += 1;
        
    }

    // beam deform group (empty string?)

    if !beam.deform_group.is_empty() {
        modifier_string = format!(r#"{}, "deformGroup":"{}" "#, modifier_string, beam.deform_group);
        modifier_count += 1;
    }

    // beam deformation trigger ratio (0.0?)

    if beam.deformation_trigger_ratio != 0.0 {
        modifier_string = format!(r#"{}, "deformationTriggerRatio":{} "#, modifier_string, beam.deformation_trigger_ratio);
        modifier_count += 1;
    }



    // close the square brackets
    modifier_string = format!("{}}}", modifier_string);

    // only write the modifiers if there are any

    if modifier_count > 0 {
        export_string = format!("{} {} ]", export_string, modifier_string);
    } else {
        export_string = format!("{} ]", export_string);
    }

    
    
    export_string
}