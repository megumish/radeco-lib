use serde_json;
use r2api::structs::LFunctionInfo;
use std::fs::File;
use std::io::prelude::*;

use middle::regfile::SubRegisterFile;
use frontend::ssaconstructor::SSAConstruct;
use middle::ssa::ssastorage::SSAStorage;

pub const REGISTER_PROFILE: &'static str = "test_files/x86_register_profile.json";
pub const BIN_LS_INSTRUCTIONS: &'static str = "test_files/bin_ls_instructions.json";
pub const CT1_INSTRUCTIONS: &'static str = "test_files/ct1_instructions.json";

pub fn new_ssa<S: Into<String>>(register_profile_path: S, instructions_path: S) -> SSAStorage{
        let instructions: LFunctionInfo;
        let mut register_profile = File::open(register_profile_path.into()).unwrap();
        let mut s = String::new();
        register_profile.read_to_string(&mut s).unwrap();
        let reg_profile = serde_json::from_str(&*s).unwrap();
        let mut instruction_file = File::open(instructions_path.into()).unwrap();
        let mut s = String::new();
        instruction_file.read_to_string(&mut s).unwrap();
        instructions = serde_json::from_str(&*s).unwrap();
        let mut ssa: SSAStorage = SSAStorage::new();
        {
            let regfile = SubRegisterFile::new(&reg_profile);
            let mut constructor = SSAConstruct::new(&mut ssa, &regfile);
            constructor.run(instructions.ops.unwrap().as_slice());
        }
        ssa
}
