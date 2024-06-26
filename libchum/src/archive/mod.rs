pub mod data;
pub mod nametable;

pub use data::{TotemArchive, TotemFile, TotemHeader};

use crate::format;
use crate::util;
use std::collections::{HashMap, HashSet};
use std::io::{self, Read, Write};
use thiserror::Error;

/// Complete Chum archive.
/// This is literally all the data that matters.
pub struct ChumArchive {
    header: data::TotemHeader,
    names: HashMap<i32, String>, // a separate HashMap used to check for name collisions
    files: HashMap<i32, ChumFile>,
    format: format::TotemFormat,
}

impl Default for ChumArchive {
    fn default() -> ChumArchive {
        ChumArchive {
            header: data::TotemHeader::new(b"Legally distinct legal notice."),
            names: HashMap::new(),
            files: HashMap::new(),
            format: format::TotemFormat::NGC,
        }
    }
}

/// A ChumFile that is returned by the Chum Archive
pub struct ChumFile {
    data: Vec<u8>,
    type_id: String,
    name_id: String,
    subtype_id: String,
}

impl ChumFile {
    /// Create a new ChumFile
    pub fn new(data: Vec<u8>, nameid: String, typeid: String, subtypeid: String) -> ChumFile {
        ChumFile {
            data,
            type_id: typeid,
            name_id: nameid,
            subtype_id: subtypeid,
        }
    }

    /// Get the file's data
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    /// Get the file's data as a mutable vector
    pub fn get_data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    /// Get the file's name
    pub fn get_name_id(&self) -> &str {
        &self.name_id
    }

    /// Get the file's type
    pub fn get_type_id(&self) -> &str {
        &self.type_id
    }

    /// Get the file's subtype
    pub fn get_subtype_id(&self) -> &str {
        &self.subtype_id
    }

    /// Replace this file's data
    pub fn replace_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }
}

#[derive(Error, Debug)]
/// Error type for loading Chum files
pub enum ChumError {
    /// A name does not exist in the name table for this file
    #[error("Could not find ID {id} in NGC archive")]
    NameMissingError { id: i32 },
    /// Two strings share the same CRC32 hash
    #[error("Names {existing_name} and {new_name} have colliding ID {id}")]
    NameCollisionError {
        id: i32,
        existing_name: String,
        new_name: String,
    },
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
}

impl ChumArchive {
    /// Create a new, empty chum archive
    pub fn new(header: data::TotemHeader, fmt: format::TotemFormat) -> ChumArchive {
        ChumArchive {
            header,
            files: HashMap::new(),
            names: HashMap::new(),
            format: fmt,
        }
    }

    /// Create a new Chum archive
    pub fn new_from_files(
        header: data::TotemHeader,
        fmt: format::TotemFormat,
        files: Vec<ChumFile>,
    ) -> Result<ChumArchive, ChumError> {
        let mut archive = ChumArchive::new(header, fmt);
        // check add to name map
        for file in &files {
            let hashname = archive.check_can_add_id(file.get_name_id())?;
            let hashtype = archive.check_can_add_id(file.get_type_id())?;
            let hashsubtype = archive.check_can_add_id(file.get_subtype_id())?;
            // Add names if they don't already exist
            if let Some(i) = hashname {
                archive.names.insert(i, file.get_name_id().into());
            }
            if let Some(i) = hashtype {
                archive.names.insert(i, file.get_type_id().into());
            }
            if let Some(i) = hashsubtype {
                archive.names.insert(i, file.get_subtype_id().into());
            }
        }
        // create file map
        archive.files = files
            .into_iter()
            .map(|x| (util::hash_name_i32(x.get_name_id()), x))
            .collect();
        Ok(archive)
    }

    /// Check if the given ID can be added.
    /// Returns Err if ID has a hash collision with an existing name
    /// Returns Ok(None) if ID already exists
    /// Returns OK(Some(i32)) if ID needs to be inserted
    fn check_can_add_id(&mut self, s: &str) -> Result<Option<i32>, ChumError> {
        let hash = util::hash_name_i32(s);
        match self.names.get(&hash) {
            Some(existing) => {
                if existing == s {
                    // OK if values are equivalent
                    Ok(None)
                } else {
                    // errors otherwise
                    Err(ChumError::NameCollisionError {
                        id: hash,
                        existing_name: existing.into(),
                        new_name: s.into(),
                    })
                }
            }
            None => Ok(Some(hash)),
        }
    }

    pub fn add_file(&mut self, file: ChumFile) -> Result<(), ChumError> {
        // Check if names can be added
        let hashname = self.check_can_add_id(file.get_name_id())?;
        let hashtype = self.check_can_add_id(file.get_type_id())?;
        let hashsubtype = self.check_can_add_id(file.get_subtype_id())?;
        let typestr = file.get_type_id().to_string();
        let subtypestr = file.get_subtype_id().to_string();
        // Add name if it doesn't already exist
        if let Some(i) = hashname {
            self.names.insert(i, file.get_name_id().into());
            // Add file
            self.files.insert(i, file);
        } else {
            // Name must not already exist
            return Err(ChumError::NameCollisionError {
                id: util::hash_name_i32(file.get_name_id()),
                existing_name: file.get_name_id().into(), // existing is same as new name
                new_name: file.get_name_id().into(),
            });
        }
        // add type/subtype
        if let Some(i) = hashtype {
            self.names.insert(i, typestr);
        }
        if let Some(i) = hashsubtype {
            self.names.insert(i, subtypestr);
        }
        Ok(())
    }

    /// Get this archive's Dgc header
    pub fn get_header(&self) -> &data::TotemHeader {
        &self.header
    }

    /// Get all files in this archive
    pub fn get_files(&self) -> impl Iterator<Item = &ChumFile> {
        self.files.values()
    }

    /// Take all the files from this archive
    pub fn take_files(self) -> impl Iterator<Item = ChumFile> {
        self.files.into_iter().map(|(_i, x)| x)
    }

    /// Get all files in this archive along with their hash
    pub fn get_files_hash(&self) -> impl Iterator<Item = (&i32, &ChumFile)> {
        self.files.iter()
    }

    /// Take all files in this archive along with their hash
    pub fn take_files_hash(self) -> impl Iterator<Item = (i32, ChumFile)> {
        self.files.into_iter()
    }

    /// Get a file from its hash
    pub fn get_file_from_hash(&self, hash: i32) -> Option<&ChumFile> {
        self.files.get(&hash)
    }

    /// Get a file from its name
    pub fn get_file_from_name(&self, name: &str) -> Option<&ChumFile> {
        let hash = util::hash_name_i32(&name);
        if let Some(x) = self.names.get(&hash) {
            if x != name {
                None
            } else {
                self.files.get(&hash)
            }
        } else {
            None
        }
    }

    /// Get a mutable file from its hash
    pub fn get_file_from_hash_mut(&mut self, hash: i32) -> Option<&mut ChumFile> {
        self.files.get_mut(&hash)
    }

    /// Get a mutable file from its name
    pub fn get_file_from_name_mut(&mut self, name: &str) -> Option<&mut ChumFile> {
        let hash = util::hash_name_i32(&name);
        if let Some(x) = self.names.get(&hash) {
            if x != name {
                None
            } else {
                self.files.get_mut(&hash)
            }
        } else {
            None
        }
    }

    /// Split this ChumArchive into an NgcArchive and a DgcArchive
    pub fn split_archives(&self) -> (nametable::TotemNameTable, data::TotemArchive) {
        let dgc = data::TotemArchive::new_from_files(
            self.header.clone(),
            self.files
                .values()
                .map(|file| {
                    data::TotemFile::new(
                        file.data.clone(),
                        util::hash_name_i32(file.get_type_id()),
                        util::hash_name_i32(file.get_name_id()),
                        util::hash_name_i32(file.get_subtype_id()),
                    )
                })
                .collect(),
            self.format,
        );
        let ngc = nametable::TotemNameTable::new(self.names.clone());
        (ngc, dgc)
    }

    /// Find unused names
    pub fn find_unused_names(&self) -> HashSet<&str> {
        let mut set: HashSet<&str> = self.names.values().map(|x| x.as_str()).collect();
        for file in self.files.values() {
            set.remove(file.get_name_id());
            set.remove(file.get_type_id());
            set.remove(file.get_subtype_id());
        }
        set
    }

    /// Merge an NGC and DGC archive
    pub fn merge_archives(
        ngc: nametable::TotemNameTable,
        dgc: data::TotemArchive,
    ) -> Result<ChumArchive, ChumError> {
        // Check NGC data for matching names
        for file in dgc.iter_files() {
            if !ngc.get_names().contains_key(&file.get_type_id()) {
                return Err(ChumError::NameMissingError {
                    id: file.get_type_id(),
                });
            }
            if !ngc.get_names().contains_key(&file.get_name_id()) {
                return Err(ChumError::NameMissingError {
                    id: file.get_name_id(),
                });
            }
            if !ngc.get_names().contains_key(&file.get_subtype_id()) {
                return Err(ChumError::NameMissingError {
                    id: file.get_subtype_id(),
                });
            }
        }
        let fmt = dgc.get_format();
        // Return archive
        Ok(ChumArchive {
            header: dgc.get_header().clone(),
            files: dgc
                .take_files()
                .into_iter()
                .map(|file| {
                    let type_id = file.get_type_id();
                    let name_id = file.get_name_id();
                    let subtype_id = file.get_subtype_id();
                    (
                        name_id,
                        ChumFile {
                            data: file.take_data(),
                            type_id: ngc.get_names()[&type_id].clone(),
                            name_id: ngc.get_names()[&name_id].clone(),
                            subtype_id: ngc.get_names()[&subtype_id].clone(),
                        },
                    )
                })
                .collect(),
            names: ngc.take_names(),
            format: fmt,
        })
    }

    /// Read the chum archive from two readers
    pub fn read_chum_archive<R: Read, S: Read>(
        ngc_reader: &mut R,
        dgc_reader: &mut S,
        format: format::TotemFormat,
    ) -> Result<ChumArchive, ChumError> {
        // Load data
        let ngc = nametable::TotemNameTable::read_from(ngc_reader)?;
        let dgc = data::TotemArchive::read_from(dgc_reader, format)?;
        // merge
        ChumArchive::merge_archives(ngc, dgc)
    }

    /// Write the chum archive to two writers
    pub fn write_chum_archive<W: Write, V: Write>(
        &self,
        ngc_writer: &mut W,
        dgc_writer: &mut V,
    ) -> Result<(), ChumError> {
        let (ngc, dgc) = self.split_archives();
        ngc.write_to(ngc_writer)?;
        dgc.write_to(dgc_writer)?;
        Ok(())
    }

    /// Get the format for this archive
    pub fn get_format(&self) -> format::TotemFormat {
        self.format
    }

    /// Get a string from the given ID
    pub fn get_name_from_id(&self, id: i32) -> Option<&str> {
        self.names.get(&id).map(|x| x.as_str())
    }

    /// Add the given name to this archive's name database.
    /// Returns an error if adding the name would result in a hash collision.
    pub fn add_name(&mut self, name: &str) -> Result<bool, ChumError> {
        match self.check_can_add_id(name)? {
            None => Ok(false),
            Some(id) => {
                self.names.insert(id, name.to_owned());
                Ok(true)
            }
        }
    }

    /// Get an iterator of IDs
    pub fn get_ids(&self) -> impl Iterator<Item = &i32> {
        self.names.keys()
    }

    /// Get an iterator of names
    pub fn get_names(&self) -> impl Iterator<Item = &String> {
        self.names.values()
    }

    /// Get this archive's name map
    pub fn get_name_map(&self) -> &HashMap<i32, String> {
        &self.names
    }
}
