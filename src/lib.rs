//! Owner Signal contract for the domain-criome component.
//!
//! This crate carries owner-only domain registry and projection-policy records.

use nota_codec::{NotaEnum, NotaRecord};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;

pub use signal_domain_criome::{
    DelegationName, DelegationTarget, DomainName, DomainNameSystemRecord, ProjectionScope,
    RecordKind, RecordValue, RedirectRule,
};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Registration {
    pub domain: DomainName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Delegation {
    pub name: DelegationName,
    pub domain: DomainName,
    pub target: DelegationTarget,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Retirement {
    pub domain: DomainName,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum ProjectionDirective {
    Enable,
    Disable,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ProjectionPolicy {
    pub domain: DomainName,
    pub scope: ProjectionScope,
    pub directive: ProjectionDirective,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Policy {
    pub projections: Vec<ProjectionPolicy>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ProjectionDeclaration {
    pub domain: DomainName,
    pub records: Vec<DomainNameSystemRecord>,
    pub redirects: Vec<RedirectRule>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DomainRegistered {
    pub domain: DomainName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DomainDelegated {
    pub name: DelegationName,
    pub domain: DomainName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DomainRetired {
    pub domain: DomainName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct PolicySet {
    pub projection_policy_count: u64,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ProjectionSet {
    pub domain: DomainName,
    pub record_count: u64,
    pub redirect_count: u64,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum RejectionReason {
    DomainAlreadyRegistered,
    DomainUnknown,
    DelegationAlreadyExists,
    DelegationUnknown,
    ProjectionUnavailable,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RequestRejected {
    pub operation: OperationKind,
    pub reason: RejectionReason,
}

signal_channel! {
    channel OwnerDomainCriome {
        operation RegisterDomain(Registration),
        operation Delegate(Delegation),
        operation RetireDomain(Retirement),
        operation SetPolicy(Policy),
        operation SetProjection(ProjectionDeclaration),
    }
    reply Reply {
        DomainRegistered(DomainRegistered),
        DomainDelegated(DomainDelegated),
        DomainRetired(DomainRetired),
        PolicySet(PolicySet),
        ProjectionSet(ProjectionSet),
        RequestRejected(RequestRejected),
    }
}

pub type ChannelRequest = signal_frame::Request<Operation>;
pub type ChannelReply = signal_frame::Reply<Reply>;

impl Operation {
    pub fn operation_kind(&self) -> OperationKind {
        self.kind()
    }
}
