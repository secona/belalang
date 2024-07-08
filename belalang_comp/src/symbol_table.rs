use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::error::CompileError;

#[derive(Debug, Default, Clone, Copy)]
pub enum SymbolScope {
    #[default]
    Global,
    Local,
}

#[derive(Debug)]
pub struct Symbol {
    pub scope: SymbolScope,
    pub index: usize,
}

#[derive(Debug, Default)]
pub struct SymbolTable {
    pub scope: SymbolScope,
    pub store: HashMap<String, Symbol>,
    pub count: usize,
}

impl SymbolTable {
    pub fn local() -> Self {
        Self {
            scope: SymbolScope::Local,
            ..Default::default()
        }
    }

    pub fn define(&mut self, name: String) -> Result<&Symbol, CompileError> {
        let symbol = Symbol {
            scope: self.scope,
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
