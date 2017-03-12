use aliases::models::AliasFile;
use aliases::factories::AliasFactory;
use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fs::File;

pub struct AliasFileRepository;

impl AliasFileRepository {

    // TODO need to handle different users
    pub fn find(directory: &PathBuf) -> AliasFile {
        Self::create(directory);
        let aliases_filename = String::from(".aliases");
        let new_file = directory.join(&aliases_filename);
        let aliases = AliasFactory::create_from_file(new_file);
        AliasFile::new(directory.clone(), aliases.unwrap()) // TODO
    }

    pub fn create(directory: &PathBuf) {
        let aliases_filename = String::from(".aliases");
        if !Path::new(&directory.join(&aliases_filename)).exists() {
            let filepath = directory.join(&aliases_filename);
            let mut new_file = File::create(filepath).unwrap();
            let template_string = Self::template_string();
            let array = template_string.as_bytes();
            let _ = new_file.write_all(array);
        }

    }

    pub fn save(alias: AliasFile) {
        let mut new_file = File::create(&alias.path.join(".aliases")).unwrap(); // TODO need to check if it's for a user or not
        let _ = new_file.write_all(&alias.as_bytes());
    }

    fn template_string() -> String {
String::from("# This file is autogenerated by the aliases tool.
# For more info about aliases type `aliases --help`
# or visit https://github.com/sebglazebrook/aliases

#alias_name:
#  command: ./super_command.sh                         # required
#  confirm: true                                       # optional
#  confirmation_message: Are you sure you are sure??   # optional
#  conditional: /bin/true                              # optional
#  backout_seconds: 3                                  # optional
#  unit_test: '[ true = true ]'                        # optional
#  quiet: false                                        # optional
")
    }


}

