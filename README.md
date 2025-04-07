# Scaleway PoC Mono repo

This repo contains a super basic example setup of a typical cloud architecture built on Scaleway.

## Architecture overview

```mermaid
---
config:
    layout: elk
---
flowchart

    Browser --> WebDistribution

    subgraph WebDistribution["Web Distribution"]
        CDN["Scaleway Edge Services"] --> S3 & ApplicationGW
        S3@{shape: cyl, label: "Assets
        Scaleway S3 "}
        ApplicationGW["
        Load balancer
        Scaleway
        "]
    end

    ApplicationGW --> BEContainer

    subgraph ApplicationLayer["Application Layer"]
        
        BEContainer["
        BE REST API
        Scaleway Container
        "]
        Functions@{shape: procs, label: "Serverless functions
        Scaleway"}
    end

    subgraph EventsLayer["Event layer"]
        IngressQueue@{shape: das, label: "Ingress
        Scaleway Queue
        "}
    end

    BEContainer --Publish-->IngressQueue
    IngressQueue --Consumer--> Functions

    subgraph PersistenceLayer["Persistence Layer" ]
        direction TB
        SQL@{shape: cyl, label: "Serverless SQL
        Scaleway"}
        REDIS@{shape: cyl, label: "Managed REDIS
        Scaleway"}
    end

    BEContainer & Functions --> PersistenceLayer
```

## Components

### Backend

Simple Rocket API.
