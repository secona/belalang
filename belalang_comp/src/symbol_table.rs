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

    pub fn resolve(&self, name: &String) -> Option<&Symbol> {
        self.store.get(name)
    }
}

pub struct SymbolTableList {
    symbol_tables: Vec<SymbolTable>,
}

impl Default for SymbolTableList {
    fn default() -> Self {
        Self {
            symbol_tables: vec![SymbolTable::default()],
        }
    }
}

impl SymbolTableList {
    // the symbol_tables vector will contain at least one value, the main SymbolTable.
    // calling unwrap should be fine.

    pub fn new_local(&mut self) -> &SymbolTable {
        self.symbol_tables.push(SymbolTable::local());
        self.symbol_tables.last().unwrap()
    }

    pub fn pop(&mut self) -> SymbolTable {
        self.symbol_tables.pop().unwrap()
    }

    pub fn current_symbols(&self) -> &SymbolTable {
        self.symbol_tables.last().unwrap()
    }

    pub fn current_symbols_mut(&mut self) -> &mut SymbolTable {
        self.symbol_tables.last_mut().unwrap()
    }

    pub fn define(&mut self, name: String) -> Result<&Symbol, CompileError> {
        let symbols = self.current_symbols_mut();
        symbols.define(name)
    }

    pub fn resolve(&self, name: String) -> Result<&Symbol, CompileError> {
        for symbol in self.symbol_tables.iter().rev() {
            if let Some(symbol) = symbol.resolve(&name) {
                return Ok(symbol);
            }
        }

        Err(CompileError::UnknownSymbol(name))
    }
}
