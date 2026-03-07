# Full-Stack Kanban Workspace

A high-performance, production-ready Kanban board application built with **Rust** and **React 19**. 

Designed with strict Domain-Driven Design (DDD) and Clean Architecture principles, this application goes beyond simple task tracking by offering Agile performance metrics, Work-In-Progress (WIP) limits, and granular Role-Based Access Control (RBAC).

## Key Features

* **Advanced Kanban Boards:** Create multiple workspaces and manage custom columns (To Do, In Progress, Done).
* **WIP Limits:** Enforce Agile best practices by optionally limiting the number of items allowed in "To Do" or "WIP" columns.
* **Seamless Drag & Drop:** Move tasks between columns or reorder entire columns smoothly using `@hello-pangea/dnd`.
* **Agile Metrics Engine:** Automatically calculates exact **Lead Time** and **Cycle Time** for tasks once they reach the "Done" column, intelligently ignoring time spent bouncing between previous columns.
* **Task History:** Full audit trail tracking every column movement with timestamps.
* **Role-Based Access Control (RBAC):** Invite users to boards as `Owner`, `Editor`, or `Viewer`. The UI dynamically adapts to block unauthorized actions.
* **Secure Authentication:** Custom JWT-based authentication using HTTP-only cookies and bcrypt password hashing.

## Technology Stack

### Backend
* **Language:** Rust 🦀
* **Framework:** Axum (v0.8)
* **Database:** PostgreSQL
* **ORM/Query Builder:** SQLx (Compile-time verified SQL queries)
* **Architecture:** Clean Architecture (Entities, Use Cases, Repositories, Adapters)

### Frontend
* **Library:** React 19 + TypeScript
* **Build Tool:** Vite
* **Styling:** Tailwind CSS v4
* **Routing:** React Router v7
* **State & Data Handling:** Custom React Hooks + Axios
* **Drag & Drop:** `@hello-pangea/dnd`

### Infrastructure
* **Containerization:** Docker & Docker Compose
* **Web Server:** Nginx (Serving the built React SPA)

## Getting Started

### Prerequisites
* Docker and Docker Compose installed on your machine.

### Running Locally

1. Clone the repository:

   ```bash
   git clone https://github.com/GonTurri/kanban-app.git
   cd kanban-app
   ```
2. Start the services using Docker Compose:

    ```bash
    docker compose up --build -d
     ```
 3. Access the application:

- Frontend (UI): Open http://localhost:28000 in your browser.

- Backend API: Available at http://localhost:28080.

Note: *The database runs securely on an internal Docker network, mapped to persistent volumes to avoid data loss between restarts.*

# Roadmap & Future Enhancements (TODO)
While this MVP is fully functional, there are several exciting features planned for future iterations:

- [ ] API Documentation: Generate interactive OpenAPI/Swagger documentation for the Axum endpoints (e.g., using utoipa) and maintain internal domain documentation with standard cargo doc.

- [ ] **IAM Integration:** Migrate custom JWT authentication to an enterprise-grade Identity and Access Management (IAM) provider like Keycloak or Auth0 for SSO, OAuth2, and advanced security policies.

- [ ] **Email Invitations:** Expand the board invitation system to integrate with an email service (e.g., AWS SES or SendGrid) to send actual invite links to users who aren't registered yet.

- [ ] **Notification System:** Implement in-app bell notifications and email digests for relevant events (e.g., when a user is assigned to a task or their role is changed).

- [ ] **Real-time Updates:** Integrate WebSockets via Axum to broadcast board updates (card movements, new tasks) to all connected clients in real-time, eliminating the need to manually refresh or refetch.

- [ ] **Dark Mode:** Add a system-aware dark mode toggle in the frontend using Tailwind's native dark mode capabilities.

- [ ] **Rich Text Descriptions:** Upgrade the task description textarea to a Markdown or Rich Text editor (like TipTap or Quill).
