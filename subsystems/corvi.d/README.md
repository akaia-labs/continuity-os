# AkaiaLabs Corvi.d subsystem

Experimental container-like environment for [Corvi.d](https://github.com/akaia-labs/corvi.d) using [Redox](https://gitlab.redox-os.org/redox-os/redox) on [RVVM](https://github.com/LekKit/RVVM).

The purpose of the project is to create a homogeneous Rust-based runtime by running [Redox](https://gitlab.redox-os.org/redox-os/redox) inside [RVVM](https://github.com/LekKit/RVVM) – packaging [SpacetimeDB](https://github.com/ClockworkLabs/SpacetimeDB) (with [Corvi.d](https://github.com/akaia-labs/corvi.d) server modules deployed within it) – and related ecosystem services into a unified container-style subsystem, ready to be deployed on a real dedicated hardware, if needed.

🚧 WORK IN PROGRESS 🚧

## Important links

[RVVM wiki](https://github.com/LekKit/RVVM/wiki/Running#running-rvvm)

## Getting started

### Prerequisites

First, you need to clone the given repo and enter the dedicated directory:

```bash
git clone https://github.com/akaia-labs/corvi.d
cd corvi.d/subsystems/corvi.d
```

### Preparing Redox image

If you already have a `redox_minimal-net_riscv64gc_*_harddrive.img` image:

```bash
cp {path-to-the-image}/redox_minimal-net_riscv64gc_*_harddrive.img {path-to-this-repo}/redox/redox_minimal-net.img
```

Otherwise, just:

```bash
cd redox
./bootstrap.sh
cd ..
```

### Starting the VM

```bash
./start.sh
```

### Connecting via SSH ( currently broken )

```bash
ssh user@localhost -p 2022
```

### Credentials

user:(empty password)

root:password
