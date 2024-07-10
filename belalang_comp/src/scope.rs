use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::error::CompileError;

#[derive(Debug, Clone, Copy)]
pub enum SymbolScope {
    Global,
    Local,
}

#[derive(Debug)]
pub struct Symbol {
    pub scope: SymbolScope,
    pub index: usize,
}

pub struct CompilationScope {
    pub scope: SymbolScope,
    pub instructions: Vec<u8>,
    pub symbol_store: HashMap<String, Symbol>,
    pub symbol_count: usize,
}

impl CompilationScope {
    pub fn global() -> Self {
        Self {
            scope: SymbolScope::Global,
            instructions: Vec::default(),
            symbol_store: HashMap::default(),
            symbol_count: usize::default(),
        }
    }

    pub fn local() -> Self {
        Self {
            scope: SymbolScope::Local,
            instructions: Vec::default(),
            symbol_store: HashMap::default(),
            symbol_count: usize::default(),
        }
    }

    pub fn define(&mut self, name: String) -> Result<&Symbol, CompileError> {
        let symbol = Symbol {
            scope: self.scope,
            index: self.symbol_count,
        };

        self.symbol_count += 1;

        match self.symbol_store.entry(name.clone()) {
            Entry::Vacant(entry) => Ok(entry.insert(symbol)),
            Entry::Occupied(_) => Err(CompileError::DuplicateSymbol(name)),
        }
    }

    pub fn resolve(&self, name: &String) -> Option<&Symbol> {
        self.symbol_store.get(name)
    }
}

pub struct ScopeManager {
    store: Vec<CompilationScope>,
}

impl Default for ScopeManager {
    fn default() -> Self {
        Self {
            store: vec![CompilationScope::global()],
        }
    }
}

impl ScopeManager {
    pub fn enter(&mut self) {
        self.store.push(CompilationScope::local());
    }

    pub fn leave(&mut self) -> CompilationScope {
        self.store.pop().unwrap()
    }

    pub fn current_mut(&mut self) -> &mut CompilationScope {
        self.store.last_mut().unwrap()
    }

    pub fn current(&self) -> &CompilationScope {
        self.store.last().unwrap()
    }

    pub fn define(&mut self, name: String) -> Result<&Symbol, CompileError> {
        self.current_mut().define(name)
    }

    pub fn resolve(&self, name: String) -> Result<&Symbol, CompileError> {
        for symbol in self.store.iter().rev() {
            if let Some(symbol) = symbol.resolve(&name) {
                return Ok(symbol);
            }
        }

        Err(CompileError::UnknownSymbol(name))
    }
}
