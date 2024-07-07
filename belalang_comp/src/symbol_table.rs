use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::error::CompileError;

#[derive(Debug)]
pub enum SymbolScope {
    Global,
}

#[derive(Debug)]
pub struct Symbol {
    pub scope: SymbolScope,
    pub index: usize,
}

#[derive(Default)]
pub struct SymbolTable {
    pub store: HashMap<String, Symbol>,
    pub count: usize,
}

impl SymbolTable {
    pub fn define(&mut self, name: String) -> Result<&Symbol, CompileError> {
        let symbol = Symbol {
            scope: SymbolScope::Global,
            index: self.count,
        };

        self.count += 1;

        match self.store.entry(name.clone()) {
            Entry::Vacant(entry) => Ok(entry.insert(symbol)),
            Entry::Occupied(_) => Err(CompileError::DuplicateSymbol(name)),
        }
    }

    pub fn resolve(&self, name: String) -> Result<&Symbol, CompileError> {
        self.store.get(&name).ok_or(CompileError::UnknownSymbol(name))
    }
}
