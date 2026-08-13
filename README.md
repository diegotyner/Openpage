# Openpage

Long term devops project: collaborative webpage text editor. See
[`docs/architecture.md`](docs/architecture.md) for full system design and
rationale.

## Layout

- `backend/` — Rust WebSocket relay (Axum + yrs)
- `frontend/` — React + TipTap + Yjs editor client

CI/CD and infrastructure (Terraform, k3s, Docker) are not yet scaffolded.
