use std::collections::hash_map::Entry;
use std::collections::HashMap;

use belalang_vm::opcode;

use crate::error::CompileError;

#[derive(Debug, Clone, Copy)]
pub enum ScopeLevel {
    Builtin,
    Global,
    Local,
}

#[derive(Debug)]
pub struct Symbol {
    pub scope: ScopeLevel,
    pub index: usize,
}

pub struct CompilationScope {
    pub scope: ScopeLevel,
    pub instructions: Vec<u8>,
    pub symbol_store: HashMap<String, Symbol>,
    pub symbol_count: usize,
}

impl CompilationScope {
    pub fn global() -> Self {
        let mut symbol_store = HashMap::new();
        let symbol_count = &symbol_store.len();

        symbol_store.insert(
            "print".into(),
            Symbol {
                scope: ScopeLevel::Builtin,
                index: *symbol_count,
            },
        );

        Self {
            scope: ScopeLevel::Global,
            instructions: Vec::default(),
            symbol_store,
            symbol_count: *symbol_count,
        }
    }

    pub fn local() -> Self {
        Self {
            scope: ScopeLevel::Local,
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
    pub main_scope: CompilationScope,
    scope_store: Vec<CompilationScope>,
}

impl Default for ScopeManager {
    fn default() -> Self {
        let mut main_scope = CompilationScope::global();

        main_scope.symbol_store.insert(
            "print".into(),
            Symbol {
                scope: ScopeLevel::Builtin,
                index: main_scope.symbol_count,
            },
        );

        Self {
            main_scope,
            scope_store: Vec::default(),
        }
    }
}

impl ScopeManager {
    pub fn enter(&mut self) {
        self.scope_store.push(CompilationScope::local());
    }

    pub fn leave(&mut self) -> CompilationScope {
        // we want to panic when trying to leave main scope
        let mut scope = self.scope_store.pop().unwrap();

        if let Some(&opcode::POP) = scope.instructions.last() {
            scope.instructions.pop();
        }

        scope
    }

    pub fn current_mut(&mut self) -> &mut CompilationScope {
        self.scope_store.last_mut().unwrap_or(&mut self.main_scope)
    }

    pub fn current(&self) -> &CompilationScope {
        self.scope_store.last().unwrap_or(&self.main_scope)
    }

    pub fn define(&mut self, name: String) -> Result<&Symbol, CompileError> {
        self.current_mut().define(name)
    }

    pub fn resolve(&self, name: String) -> Result<&Symbol, CompileError> {
        for symbol in self.scope_store.iter().rev() {
            if let Some(symbol) = symbol.resolve(&name) {
                return Ok(symbol);
            }
        }

        self.main_scope
            .resolve(&name)
            .ok_or(CompileError::UnknownSymbol(name))
    }
}
