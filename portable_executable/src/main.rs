use std::error::Error;
use exe::{ExportDirectory, PE};
use exe::pe::VecPE;
use exe::types::{CCharString, ImportData, ImportDirectory};

fn main() -> Result<(), Box<dyn Error>> {
    let source = r#"C:\Users\boni\Downloads\102 - Copy\compiled.exe"#;
    // let source = r#"C:\Users\boni\Downloads\102 - Copy\MicrosoftEdge_X64_116.0.1938.76.exe"#;

    let pefile = VecPE::from_disk_file(source).unwrap();
    let size = pefile.calculate_disk_size()?;
    let arch = pefile.get_arch()?;
    println!("size {}", size);
    println!("arch {:?}", arch);

    let data_directory_offset = pefile.get_data_directory_offset()?;
    println!("data_directory_offset {:?}", data_directory_offset);

    // let headers = pefile.get_valid_nt_headers_64()?;
    // let number_of_sections = headers.file_header.number_of_sections;
    // println!("number_of_sections {}", number_of_sections);

    let section_table = pefile.get_section_table()?;
    section_table.iter().for_each(|s| println!("{}", s.name.as_str().unwrap()));

    let directory = ExportDirectory::parse(&pefile);

    println!("directory {:?}",directory);


    let import_directory = ImportDirectory::parse(&pefile).unwrap();
    for descriptor in import_directory.descriptors {
        println!(
            "Module: {}",
            descriptor.get_name(&pefile).unwrap().as_str().unwrap()
        );
        println!("Imports:");

        for import in descriptor.get_imports(&pefile).unwrap() {
            match import {
                ImportData::Ordinal(x) => println!("   #{}", x),
                ImportData::ImportByName(s) => println!("   {}", s),
            }
        }
    }

    Ok(())
}
