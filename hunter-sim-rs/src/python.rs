//! Python bindings for the Hunter Simulator using PyO3

use pyo3::prelude::*;
use pyo3::types::PyDict;
use crate::config::{BuildConfig, HunterType, Meta};
use crate::simulation::run_and_aggregate;
use crate::build_generator::{BuildGenerator, AttributeInfo, TalentInfo};
use std::collections::HashMap;

/// Python-callable simulation function
#[pyfunction]
#[pyo3(signature = (config_json, num_sims, parallel=false))]
fn simulate(py: Python<'_>, config_json: &str, num_sims: usize, parallel: bool) -> PyResult<String> {
    let config: BuildConfig = serde_json::from_str(config_json)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid config JSON: {}", e)))?;
    
    // Release GIL during computation to prevent GUI freezing
    let stats = py.allow_threads(|| run_and_aggregate(&config, num_sims, parallel));
    
    let result = serde_json::to_string(&stats)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to serialize results: {}", e)))?;
    
    Ok(result)
}

/// Python-callable simulation function from YAML file
#[pyfunction]
#[pyo3(signature = (config_path, num_sims, parallel=false))]
fn simulate_from_file(py: Python<'_>, config_path: &str, num_sims: usize, parallel: bool) -> PyResult<String> {
    let config = BuildConfig::from_file(config_path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to load config: {}", e)))?;
    
    // Release GIL during computation to prevent GUI freezing
    let stats = py.allow_threads(|| run_and_aggregate(&config, num_sims, parallel));
    
    let result = serde_json::to_string(&stats)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to serialize results: {}", e)))?;
    
    Ok(result)
}

/// Python-callable function to create a BuildConfig from Python dicts
#[pyfunction]
#[pyo3(signature = (hunter, level, stats, talents, attributes, inscryptions=None, mods=None, relics=None, gems=None))]
fn create_config(
    hunter: &str,
    level: i32,
    stats: &Bound<'_, PyDict>,
    talents: &Bound<'_, PyDict>,
    attributes: &Bound<'_, PyDict>,
    inscryptions: Option<&Bound<'_, PyDict>>,
    mods: Option<&Bound<'_, PyDict>>,
    relics: Option<&Bound<'_, PyDict>>,
    gems: Option<&Bound<'_, PyDict>>,
) -> PyResult<String> {
    let hunter_type = match hunter.to_lowercase().as_str() {
        "borge" => HunterType::Borge,
        "ozzy" => HunterType::Ozzy,
        "knox" => HunterType::Knox,
        _ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("Invalid hunter type: {}", hunter)
        )),
    };
    
    fn pydict_to_hashmap_i32(dict: &Bound<'_, PyDict>) -> PyResult<HashMap<String, i32>> {
        let mut map = HashMap::new();
        for (key, value) in dict.iter() {
            let k: String = key.extract()?;
            let v: i32 = value.extract()?;
            map.insert(k, v);
        }
        Ok(map)
    }
    
    fn pydict_to_hashmap_bool(dict: &Bound<'_, PyDict>) -> PyResult<HashMap<String, bool>> {
        let mut map = HashMap::new();
        for (key, value) in dict.iter() {
            let k: String = key.extract()?;
            let v: bool = value.extract()?;
            map.insert(k, v);
        }
        Ok(map)
    }
    
    let config = BuildConfig {
        meta: Some(Meta {
            hunter: hunter_type,
            level,
        }),
        hunter: None,
        level: None,
        stats: pydict_to_hashmap_i32(stats)?,
        talents: pydict_to_hashmap_i32(talents)?,
        attributes: pydict_to_hashmap_i32(attributes)?,
        inscryptions: inscryptions.map(|d| pydict_to_hashmap_i32(d)).transpose()?.unwrap_or_default(),
        mods: mods.map(|d| pydict_to_hashmap_bool(d)).transpose()?.unwrap_or_default(),
        relics: relics.map(|d| pydict_to_hashmap_i32(d)).transpose()?.unwrap_or_default(),
        gems: gems.map(|d| pydict_to_hashmap_i32(d)).transpose()?.unwrap_or_default(),
        gadgets: HashMap::new(),
        bonuses: HashMap::new(),
    };
    
    let json = serde_json::to_string(&config)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to serialize config: {}", e)))?;
    
    Ok(json)
}

/// Get number of threads being used for parallel simulation
#[pyfunction]
fn get_thread_count() -> PyResult<usize> {
    Ok(rayon::current_num_threads())
}

/// Get number of available CPU cores
#[pyfunction]
fn get_available_cores() -> PyResult<usize> {
    Ok(std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1))
}

/// Python-callable batch simulation function - simulate multiple configs at once
#[pyfunction]
#[pyo3(signature = (config_jsons, num_sims, parallel=false))]
fn simulate_batch(py: Python<'_>, config_jsons: Vec<String>, num_sims: usize, parallel: bool) -> PyResult<Vec<String>> {
    // Parse all configs first (inside GIL)
    let configs: Result<Vec<BuildConfig>, _> = config_jsons.iter()
        .map(|json| serde_json::from_str(json))
        .collect();
    
    let configs = configs.map_err(|e| 
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid config JSON: {}", e))
    )?;
    
    // Release GIL and run all simulations in parallel
    let results = py.allow_threads(|| {
        configs.iter()
            .map(|config| run_and_aggregate(config, num_sims, parallel))
            .collect::<Vec<_>>()
    });
    
    // Serialize results (inside GIL)
    let json_results: Result<Vec<String>, _> = results.iter()
        .map(|stats| serde_json::to_string(stats))
        .collect();
    
    let json_results = json_results.map_err(|e| 
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to serialize results: {}", e))
    )?;
    
    Ok(json_results)
}

/// Python-callable build generation function - generate multiple valid builds at once
#[pyfunction]
#[pyo3(signature = (level, talents, attributes, attribute_dependencies, attribute_point_gates, attribute_exclusions, count))]
fn generate_builds(
    py: Python<'_>,
    level: i32,
    talents: &Bound<'_, PyDict>,
    attributes: &Bound<'_, PyDict>,
    attribute_dependencies: &Bound<'_, PyDict>,
    attribute_point_gates: &Bound<'_, PyDict>,
    attribute_exclusions: Vec<(String, String)>,
    count: usize,
) -> PyResult<Vec<(HashMap<String, i32>, HashMap<String, i32>)>> {
    // Parse talents
    let mut talent_map = HashMap::new();
    for (key, value) in talents.iter() {
        let name: String = key.extract()?;
        let dict: &Bound<'_, PyDict> = value.downcast()?;
        let cost: i32 = dict.get_item("cost")?.unwrap().extract()?;
        let max: i32 = dict.get_item("max")?.unwrap().extract()?;
        talent_map.insert(name, TalentInfo { cost, max });
    }
    
    // Parse attributes
    let mut attr_map = HashMap::new();
    for (key, value) in attributes.iter() {
        let name: String = key.extract()?;
        let dict: &Bound<'_, PyDict> = value.downcast()?;
        let cost: i32 = dict.get_item("cost")?.unwrap().extract()?;
        let max_val = dict.get_item("max")?.unwrap();
        
        let max: f64 = if let Ok(v) = max_val.extract::<i32>() {
            v as f64
        } else if let Ok(v) = max_val.extract::<f64>() {
            v
        } else {
            f64::INFINITY
        };
        
        attr_map.insert(name, AttributeInfo { cost, max });
    }
    
    // Parse dependencies
    let mut deps_map = HashMap::new();
    for (key, value) in attribute_dependencies.iter() {
        let attr_name: String = key.extract()?;
        let deps_dict: &Bound<'_, PyDict> = value.downcast()?;
        
        let mut dep_reqs = HashMap::new();
        for (dep_key, dep_val) in deps_dict.iter() {
            let dep_name: String = dep_key.extract()?;
            let dep_level: i32 = dep_val.extract()?;
            dep_reqs.insert(dep_name, dep_level);
        }
        
        deps_map.insert(attr_name, dep_reqs);
    }
    
    // Parse point gates
    let mut gates_map = HashMap::new();
    for (key, value) in attribute_point_gates.iter() {
        let name: String = key.extract()?;
        let gate: i32 = value.extract()?;
        gates_map.insert(name, gate);
    }
    
    // Create generator
    let generator = BuildGenerator::new(
        level,
        talent_map,
        attr_map,
        deps_map,
        gates_map,
        attribute_exclusions,
    );
    
    // Generate builds (release GIL)
    let builds = py.allow_threads(|| generator.generate_builds(count));
    
    Ok(builds)
}

/// Python module definition
#[pymodule]
fn hunter_sim_lib(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(simulate, m)?)?;
    m.add_function(wrap_pyfunction!(simulate_from_file, m)?)?;
    m.add_function(wrap_pyfunction!(simulate_batch, m)?)?;
    m.add_function(wrap_pyfunction!(create_config, m)?)?;
    m.add_function(wrap_pyfunction!(get_thread_count, m)?)?;
    m.add_function(wrap_pyfunction!(get_available_cores, m)?)?;
    m.add_function(wrap_pyfunction!(generate_builds, m)?)?;
    Ok(())
}
