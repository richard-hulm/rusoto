// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
/// <p>Contains information about a backup of an AWS CloudHSM cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Backup {
    /// <p>The identifier (ID) of the backup.</p>
    #[serde(rename = "BackupId")]
    pub backup_id: String,
    /// <p>The state of the backup.</p>
    #[serde(rename = "BackupState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_state: Option<String>,
    /// <p>The identifier (ID) of the cluster that was backed up.</p>
    #[serde(rename = "ClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "CopyTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_timestamp: Option<f64>,
    /// <p>The date and time when the backup was created.</p>
    #[serde(rename = "CreateTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_timestamp: Option<f64>,
    /// <p>The date and time when the backup will be permanently deleted.</p>
    #[serde(rename = "DeleteTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_timestamp: Option<f64>,
    #[serde(rename = "SourceBackup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup: Option<String>,
    #[serde(rename = "SourceCluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_cluster: Option<String>,
    #[serde(rename = "SourceRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_region: Option<String>,
}

/// <p>Contains one or more certificates or a certificate signing request (CSR).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Certificates {
    /// <p>The HSM hardware certificate issued (signed) by AWS CloudHSM.</p>
    #[serde(rename = "AwsHardwareCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_hardware_certificate: Option<String>,
    /// <p>The cluster certificate issued (signed) by the issuing certificate authority (CA) of the cluster's owner.</p>
    #[serde(rename = "ClusterCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_certificate: Option<String>,
    /// <p>The cluster's certificate signing request (CSR). The CSR exists only when the cluster's state is <code>UNINITIALIZED</code>.</p>
    #[serde(rename = "ClusterCsr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_csr: Option<String>,
    /// <p>The HSM certificate issued (signed) by the HSM hardware.</p>
    #[serde(rename = "HsmCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_certificate: Option<String>,
    /// <p>The HSM hardware certificate issued (signed) by the hardware manufacturer.</p>
    #[serde(rename = "ManufacturerHardwareCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer_hardware_certificate: Option<String>,
}

/// <p>Contains information about an AWS CloudHSM cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Cluster {
    /// <p>The cluster's backup policy.</p>
    #[serde(rename = "BackupPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_policy: Option<String>,
    /// <p>Contains one or more certificates or a certificate signing request (CSR).</p>
    #[serde(rename = "Certificates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Certificates>,
    /// <p>The cluster's identifier (ID).</p>
    #[serde(rename = "ClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// <p>The date and time when the cluster was created.</p>
    #[serde(rename = "CreateTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_timestamp: Option<f64>,
    /// <p>The type of HSM that the cluster contains.</p>
    #[serde(rename = "HsmType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_type: Option<String>,
    /// <p>Contains information about the HSMs in the cluster.</p>
    #[serde(rename = "Hsms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsms: Option<Vec<Hsm>>,
    /// <p>The default password for the cluster's Pre-Crypto Officer (PRECO) user.</p>
    #[serde(rename = "PreCoPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_co_password: Option<String>,
    /// <p>The identifier (ID) of the cluster's security group.</p>
    #[serde(rename = "SecurityGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group: Option<String>,
    /// <p>The identifier (ID) of the backup used to create the cluster. This value exists only when the cluster was created from a backup.</p>
    #[serde(rename = "SourceBackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup_id: Option<String>,
    /// <p>The cluster's state.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A description of the cluster's state.</p>
    #[serde(rename = "StateMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_message: Option<String>,
    /// <p>A map of the cluster's subnets and their corresponding Availability Zones.</p>
    #[serde(rename = "SubnetMapping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_mapping: Option<::std::collections::HashMap<String, String>>,
    /// <p>The identifier (ID) of the virtual private cloud (VPC) that contains the cluster.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CopyBackupToRegionRequest {
    /// <p>The ID of the backup that will be copied to the destination region. </p>
    #[serde(rename = "BackupId")]
    pub backup_id: String,
    /// <p>The AWS region that will contain your copied CloudHSM cluster backup.</p>
    #[serde(rename = "DestinationRegion")]
    pub destination_region: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CopyBackupToRegionResponse {
    /// <p>Information on the backup that will be copied to the destination region, including CreateTimestamp, SourceBackup, SourceCluster, and Source Region. CreateTimestamp of the destination backup will be the same as that of the source backup.</p> <p>You will need to use the <code>sourceBackupID</code> returned in this operation to use the <a>DescribeBackups</a> operation on the backup that will be copied to the destination region.</p>
    #[serde(rename = "DestinationBackup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_backup: Option<DestinationBackup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateClusterRequest {
    /// <p>The type of HSM to use in the cluster. Currently the only allowed value is <code>hsm1.medium</code>.</p>
    #[serde(rename = "HsmType")]
    pub hsm_type: String,
    /// <p>The identifier (ID) of the cluster backup to restore. Use this value to restore the cluster from a backup instead of creating a new cluster. To find the backup ID, use <a>DescribeBackups</a>.</p>
    #[serde(rename = "SourceBackupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup_id: Option<String>,
    /// <p><p>The identifiers (IDs) of the subnets where you are creating the cluster. You must specify at least one subnet. If you specify multiple subnets, they must meet the following criteria:</p> <ul> <li> <p>All subnets must be in the same virtual private cloud (VPC).</p> </li> <li> <p>You can specify only one subnet per Availability Zone.</p> </li> </ul></p>
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateClusterResponse {
    /// <p>Information about the cluster that was created.</p>
    #[serde(rename = "Cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateHsmRequest {
    /// <p>The Availability Zone where you are creating the HSM. To find the cluster's Availability Zones, use <a>DescribeClusters</a>.</p>
    #[serde(rename = "AvailabilityZone")]
    pub availability_zone: String,
    /// <p>The identifier (ID) of the HSM's cluster. To find the cluster ID, use <a>DescribeClusters</a>.</p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
    /// <p>The HSM's IP address. If you specify an IP address, use an available address from the subnet that maps to the Availability Zone where you are creating the HSM. If you don't specify an IP address, one is chosen for you from that subnet.</p>
    #[serde(rename = "IpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateHsmResponse {
    /// <p>Information about the HSM that was created.</p>
    #[serde(rename = "Hsm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm: Option<Hsm>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteBackupRequest {
    /// <p>The ID of the backup to be deleted. To find the ID of a backup, use the <a>DescribeBackups</a> operation.</p>
    #[serde(rename = "BackupId")]
    pub backup_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteBackupResponse {
    /// <p>Information on the <code>Backup</code> object deleted.</p>
    #[serde(rename = "Backup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup: Option<Backup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteClusterRequest {
    /// <p>The identifier (ID) of the cluster that you are deleting. To find the cluster ID, use <a>DescribeClusters</a>.</p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteClusterResponse {
    /// <p>Information about the cluster that was deleted.</p>
    #[serde(rename = "Cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteHsmRequest {
    /// <p>The identifier (ID) of the cluster that contains the HSM that you are deleting.</p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
    /// <p>The identifier (ID) of the elastic network interface (ENI) of the HSM that you are deleting.</p>
    #[serde(rename = "EniId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eni_id: Option<String>,
    /// <p>The IP address of the elastic network interface (ENI) of the HSM that you are deleting.</p>
    #[serde(rename = "EniIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eni_ip: Option<String>,
    /// <p>The identifier (ID) of the HSM that you are deleting.</p>
    #[serde(rename = "HsmId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteHsmResponse {
    /// <p>The identifier (ID) of the HSM that was deleted.</p>
    #[serde(rename = "HsmId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeBackupsRequest {
    /// <p>One or more filters to limit the items returned in the response.</p> <p>Use the <code>backupIds</code> filter to return only the specified backups. Specify backups by their backup identifier (ID).</p> <p>Use the <code>sourceBackupIds</code> filter to return only the backups created from a source backup. The <code>sourceBackupID</code> of a source backup is returned by the <a>CopyBackupToRegion</a> operation.</p> <p>Use the <code>clusterIds</code> filter to return only the backups for the specified clusters. Specify clusters by their cluster identifier (ID).</p> <p>Use the <code>states</code> filter to return only backups that match the specified state.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The maximum number of backups to return in the response. When there are more backups than the number you specify, the response contains a <code>NextToken</code> value.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The <code>NextToken</code> value that you received in the previous response. Use this value to get more backups.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SortAscending")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_ascending: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeBackupsResponse {
    /// <p>A list of backups.</p>
    #[serde(rename = "Backups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backups: Option<Vec<Backup>>,
    /// <p>An opaque string that indicates that the response contains only a subset of backups. Use this value in a subsequent <code>DescribeBackups</code> request to get more backups.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeClustersRequest {
    /// <p>One or more filters to limit the items returned in the response.</p> <p>Use the <code>clusterIds</code> filter to return only the specified clusters. Specify clusters by their cluster identifier (ID).</p> <p>Use the <code>vpcIds</code> filter to return only the clusters in the specified virtual private clouds (VPCs). Specify VPCs by their VPC identifier (ID).</p> <p>Use the <code>states</code> filter to return only clusters that match the specified state.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The maximum number of clusters to return in the response. When there are more clusters than the number you specify, the response contains a <code>NextToken</code> value.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The <code>NextToken</code> value that you received in the previous response. Use this value to get more clusters.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeClustersResponse {
    /// <p>A list of clusters.</p>
    #[serde(rename = "Clusters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<Cluster>>,
    /// <p>An opaque string that indicates that the response contains only a subset of clusters. Use this value in a subsequent <code>DescribeClusters</code> request to get more clusters.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DestinationBackup {
    #[serde(rename = "CreateTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_timestamp: Option<f64>,
    #[serde(rename = "SourceBackup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup: Option<String>,
    #[serde(rename = "SourceCluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_cluster: Option<String>,
    #[serde(rename = "SourceRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_region: Option<String>,
}

/// <p>Contains information about a hardware security module (HSM) in an AWS CloudHSM cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Hsm {
    /// <p>The Availability Zone that contains the HSM.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The identifier (ID) of the cluster that contains the HSM.</p>
    #[serde(rename = "ClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    /// <p>The identifier (ID) of the HSM's elastic network interface (ENI).</p>
    #[serde(rename = "EniId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eni_id: Option<String>,
    /// <p>The IP address of the HSM's elastic network interface (ENI).</p>
    #[serde(rename = "EniIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eni_ip: Option<String>,
    /// <p>The HSM's identifier (ID).</p>
    #[serde(rename = "HsmId")]
    pub hsm_id: String,
    /// <p>The HSM's state.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A description of the HSM's state.</p>
    #[serde(rename = "StateMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_message: Option<String>,
    /// <p>The subnet that contains the HSM's elastic network interface (ENI).</p>
    #[serde(rename = "SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InitializeClusterRequest {
    /// <p>The identifier (ID) of the cluster that you are claiming. To find the cluster ID, use <a>DescribeClusters</a>.</p>
    #[serde(rename = "ClusterId")]
    pub cluster_id: String,
    /// <p>The cluster certificate issued (signed) by your issuing certificate authority (CA). The certificate must be in PEM format and can contain a maximum of 5000 characters.</p>
    #[serde(rename = "SignedCert")]
    pub signed_cert: String,
    /// <p>The issuing certificate of the issuing certificate authority (CA) that issued (signed) the cluster certificate. This can be a root (self-signed) certificate or a certificate chain that begins with the certificate that issued the cluster certificate and ends with a root certificate. The certificate or certificate chain must be in PEM format and can contain a maximum of 5000 characters.</p>
    #[serde(rename = "TrustAnchor")]
    pub trust_anchor: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct InitializeClusterResponse {
    /// <p>The cluster's state.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A description of the cluster's state.</p>
    #[serde(rename = "StateMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_message: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsRequest {
    /// <p>The maximum number of tags to return in the response. When there are more tags than the number you specify, the response contains a <code>NextToken</code> value.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The <code>NextToken</code> value that you received in the previous response. Use this value to get more tags.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The cluster identifier (ID) for the cluster whose tags you are getting. To find the cluster ID, use <a>DescribeClusters</a>.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListTagsResponse {
    /// <p>An opaque string that indicates that the response contains only a subset of tags. Use this value in a subsequent <code>ListTags</code> request to get more tags.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of tags.</p>
    #[serde(rename = "TagList")]
    pub tag_list: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RestoreBackupRequest {
    /// <p>The ID of the backup to be restored. To find the ID of a backup, use the <a>DescribeBackups</a> operation.</p>
    #[serde(rename = "BackupId")]
    pub backup_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RestoreBackupResponse {
    /// <p>Information on the <code>Backup</code> object created.</p>
    #[serde(rename = "Backup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup: Option<Backup>,
}

/// <p>Contains a tag. A tag is a key-value pair.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The key of the tag.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value of the tag.</p>
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>The cluster identifier (ID) for the cluster that you are tagging. To find the cluster ID, use <a>DescribeClusters</a>.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>A list of one or more tags.</p>
    #[serde(rename = "TagList")]
    pub tag_list: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>The cluster identifier (ID) for the cluster whose tags you are removing. To find the cluster ID, use <a>DescribeClusters</a>.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>A list of one or more tag keys for the tags that you are removing. Specify only the tag keys, not the tag values.</p>
    #[serde(rename = "TagKeyList")]
    pub tag_key_list: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UntagResourceResponse {}

/// Errors returned by CopyBackupToRegion
#[derive(Debug, PartialEq)]
pub enum CopyBackupToRegionError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
}

impl CopyBackupToRegionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CopyBackupToRegionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmAccessDeniedException" => {
                    return RusotoError::Service(CopyBackupToRegionError::CloudHsmAccessDenied(
                        err.msg,
                    ))
                }
                "CloudHsmInternalFailureException" => {
                    return RusotoError::Service(CopyBackupToRegionError::CloudHsmInternalFailure(
                        err.msg,
                    ))
                }
                "CloudHsmInvalidRequestException" => {
                    return RusotoError::Service(CopyBackupToRegionError::CloudHsmInvalidRequest(
                        err.msg,
                    ))
                }
                "CloudHsmResourceNotFoundException" => {
                    return RusotoError::Service(CopyBackupToRegionError::CloudHsmResourceNotFound(
                        err.msg,
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(CopyBackupToRegionError::CloudHsmService(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CopyBackupToRegionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CopyBackupToRegionError {
    fn description(&self) -> &str {
        match *self {
            CopyBackupToRegionError::CloudHsmAccessDenied(ref cause) => cause,
            CopyBackupToRegionError::CloudHsmInternalFailure(ref cause) => cause,
            CopyBackupToRegionError::CloudHsmInvalidRequest(ref cause) => cause,
            CopyBackupToRegionError::CloudHsmResourceNotFound(ref cause) => cause,
            CopyBackupToRegionError::CloudHsmService(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCluster
#[derive(Debug, PartialEq)]
pub enum CreateClusterError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
}

impl CreateClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateClusterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmAccessDeniedException" => {
                    return RusotoError::Service(CreateClusterError::CloudHsmAccessDenied(err.msg))
                }
                "CloudHsmInternalFailureException" => {
                    return RusotoError::Service(CreateClusterError::CloudHsmInternalFailure(
                        err.msg,
                    ))
                }
                "CloudHsmInvalidRequestException" => {
                    return RusotoError::Service(CreateClusterError::CloudHsmInvalidRequest(
                        err.msg,
                    ))
                }
                "CloudHsmResourceNotFoundException" => {
                    return RusotoError::Service(CreateClusterError::CloudHsmResourceNotFound(
                        err.msg,
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(CreateClusterError::CloudHsmService(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateClusterError {
    fn description(&self) -> &str {
        match *self {
            CreateClusterError::CloudHsmAccessDenied(ref cause) => cause,
            CreateClusterError::CloudHsmInternalFailure(ref cause) => cause,
            CreateClusterError::CloudHsmInvalidRequest(ref cause) => cause,
            CreateClusterError::CloudHsmResourceNotFound(ref cause) => cause,
            CreateClusterError::CloudHsmService(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateHsm
#[derive(Debug, PartialEq)]
pub enum CreateHsmError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
}

impl CreateHsmError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateHsmError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmAccessDeniedException" => {
                    return RusotoError::Service(CreateHsmError::CloudHsmAccessDenied(err.msg))
                }
                "CloudHsmInternalFailureException" => {
                    return RusotoError::Service(CreateHsmError::CloudHsmInternalFailure(err.msg))
                }
                "CloudHsmInvalidRequestException" => {
                    return RusotoError::Service(CreateHsmError::CloudHsmInvalidRequest(err.msg))
                }
                "CloudHsmResourceNotFoundException" => {
                    return RusotoError::Service(CreateHsmError::CloudHsmResourceNotFound(err.msg))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(CreateHsmError::CloudHsmService(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateHsmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateHsmError {
    fn description(&self) -> &str {
        match *self {
            CreateHsmError::CloudHsmAccessDenied(ref cause) => cause,
            CreateHsmError::CloudHsmInternalFailure(ref cause) => cause,
            CreateHsmError::CloudHsmInvalidRequest(ref cause) => cause,
            CreateHsmError::CloudHsmResourceNotFound(ref cause) => cause,
            CreateHsmError::CloudHsmService(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBackup
#[derive(Debug, PartialEq)]
pub enum DeleteBackupError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
}

impl DeleteBackupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBackupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmAccessDeniedException" => {
                    return RusotoError::Service(DeleteBackupError::CloudHsmAccessDenied(err.msg))
                }
                "CloudHsmInternalFailureException" => {
                    return RusotoError::Service(DeleteBackupError::CloudHsmInternalFailure(
                        err.msg,
                    ))
                }
                "CloudHsmInvalidRequestException" => {
                    return RusotoError::Service(DeleteBackupError::CloudHsmInvalidRequest(err.msg))
                }
                "CloudHsmResourceNotFoundException" => {
                    return RusotoError::Service(DeleteBackupError::CloudHsmResourceNotFound(
                        err.msg,
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(DeleteBackupError::CloudHsmService(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteBackupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBackupError {
    fn description(&self) -> &str {
        match *self {
            DeleteBackupError::CloudHsmAccessDenied(ref cause) => cause,
            DeleteBackupError::CloudHsmInternalFailure(ref cause) => cause,
            DeleteBackupError::CloudHsmInvalidRequest(ref cause) => cause,
            DeleteBackupError::CloudHsmResourceNotFound(ref cause) => cause,
            DeleteBackupError::CloudHsmService(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCluster
#[derive(Debug, PartialEq)]
pub enum DeleteClusterError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
}

impl DeleteClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteClusterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmAccessDeniedException" => {
                    return RusotoError::Service(DeleteClusterError::CloudHsmAccessDenied(err.msg))
                }
                "CloudHsmInternalFailureException" => {
                    return RusotoError::Service(DeleteClusterError::CloudHsmInternalFailure(
                        err.msg,
                    ))
                }
                "CloudHsmInvalidRequestException" => {
                    return RusotoError::Service(DeleteClusterError::CloudHsmInvalidRequest(
                        err.msg,
                    ))
                }
                "CloudHsmResourceNotFoundException" => {
                    return RusotoError::Service(DeleteClusterError::CloudHsmResourceNotFound(
                        err.msg,
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(DeleteClusterError::CloudHsmService(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteClusterError {
    fn description(&self) -> &str {
        match *self {
            DeleteClusterError::CloudHsmAccessDenied(ref cause) => cause,
            DeleteClusterError::CloudHsmInternalFailure(ref cause) => cause,
            DeleteClusterError::CloudHsmInvalidRequest(ref cause) => cause,
            DeleteClusterError::CloudHsmResourceNotFound(ref cause) => cause,
            DeleteClusterError::CloudHsmService(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteHsm
#[derive(Debug, PartialEq)]
pub enum DeleteHsmError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
}

impl DeleteHsmError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteHsmError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmAccessDeniedException" => {
                    return RusotoError::Service(DeleteHsmError::CloudHsmAccessDenied(err.msg))
                }
                "CloudHsmInternalFailureException" => {
                    return RusotoError::Service(DeleteHsmError::CloudHsmInternalFailure(err.msg))
                }
                "CloudHsmInvalidRequestException" => {
                    return RusotoError::Service(DeleteHsmError::CloudHsmInvalidRequest(err.msg))
                }
                "CloudHsmResourceNotFoundException" => {
                    return RusotoError::Service(DeleteHsmError::CloudHsmResourceNotFound(err.msg))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(DeleteHsmError::CloudHsmService(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteHsmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteHsmError {
    fn description(&self) -> &str {
        match *self {
            DeleteHsmError::CloudHsmAccessDenied(ref cause) => cause,
            DeleteHsmError::CloudHsmInternalFailure(ref cause) => cause,
            DeleteHsmError::CloudHsmInvalidRequest(ref cause) => cause,
            DeleteHsmError::CloudHsmResourceNotFound(ref cause) => cause,
            DeleteHsmError::CloudHsmService(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeBackups
#[derive(Debug, PartialEq)]
pub enum DescribeBackupsError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
}

impl DescribeBackupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeBackupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmAccessDeniedException" => {
                    return RusotoError::Service(DescribeBackupsError::CloudHsmAccessDenied(
                        err.msg,
                    ))
                }
                "CloudHsmInternalFailureException" => {
                    return RusotoError::Service(DescribeBackupsError::CloudHsmInternalFailure(
                        err.msg,
                    ))
                }
                "CloudHsmInvalidRequestException" => {
                    return RusotoError::Service(DescribeBackupsError::CloudHsmInvalidRequest(
                        err.msg,
                    ))
                }
                "CloudHsmResourceNotFoundException" => {
                    return RusotoError::Service(DescribeBackupsError::CloudHsmResourceNotFound(
                        err.msg,
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(DescribeBackupsError::CloudHsmService(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeBackupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeBackupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeBackupsError::CloudHsmAccessDenied(ref cause) => cause,
            DescribeBackupsError::CloudHsmInternalFailure(ref cause) => cause,
            DescribeBackupsError::CloudHsmInvalidRequest(ref cause) => cause,
            DescribeBackupsError::CloudHsmResourceNotFound(ref cause) => cause,
            DescribeBackupsError::CloudHsmService(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeClusters
#[derive(Debug, PartialEq)]
pub enum DescribeClustersError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
}

impl DescribeClustersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeClustersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmAccessDeniedException" => {
                    return RusotoError::Service(DescribeClustersError::CloudHsmAccessDenied(
                        err.msg,
                    ))
                }
                "CloudHsmInternalFailureException" => {
                    return RusotoError::Service(DescribeClustersError::CloudHsmInternalFailure(
                        err.msg,
                    ))
                }
                "CloudHsmInvalidRequestException" => {
                    return RusotoError::Service(DescribeClustersError::CloudHsmInvalidRequest(
                        err.msg,
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(DescribeClustersError::CloudHsmService(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeClustersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeClustersError {
    fn description(&self) -> &str {
        match *self {
            DescribeClustersError::CloudHsmAccessDenied(ref cause) => cause,
            DescribeClustersError::CloudHsmInternalFailure(ref cause) => cause,
            DescribeClustersError::CloudHsmInvalidRequest(ref cause) => cause,
            DescribeClustersError::CloudHsmService(ref cause) => cause,
        }
    }
}
/// Errors returned by InitializeCluster
#[derive(Debug, PartialEq)]
pub enum InitializeClusterError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
}

impl InitializeClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<InitializeClusterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmAccessDeniedException" => {
                    return RusotoError::Service(InitializeClusterError::CloudHsmAccessDenied(
                        err.msg,
                    ))
                }
                "CloudHsmInternalFailureException" => {
                    return RusotoError::Service(InitializeClusterError::CloudHsmInternalFailure(
                        err.msg,
                    ))
                }
                "CloudHsmInvalidRequestException" => {
                    return RusotoError::Service(InitializeClusterError::CloudHsmInvalidRequest(
                        err.msg,
                    ))
                }
                "CloudHsmResourceNotFoundException" => {
                    return RusotoError::Service(InitializeClusterError::CloudHsmResourceNotFound(
                        err.msg,
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(InitializeClusterError::CloudHsmService(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for InitializeClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for InitializeClusterError {
    fn description(&self) -> &str {
        match *self {
            InitializeClusterError::CloudHsmAccessDenied(ref cause) => cause,
            InitializeClusterError::CloudHsmInternalFailure(ref cause) => cause,
            InitializeClusterError::CloudHsmInvalidRequest(ref cause) => cause,
            InitializeClusterError::CloudHsmResourceNotFound(ref cause) => cause,
            InitializeClusterError::CloudHsmService(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTags
#[derive(Debug, PartialEq)]
pub enum ListTagsError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
}

impl ListTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmAccessDeniedException" => {
                    return RusotoError::Service(ListTagsError::CloudHsmAccessDenied(err.msg))
                }
                "CloudHsmInternalFailureException" => {
                    return RusotoError::Service(ListTagsError::CloudHsmInternalFailure(err.msg))
                }
                "CloudHsmInvalidRequestException" => {
                    return RusotoError::Service(ListTagsError::CloudHsmInvalidRequest(err.msg))
                }
                "CloudHsmResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsError::CloudHsmResourceNotFound(err.msg))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(ListTagsError::CloudHsmService(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsError {
    fn description(&self) -> &str {
        match *self {
            ListTagsError::CloudHsmAccessDenied(ref cause) => cause,
            ListTagsError::CloudHsmInternalFailure(ref cause) => cause,
            ListTagsError::CloudHsmInvalidRequest(ref cause) => cause,
            ListTagsError::CloudHsmResourceNotFound(ref cause) => cause,
            ListTagsError::CloudHsmService(ref cause) => cause,
        }
    }
}
/// Errors returned by RestoreBackup
#[derive(Debug, PartialEq)]
pub enum RestoreBackupError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
}

impl RestoreBackupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RestoreBackupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmAccessDeniedException" => {
                    return RusotoError::Service(RestoreBackupError::CloudHsmAccessDenied(err.msg))
                }
                "CloudHsmInternalFailureException" => {
                    return RusotoError::Service(RestoreBackupError::CloudHsmInternalFailure(
                        err.msg,
                    ))
                }
                "CloudHsmInvalidRequestException" => {
                    return RusotoError::Service(RestoreBackupError::CloudHsmInvalidRequest(
                        err.msg,
                    ))
                }
                "CloudHsmResourceNotFoundException" => {
                    return RusotoError::Service(RestoreBackupError::CloudHsmResourceNotFound(
                        err.msg,
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(RestoreBackupError::CloudHsmService(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RestoreBackupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RestoreBackupError {
    fn description(&self) -> &str {
        match *self {
            RestoreBackupError::CloudHsmAccessDenied(ref cause) => cause,
            RestoreBackupError::CloudHsmInternalFailure(ref cause) => cause,
            RestoreBackupError::CloudHsmInvalidRequest(ref cause) => cause,
            RestoreBackupError::CloudHsmResourceNotFound(ref cause) => cause,
            RestoreBackupError::CloudHsmService(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmAccessDeniedException" => {
                    return RusotoError::Service(TagResourceError::CloudHsmAccessDenied(err.msg))
                }
                "CloudHsmInternalFailureException" => {
                    return RusotoError::Service(TagResourceError::CloudHsmInternalFailure(err.msg))
                }
                "CloudHsmInvalidRequestException" => {
                    return RusotoError::Service(TagResourceError::CloudHsmInvalidRequest(err.msg))
                }
                "CloudHsmResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::CloudHsmResourceNotFound(
                        err.msg,
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(TagResourceError::CloudHsmService(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for TagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagResourceError {
    fn description(&self) -> &str {
        match *self {
            TagResourceError::CloudHsmAccessDenied(ref cause) => cause,
            TagResourceError::CloudHsmInternalFailure(ref cause) => cause,
            TagResourceError::CloudHsmInvalidRequest(ref cause) => cause,
            TagResourceError::CloudHsmResourceNotFound(ref cause) => cause,
            TagResourceError::CloudHsmService(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The request was rejected because the requester does not have permission to perform the requested operation.</p>
    CloudHsmAccessDenied(String),
    /// <p>The request was rejected because of an AWS CloudHSM internal failure. The request can be retried.</p>
    CloudHsmInternalFailure(String),
    /// <p>The request was rejected because it is not a valid request.</p>
    CloudHsmInvalidRequest(String),
    /// <p>The request was rejected because it refers to a resource that cannot be found.</p>
    CloudHsmResourceNotFound(String),
    /// <p>The request was rejected because an error occurred.</p>
    CloudHsmService(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CloudHsmAccessDeniedException" => {
                    return RusotoError::Service(UntagResourceError::CloudHsmAccessDenied(err.msg))
                }
                "CloudHsmInternalFailureException" => {
                    return RusotoError::Service(UntagResourceError::CloudHsmInternalFailure(
                        err.msg,
                    ))
                }
                "CloudHsmInvalidRequestException" => {
                    return RusotoError::Service(UntagResourceError::CloudHsmInvalidRequest(
                        err.msg,
                    ))
                }
                "CloudHsmResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::CloudHsmResourceNotFound(
                        err.msg,
                    ))
                }
                "CloudHsmServiceException" => {
                    return RusotoError::Service(UntagResourceError::CloudHsmService(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UntagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagResourceError {
    fn description(&self) -> &str {
        match *self {
            UntagResourceError::CloudHsmAccessDenied(ref cause) => cause,
            UntagResourceError::CloudHsmInternalFailure(ref cause) => cause,
            UntagResourceError::CloudHsmInvalidRequest(ref cause) => cause,
            UntagResourceError::CloudHsmResourceNotFound(ref cause) => cause,
            UntagResourceError::CloudHsmService(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the CloudHSM V2 API. CloudHSM V2 clients implement this trait.
pub trait CloudHsmv2 {
    /// <p>Copy an AWS CloudHSM cluster backup to a different region.</p>
    fn copy_backup_to_region(
        &self,
        input: CopyBackupToRegionRequest,
    ) -> RusotoFuture<CopyBackupToRegionResponse, CopyBackupToRegionError>;

    /// <p>Creates a new AWS CloudHSM cluster.</p>
    fn create_cluster(
        &self,
        input: CreateClusterRequest,
    ) -> RusotoFuture<CreateClusterResponse, CreateClusterError>;

    /// <p>Creates a new hardware security module (HSM) in the specified AWS CloudHSM cluster.</p>
    fn create_hsm(
        &self,
        input: CreateHsmRequest,
    ) -> RusotoFuture<CreateHsmResponse, CreateHsmError>;

    /// <p>Deletes a specified AWS CloudHSM backup. A backup can be restored up to 7 days after the DeleteBackup request. For more information on restoring a backup, see <a>RestoreBackup</a> </p>
    fn delete_backup(
        &self,
        input: DeleteBackupRequest,
    ) -> RusotoFuture<DeleteBackupResponse, DeleteBackupError>;

    /// <p>Deletes the specified AWS CloudHSM cluster. Before you can delete a cluster, you must delete all HSMs in the cluster. To see if the cluster contains any HSMs, use <a>DescribeClusters</a>. To delete an HSM, use <a>DeleteHsm</a>.</p>
    fn delete_cluster(
        &self,
        input: DeleteClusterRequest,
    ) -> RusotoFuture<DeleteClusterResponse, DeleteClusterError>;

    /// <p>Deletes the specified HSM. To specify an HSM, you can use its identifier (ID), the IP address of the HSM's elastic network interface (ENI), or the ID of the HSM's ENI. You need to specify only one of these values. To find these values, use <a>DescribeClusters</a>.</p>
    fn delete_hsm(
        &self,
        input: DeleteHsmRequest,
    ) -> RusotoFuture<DeleteHsmResponse, DeleteHsmError>;

    /// <p>Gets information about backups of AWS CloudHSM clusters.</p> <p>This is a paginated operation, which means that each response might contain only a subset of all the backups. When the response contains only a subset of backups, it includes a <code>NextToken</code> value. Use this value in a subsequent <code>DescribeBackups</code> request to get more backups. When you receive a response with no <code>NextToken</code> (or an empty or null value), that means there are no more backups to get.</p>
    fn describe_backups(
        &self,
        input: DescribeBackupsRequest,
    ) -> RusotoFuture<DescribeBackupsResponse, DescribeBackupsError>;

    /// <p>Gets information about AWS CloudHSM clusters.</p> <p>This is a paginated operation, which means that each response might contain only a subset of all the clusters. When the response contains only a subset of clusters, it includes a <code>NextToken</code> value. Use this value in a subsequent <code>DescribeClusters</code> request to get more clusters. When you receive a response with no <code>NextToken</code> (or an empty or null value), that means there are no more clusters to get.</p>
    fn describe_clusters(
        &self,
        input: DescribeClustersRequest,
    ) -> RusotoFuture<DescribeClustersResponse, DescribeClustersError>;

    /// <p>Claims an AWS CloudHSM cluster by submitting the cluster certificate issued by your issuing certificate authority (CA) and the CA's root certificate. Before you can claim a cluster, you must sign the cluster's certificate signing request (CSR) with your issuing CA. To get the cluster's CSR, use <a>DescribeClusters</a>.</p>
    fn initialize_cluster(
        &self,
        input: InitializeClusterRequest,
    ) -> RusotoFuture<InitializeClusterResponse, InitializeClusterError>;

    /// <p>Gets a list of tags for the specified AWS CloudHSM cluster.</p> <p>This is a paginated operation, which means that each response might contain only a subset of all the tags. When the response contains only a subset of tags, it includes a <code>NextToken</code> value. Use this value in a subsequent <code>ListTags</code> request to get more tags. When you receive a response with no <code>NextToken</code> (or an empty or null value), that means there are no more tags to get.</p>
    fn list_tags(&self, input: ListTagsRequest) -> RusotoFuture<ListTagsResponse, ListTagsError>;

    /// <p>Restores a specified AWS CloudHSM backup that is in the <code>PENDING_DELETION</code> state. For more information on deleting a backup, see <a>DeleteBackup</a>.</p>
    fn restore_backup(
        &self,
        input: RestoreBackupRequest,
    ) -> RusotoFuture<RestoreBackupResponse, RestoreBackupError>;

    /// <p>Adds or overwrites one or more tags for the specified AWS CloudHSM cluster.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError>;

    /// <p>Removes the specified tag or tags from the specified AWS CloudHSM cluster.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError>;
}
/// A client for the CloudHSM V2 API.
#[derive(Clone)]
pub struct CloudHsmv2Client {
    client: Client,
    region: region::Region,
}

impl CloudHsmv2Client {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CloudHsmv2Client {
        CloudHsmv2Client {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CloudHsmv2Client
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        CloudHsmv2Client {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl CloudHsmv2 for CloudHsmv2Client {
    /// <p>Copy an AWS CloudHSM cluster backup to a different region.</p>
    fn copy_backup_to_region(
        &self,
        input: CopyBackupToRegionRequest,
    ) -> RusotoFuture<CopyBackupToRegionResponse, CopyBackupToRegionError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");
        request.set_endpoint_prefix("cloudhsmv2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "BaldrApiService.CopyBackupToRegion");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CopyBackupToRegionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CopyBackupToRegionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new AWS CloudHSM cluster.</p>
    fn create_cluster(
        &self,
        input: CreateClusterRequest,
    ) -> RusotoFuture<CreateClusterResponse, CreateClusterError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");
        request.set_endpoint_prefix("cloudhsmv2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "BaldrApiService.CreateCluster");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateClusterResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateClusterError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new hardware security module (HSM) in the specified AWS CloudHSM cluster.</p>
    fn create_hsm(
        &self,
        input: CreateHsmRequest,
    ) -> RusotoFuture<CreateHsmResponse, CreateHsmError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");
        request.set_endpoint_prefix("cloudhsmv2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "BaldrApiService.CreateHsm");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateHsmResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateHsmError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a specified AWS CloudHSM backup. A backup can be restored up to 7 days after the DeleteBackup request. For more information on restoring a backup, see <a>RestoreBackup</a> </p>
    fn delete_backup(
        &self,
        input: DeleteBackupRequest,
    ) -> RusotoFuture<DeleteBackupResponse, DeleteBackupError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");
        request.set_endpoint_prefix("cloudhsmv2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "BaldrApiService.DeleteBackup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteBackupResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteBackupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the specified AWS CloudHSM cluster. Before you can delete a cluster, you must delete all HSMs in the cluster. To see if the cluster contains any HSMs, use <a>DescribeClusters</a>. To delete an HSM, use <a>DeleteHsm</a>.</p>
    fn delete_cluster(
        &self,
        input: DeleteClusterRequest,
    ) -> RusotoFuture<DeleteClusterResponse, DeleteClusterError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");
        request.set_endpoint_prefix("cloudhsmv2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "BaldrApiService.DeleteCluster");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteClusterResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteClusterError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the specified HSM. To specify an HSM, you can use its identifier (ID), the IP address of the HSM's elastic network interface (ENI), or the ID of the HSM's ENI. You need to specify only one of these values. To find these values, use <a>DescribeClusters</a>.</p>
    fn delete_hsm(
        &self,
        input: DeleteHsmRequest,
    ) -> RusotoFuture<DeleteHsmResponse, DeleteHsmError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");
        request.set_endpoint_prefix("cloudhsmv2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "BaldrApiService.DeleteHsm");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteHsmResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteHsmError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets information about backups of AWS CloudHSM clusters.</p> <p>This is a paginated operation, which means that each response might contain only a subset of all the backups. When the response contains only a subset of backups, it includes a <code>NextToken</code> value. Use this value in a subsequent <code>DescribeBackups</code> request to get more backups. When you receive a response with no <code>NextToken</code> (or an empty or null value), that means there are no more backups to get.</p>
    fn describe_backups(
        &self,
        input: DescribeBackupsRequest,
    ) -> RusotoFuture<DescribeBackupsResponse, DescribeBackupsError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");
        request.set_endpoint_prefix("cloudhsmv2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "BaldrApiService.DescribeBackups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeBackupsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeBackupsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets information about AWS CloudHSM clusters.</p> <p>This is a paginated operation, which means that each response might contain only a subset of all the clusters. When the response contains only a subset of clusters, it includes a <code>NextToken</code> value. Use this value in a subsequent <code>DescribeClusters</code> request to get more clusters. When you receive a response with no <code>NextToken</code> (or an empty or null value), that means there are no more clusters to get.</p>
    fn describe_clusters(
        &self,
        input: DescribeClustersRequest,
    ) -> RusotoFuture<DescribeClustersResponse, DescribeClustersError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");
        request.set_endpoint_prefix("cloudhsmv2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "BaldrApiService.DescribeClusters");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeClustersResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeClustersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Claims an AWS CloudHSM cluster by submitting the cluster certificate issued by your issuing certificate authority (CA) and the CA's root certificate. Before you can claim a cluster, you must sign the cluster's certificate signing request (CSR) with your issuing CA. To get the cluster's CSR, use <a>DescribeClusters</a>.</p>
    fn initialize_cluster(
        &self,
        input: InitializeClusterRequest,
    ) -> RusotoFuture<InitializeClusterResponse, InitializeClusterError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");
        request.set_endpoint_prefix("cloudhsmv2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "BaldrApiService.InitializeCluster");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<InitializeClusterResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(InitializeClusterError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a list of tags for the specified AWS CloudHSM cluster.</p> <p>This is a paginated operation, which means that each response might contain only a subset of all the tags. When the response contains only a subset of tags, it includes a <code>NextToken</code> value. Use this value in a subsequent <code>ListTags</code> request to get more tags. When you receive a response with no <code>NextToken</code> (or an empty or null value), that means there are no more tags to get.</p>
    fn list_tags(&self, input: ListTagsRequest) -> RusotoFuture<ListTagsResponse, ListTagsError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");
        request.set_endpoint_prefix("cloudhsmv2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "BaldrApiService.ListTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListTagsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListTagsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Restores a specified AWS CloudHSM backup that is in the <code>PENDING_DELETION</code> state. For more information on deleting a backup, see <a>DeleteBackup</a>.</p>
    fn restore_backup(
        &self,
        input: RestoreBackupRequest,
    ) -> RusotoFuture<RestoreBackupResponse, RestoreBackupError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");
        request.set_endpoint_prefix("cloudhsmv2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "BaldrApiService.RestoreBackup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RestoreBackupResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RestoreBackupError::from_response(response))),
                )
            }
        })
    }

    /// <p>Adds or overwrites one or more tags for the specified AWS CloudHSM cluster.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");
        request.set_endpoint_prefix("cloudhsmv2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "BaldrApiService.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<TagResourceResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TagResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes the specified tag or tags from the specified AWS CloudHSM cluster.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError> {
        let mut request = SignedRequest::new("POST", "cloudhsm", &self.region, "/");
        request.set_endpoint_prefix("cloudhsmv2".to_string());
        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "BaldrApiService.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UntagResourceResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UntagResourceError::from_response(response))),
                )
            }
        })
    }
}
