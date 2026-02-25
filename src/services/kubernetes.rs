use anyhow::{anyhow, Result};
use k8s_openapi::api::core::v1::{Namespace, Secret, ServiceAccount};
use k8s_openapi::api::rbac::v1::{ClusterRoleBinding, RoleBinding, RoleRef, Subject};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::{
    api::{Api, DeleteParams, ListParams, PostParams},
    Client, ResourceExt,
};
use serde::Serialize;
use serde_json::json;
use std::collections::BTreeMap;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use crate::config::{rbac, sa};

// Template Helpers
fn role_binding_name(username: &str, namespace: &str, permission: &str) -> String {
    format!("{}___template-namespaced-resources___{}___{}", username, permission, namespace)
}

fn role_name(permission: &str) -> String {
    format!("template-namespaced-resources___{}", permission)
}

fn cluster_role_binding_name(username: &str, permission: &str) -> String {
    format!("{}___template-cluster-resources___{}", username, permission)
}

fn cluster_role_name(permission: &str) -> String {
    format!("template-cluster-resources___{}", permission)
}

fn get_current_namespace() -> String {
    std::fs::read_to_string("/var/run/secrets/kubernetes.io/serviceaccount/namespace")
        .unwrap_or_else(|_| "kanrigate".to_string())
        .trim()
        .to_string()
}

// Structs for KubeConfig generation
#[derive(Serialize)]
struct KubeConfig {
    #[serde(rename = "apiVersion")]
    api_version: String,
    kind: String,
    clusters: Vec<NamedCluster>,
    contexts: Vec<NamedContext>,
    #[serde(rename = "current-context")]
    current_context: String,
    users: Vec<NamedUser>,
}

#[derive(Serialize)]
struct NamedCluster {
    name: String,
    cluster: ClusterConfig,
}

#[derive(Serialize)]
struct ClusterConfig {
    #[serde(rename = "certificate-authority-data")]
    certificate_authority_data: String,
    server: String,
}

#[derive(Serialize)]
struct NamedContext {
    name: String,
    context: ContextConfig,
}

#[derive(Serialize)]
struct ContextConfig {
    cluster: String,
    namespace: String,
    user: String,
}

#[derive(Serialize)]
struct NamedUser {
    name: String,
    user: UserConfig,
}

#[derive(Serialize)]
struct UserConfig {
    token: String,
}

#[derive(Clone)]
pub struct KubeOps {
    client: Client,
    current_ns: String,
}

impl KubeOps {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            current_ns: get_current_namespace(),
        }
    }

    pub async fn get_namespaces(&self) -> Result<Vec<String>> {
        let api: Api<Namespace> = Api::all(self.client.clone());
        let list = api.list(&ListParams::default()).await?;
        Ok(list.iter().map(|ns| ns.name_any()).collect())
    }

    pub async fn get_service_accounts(&self) -> Result<Vec<String>> {
        let api: Api<ServiceAccount> = Api::namespaced(self.client.clone(), &self.current_ns);
        let list = api.list(&ListParams::default()).await?;
        Ok(list.iter().map(|sa| sa.name_any()).collect())
    }

    pub async fn create_service_account(&self, username: &str) -> Result<String> {
        let api: Api<ServiceAccount> = Api::namespaced(self.client.clone(), &self.current_ns);
        let sa = ServiceAccount {
            metadata: ObjectMeta {
                name: Some(username.to_string()),
                ..Default::default()
            },
            ..Default::default()
        };
        api.create(&PostParams::default(), &sa).await?;
        Ok(username.to_string())
    }

    pub async fn delete_service_account(&self, username: &str) -> Result<String> {
        let api: Api<ServiceAccount> = Api::namespaced(self.client.clone(), &self.current_ns);
        api.delete(username, &DeleteParams::default()).await?;
        Ok(username.to_string())
    }

    pub async fn create_secret(&self, username: &str) -> Result<String> {
        let api: Api<Secret> = Api::namespaced(self.client.clone(), &self.current_ns);
        
        // Check if secret exists via annotation
        let list = api.list(&ListParams::default()).await?;
        let secret_name = list.items.iter().find_map(|s| {
            s.metadata.annotations.as_ref()
                .and_then(|a| a.get(sa::ANNOTATIONS_NAME))
                .filter(|v| *v == username)
                .map(|_| s.name_any())
        });

        if let Some(name) = secret_name {
            // Already exists
            return Ok(name);
        }

        let token_secret_name = format!("{}-token", username);
        let mut annotations = BTreeMap::new();
        annotations.insert(sa::ANNOTATIONS_NAME.to_string(), username.to_string());

        let secret = Secret {
            metadata: ObjectMeta {
                name: Some(token_secret_name.clone()),
                annotations: Some(annotations),
                ..Default::default()
            },
            type_: Some(sa::TYPE_TOKEN.to_string()),
            ..Default::default()
        };

        api.create(&PostParams::default(), &secret).await?;
        Ok(token_secret_name)
    }

    pub async fn delete_secret(&self, username: &str) -> Result<Vec<String>> {
        let api: Api<Secret> = Api::namespaced(self.client.clone(), &self.current_ns);
        let list = api.list(&ListParams::default()).await?;
        
        let mut deleted = Vec::new();
        for secret in list.items {
            if let Some(annotations) = &secret.metadata.annotations {
                if let Some(sa_name) = annotations.get(sa::ANNOTATIONS_NAME) {
                    if sa_name == username {
                        let name = secret.name_any();
                        api.delete(&name, &DeleteParams::default()).await?;
                        deleted.push(name);
                    }
                }
            }
        }
        Ok(deleted)
    }

    pub async fn create_role_binding(&self, username: &str, namespace: &str, permission: &str) -> Result<String> {
        let api: Api<RoleBinding> = Api::namespaced(self.client.clone(), namespace);
        let name = role_binding_name(username, namespace, permission);
        let role_ref = role_name(permission);

        let rb = RoleBinding {
            metadata: ObjectMeta {
                name: Some(name.clone()),
                ..Default::default()
            },
            role_ref: RoleRef {
                api_group: rbac::API_GROUP.to_string(),
                kind: "ClusterRole".to_string(),
                name: role_ref,
            },
            subjects: Some(vec![Subject {
                kind: "ServiceAccount".to_string(),
                name: username.to_string(),
                namespace: Some(self.current_ns.clone()),
                ..Default::default()
            }]),
        };

        api.create(&PostParams::default(), &rb).await?;
        Ok(name)
    }

    pub async fn delete_role_binding(&self, username: &str, namespace: &str, permission: &str) -> Result<String> {
        let api: Api<RoleBinding> = Api::namespaced(self.client.clone(), namespace);
        let name = role_binding_name(username, namespace, permission);
        api.delete(&name, &DeleteParams::default()).await?;
        Ok(name)
    }

    pub async fn create_cluster_role_binding(&self, username: &str, permission: &str) -> Result<String> {
        let api: Api<ClusterRoleBinding> = Api::all(self.client.clone());
        let name = cluster_role_binding_name(username, permission);
        let role_ref = cluster_role_name(permission);

        let crb = ClusterRoleBinding {
            metadata: ObjectMeta {
                name: Some(name.clone()),
                ..Default::default()
            },
            role_ref: RoleRef {
                api_group: rbac::API_GROUP.to_string(),
                kind: "ClusterRole".to_string(),
                name: role_ref,
            },
            subjects: Some(vec![Subject {
                kind: "ServiceAccount".to_string(),
                name: username.to_string(),
                namespace: Some(self.current_ns.clone()),
                ..Default::default()
            }]),
        };

        api.create(&PostParams::default(), &crb).await?;
        Ok(name)
    }

    pub async fn delete_cluster_role_binding(&self, username: &str, permission: &str) -> Result<String> {
        let api: Api<ClusterRoleBinding> = Api::all(self.client.clone());
        let name = cluster_role_binding_name(username, permission);
        api.delete(&name, &DeleteParams::default()).await?;
        Ok(name)
    }

    pub async fn get_filtered_role_bindings(&self, username: &str) -> Result<serde_json::Value> {
        // This iterates all namespaces and checks RBs.
        let namespaces = self.get_namespaces().await?;
        let mut ns_permissions = serde_json::Map::new();

        for ns in namespaces {
            let api: Api<RoleBinding> = Api::namespaced(self.client.clone(), &ns);
            // This is n+1 but mimicking logic. Optimized implementations would use label selectors if possible
            if let Ok(list) = api.list(&ListParams::default()).await {
                for rb in list.items {
                    let has_user = rb.subjects.as_ref().map_or(false, |subs| {
                        subs.iter().any(|s| s.name == username)
                    });

                    if has_user {
                        let role_ref_name = rb.role_ref.name;
                        // Attempt to parse permission from role name if it follows convention
                        let parts: Vec<&str> = role_ref_name.split("___").collect();
                        let role_short = if parts.len() > 1 { parts[1] } else { &role_ref_name };
                        
                        let entry = ns_permissions.entry(ns.clone()).or_insert(json!({}));
                        if let Some(obj) = entry.as_object_mut() {
                            obj.insert(role_short.to_string(), json!(true));
                        }
                    }
                }
            }
        }
        Ok(serde_json::Value::Object(ns_permissions))
    }

    pub async fn get_filtered_cluster_role_bindings(&self, username: &str) -> Result<serde_json::Value> {
        let api: Api<ClusterRoleBinding> = Api::all(self.client.clone());
        let list = api.list(&ListParams::default()).await?;
        let mut cluster_permissions = serde_json::Map::new();

        for crb in list.items {
            let has_user = crb.subjects.as_ref().map_or(false, |subs| {
                subs.iter().any(|s| s.name == username)
            });

            if has_user {
                let role_ref_name = crb.role_ref.name;
                let parts: Vec<&str> = role_ref_name.split("___").collect();
                let role_short = if parts.len() > 1 { parts[1] } else { &role_ref_name };
                cluster_permissions.insert(role_short.to_string(), json!(true));
            }
        }
        Ok(serde_json::Value::Object(cluster_permissions))
    }

    pub async fn generate_k8s_config(&self, username: &str, namespace: &str, cluster_name: &str, control_plane: &str) -> Result<String> {
        let api: Api<Secret> = Api::namespaced(self.client.clone(), &self.current_ns);
        let list = api.list(&ListParams::default()).await?;
        
        let secret = list.items.iter().find(|s| {
            s.metadata.annotations.as_ref()
                .and_then(|a| a.get(sa::ANNOTATIONS_NAME))
                .map_or(false, |v| v == username)
        }).ok_or_else(|| anyhow!("No secret found for user {}", username))?;

        let data = secret.data.as_ref().ok_or_else(|| anyhow!("Secret has no data"))?;
        
        let token_bytes = &data.get("token").ok_or_else(|| anyhow!("Secret missing token"))?.0;
        let ca_cert_bytes = &data.get("ca.crt").ok_or_else(|| anyhow!("Secret missing ca.crt"))?.0;

        let token = String::from_utf8(token_bytes.clone())?;
        let ca_cert_b64 = BASE64.encode(ca_cert_bytes);

        let context_name = format!("{}-{}@{}", username, namespace, cluster_name);

        let config = KubeConfig {
            api_version: "v1".to_string(),
            kind: "Config".to_string(),
            clusters: vec![NamedCluster {
                name: cluster_name.to_string(),
                cluster: ClusterConfig {
                    certificate_authority_data: ca_cert_b64,
                    server: control_plane.to_string(),
                },
            }],
            contexts: vec![NamedContext {
                name: context_name.clone(),
                context: ContextConfig {
                    cluster: cluster_name.to_string(),
                    namespace: namespace.to_string(),
                    user: username.to_string(),
                },
            }],
            current_context: context_name,
            users: vec![NamedUser {
                name: username.to_string(),
                user: UserConfig {
                    token,
                },
            }],
        };

        serde_yaml::to_string(&config).map_err(|e| anyhow!(e))
    }
}
