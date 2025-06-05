# 🪐 AkaiaLabs Continuity OS

Decentralized autonomous community operating system.

🚧 WORK IN PROGRESS 🚧

## Overview

Continuity mainly consists of two parts — Core and Subsystems.

### Core

Implements foundational functionalities and data storages and configures crucial infrastructure services, tying the whole system together.

#### Singularity

The central part of the system, a [SpacetimeDB](https://github.com/clockworklabs/SpacetimeDB.git) module that:

- Acts as an identity provider and handles authentication / authorization
- Stores basic social profiles and agent personas
- Serves as a communication buffer, implementing common ground messaging primitives
- Provides configuration storage for subsystems

#### Network

##### TODO

- [ ] ⏳ A resilient community-owned network infrastructure solution ready for off-grid applications.

  1. [ ] ⏳ [Reticulum](https://reticulum.network/manual/whatis.html) as intranet topology basis
  2. [ ] [Yggdrasil](https://yggdrasil-network.github.io/) as Internet bridge

### Subsystems

Subsystems are complex software solutions each focused on a specific part of the use case spectrum.

[Corvi.d](./subsystems/corvi.d/README.md) - communication
[RAIven](./subsystems/raiven/README.md) - artificial intelligence
[Knowledge](./subsystems/kb/README.md) - knowledge base development
[Intelligence](./subsystems/intel/README.md) - intel gathering and environmental awareness
[Consensus](./subsystems/consensus/README.md) - decentralized governance

## High level TODO

- [ ] MicroVM encapsulation for each individual subsystem
- [ ] A modular OS distribution tailored for hosting subsystems on dedicated hardware nodes

## Development

### Environment setup

🚧 TBD 🚧

### Conventions

🚧 TBD 🚧

## License

[![License: AkaiaLabs NonCommercial Attribution 1.0](https://img.shields.io/badge/License-AkaiaLabs_NC--By_1.0-black.svg)](./LICENSE.md)

This project is licensed under the AkaiaLabs NonCommercial Attribution License 1.0.
Commercial and military use require explicit permission — contact [Akaia CVO](mailto:cvo.akaia@gmail.com).
