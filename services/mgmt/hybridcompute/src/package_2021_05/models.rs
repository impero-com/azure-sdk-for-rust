#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing)]
    pub value: Vec<OperationValue>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationValue {
    #[serde(skip_serializing)]
    pub origin: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationValueDisplay>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationValueDisplay {
    #[serde(skip_serializing)]
    pub operation: Option<String>,
    #[serde(skip_serializing)]
    pub resource: Option<String>,
    #[serde(skip_serializing)]
    pub description: Option<String>,
    #[serde(skip_serializing)]
    pub provider: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OsProfile {
    #[serde(rename = "computerName", skip_serializing)]
    pub computer_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DetectedProperties {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineProperties {
    #[serde(rename = "locationData", default, skip_serializing_if = "Option::is_none")]
    pub location_data: Option<LocationData>,
    #[serde(rename = "osProfile", skip_serializing)]
    pub os_profile: Option<OsProfile>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
    #[serde(skip_serializing)]
    pub status: Option<machine_properties::Status>,
    #[serde(rename = "lastStatusChange", skip_serializing)]
    pub last_status_change: Option<String>,
    #[serde(rename = "errorDetails", skip_serializing)]
    pub error_details: Vec<ErrorDetail>,
    #[serde(rename = "agentVersion", skip_serializing)]
    pub agent_version: Option<String>,
    #[serde(rename = "vmId", default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    #[serde(rename = "displayName", skip_serializing)]
    pub display_name: Option<String>,
    #[serde(rename = "machineFqdn", skip_serializing)]
    pub machine_fqdn: Option<String>,
    #[serde(rename = "clientPublicKey", default, skip_serializing_if = "Option::is_none")]
    pub client_public_key: Option<String>,
    #[serde(rename = "osName", skip_serializing)]
    pub os_name: Option<String>,
    #[serde(rename = "osVersion", skip_serializing)]
    pub os_version: Option<String>,
    #[serde(rename = "vmUuid", skip_serializing)]
    pub vm_uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<MachineExtensionInstanceView>,
    #[serde(rename = "osSku", skip_serializing)]
    pub os_sku: Option<String>,
    #[serde(rename = "domainName", skip_serializing)]
    pub domain_name: Option<String>,
    #[serde(rename = "adFqdn", skip_serializing)]
    pub ad_fqdn: Option<String>,
    #[serde(rename = "dnsFqdn", skip_serializing)]
    pub dns_fqdn: Option<String>,
    #[serde(rename = "privateLinkScopeResourceId", default, skip_serializing_if = "Option::is_none")]
    pub private_link_scope_resource_id: Option<String>,
    #[serde(rename = "parentClusterResourceId", default, skip_serializing_if = "Option::is_none")]
    pub parent_cluster_resource_id: Option<String>,
    #[serde(rename = "detectedProperties", skip_serializing)]
    pub detected_properties: Option<DetectedProperties>,
}
pub mod machine_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Connected,
        Disconnected,
        Error,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineUpdateProperties {
    #[serde(rename = "locationData", default, skip_serializing_if = "Option::is_none")]
    pub location_data: Option<LocationData>,
    #[serde(rename = "parentClusterResourceId", default, skip_serializing_if = "Option::is_none")]
    pub parent_cluster_resource_id: Option<String>,
    #[serde(rename = "privateLinkScopeResourceId", default, skip_serializing_if = "Option::is_none")]
    pub private_link_scope_resource_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Machine {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MachineProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineUpdate {
    #[serde(flatten)]
    pub resource_update: ResourceUpdate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MachineUpdateProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineListResult {
    pub value: Vec<Machine>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineExtension {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MachineExtensionProperties>,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineExtensionUpdate {
    #[serde(flatten)]
    pub resource_update: ResourceUpdate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MachineExtensionUpdateProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineExtensionProperties {
    #[serde(rename = "forceUpdateTag", default, skip_serializing_if = "Option::is_none")]
    pub force_update_tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "typeHandlerVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_handler_version: Option<String>,
    #[serde(rename = "autoUpgradeMinorVersion", default, skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_minor_version: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    #[serde(rename = "protectedSettings", default, skip_serializing_if = "Option::is_none")]
    pub protected_settings: Option<serde_json::Value>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
    #[serde(rename = "instanceView", default, skip_serializing_if = "Option::is_none")]
    pub instance_view: Option<MachineExtensionInstanceView>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineExtensionUpdateProperties {
    #[serde(rename = "forceUpdateTag", default, skip_serializing_if = "Option::is_none")]
    pub force_update_tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "typeHandlerVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_handler_version: Option<String>,
    #[serde(rename = "autoUpgradeMinorVersion", default, skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_minor_version: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    #[serde(rename = "protectedSettings", default, skip_serializing_if = "Option::is_none")]
    pub protected_settings: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineExtensionInstanceView {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "typeHandlerVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_handler_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<machine_extension_instance_view::Status>,
}
pub mod machine_extension_instance_view {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Status {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub level: Option<status::Level>,
        #[serde(rename = "displayStatus", default, skip_serializing_if = "Option::is_none")]
        pub display_status: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub time: Option<String>,
    }
    pub mod status {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Level {
            Info,
            Warning,
            Error,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineExtensionsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MachineExtension>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetVersion {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtensionTargetProperties {
    #[serde(rename = "targetVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_version: Option<TargetVersion>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtensionTarget {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineExtensionUpgrade {
    #[serde(rename = "extensionTargets", default, skip_serializing_if = "Option::is_none")]
    pub extension_targets: Option<ExtensionTarget>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkScopesResource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagsResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HybridComputePrivateLinkScope {
    #[serde(flatten)]
    pub private_link_scopes_resource: PrivateLinkScopesResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HybridComputePrivateLinkScopeProperties>,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkScopeValidationDetails {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<PublicNetworkAccessType>,
    #[serde(rename = "connectionDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub connection_details: Vec<ConnectionDetail>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionDetail {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "privateIpAddress", skip_serializing)]
    pub private_ip_address: Option<String>,
    #[serde(rename = "linkIdentifier", skip_serializing)]
    pub link_identifier: Option<String>,
    #[serde(rename = "groupId", skip_serializing)]
    pub group_id: Option<String>,
    #[serde(rename = "memberName", skip_serializing)]
    pub member_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HybridComputePrivateLinkScopeProperties {
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<PublicNetworkAccessType>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
    #[serde(rename = "privateLinkScopeId", skip_serializing)]
    pub private_link_scope_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HybridComputePrivateLinkScopeListResult {
    pub value: Vec<HybridComputePrivateLinkScope>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourceListResult {
    #[serde(skip_serializing)]
    pub value: Vec<PrivateLinkResource>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourceProperties {
    #[serde(rename = "groupId", skip_serializing)]
    pub group_id: Option<String>,
    #[serde(rename = "requiredMembers", skip_serializing)]
    pub required_members: Vec<String>,
    #[serde(rename = "requiredZoneNames", skip_serializing)]
    pub required_zone_names: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpointProperty>,
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionStateProperty>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServiceConnectionStateProperty {
    pub status: String,
    pub description: String,
    #[serde(rename = "actionsRequired", skip_serializing)]
    pub actions_required: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
    #[serde(rename = "systemData", skip_serializing)]
    pub system_data: Option<SystemData>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionListResult {
    #[serde(skip_serializing)]
    pub value: Vec<PrivateEndpointConnection>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PublicNetworkAccessType {
    Enabled,
    Disabled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub target: Option<String>,
    #[serde(skip_serializing)]
    pub details: Vec<ErrorDetail>,
    #[serde(rename = "additionalInfo", skip_serializing)]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorAdditionalInfo {
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub info: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocationData {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    #[serde(rename = "countryOrRegion", default, skip_serializing_if = "Option::is_none")]
    pub country_or_region: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    #[serde(rename = "principalId", skip_serializing)]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity::Type>,
}
pub mod identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemData {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[serde(rename = "lastModifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
pub mod system_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
