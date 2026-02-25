# Kanrigate

**Kanrigate** is a professional, high-performance Kubernetes RBAC (Role-Based Access Control) Management tool. It combines a modern **SvelteKit** frontend with a blazing-fast **Rust** backend to provide a seamless experience for managing cluster permissions.

## 🚀 Overview

Managing Kubernetes RBAC can be complex and error-prone. Kanrigate simplifies this by providing:
- A clean, intuitive Web UI for managing ServiceAccounts, Roles, and Bindings.
- Pre-defined, industry-standard RBAC templates for common personas.
- Real-time validation and management of cluster-wide and namespaced resources.

## 🛠 Tech Stack

- **Frontend**: SvelteKit (TypeScript, Tailwind-ready)
- **Backend**: Rust (Axum/Tokio stack)
- **Database**: In-memory state with Kubernetes as the source of truth.
- **Deployment**: Docker (Multi-stage builds), Helm 3.

## 🏗 Architecture

Kanrigate is designed to run as a single containerized unit:
1.  **Backend (Rust)**: Handles high-speed API requests, Kubernetes cluster communication, and RBAC logic.
2.  **Frontend (SvelteKit)**: Served via Node.js within the same container, providing a responsive SPA interface.
3.  **Entrypoint**: A unified script that orchestrates both services, exposing the UI on port `3000` and the API on port `3232`.

## 📦 Getting Started

### Prerequisites
- Kubernetes Cluster
- Docker & Node.js (for local development)
- Helm 3 (for deployment)

### Local Development

1. **Frontend**:
   ```bash
   cd src/ui
   npm install
   npm run dev
   ```

2. **Backend**:
   ```bash
   # Ensure you have a valid Kubeconfig
   cargo run
   ```

### Docker
Build the unified image:
```bash
docker build -t kanrigate:latest .
```

## ☸️ Deployment with Helm

The preferred way to deploy Kanrigate is via the provided Helm chart.

### Quick Install
```bash
helm install kanrigate ./helm/kanrigate
```

### Configuration (Values.yaml)

Key configurations available in `helm/kanrigate/values.yaml`:

| Key | Description | Default |
|-----|-------------|---------|
| `env.APP_CLUSTER_NAME` | Name of the target cluster | `kubernetes-admin@kubernetes` |
| `env.APP_CONTROL_PLANE_ADDRESS` | K8s API Address | `https://kubernetes.default.svc:443` |
| `service.type` | Kubernetes Service type | `ClusterIP` |
| `rbac.create` | Auto-create necessary permissions for Kanrigate | `true` |

## 🔐 RBAC Templates

Kanrigate automatically provisions several `ClusterRoles` to standardise access:
- `template-cluster-resources___admin`: Full cluster administrative rights.
- `template-cluster-resources___read-only`: Global view-only access.
- `template-namespaced-resources___developer`: Essential CRUD for developers in a namespace.
- `template-namespaced-resources___monitoring`: Access for monitoring tools and dashboards.
- `template-namespaced-resources___operation`: Operational control within namespaces.

## 🛡 Security

- **Minimal Footprint**: Uses `debian:bookworm-slim` for the final production image.
- **RBAC Isolation**: Operates with its own ServiceAccount and specifically scoped ClusterRoles.
- **No Persistence**: Relies on Kubernetes native storage for all state, ensuring no data fragmentation.

## 📝 License
[Insert License Here]
