[![Release](https://img.shields.io/github/v/release/nexus-xyz/nexus-cli.svg)](https://github.com/nexus-xyz/nexus-cli/releases)
[![CI](https://github.com/nexus-xyz/nexus-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/nexus-xyz/nexus-cli/actions)
[![License](https://img.shields.io/badge/License-Apache_2.0-green.svg)](https://github.com/nexus-xyz/nexus-cli/blob/main/LICENSE-APACHE)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](https://github.com/nexus-xyz/nexus-cli/blob/main/LICENSE-MIT)
[![Twitter](https://img.shields.io/twitter/follow/NexusLabs)](https://x.com/NexusLabs)
[![Discord](https://img.shields.io/badge/Discord-Join-7289da.svg?logo=discord&logoColor=white)](https://discord.com/invite/nexus-xyz)

# Nexus CLI

A high-performance command-line interface for contributing proofs to the Nexus network.

<figure>
    <a href="https://nexus.xyz/">
        <img src="assets/images/nexus-network-image.png" alt="Nexus Network visualization showing a distributed network of interconnected nodes with a 'Launch Network' button in the center">
    </a>
    <figcaption>
        <strong>Verifiable Computation on a Global Scale</strong><br>
        We're building a global distributed prover network to unite the world's computers and power a new and better Internet: the Verifiable Internet. Connect to the beta and give it a try today.
    </figcaption>
</figure>

## Nexus Network

[Nexus](https://nexus.xyz/) is a global distributed prover network that unites the world's computers to power a new and
better Internet: the Verifiable Internet.

There have been several testnets so far:

- Testnet 0: [October 8 – 28, 2024](https://blog.nexus.xyz/nexus-launches-worlds-first-open-prover-network/)
- Testnet I: [December 9 – 13, 2024](https://blog.nexus.xyz/the-new-nexus-testnet-is-live/)
- Testnet II: [February 18 – 22, 2025](https://blog.nexus.xyz/testnet-ii-is-open/)
- Devnet: [February 22 - June 20 2025](https://docs.nexus.xyz/layer-1/testnet/devnet)
- Testnet III: [Ongoing](https://blog.nexus.xyz/live-everywhere/)

---

## Quick Start

### Installation

#### Precompiled Binary (Recommended)

For the simplest and most reliable installation:

```bash
curl https://cli.nexus.xyz/ | sh
```

This will:
1. Download and install the latest precompiled binary for your platform.
2. Prompt you to accept the Terms of Use.
3. Start the CLI in interactive mode.

The template installation script is viewable [here](./public/install.sh.template).

Of course, here is a README file with a step-by-step installation guide.

## Nexus CLI Installation Guide

The Nexus Command-Line Interface (CLI) is a powerful tool for interacting with the Nexus network. This guide will walk you through the installation process.

-----

### Prerequisites

Before you begin, ensure you have the following installed on your system:

  * **Git**: For cloning the repository.
  * **Rust and Cargo**: The CLI is built using Rust.
  * **Protobuf Compiler**: Necessary for compiling protocol buffers.

-----

### Installation Steps

1.  **Clone the repository**:

    ```bash
    git clone https://github.com/abit6666/nexus-cli-abit.git
    cd nexus-cli-abit/clients/cli
    ```

2.  **Build the CLI**:

   ```bash
    cargo build --release
    ```



-----

### Using Docker

For a more streamlined setup, you can use Docker:

1.  **Build the Docker image**:

    ```bash
    docker compose build --no-cache
    ```

2.  **Run the CLI in a container**:

    ```bash
    docker compose up -d
    ```

To view the logs, use the following command:

```bash
docker compose logs
```

To stop the container, run:

```bash
docker compose down
```

## License

Nexus CLI is distributed under the terms of both the [MIT License](./LICENSE-MIT) and the [Apache License (Version 2.0)](./LICENSE-APACHE).
