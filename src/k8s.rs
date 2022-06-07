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
    #[serde(rename = "imagePullPolicy")]
    pub image_pull_policy: String,
    #[serde(rename = "mainApplicationFile")]
    pub main_app_file: String,
    pub mode: String,
    #[serde(rename = "pythonVersion")]
    pub python_version: String,
    #[serde(rename = "sparkVersion")]
    pub spark_version: String,
    #[serde(rename = "type")]
    pub app_type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct Driver {
    #[serde(rename = "coreLimit")]
    pub core_limit: String,
    pub cores: u32,
    pub env: Vec<EnvVar>,
    pub labels: HashMap<String, String>,
    #[serde(rename = "serviceAccount")]
    pub service_account: String,
    pub memory: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct Worker {
    #[serde(rename = "coreLimit")]
    pub core_limit: String,
    pub instances: u32,
    pub cores: u32,
    pub env: Vec<EnvVar>,
    pub labels: HashMap<String, String>,
    #[serde(rename = "serviceAccount")]
    pub service_account: String,
    pub memory: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct EnvVar {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct SparkApplicationStatus {
    #[serde(rename = "applicationState")]
    pub app_state: State,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct State {
    pub state: String,
}
