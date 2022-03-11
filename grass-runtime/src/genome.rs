use lazy_static::lazy_static;
use std::{sync::RwLock, collections::HashMap};

#[derive(Default)]
pub struct Genome {
    chr_name_list: Vec<String>,
    chr_size_list: Vec<Option<usize>>,
    name_id_map: HashMap<String, usize>,
}

pub enum ChrRef<'a>{
    Assigned(usize),
    Unassigned(&'a str),
}

impl <'a> ChrRef<'a> {
    pub fn get_chr_name(&self) -> &'a str {
        match self {
            Self::Unassigned(name) => name,
            Self::Assigned(id) => {
                let storage = GENOME_STORAGE.read().unwrap();
                unsafe {
                    std::mem::transmute_copy(&storage.chr_name_list[*id])
                }
            }
        }
    }
    pub fn id(&self) -> Option<usize> {
        match self {
            Self::Unassigned(_) => None,
            Self::Assigned(id) => Some(*id),
        }
    }
    pub fn get_id_or_update(&self) -> usize  {
        match self {
            Self::Unassigned(name) => {
                let mut storage = GENOME_STORAGE.write().unwrap();
                let id = storage.chr_name_list.len();
                storage.name_id_map.insert(name.to_string(), id);
                storage.chr_name_list.push(name.to_string());
                storage.chr_size_list.push(None);
                id
            },
            Self::Assigned(id) => *id,
        }
    }
    pub fn get_chr_size(&self) -> Option<usize> {
        self.id().map(|id| {
            let storage = GENOME_STORAGE.read().unwrap();
            storage.chr_size_list[id]
        }).unwrap_or(None)
    }
    pub fn verify_size(&self, size: usize) -> bool {
        Some(size) == self.get_chr_size()
    }
    pub fn verify_size_or_update(&self, size: usize) -> bool {
        if let Some(actual_size) = self.get_chr_size() {
            return size == actual_size;
        }
        let mut storage = GENOME_STORAGE.write().unwrap();
        storage.chr_size_list[self.get_id_or_update()] = Some(size);
        true
    }
}

impl Genome {
    pub fn query_chr(name: &str) -> ChrRef {
       let storage = GENOME_STORAGE.read().unwrap(); 
       if let Some(id) = storage.name_id_map.get(name) {
           return ChrRef::Assigned(*id);
       }
       ChrRef::Unassigned(name)
    }
}

lazy_static! {
    static ref GENOME_STORAGE : RwLock<Genome> = {
        let inner = Default::default();
        RwLock::new(inner)
    };
}
