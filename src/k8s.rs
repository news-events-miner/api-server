use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "sparkoperator.k8s.io",
    version = "v1beta2",
    kind = "SparkApplication",
    namespaced,
    status = "SparkApplicationStatus"
)]
pub struct SparkApplicationSpec {
    pub driver: Driver,
    pub executor: Worker,
    pub image: String,
    pub imagePullPolicy: String,
    pub mainApplicationFile: String,
    pub mode: String,
    pub pythonVersion: String,
    pub sparkVersion: String,
    #[serde(rename(serialize = "type", deserialize = "Type"))]
    #[serde(alias = "type")]
    pub Type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct Driver {
    pub coreLimit: String,
    pub cores: u32,
    pub env: Vec<EnvVar>,
    pub labels: HashMap<String, String>,
    pub serviceAccount: String,
    pub memory: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct Worker {
    pub coreLimit: String,
    pub instances: u32,
    pub cores: u32,
    pub env: Vec<EnvVar>,
    pub labels: HashMap<String, String>,
    pub serviceAccount: String,
    pub memory: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct EnvVar {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct SparkApplicationStatus {
    pub applicationState: State,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct State {
    pub state: String,
}
