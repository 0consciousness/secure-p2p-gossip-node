# Secure P2P Gossip Node (Rust)

## Overview

This repository is a simplified public demonstration of peer-to-peer networking patterns commonly used in blockchain and distributed systems.

It focuses on:

- gossip-based message propagation

- peer discovery and churn handling

- basic security hardening

- observability and performance awareness

This is not a production blockchain node, but a minimal system designed to highlight architectural decisions and tradeoffs.

## Architecture

Each node consists of:

- a peer manager (connections, liveness, scoring)

- a gossip router (fanout, deduplication, TTL)

- a transport layer (async TCP)

- observability endpoints (metrics + logs)

Nodes bootstrap from known peers and form a loosely connected overlay network.

## Design Decisions

- Gossip over direct broadcast for scalability and fault tolerance

- Message deduplication using IDs to prevent amplification

- Peer scoring to reduce Sybil and Eclipse risk

- Rate limiting to mitigate basic DoS-style traffic

- Async architecture for predictable performance under load

Tradeoffs were intentionally made to keep the system readable and focused.

## Security Considerations

This demo includes:

- per-peer rate limiting

- reputation-based peer scoring

- connection limits

Out of scope (by design):

- full Sybil resistance

- cryptographic identity schemes

- advanced NAT traversal

These would be required in a production environment.

## Observability

The node exposes Prometheus-compatible metrics:

- active peers

- messages sent / received

- dropped or rate-limited messages

Structured logs are emitted using tracing.

## What I’d Improve Next

- NAT traversal (STUN/TURN, hole punching)

- adaptive gossip fanout

- DHT-based peer discovery

- cryptographic peer identity

- chaos testing and fault injection

## Diagram

flowchart TD
    Peer[Remote Peer] --> Transport
    Transport --> PeerManager
    PeerManager --> Gossip
    Gossip --> Transport

    PeerManager --> Security
    Gossip --> Security

    PeerManager --> Metrics
    Gossip --> Metrics

    subgraph Node["P2P Node"]
        Transport[Transport Layer\n(Async TCP)]
        PeerManager[Peer Manager\n- registry\n- liveness\n- scoring]
        Gossip[Gossip Router\n- fanout\n- dedup\n- TTL]
        Security[Security\n- rate limits\n- conn caps]
        Metrics[Observability\n- logs\n- metrics]
    end


## Disclaimer

This repository is a learning and demonstration project.
It reflects patterns used in real systems without exposing proprietary or client-owned code.