use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use owner_signal_domain_criome::{
    Delegation, DelegationName, DelegationTarget, DomainName, DomainNameSystemRecord,
    DomainRegistered, Operation, OperationKind, Policy, PolicySet, ProjectionDeclaration,
    ProjectionDirective, ProjectionPolicy, ProjectionScope, RecordKind, RecordValue, Registration,
    Reply, ReplyKind,
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
            "RetireDomain",
            "SetPolicy",
            "SetProjection",
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
fn projection_declarations_carry_provider_neutral_records() {
    let operation = Operation::SetProjection(ProjectionDeclaration {
        domain: DomainName::new("goldragon.criome"),
        records: vec![DomainNameSystemRecord {
            name: DomainName::new("goldragon.criome"),
            kind: RecordKind::AddressV4,
            value: RecordValue::new("203.0.113.10"),
        }],
        redirects: vec![],
    });

    assert_eq!(operation.operation_kind(), OperationKind::SetProjection);
}

#[test]
fn replies_round_trip_through_nota() {
    let registered = Reply::DomainRegistered(DomainRegistered {
        domain: DomainName::new("goldragon.criome"),
    });
    let policy = Reply::PolicySet(PolicySet {
        projection_policy_count: 1,
    });

    assert_eq!(registered.kind(), ReplyKind::DomainRegistered);
    assert_eq!(policy.kind(), ReplyKind::PolicySet);

    let text = encode_to_text(&registered);
    let mut decoder = Decoder::new(&text);
    let decoded = Reply::decode(&mut decoder).expect("decode");
    assert_eq!(decoded, registered);
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
