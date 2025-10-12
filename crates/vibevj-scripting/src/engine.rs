use rhai::{Engine, EvalAltResult, Scope, AST};
use vibevj_common::{Result, VibeVJError};
use std::collections::HashMap;

/// Script engine wrapper
pub struct ScriptEngine {
    engine: Engine,
    scripts: HashMap<String, AST>,
}

impl ScriptEngine {
    /// Create a new script engine
    pub fn new() -> Self {
        let mut engine = Engine::new();
        
        // Register VibeVJ API
        crate::api::register_api(&mut engine);

        Self {
            engine,
            scripts: HashMap::new(),
        }
    }

    /// Load and compile a script
    pub fn load_script(&mut self, name: String, source: &str) -> Result<()> {
        let ast = self
            .engine
            .compile(source)
            .map_err(|e| VibeVJError::ScriptingError(format!("Compilation error: {}", e)))?;

        self.scripts.insert(name, ast);
        Ok(())
    }

    /// Execute a loaded script
    pub fn execute_script(&mut self, name: &str, scope: &mut Scope) -> Result<()> {
        let ast = self
            .scripts
            .get(name)
            .ok_or_else(|| VibeVJError::ScriptingError(format!("Script '{}' not found", name)))?;

        self.engine
            .run_ast_with_scope(scope, ast)
            .map_err(|e| VibeVJError::ScriptingError(format!("Execution error: {}", e)))?;

        Ok(())
    }

    /// Evaluate a script expression
    pub fn eval<T: Clone + 'static>(&mut self, script: &str) -> Result<T> {
        self.engine
            .eval(script)
            .map_err(|e| VibeVJError::ScriptingError(format!("Evaluation error: {}", e)))
    }

    /// Get a reference to the underlying Rhai engine
    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    /// Get a mutable reference to the underlying Rhai engine
    pub fn engine_mut(&mut self) -> &mut Engine {
        &mut self.engine
    }
}

impl Default for ScriptEngine {
    fn default() -> Self {
        Self::new()
    }
}
