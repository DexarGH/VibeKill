use std::collections::HashMap;

use crate::os::process::Process;

pub struct Schema {
    scopes: HashMap<String, ModuleScope>,
}

impl Schema {
    pub fn new(process: &Process, schema_module: u64) -> Option<Self> {
        let schema_system = process.scan(
            "48 8D 3D ? ? ? ? E8 ? ? ? ? 48 8B BD ? ? ? ? 31 F6 E8 ? ? ? ? E9",
            schema_module,
        )?;
        let schema_system = process.get_relative_address(schema_system, 3, 7);

        let type_scopes_len: i32 = process.read(schema_system + 0x1F0);
        let type_scopes_vec: u64 = process.read(schema_system + 0x1F8);
        let mut scopes = HashMap::new();
        for i in 0..type_scopes_len as u64 {
            let type_scope_address = process.read(type_scopes_vec + (i * 8));
            let type_scope = ModuleScope::new(process, type_scope_address);
            scopes.insert(type_scope.name.clone(), type_scope);
        }

        Some(Self { scopes })
    }

    #[allow(unused)]
    pub fn get(&self, library: &str, class: &str, field: &str) -> Option<u64> {
        let scope = self.scopes.get(library)?;
        let class = scope.get_class(class)?;
        class.get(field)
    }

    #[allow(unused)]
    pub fn get_any_class(&self, library: &str, field: &str) -> Option<(String, u64)> {
        let scope = self.scopes.get(library)?;
        for (class_name, class) in &scope.classes {
            if let Some(offset) = class.get(field) {
                return Some((class_name.clone(), offset));
            }
        }
        None
    }

    pub fn get_library(&self, library: &str) -> Option<&ModuleScope> {
        self.scopes.get(library)
    }

    pub fn dump_weapon_classes(&self, library: &str) {
        let Some(scope) = self.scopes.get(library) else {
            println!("[dump] library '{}' not found", library);
            return;
        };
        for (name, class) in &scope.classes {
            if name.contains("Weapon") || name.contains("weapon") {
                println!("[dump] class: {}", name);
                println!("[dump]   size: {}", class.size);
                println!("[dump]   fields ({}):", class.fields.len());
                let mut fields: Vec<(&String, &u64)> = class.fields.iter().collect();
                fields.sort_by_key(|&(_, &v)| v);
                for (field, offset) in &fields {
                    println!("[dump]     0x{:04x} {}", offset, field);
                }
                println!();
            }
        }
    }

    pub fn dump_all_field_names(&self, library: &str, term: &str) {
        let Some(scope) = self.scopes.get(library) else {
            println!("[dump] library '{}' not found", library);
            return;
        };
        for (name, class) in &scope.classes {
            for field in class.fields.keys() {
                if field.contains(term) {
                    println!("[dump] {}.{} (0x{:04x})", name, field, class.fields.get(field).unwrap());
                }
            }
        }
    }
}

pub struct ModuleScope {
    name: String,
    classes: HashMap<String, Class>,
}

impl ModuleScope {
    fn new(process: &Process, address: u64) -> Self {
        let name = process.read_string_uncached(address + 0x08);

        let mut classes = HashMap::new();
        // has 1024 buckets
        let hash_vector = address + 0x560 + 0x90;
        for i in 0..1024 {
            // first_uncomitted
            let mut current_element: u64 = process.read(hash_vector + (i * 24) + 0x28);

            while current_element != 0 {
                let data: u64 = process.read(current_element + 0x10);
                if data != 0 {
                    let class = Class::new(process, data);
                    classes.insert(class.name.clone(), class);
                }
                current_element = process.read(current_element + 0x08);
            }
        }

        // free_list (HashAllocatedBlob)
        let mut current_blob: u64 = process.read(address + 0x560 + 0x20);
        while current_blob != 0 {
            let data: u64 = process.read(current_blob + 0x10);
            if data > process.min && data < process.max {
                let class = Class::new(process, data);
                classes.insert(class.name.clone(), class);
            }
            current_blob = process.read(current_blob);
        }

        Self { name, classes }
    }

    pub fn get(&self, class: &str, field: &str) -> Option<u64> {
        let Some(c) = self.classes.get(class) else {
            utils::warn!("could not find class {class}");
            return None;
        };
        let f = c.get(field);
        if f.is_none() {
            utils::warn!("could not find field {field} in class {class}");
        }
        f
    }

    pub fn get_class(&self, class: &str) -> Option<&Class> {
        self.classes.get(class)
    }

    pub fn dump_class_pointer_fields(&self, class: &str) {
        let Some(cls) = self.classes.get(class) else {
            println!("[dump] class '{}' not found", class);
            return;
        };
        for (f, &o) in &cls.fields {
            if f.starts_with("m_p") || f.starts_with("m_h") || f.starts_with("m_bVData") {
                println!("[dump]   {}.{} (0x{:04x})", class, f, o);
            }
        }
    }

    pub fn dump_class_names(&self, term: &str) {
        for name in self.classes.keys() {
            if name.contains(term) {
                println!("[dump]   class: {}", name);
            }
        }
    }

    pub fn dump_all_fields_with_condition(&self, field_term: &str, pointer_only: bool) {
        for (cname, cls) in &self.classes {
            for (fname, &offset) in &cls.fields {
                if fname.contains(field_term) {
                    if !pointer_only || fname.starts_with("m_p") || fname.starts_with("m_h") {
                        println!("[dump]   {}.{} (0x{:04x})", cname, fname, offset);
                    }
                }
            }
        }
    }

    pub fn dump_all_classes_with_field(&self, term: &str) {
        for (name, class) in &self.classes {
            for fname in class.fields.keys() {
                if fname.contains(term) {
                    println!("[dump]   {}.{}", name, fname);
                }
            }
        }
    }

    pub fn dump_class_all_fields(&self, class: &str) {
        let Some(cls) = self.classes.get(class) else {
            println!("[dump] class '{}' not found", class);
            return;
        };
        let mut fields: Vec<(&String, &u64)> = cls.fields.iter().collect();
        fields.sort_by_key(|&(_, &v)| v);
        for &(ref f, &o) in &fields {
            println!("[dump]   0x{:04x} {}", o, f);
        }
    }
}

pub struct Class {
    name: String,
    fields: HashMap<String, u64>,
    size: i32,
}

impl Class {
    fn new(process: &Process, address: u64) -> Self {
        let name = process.read_string_uncached(process.read(address + 0x08));

        let mut fields = HashMap::new();
        let field_count: i16 = process.read(address + 0x24);
        let size = process.read(address + 0x20);
        if !(0..=20000).contains(&field_count) {
            return Self { name, fields, size };
        }
        let fields_vec: u64 = process.read(address + 0x30);
        for i in 0..field_count as u64 {
            let field = Field::new(process, fields_vec + (0x20 * i));
            fields.insert(field.name, field.offset);
        }
        Self { name, fields, size }
    }

    fn get(&self, field: &str) -> Option<u64> {
        self.fields.get(field).copied()
    }

    pub fn size(&self) -> i32 {
        self.size
    }
}

struct Field {
    name: String,
    offset: u64,
}

impl Field {
    fn new(process: &Process, address: u64) -> Self {
        let name = process.read_string_uncached(process.read(address));
        let offset = process.read::<i32>(address + 0x10) as u64;

        Self { name, offset }
    }
}
