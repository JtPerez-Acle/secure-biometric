# Secure Biometric - Rust Service

This directory contains the Rust implementation of the web service component of the Secure Biometric system.

## Architecture

```mermaid
graph TD
    subgraph HTTP Layer
        A[Actix Web Server] --> B[Middleware Stack]
        B --> C[API Routes]
    end
    
    subgraph Service Layer
        C --> D[Services]
        D --> E[Repositories]
    end
    
    subgraph Data Layer
        E --> F[PostgreSQL]
    end

    subgraph Monitoring
        G[Health Check] --> H[System Status]
        I[OpenAPI Docs] --> J[Swagger UI]
    end
```

## Project Structure

```
rust-process/
├── migrations/
│   ├── 20240101000000_create_users_table.sql
│   ├── 20240101000001_create_tasks_table.sql
│   ├── 20240101000002_create_projects_table.sql
│   ├── 20240101000003_create_sessions_table.sql
│   └── 20240101000004_create_api_keys_table.sql
├── src/
│   ├── api/
│   │   ├── auth.rs
│   │   ├── health.rs
│   │   ├── mod.rs
│   │   ├── openapi.rs
│   │   ├── projects.rs
│   │   ├── tasks.rs
│   │   └── users.rs
│   ├── config/
│   │   ├── database.rs
│   │   ├── mod.rs
│   │   └── settings.py
│   ├── middleware/
│   │   ├── auth_middleware.rs
│   │   ├── logger.rs
│   │   ├── mod.rs
│   │   └── rate_limiter.rs
│   ├── repositories/
│   │   ├── api_key_repository.rs
│   │   ├── mod.rs
│   │   ├── project_repository.rs
│   │   ├── session_repository.rs
│   │   ├── task_repository.rs
│   │   └── user_repository.rs
│   ├── services/
│   │   ├── auth_service.rs
│   │   └── mod.rs
│   ├── main.rs
│   └── models.rs
├── target/
├── Cargo.lock
├── Cargo.toml
└── README.md
```

## Dependencies

### Runtime Dependencies
- **Web Framework**: actix-web 4.0
- **Database**: sqlx 0.8.2 with PostgreSQL
- **Authentication**: jsonwebtoken 9.0
- **Serialization**: serde 1.0, serde_json 1.0
- **Async Runtime**: tokio 1.0
- **Documentation**: utoipa 4.0, utoipa-swagger-ui 4.0
- **Monitoring**: prometheus 0.13
- **Utilities**: 
  - uuid 1.0 (UUIDs)
  - chrono 0.4 (DateTime)
  - dotenvy 0.15 (Environment variables)
  - thiserror 1.0 (Error handling)

### Development Dependencies
- **Testing**: actix-rt 2.0, futures 0.3
- **Database**: sqlx with test features

## Database Schema

```mermaid
erDiagram
    User ||--o{ Project : creates
    User ||--o{ Task : owns
    User ||--o{ Session : has
    User ||--o{ ApiKey : owns

    User {
        UUID id PK
        string username
        string email
        timestamp created_at
        timestamp updated_at
    }

    Project {
        UUID id PK
        UUID user_id FK
        string name
        string description
        timestamp created_at
        timestamp updated_at
    }

    Task {
        UUID id PK
        UUID user_id FK
        string title
        string description
        boolean completed
        timestamp created_at
        timestamp updated_at
    }

    Session {
        UUID id PK
        UUID user_id FK
        timestamp created_at
        timestamp expires_at
    }

    ApiKey {
        UUID id PK
        UUID user_id FK
        string key
        timestamp created_at
        timestamp expires_at
    }
```

## API Endpoints

### Authentication
```
POST /api/auth/login    # User login
POST /api/auth/refresh  # Refresh JWT token
```

### User Management
```
GET    /api/users      # List users
POST   /api/users      # Create user
GET    /api/users/{id} # Get user details
```

### Project Management
```
GET    /api/projects          # List projects
POST   /api/projects          # Create project
GET    /api/projects/{id}     # Get project
PUT    /api/projects/{id}     # Update project
DELETE /api/projects/{id}     # Delete project
```

### Task Management
```
GET    /api/tasks          # List tasks
POST   /api/tasks          # Create task
GET    /api/tasks/{id}     # Get task
PUT    /api/tasks/{id}     # Update task
DELETE /api/tasks/{id}     # Delete task
```

## Development

### Setup
1. Install dependencies:
```bash
cargo build
```

2. Set up environment:
```bash
cp .env.example .env
# Edit .env with your configuration
```

3. Run migrations:
```bash
cargo run -- migrate
```

### Running
1. Start the server:
```bash
cargo run
```

2. Run tests:
```bash
cargo test
```

### Documentation
- OpenAPI documentation: `http://localhost:8080/api/docs`
- Health check: `http://localhost:8080/health`

## Configuration

Environment variables:
```env
DATABASE_URL=postgres://user:password@localhost/dbname
JWT_SECRET=your-secret-key
RUST_LOG=debug
RATE_LIMIT_REQUESTS=100
RATE_LIMIT_WINDOW=60
```

## Monitoring

The service provides:
1. Health endpoint (`/health`)
   - Database connectivity
   - Service uptime
   - System metrics

2. Prometheus metrics (`/metrics`)
   - Request rates
   - Response times
   - Error rates
   - Database pool stats
