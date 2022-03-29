use lazy_static::lazy_static;
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::Display,
    hash::{Hash, Hasher},
    io::{BufRead, BufReader, Read},
    sync::RwLock,
};

#[derive(Default)]
pub struct Genome {
    chr_name_list: Vec<String>,
    chr_size_list: Vec<Option<usize>>,
    name_id_map: HashMap<String, usize>,
}

#[derive(Clone, Copy)]
pub enum ChrRef<'a> {
    Assigned(usize),
    Unassigned(&'a str),
    Dummy,
}

impl<'a> Display for ChrRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.get_chr_name();
        write!(f, "{}", name)
    }
}

impl<'a> PartialEq for ChrRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Assigned(l0), Self::Assigned(r0)) => l0 == r0,
            (Self::Unassigned(l0), Self::Unassigned(r0)) => l0 == r0,
            (Self::Dummy, Self::Dummy) => true,
            (_, Self::Dummy) => false,
            (Self::Dummy, _) => false,
            _ => {
                let this_str = self.get_chr_name();
                let that_str = self.get_chr_name();
                this_str == that_str
            }
        }
    }
}

impl<'a> PartialOrd for ChrRef<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let Some(this_id) = self.id() {
            if let Some(that_id) = other.id() {
                return this_id.partial_cmp(&that_id);
            }
        }
        None
    }
}

impl<'a> Eq for ChrRef<'a> {}

impl<'a> Ord for ChrRef<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let this_id = self.get_id_or_update();
        let that_id = other.get_id_or_update();
        this_id.cmp(&that_id)
    }
}

impl<'a> ChrRef<'a> {
    pub fn to_static(&self) -> ChrRef<'static> {
        let id = self.get_id_or_update();
        if id < usize::MAX {
            ChrRef::Assigned(id)
        } else {
            ChrRef::Dummy
        }
    }
    pub fn get_chr_name(&self) -> &'a str {
        match self {
            Self::Unassigned(name) => name,
            Self::Assigned(id) => {
                if let Some(name) = LAST_NAME.with(|cached_name| {
                    if let Some(cached_name) = cached_name.borrow().as_ref() {
                        if cached_name.0 == *id {
                            return Some(cached_name.1);
                        }
                    }
                    None
                }) {
                    return name;
                }

                let storage = GENOME_STORAGE.read().unwrap();

                let ret = unsafe { std::mem::transmute(storage.chr_name_list[*id].as_str()) };

                LAST_NAME.with(|cached_name| {
                    *cached_name.borrow_mut() = Some((*id, ret));
                });

                ret
            }
            Self::Dummy => ".",
        }
    }
    pub fn id(&self) -> Option<usize> {
        match self {
            Self::Unassigned(_) => None,
            Self::Assigned(id) => Some(*id),
            Self::Dummy => None,
        }
    }
    pub fn get_id_or_update(&self) -> usize {
        match self {
            Self::Unassigned(name) => {
                let mut storage = GENOME_STORAGE.write().unwrap();
                let id = storage.chr_name_list.len();
                storage.name_id_map.insert(name.to_string(), id);
                storage.chr_name_list.push(name.to_string());
                storage.chr_size_list.push(None);
                id
            }
            Self::Assigned(id) => *id,
            _ => usize::MAX,
        }
    }
    pub fn get_chr_size(&self) -> Option<usize> {
        self.id()
            .map(|id| {
                let storage = GENOME_STORAGE.read().unwrap();
                storage.chr_size_list[id]
            })
            .unwrap_or(None)
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

thread_local! {
    static LAST_QUERY : RefCell<Option<(usize, u64)>> = RefCell::new(None);
    static LAST_NAME  : RefCell<Option<(usize, &'static str)>> = RefCell::new(None);
}

impl Genome {
    pub fn clear_genome_definition() {
        let mut storage = GENOME_STORAGE.write().unwrap();
        LAST_NAME.with(|last_name| {
            *last_name.borrow_mut() = None;
        });
        LAST_QUERY.with(|last_query| {
            *last_query.borrow_mut() = None;
        });
        *storage = Default::default();
    }
    pub fn get_chrom_sizes() -> Vec<(&'static str, usize)> {
        let storage = GENOME_STORAGE.read().unwrap();

        storage.chr_name_list.iter().zip(storage.chr_size_list.iter()).filter_map(|(name, size)| {
            let name = name.as_str();
            let size = size.clone();
            size.map(|size| (
                unsafe { std::mem::transmute::<_, &'static str>(name) },
                size
            ))
        }).collect()
    }
    pub fn query_chr(name: &str) -> ChrRef {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        name.hash(&mut hasher);
        let hash = hasher.finish();

        if let Some((id, cached_hash)) = LAST_QUERY.with(|id| id.borrow().clone()) {
            // Definitely, hash == cached_hash doesn't means it's the same. But in practise, chrom
            // name's hash code never collides
            if hash == cached_hash {
                return ChrRef::Assigned(id);
            }
        }

        let storage = GENOME_STORAGE.read().unwrap();
        if let Some(id) = storage.name_id_map.get(name) {
            LAST_QUERY.with(|cache| {
                *cache.borrow_mut() = Some((*id, hash));
            });
            return ChrRef::Assigned(*id);
        }
        ChrRef::Unassigned(name)
    }
    pub fn load_genome_file<R: Read>(reader: R) -> Result<(), Box<dyn std::error::Error>> {
        let mut storage = GENOME_STORAGE.write()?;
        if storage.chr_name_list.len() != 0 {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Genome definition has been already loaded",
            ))?;
        }
        let mut br = BufReader::new(reader);
        let mut buf = String::new();
        let mut id = 0;
        while let Ok(sz) = br.read_line(&mut buf) {
            if sz == 0 {
                break;
            }

            let line = buf.trim_end();
            let mut tokenized = line.split('\t');
            if let Some(chr_name) = tokenized.next() {
                if let Some(chr_size_txt) = tokenized.next() {

                    let chr_size : usize = chr_size_txt.parse()?;
                    
                    storage.chr_name_list.push(chr_name.to_string());
                    storage.chr_size_list.push(Some(chr_size));
                    storage.name_id_map.insert(chr_name.to_string(), id);
                }
            }

            buf.clear();
            id += 1;
        }
        Ok(())
    }
}

lazy_static! {
    static ref GENOME_STORAGE: RwLock<Genome> = {
        let inner = Default::default();
        RwLock::new(inner)
    };
}
