use lazy_static::lazy_static;
use std::{sync::RwLock, collections::HashMap, io::{Read, BufReader, BufRead}};

#[derive(Default)]
pub struct Genome {
    chr_name_list: Vec<String>,
    chr_size_list: Vec<Option<usize>>,
    name_id_map: HashMap<String, usize>,
}

#[derive(Clone, Copy)]
pub enum ChrRef<'a>{
    Assigned(usize),
    Unassigned(&'a str),
}

impl <'a> PartialEq for ChrRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Assigned(l0), Self::Assigned(r0)) => l0 == r0,
            (Self::Unassigned(l0), Self::Unassigned(r0)) => l0 == r0,
            _ => {
                let this_str = self.get_chr_name();
                let that_str = self.get_chr_name();
                this_str == that_str
            }
        }
    }
}

impl <'a> PartialOrd for ChrRef<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let Some(this_id) = self.id() {
            if let Some(that_id) = other.id() {
                return this_id.partial_cmp(&that_id);
            }
        }
        None
    }
}

impl <'a> Eq for ChrRef<'a> {}

impl <'a> Ord for ChrRef<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let this_id = self.get_id_or_update();
        let that_id = other.get_id_or_update();
        this_id.cmp(&that_id)
    }
}

impl <'a> ToString for ChrRef<'a> {
    fn to_string(&self) -> String {
        self.get_chr_name().to_string()
    }
}

impl <'a> ChrRef<'a> {
    pub fn to_static(&self) -> ChrRef<'static> {
        let id = self.get_id_or_update();
        ChrRef::Assigned(id)
    }
    pub fn get_chr_name(&self) -> &'a str {
        match self {
            Self::Unassigned(name) => name,
            Self::Assigned(id) => {
                let storage = GENOME_STORAGE.read().unwrap();
                unsafe {
                    std::mem::transmute(storage.chr_name_list[*id].as_str())
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
    pub fn load_genome_file<R: Read>(reader: R) -> Result<(), Box<dyn std::error::Error>> {
        let mut storage = GENOME_STORAGE.write()?;
        if storage.chr_name_list.len() != 0 {
            Err(
                std::io::Error::new(std::io::ErrorKind::Other, "Genome definition has been already loaded")
            )?;
        }
        let mut br = BufReader::new(reader);
        let mut buf = String::new();
        let mut id = 0;
        while let Ok(_sz) = br.read_line(&mut buf) {
            let mut tokenized = buf.split('\t').take(2);
            let chr_name = tokenized.next().unwrap();
            let chr_size : usize = tokenized.next().unwrap().parse()?;

            storage.chr_name_list.push(chr_name.to_string());
            storage.chr_size_list.push(Some(chr_size));
            storage.name_id_map.insert(chr_name.to_string(), id);

            buf.clear();
            id += 1;
        }
        Ok(())
    }
}

lazy_static! {
    static ref GENOME_STORAGE : RwLock<Genome> = {
        let inner = Default::default();
        RwLock::new(inner)
    };
}
