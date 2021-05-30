#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentExtendedFilter {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericResourceFilter {
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tagname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tagvalue: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceGroupFilter {
    #[serde(rename = "tagName", default, skip_serializing_if = "Option::is_none")]
    pub tag_name: Option<String>,
    #[serde(rename = "tagValue", default, skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemplateLink {
    pub uri: String,
    #[serde(rename = "contentVersion", default, skip_serializing_if = "Option::is_none")]
    pub content_version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParametersLink {
    pub uri: String,
    #[serde(rename = "contentVersion", default, skip_serializing_if = "Option::is_none")]
    pub content_version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<serde_json::Value>,
    #[serde(rename = "templateLink", default, skip_serializing_if = "Option::is_none")]
    pub template_link: Option<TemplateLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[serde(rename = "parametersLink", default, skip_serializing_if = "Option::is_none")]
    pub parameters_link: Option<ParametersLink>,
    pub mode: deployment_properties::Mode,
    #[serde(rename = "debugSetting", default, skip_serializing_if = "Option::is_none")]
    pub debug_setting: Option<DebugSetting>,
    #[serde(rename = "onErrorDeployment", default, skip_serializing_if = "Option::is_none")]
    pub on_error_deployment: Option<OnErrorDeployment>,
}
pub mod deployment_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Mode {
        Incremental,
        Complete,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DebugSetting {
    #[serde(rename = "detailLevel", default, skip_serializing_if = "Option::is_none")]
    pub detail_level: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Deployment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    pub properties: DeploymentProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScopedDeployment {
    pub location: String,
    pub properties: DeploymentProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentExportResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentWhatIf {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    pub properties: DeploymentWhatIfProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentWhatIfProperties {
    #[serde(flatten)]
    pub deployment_properties: DeploymentProperties,
    #[serde(rename = "whatIfSettings", default, skip_serializing_if = "Option::is_none")]
    pub what_if_settings: Option<DeploymentWhatIfSettings>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentWhatIfSettings {
    #[serde(rename = "resultFormat", default, skip_serializing_if = "Option::is_none")]
    pub result_format: Option<deployment_what_if_settings::ResultFormat>,
}
pub mod deployment_what_if_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResultFormat {
        ResourceIdOnly,
        FullResourcePayloads,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AliasPathType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "apiVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub api_versions: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AliasType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<AliasPathType>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProviderResourceType {
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<AliasType>,
    #[serde(rename = "apiVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub api_versions: Vec<String>,
    #[serde(rename = "zoneMappings", default, skip_serializing_if = "Vec::is_empty")]
    pub zone_mappings: Vec<ZoneMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Provider {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "registrationState", skip_serializing)]
    pub registration_state: Option<String>,
    #[serde(rename = "registrationPolicy", skip_serializing)]
    pub registration_policy: Option<String>,
    #[serde(rename = "resourceTypes", skip_serializing)]
    pub resource_types: Vec<ProviderResourceType>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BasicDependency {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dependency {
    #[serde(rename = "dependsOn", default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<BasicDependency>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentPropertiesExtended {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
    #[serde(rename = "correlationId", skip_serializing)]
    pub correlation_id: Option<String>,
    #[serde(skip_serializing)]
    pub timestamp: Option<String>,
    #[serde(skip_serializing)]
    pub duration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub providers: Vec<Provider>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<Dependency>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<serde_json::Value>,
    #[serde(rename = "templateLink", default, skip_serializing_if = "Option::is_none")]
    pub template_link: Option<TemplateLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[serde(rename = "parametersLink", default, skip_serializing_if = "Option::is_none")]
    pub parameters_link: Option<ParametersLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<deployment_properties_extended::Mode>,
    #[serde(rename = "debugSetting", default, skip_serializing_if = "Option::is_none")]
    pub debug_setting: Option<DebugSetting>,
    #[serde(rename = "onErrorDeployment", default, skip_serializing_if = "Option::is_none")]
    pub on_error_deployment: Option<OnErrorDeploymentExtended>,
}
pub mod deployment_properties_extended {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Mode {
        Incremental,
        Complete,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnErrorDeployment {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<on_error_deployment::Type>,
    #[serde(rename = "deploymentName", default, skip_serializing_if = "Option::is_none")]
    pub deployment_name: Option<String>,
}
pub mod on_error_deployment {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        LastSuccessful,
        SpecificDeployment,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnErrorDeploymentExtended {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<on_error_deployment_extended::Type>,
    #[serde(rename = "deploymentName", default, skip_serializing_if = "Option::is_none")]
    pub deployment_name: Option<String>,
}
pub mod on_error_deployment_extended {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        LastSuccessful,
        SpecificDeployment,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentValidateResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeploymentPropertiesExtended>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentExtended {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeploymentPropertiesExtended>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DeploymentExtended>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProviderListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Provider>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericResourceExpanded {
    #[serde(flatten)]
    pub generic_resource: GenericResource,
    #[serde(rename = "createdTime", skip_serializing)]
    pub created_time: Option<String>,
    #[serde(rename = "changedTime", skip_serializing)]
    pub changed_time: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(rename = "promotionCode", default, skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    #[serde(rename = "principalId", skip_serializing)]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity::Type>,
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
pub mod identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned, UserAssigned")]
        SystemAssignedUserAssigned,
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GenericResourceExpanded>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceGroup {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ResourceGroupProperties>,
    pub location: String,
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceGroupPatchable {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ResourceGroupProperties>,
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceGroupProperties {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceGroupListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceGroup>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourcesMoveInfo {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,
    #[serde(rename = "targetResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_group: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportTemplateRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagCount {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagValue {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "tagValue", default, skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<TagCount>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagDetails {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "tagName", default, skip_serializing_if = "Option::is_none")]
    pub tag_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<TagCount>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<TagValue>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TagDetails>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpMessage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentOperationProperties {
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
    #[serde(skip_serializing)]
    pub timestamp: Option<String>,
    #[serde(skip_serializing)]
    pub duration: Option<String>,
    #[serde(rename = "serviceRequestId", skip_serializing)]
    pub service_request_id: Option<String>,
    #[serde(rename = "statusCode", skip_serializing)]
    pub status_code: Option<String>,
    #[serde(rename = "statusMessage", skip_serializing)]
    pub status_message: Option<serde_json::Value>,
    #[serde(rename = "targetResource", default, skip_serializing_if = "Option::is_none")]
    pub target_resource: Option<TargetResource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<HttpMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<HttpMessage>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentOperation {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(rename = "operationId", skip_serializing)]
    pub operation_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeploymentOperationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentOperationsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DeploymentOperation>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceProviderOperationDisplayProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceGroupExportResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemplateHashResult {
    #[serde(rename = "minifiedTemplate", default, skip_serializing_if = "Option::is_none")]
    pub minified_template: Option<String>,
    #[serde(rename = "templateHash", default, skip_serializing_if = "Option::is_none")]
    pub template_hash: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WhatIfPropertyChange {
    pub path: String,
    #[serde(rename = "propertyChangeType")]
    pub property_change_type: what_if_property_change::PropertyChangeType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<WhatIfPropertyChange>,
}
pub mod what_if_property_change {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PropertyChangeType {
        Create,
        Delete,
        Modify,
        Array,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WhatIfChange {
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[serde(rename = "changeType")]
    pub change_type: what_if_change::ChangeType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub delta: Vec<WhatIfPropertyChange>,
}
pub mod what_if_change {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ChangeType {
        Create,
        Delete,
        Ignore,
        Deploy,
        NoChange,
        Modify,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WhatIfOperationProperties {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub changes: Vec<WhatIfChange>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WhatIfOperationResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WhatIfOperationProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZoneMapping {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub target: Option<String>,
    #[serde(skip_serializing)]
    pub details: Vec<ErrorResponse>,
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
