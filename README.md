# Flatboat

![Logo](https://github.com/JuanCSUCoder/flatboat-brand/blob/main/logo.png?raw=true)

Flatboat is a CLI tool that integrates **Docker** and **Kubernetes** tooling into the **ROS workspace** workflow.

[![AGPL License](https://img.shields.io/badge/license-AGPL-blue.svg)](http://www.gnu.org/licenses/agpl-3.0)

## Demo

Insert gif or link to demo

## Screenshots

![App Screenshot](https://via.placeholder.com/468x300?text=App+Screenshot+Here)

## Features

- **Automatic** container lifecycle managment
- **Create ROS workspaces** from devcontainer templates
- **Execute ROS2 commands** inside the container
- **Execute shell commands** inside the container
- Out of the box graphical user interface **GUI support**
- Out of the box **GPU support**
- Out of the box workspace **volume binding**
- Out of the box **host network** connectivity
- Out of the box **avahi service** for .local domain resolution
- Compatible with **Devcontainer Specification**

## Installation

Install Flatboat with `cargo`

```bash
  cargo install flatboat
```

## Usage

Learn about Flatboat with:

```bash
  flatboat -h
```

## Authors

- [@JuanCSUCoder](https://www.github.com/JuanCSUCoder)

## Roadmap

1. Templates
    1. Package Dockerfile
    2. Kubernetes Workers
    3. Kubernetes Master
2. Integration of Templates
3. Local Kubernetes Driver
