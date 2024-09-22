use std::collections::HashMap;
use std::str::FromStr;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::utils::{parse_semver_from_str, parse_semver_from_vec, SemVer};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
enum PreVer {
    Vec(Vec<i32>),
    Str(String)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct PreManifest {
    format_version: i32,
    header: PreManifestHeader,
    modules: Vec<PreManifestModule>,
    dependencies: Vec<PreManifestDependency>,
    capabilities: Vec<String>,
    subpacks: Vec<ManifestSubpack>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct PreManifestHeader {
    name: String,
    description: String,
    min_engine_version: Vec<i32>,
    uuid: String,
    version: Vec<i32>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct PreManifestModule {
    #[serde(rename(deserialize = "type"))]
    type_id: String,
    uuid: String,
    version: Vec<i32>,
    language: Option<String>,
    entry: Option<String>,
    description: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct PreManifestDependency {
    uuid: Option<String>,
    module_name: Option<String>,
    version: PreVer
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ManifestSubpack {
    folder_name: String,
    name: String,
    memory_tier: i32
}

pub fn deserialize_manifest_from_str(src: &str) -> Manifest {
    let deserialized_pre: PreManifest = serde_json::from_str(src).unwrap();

    let mut header = ManifestHeader {
        name: deserialized_pre.clone().header.name,
        description: deserialized_pre.clone().header.description,
        min_engine_version: parse_semver_from_vec(deserialized_pre.clone().header.min_engine_version),
        version: parse_semver_from_vec(deserialized_pre.clone().header.version),
        uuid: Uuid::from_str(&deserialized_pre.clone().header.uuid).unwrap(),
    };

    let mut modules: Vec<ManifestModule> = vec![];

    for module in deserialized_pre.clone().modules {
        if module.type_id == "script" {
            modules.push(
                ManifestModule::Script(
                    Uuid::from_str(&module.uuid).unwrap(),
                    parse_semver_from_vec(module.version),
                    ScriptManifestModule { entry: module.entry.unwrap() }
                )
            );
        }
        else if module.type_id == "data" || module.type_id == "resources" {
            modules.push(
                ManifestModule::Data(
                    Uuid::from_str(&module.uuid).unwrap(),
                    parse_semver_from_vec(module.version),
                )
            )
        }
    }

    let mut dependencies: Vec<ManifestDependency> = vec![];

    for dep in deserialized_pre.clone().dependencies {
        if dep.clone().module_name.is_some() {
            dependencies.push(
              ManifestDependency::ScriptDependency(
                  {
                      if SCRIPT_MANIFEST_DEPENDENCIES.get(dep.clone().module_name.unwrap().as_str()).is_some() {
                          SCRIPT_MANIFEST_DEPENDENCIES.get(dep.clone().module_name.unwrap().as_str()).unwrap().clone()
                      } else {
                          ScriptManifestDependency::Custom(dep.clone().module_name.unwrap())
                      }
                  },
                  {
                      if let PreVer::Str(str) = dep.clone().version {
                          parse_semver_from_str(&str)
                      } else if let PreVer::Vec(vec) = dep.clone().version {
                          parse_semver_from_vec(vec)
                      } else {
                          SemVer {
                              major: 1, minor: 0, patch: 0, beta: false
                          }
                      }
                  }
              )
            );
        } else {
            dependencies.push(ManifestDependency::UuidDependency(
                Uuid::parse_str(&dep.clone().uuid.unwrap()).unwrap(),
                {
                    if let PreVer::Str(str) = dep.clone().version {
                        parse_semver_from_str(&str)
                    } else if let PreVer::Vec(vec) = dep.clone().version {
                        parse_semver_from_vec(vec)
                    } else {
                        SemVer {
                            major: 1, minor: 0, patch: 0, beta: false
                        }
                    }
                }
            ))
        }
    }

    let subpacks = deserialized_pre.clone().subpacks;

    let mut capabilities: Vec<ManifestCapability> = vec![];

    for cap in deserialized_pre.clone().capabilities {
        if MANIFEST_CAPABILITIES.get(cap.as_str()).is_some() {
            capabilities.push(
                MANIFEST_CAPABILITIES.get(cap.as_str()).unwrap().clone()
            )
        } else {
            capabilities.push(ManifestCapability::Custom(cap))
        }
    }

    Manifest {
        header, modules, dependencies, subpacks, capabilities
    }
}

#[derive(Clone, Debug)]
pub struct Manifest {
    pub header: ManifestHeader,
    pub modules: Vec<ManifestModule>,
    pub dependencies: Vec<ManifestDependency>,
    pub subpacks: Vec<ManifestSubpack>,
    pub capabilities: Vec<ManifestCapability>
}

#[derive(Clone, Debug)]
pub struct ManifestHeader {
    uuid: Uuid,
    name: String,
    description: String,
    min_engine_version: SemVer,
    version: SemVer
}

#[derive(Clone, Debug)]
pub enum ManifestModule {
    Data(Uuid, SemVer),
    Resources(Uuid, SemVer),
    Script(Uuid, SemVer, ScriptManifestModule)
}

#[derive(Clone, Debug)]
pub struct ScriptManifestModule {
    pub entry: String
}

#[derive(Clone, Debug)]
pub enum ManifestDependency {
    ScriptDependency(ScriptManifestDependency, SemVer),
    UuidDependency(Uuid, SemVer)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ScriptManifestDependency {
    MinecraftServer,
    MinecraftServerUi,
    MinecraftServerNet,
    MinecraftServerGametest,
    MinecraftServerAdmin,
    MinecraftServerEditor,
    MinecraftDebugUtilities,
    Custom(String)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ManifestCapability {
    Chemistry,
    EditorExtension,
    ExperimentalCustomUi,
    PBR,
    ScriptEval,
    Raytraced,
    Custom(String)
}

static SCRIPT_MANIFEST_DEPENDENCIES: Lazy<HashMap<&str, ScriptManifestDependency>> = Lazy::new(||
    HashMap::from(
        [
            ("@minecraft/server", ScriptManifestDependency::MinecraftServer),
            ("@minecraft/server-ui", ScriptManifestDependency::MinecraftServerUi),
            ("@minecraft/server-gametest", ScriptManifestDependency::MinecraftServerGametest),
            ("@minecraft/server-net", ScriptManifestDependency::MinecraftServerNet),
            ("@minecraft/server-admin", ScriptManifestDependency::MinecraftServerAdmin),
            ("@minecraft/server-editor", ScriptManifestDependency::MinecraftServerEditor),
            ("@minecraft/debug-utilities", ScriptManifestDependency::MinecraftDebugUtilities),
        ]
    )
);

static MANIFEST_CAPABILITIES: Lazy<HashMap<&str, ManifestCapability>> = Lazy::new(|| HashMap::from(
    [
        ("raytraced", ManifestCapability::Raytraced),
        ("chemistry", ManifestCapability::Chemistry),
        ("editorExtension", ManifestCapability::EditorExtension),
        ("experimental_custom_ui", ManifestCapability::ExperimentalCustomUi),
        ("pbr", ManifestCapability::PBR),
        ("script_eval", ManifestCapability::ScriptEval),
    ]
));