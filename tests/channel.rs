use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use owner_signal_domain_criome::{
    AuthorityEndpoint, AuthorityRegistered, AuthorityRegistration, Delegation, DelegationName,
    DelegationTarget, DomainName, DomainRegistered, Operation, OperationKind, Policy, PolicySet,
    ProjectionDirective, ProjectionPolicy, ProjectionScope, Registration, Reply, ReplyKind,
};
use signal_frame::{RequestPayload, SignalOperationHeads};

fn encode_to_text<T: NotaEncode>(value: &T) -> String {
    let mut encoder = Encoder::new();
    value.encode(&mut encoder).expect("encode");
    encoder.into_string()
}

#[test]
fn operations_are_owner_registry_verbs() {
    assert_eq!(
        <Operation as SignalOperationHeads>::HEADS,
        &[
            "RegisterDomain",
            "Delegate",
            "RegisterAuthority",
            "RetireDomain",
            "SetPolicy"
        ]
    );

    let operation = Operation::RegisterDomain(Registration {
        domain: DomainName::new("goldragon.criome"),
    });
    assert_eq!(operation.operation_kind(), OperationKind::RegisterDomain);
}

#[test]
fn registration_round_trips_through_nota() {
    let operation = Operation::RegisterDomain(Registration {
        domain: DomainName::new("goldragon.criome"),
    });

    let text = encode_to_text(&operation);
    assert_eq!(text, "(RegisterDomain ([goldragon.criome]))");

    let mut decoder = Decoder::new(&text);
    let decoded = Operation::decode(&mut decoder).expect("decode");
    assert_eq!(decoded, operation);
}

#[test]
fn delegation_uses_named_target_not_provider_terms() {
    let operation = Operation::Delegate(Delegation {
        name: DelegationName::new("www"),
        domain: DomainName::new("goldragon.criome"),
        target: DelegationTarget::new("goldragon.criome"),
    });

    let request = operation.into_request();
    assert_eq!(request.payloads().len(), 1);
}

#[test]
fn authority_registration_carries_endpoint_without_provider_terms() {
    let operation = Operation::RegisterAuthority(AuthorityRegistration {
        domain: DomainName::new("goldragon.criome"),
        endpoint: AuthorityEndpoint::new("domain-criome://goldragon.criome"),
    });

    assert_eq!(operation.operation_kind(), OperationKind::RegisterAuthority);
    let request = operation.into_request();
    assert_eq!(request.payloads().len(), 1);
}

#[test]
fn policy_uses_projection_directives_not_boolean_flags() {
    let operation = Operation::SetPolicy(Policy {
        projections: vec![ProjectionPolicy {
            domain: DomainName::new("goldragon.criome"),
            scope: ProjectionScope::Everything,
            directive: ProjectionDirective::Enable,
        }],
    });

    assert_eq!(operation.operation_kind(), OperationKind::SetPolicy);
}

#[test]
fn replies_round_trip_through_nota() {
    let registered = Reply::DomainRegistered(DomainRegistered {
        domain: DomainName::new("goldragon.criome"),
    });
    let authority_registered = Reply::AuthorityRegistered(AuthorityRegistered {
        domain: DomainName::new("goldragon.criome"),
        endpoint: AuthorityEndpoint::new("domain-criome://goldragon.criome"),
    });
    let policy = Reply::PolicySet(PolicySet {
        projection_policy_count: 1,
    });

    assert_eq!(registered.kind(), ReplyKind::DomainRegistered);
    assert_eq!(authority_registered.kind(), ReplyKind::AuthorityRegistered);
    assert_eq!(policy.kind(), ReplyKind::PolicySet);

    let text = encode_to_text(&registered);
    let mut decoder = Decoder::new(&text);
    let decoded = Reply::decode(&mut decoder).expect("decode");
    assert_eq!(decoded, registered);

    let text = encode_to_text(&authority_registered);
    let mut decoder = Decoder::new(&text);
    let decoded = Reply::decode(&mut decoder).expect("decode");
    assert_eq!(decoded, authority_registered);
}

#[test]
fn owner_contract_has_no_provider_vocabulary() {
    let manifest = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))
        .expect("manifest");
    let source = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"))
        .expect("source");

    assert!(!manifest.contains("signal-core"));
    assert!(!source.contains("Cloudflare"));
    assert!(!source.contains("Google"));
    assert!(!source.contains("Hetzner"));
}
