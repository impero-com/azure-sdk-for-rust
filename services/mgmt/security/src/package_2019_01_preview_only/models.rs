#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegulatoryComplianceStandardList {
    pub value: Vec<RegulatoryComplianceStandard>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegulatoryComplianceStandard {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegulatoryComplianceStandardProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegulatoryComplianceStandardProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<regulatory_compliance_standard_properties::State>,
    #[serde(rename = "passedControls", skip_serializing)]
    pub passed_controls: Option<i64>,
    #[serde(rename = "failedControls", skip_serializing)]
    pub failed_controls: Option<i64>,
    #[serde(rename = "skippedControls", skip_serializing)]
    pub skipped_controls: Option<i64>,
    #[serde(rename = "unsupportedControls", skip_serializing)]
    pub unsupported_controls: Option<i64>,
}
pub mod regulatory_compliance_standard_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Passed,
        Failed,
        Skipped,
        Unsupported,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegulatoryComplianceControlList {
    pub value: Vec<RegulatoryComplianceControl>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegulatoryComplianceControl {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegulatoryComplianceControlProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegulatoryComplianceControlProperties {
    #[serde(skip_serializing)]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<regulatory_compliance_control_properties::State>,
    #[serde(rename = "passedAssessments", skip_serializing)]
    pub passed_assessments: Option<i64>,
    #[serde(rename = "failedAssessments", skip_serializing)]
    pub failed_assessments: Option<i64>,
    #[serde(rename = "skippedAssessments", skip_serializing)]
    pub skipped_assessments: Option<i64>,
}
pub mod regulatory_compliance_control_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Passed,
        Failed,
        Skipped,
        Unsupported,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegulatoryComplianceAssessmentList {
    pub value: Vec<RegulatoryComplianceAssessment>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegulatoryComplianceAssessment {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegulatoryComplianceAssessmentProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegulatoryComplianceAssessmentProperties {
    #[serde(skip_serializing)]
    pub description: Option<String>,
    #[serde(rename = "assessmentType", skip_serializing)]
    pub assessment_type: Option<String>,
    #[serde(rename = "assessmentDetailsLink", skip_serializing)]
    pub assessment_details_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<regulatory_compliance_assessment_properties::State>,
    #[serde(rename = "passedResources", skip_serializing)]
    pub passed_resources: Option<i64>,
    #[serde(rename = "failedResources", skip_serializing)]
    pub failed_resources: Option<i64>,
    #[serde(rename = "skippedResources", skip_serializing)]
    pub skipped_resources: Option<i64>,
    #[serde(rename = "unsupportedResources", skip_serializing)]
    pub unsupported_resources: Option<i64>,
}
pub mod regulatory_compliance_assessment_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Passed,
        Failed,
        Skipped,
        Unsupported,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertsSuppressionRulesList {
    pub value: Vec<AlertsSuppressionRule>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertsSuppressionRule {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertsSuppressionRuleProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertsSuppressionRuleProperties {
    #[serde(rename = "alertType")]
    pub alert_type: String,
    #[serde(rename = "lastModifiedUtc", skip_serializing)]
    pub last_modified_utc: Option<String>,
    #[serde(rename = "expirationDateUtc", default, skip_serializing_if = "Option::is_none")]
    pub expiration_date_utc: Option<String>,
    pub reason: String,
    pub state: alerts_suppression_rule_properties::State,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "suppressionAlertsScope", default, skip_serializing_if = "Option::is_none")]
    pub suppression_alerts_scope: Option<SuppressionAlertsScope>,
}
pub mod alerts_suppression_rule_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Disabled,
        Expired,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScopeElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuppressionAlertsScope {
    #[serde(rename = "allOf")]
    pub all_of: Vec<ScopeElement>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityAssessmentMetadataList {
    #[serde(skip_serializing)]
    pub value: Vec<SecurityAssessmentMetadata>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityAssessmentMetadata {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityAssessmentMetadataProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityAssessmentMetadataProperties {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "policyDefinitionId", skip_serializing)]
    pub policy_definition_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "remediationDescription", default, skip_serializing_if = "Option::is_none")]
    pub remediation_description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<String>,
    pub severity: security_assessment_metadata_properties::Severity,
    #[serde(rename = "userImpact", default, skip_serializing_if = "Option::is_none")]
    pub user_impact: Option<security_assessment_metadata_properties::UserImpact>,
    #[serde(rename = "implementationEffort", default, skip_serializing_if = "Option::is_none")]
    pub implementation_effort: Option<security_assessment_metadata_properties::ImplementationEffort>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub threats: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview: Option<bool>,
    #[serde(rename = "assessmentType")]
    pub assessment_type: security_assessment_metadata_properties::AssessmentType,
}
pub mod security_assessment_metadata_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Severity {
        Low,
        Medium,
        High,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UserImpact {
        Low,
        Moderate,
        High,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ImplementationEffort {
        Low,
        Moderate,
        High,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AssessmentType {
        BuiltIn,
        CustomPolicy,
        CustomerManaged,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityAssessmentList {
    #[serde(skip_serializing)]
    pub value: Vec<SecurityAssessment>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityAssessment {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SecurityAssessmentProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityAssessmentProperties {
    #[serde(rename = "resourceDetails")]
    pub resource_details: ResourceDetails,
    #[serde(rename = "displayName", skip_serializing)]
    pub display_name: Option<String>,
    pub status: AssessmentStatus,
    #[serde(rename = "additionalData", default, skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<serde_json::Value>,
    #[serde(skip_serializing)]
    pub links: Option<AssessmentLinks>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssessmentLinks {
    #[serde(rename = "azurePortalUri", skip_serializing)]
    pub azure_portal_uri: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssessmentStatus {
    pub code: assessment_status::Code,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
pub mod assessment_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Code {
        Healthy,
        Unhealthy,
        NotApplicable,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudErrorBody {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub target: Option<String>,
    #[serde(skip_serializing)]
    pub details: Vec<CloudErrorBody>,
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
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceDetails {
    pub source: resource_details::Source,
}
pub mod resource_details {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Source {
        Azure,
        OnPremise,
        OnPremiseSql,
    }
}
