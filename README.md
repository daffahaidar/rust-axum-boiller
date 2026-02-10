# Rust Axum Authentication Service

Backend REST API menggunakan Rust + Axum dengan arsitektur Clean Architecture. Service ini menyediakan fitur authentication dan user management dengan JWT-based authentication dan role-based access control (RBAC).

## 🚀 Features

- ✅ User Registration & Login
- ✅ **Login with GitHub (OAuth 2.0)**
- ✅ JWT Access Token + Refresh Token
- ✅ Role-Based Access Control (RBAC)
- ✅ User Management (Create, Update, Delete, Suspend)
- ✅ Password Hashing (Argon2)
- ✅ Clean Architecture
- ✅ PostgreSQL Database
- ✅ Input Validation
- ✅ Centralized Error Handling
- ✅ Hot Reload Development

## 📋 Prerequisites

- **Rust** (latest stable) - [Install](https://rustup.rs/)
- **PostgreSQL** 12+
- **cargo-watch** (optional, untuk hot reload)

## 🛠️ Setup Instructions

### Linux/MacOS

#### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### 2. Install PostgreSQL

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install postgresql postgresql-contrib

# MacOS (dengan Homebrew)
brew install postgresql@15
brew services start postgresql@15
```

#### 3. Clone & Setup Project

```bash
git clone https://github.com/daffahaidar/rust-axum-boiller.git
cd rust-axum-boiller

# Copy environment file
cp .env.example .env
```

#### 4. Setup Database

```bash
# Buat database
createdb dimentorin

# Edit .env dengan kredensial database Anda
nano .env  # atau vim/code
```

#### 5. Install Dependencies (Optional)

```bash
# Untuk hot reload development
cargo install cargo-watch
```

#### 6. Run Project

```bash
# Development dengan hot reload
make dev

# Atau production
make start

# Atau manual
cargo run --release
```

---

### Windows

#### 1. Install Rust

Download dan install dari [rustup.rs](https://rustup.rs/)

Atau via PowerShell:

```powershell
# Run as Administrator
Set-ExecutionPolicy Bypass -Scope Process -Force
[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
iex ((New-Object System.Net.WebClient).DownloadString('https://win.rustup.rs'))
```

#### 2. Install PostgreSQL

Download dari [postgresql.org](https://www.postgresql.org/download/windows/)

Atau via Chocolatey:

```powershell
choco install postgresql
```

#### 3. Clone & Setup Project

```powershell
git clone https://github.com/daffahaidar/rust-axum-boiller.git
cd rust-axum-boiller

# Copy environment file
copy .env.example .env
```

#### 4. Setup Database

Buka **pgAdmin** atau **psql**:

```sql
CREATE DATABASE dimentorin;
```

Edit `.env` dengan text editor (Notepad/VS Code):

```env
DATABASE_URL=postgres://postgres:password@localhost:5432/dimentorin
JWT_SECRET=supersecretkeyShouldChangeInProduction
RUST_LOG=debug
```

#### 5. Install Dependencies (Optional)

```powershell
cargo install cargo-watch
```

#### 6. Run Project

```powershell
# Development dengan hot reload
cargo watch -c -x run

# Production
cargo run --release
```

**Catatan Windows:** `make` tidak tersedia secara default. Gunakan command cargo langsung atau install [make for Windows](http://gnuwin32.sourceforge.net/packages/make.htm)

---

## 🔧 Development Commands

### Menggunakan Make (Linux/MacOS)

```bash
make dev           # Development dengan hot reload
make start         # Production server
make build         # Build production binary
make test          # Run tests
make fmt           # Format code
make check         # Check code
make clean         # Clean build artifacts
make help          # Lihat semua commands
```

### Tanpa Make (Semua Platform)

```bash
# Development dengan hot reload
cargo watch -c -x run

# Production
cargo run --release

# Build
cargo build --release

# Test
cargo test

# Format
cargo fmt

# Check
cargo check
```

## 🌐 Environment Variables

Edit file `.env`:

```env
# Database
DATABASE_URL=postgres://username:password@localhost:5432/dimentorin

# JWT Secret (PENTING: Ganti di production!)
JWT_SECRET=supersecretkeyShouldChangeInProduction

# Logging Level
RUST_LOG=debug

# GitHub OAuth (untuk Login with GitHub)
GITHUB_CLIENT_ID=your_github_client_id
GITHUB_CLIENT_SECRET=your_github_client_secret
GITHUB_REDIRECT_URI=http://localhost:8000/api/v1/auth/github/callback
```

**⚠️ SECURITY:** Jangan commit file `.env` ke Git!

## 🔐 User Roles

| Role           | Description                                             |
| -------------- | ------------------------------------------------------- |
| **User**       | Default role untuk user baru                            |
| **Mentor**     | Mentor role                                             |
| **Admin**      | Dapat membuat user dan suspend user                     |
| **SuperAdmin** | Full access - dapat edit, delete, dan manage semua user |

## 📡 API Documentation

Base URL: `http://localhost:8000/api/v1`

### Authentication Endpoints

#### 1. Register User

```bash
POST /auth/sign-up
Content-Type: application/json

{
  "name": "Daffa",
  "phone": "08123456789",
  "email": "daffa@email.com",
  "password": "password123"
}
```

#### 2. Login

```bash
POST /auth/sign-in
Content-Type: application/json

{
  "email": "daffa@email.com",
  "password": "password123"
}
```

#### 3. Refresh Token

```bash
POST /auth/refresh
Content-Type: application/json

{
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

### GitHub OAuth Endpoints

![alt text](image.png)

#### 4. Login with GitHub

```bash
GET /auth/github
```

Redirect user ke endpoint ini. Backend akan redirect ke halaman otorisasi GitHub.

#### 5. GitHub Callback

```bash
GET /auth/github/callback?code={authorization_code}
```

Dipanggil otomatis oleh GitHub setelah user authorize. Mengembalikan JWT tokens.

**Response:**

```json
{
  "meta": {
    "status": "success",
    "message": "GitHub login successful"
  },
  "results": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "token_type": "Bearer",
    "expires_in": 900
  }
}
```

**OAuth Flow:**

```
User → GET /auth/github → Redirect ke GitHub → User authorize
→ GitHub redirect ke /auth/github/callback?code=xxx
→ Backend exchange code → Fetch user info → Create/link user → Return JWT
```

> **📝 Note:** Jika email GitHub sudah terdaftar, akun akan otomatis di-link. User OAuth tidak bisa login via email/password.

### User Management Endpoints

> **⚠️ Semua endpoint ini memerlukan Authorization header**

#### 6. Get All Users (Admin, SuperAdmin)

```bash
GET /users
Authorization: Bearer {access_token}
```

#### 7. Create User (Admin, SuperAdmin)

```bash
POST /users
Authorization: Bearer {access_token}
Content-Type: application/json

{
  "name": "John Doe",
  "phone": "08123456789",
  "email": "john@email.com",
  "password": "password123",
  "role": "User"
}
```

#### 8. Update User (SuperAdmin only)

```bash
PUT /users/{id}
Authorization: Bearer {access_token}
Content-Type: application/json

{
  "name": "Jane Doe",
  "email": "jane@email.com",
  "role": "Admin"
}
```

#### 9. Delete User (SuperAdmin only)

```bash
DELETE /users/{id}
Authorization: Bearer {access_token}
```

**Note:** SuperAdmin tidak bisa menghapus akun mereka sendiri.

#### 10. Suspend/Activate User (Admin, SuperAdmin)

```bash
PATCH /users/{id}/status
Authorization: Bearer {access_token}
Content-Type: application/json

{
  "status": "Suspended"
}
```

Valid status: `Active`, `Suspended`

## 🧪 Testing Examples

### Register

```bash
curl -X POST http://localhost:8000/api/v1/auth/sign-up \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test User",
    "phone": "08123456789",
    "email": "test@email.com",
    "password": "password123"
  }'
```

### Login

```bash
curl -X POST http://localhost:8000/api/v1/auth/sign-in \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@email.com",
    "password": "password123"
  }'
```

### Login with GitHub

```bash
# Buka URL ini di browser (akan redirect ke GitHub)
curl -v http://localhost:8000/api/v1/auth/github

# Atau langsung buka di browser:
# http://localhost:8000/api/v1/auth/github
```

### Get All Users

```bash
curl -X GET http://localhost:8000/api/v1/users \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
```

## 📊 Access Control Matrix

| Action          | User | Mentor | Admin | SuperAdmin |
| --------------- | ---- | ------ | ----- | ---------- |
| Register        | ✅   | ✅     | ✅    | ✅         |
| Login           | ✅   | ✅     | ✅    | ✅         |
| Login w/ GitHub | ✅   | ✅     | ✅    | ✅         |
| View All Users  | ❌   | ❌     | ✅    | ✅         |
| Create User     | ❌   | ❌     | ✅    | ✅         |
| Edit User       | ❌   | ❌     | ❌    | ✅         |
| Delete User     | ❌   | ❌     | ❌    | ✅\*       |
| Suspend User    | ❌   | ❌     | ✅    | ✅         |

\*SuperAdmin tidak dapat menghapus akun mereka sendiri

## 🏗️ Project Structure

```
rust-axum/
├── src/
│   ├── domain/           # Business logic & entities
│   │   ├── entities/     # User, Role, UserStatus
│   │   ├── repositories/ # Repository traits
│   │   └── dtos/         # Data Transfer Objects
│   ├── usecases/         # Application logic
│   ├── handlers/         # HTTP handlers
│   ├── infrastructure/   # External dependencies
│   │   ├── database/
│   │   ├── repositories/
│   │   ├── auth/         # JWT, Password, GitHub OAuth
│   │   └── errors/
│   ├── routes/           # Route configuration
│   ├── utils/            # Helpers
│   └── main.rs
├── migrations/           # Database migrations
├── Cargo.toml           # Dependencies
├── Makefile             # Development scripts
└── .env.example         # Environment template
```

## 🚨 Troubleshooting

### Port Already in Use

```bash
# Cari process yang pakai port 8000
lsof -i :8000

# Kill process
pkill -f rust-axum
```

### Database Connection Error

```bash
# Pastikan PostgreSQL running
# MacOS
brew services list

# Linux
sudo systemctl status postgresql

# Windows
# Check di Services atau pgAdmin
```

### cargo-watch Not Found

```bash
cargo install cargo-watch
```

## 🔒 Security Features

1. ✅ Password Hashing - Argon2
2. ✅ JWT Authentication - HS256
3. ✅ GitHub OAuth 2.0 - Secure third-party login
4. ✅ Email Uniqueness - Database constraint
5. ✅ RBAC - Endpoint-level authorization
6. ✅ Input Validation - Request validation
7. ✅ Self-Deletion Prevention
8. ✅ Centralized Error Handling
9. ✅ Account Linking - GitHub ↔ Email

## 📝 License

MIT License

---

**Made with ❤️ using Rust + Axum**
