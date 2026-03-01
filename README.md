# Kanrigate

**Kanrigate** is a professional, high-performance Kubernetes RBAC (Role-Based Access Control) Management tool. It combines a modern **SvelteKit** frontend with a blazing-fast **Rust** backend to provide a seamless experience for managing cluster permissions.

## 🚀 Overview

Managing Kubernetes RBAC can be complex and error-prone. Kanrigate simplifies this by providing:
- A clean, intuitive Web UI for managing ServiceAccounts, Roles, and Bindings.
- **Secure Authentication**: Protected dashboard with Argon2id password hashing and JWT sessions.
- Pre-defined, industry-standard RBAC templates for common personas.
- Real-time validation and management of cluster-wide and namespaced resources.

## 🛠 Tech Stack

- **Frontend**: SvelteKit (TypeScript, Svelte 5 Runes)
- **Backend**: Rust (Axum/Tokio stack)
- **Security**: Argon2id for hashing, JWT for session management.
- **Database**: Stateless (Kubernetes API as the source of truth).
- **Deployment**: Docker (Multi-stage builds), Helm 3.

## 🏗 Architecture

Kanrigate is designed to run as a single containerized unit:
1.  **Backend (Rust)**: Handles high-speed API requests, Kubernetes cluster communication, and RBAC logic.
2.  **Frontend (SvelteKit)**: Served via Node.js (or Adapter-Node in production), providing a responsive SPA interface.
3.  **Authentication Middleware**: Every API request (except login) is validated against a JWT token stored in a secure HttpOnly cookie.

## 📦 Getting Started

### Prerequisites
- Kubernetes Cluster
- Helm 3 (for deployment)

### 🔐 Generate Admin Hash
Use the built-in tool to generate a secure Argon2id hash for your admin password:

1.  **Edit the tool**:
    Open `src/bin/hash_tool.rs` and change `let password = "password";` to your desired password.

2.  **Run the tool**:
    ```bash
    cargo run --bin hash_tool
    ```

3.  **Copy the output**:
    Copy the generated hash into your `values.yaml` under `secrets.APP_ADMIN_PASSWORD_HASH`.

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
| `image.repository` | Docker image repository | `docker.io/muhfalihr/kanrigate` |
| `env.APP_CLUSTER_NAME` | Name of the target cluster | `kubernetes-admin@kubernetes` |
| `env.APP_CONTROL_PLANE_ADDRESS` | K8s API Address | `https://kubernetes.default.svc:443` |
| `env.APP_ADMIN_USERNAME` | Administrator username | `admin` |
| `secrets.APP_ADMIN_PASSWORD_HASH` | Argon2id hash of the admin password | (Default is 'admin') |
| `secrets.APP_JWT_SECRET` | Secret key for signing session tokens | `replace-with-a-secure-key` |
| `service.type` | Kubernetes Service type | `ClusterIP` |

> **Security Tip**: Always use single quotes around the `APP_ADMIN_PASSWORD_HASH` in `.env` or shell environments to prevent characters like `$` being interpreted as variables.

## 🔐 RBAC Templates

Kanrigate automatically provisions several `ClusterRoles` to standardise access:
- `template-cluster-resources___admin`: Full cluster administrative rights.
- `template-cluster-resources___read-only`: Global view-only access.
- `template-namespaced-resources___developer`: Essential CRUD for developers in a namespace.
- `template-namespaced-resources___monitoring`: Access for monitoring tools and dashboards.
- `template-namespaced-resources___operation`: Operational control within namespaces.

## 🛡 Security

- **Argon2id Hashing**: Industry-standard protection against brute-force attacks.
- **JWT Sessions**: Secure, stateless session management with 24-hour expiration.
- **HttpOnly Cookies**: Prevents XSS-based token theft.
- **Minimal Footprint**: Uses `debian:bookworm-slim` for the final production image.
- **RBAC Isolation**: Operates with its own ServiceAccount and specifically scoped ClusterRoles.

## 📝 License
[Insert License Here]
