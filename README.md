# Rust Axum Authentication Service

Backend REST API menggunakan Rust + Axum dengan arsitektur Clean Architecture. Service ini menyediakan fitur authentication dan user management dengan JWT-based authentication dan role-based access control (RBAC).

## 🚀 Features

- ✅ User Registration & Login
- ✅ JWT Access Token + Refresh Token
- ✅ Role-Based Access Control (RBAC)
- ✅ User Management (Create, Update, Delete, Suspend)
- ✅ Password Hashing (Argon2)
- ✅ Clean Architecture
- ✅ PostgreSQL Database
- ✅ Input Validation
- ✅ Centralized Error Handling

## 📋 Prerequisites

- Rust (latest stable)
- PostgreSQL
- `sqlx-cli` (optional): `cargo install sqlx-cli`

## 🛠️ Setup

### 1. Environment Variables

Copy `.env.example` ke `.env`:

```bash
cp .env.example .env
```

Isi `.env`:

```env
DATABASE_URL=postgres://postgres:admin@localhost:5432/dimentorin
JWT_SECRET=supersecretkeyShouldChangeInProduction
RUST_LOG=debug
```

### 2. Database Setup

Buat database:

```bash
createdb dimentorin
```

Aplikasi akan otomatis menjalankan migrations saat startup.

### 3. Run Application

```bash
cargo run
```

Server akan berjalan di `http://127.0.0.1:8000`

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

**Endpoint:** `POST /auth/sign-up`

**Request Body:**

```json
{
  "name": "Daffa",
  "phone": "08123456789",
  "email": "daffa@email.com",
  "password": "password123"
}
```

**Response:**

```json
{
  "meta": {
    "status": "success",
    "message": "User registered successfully"
  },
  "results": {
    "name": "Daffa",
    "phone": "08123456789",
    "email": "daffa@email.com",
    "role": "User"
  }
}
```

**Validation:**

- Email harus valid dan unique
- Password minimal 8 karakter
- Default role: `User`

---

#### 2. Login

**Endpoint:** `POST /auth/sign-in`

**Request Body:**

```json
{
  "email": "daffa@email.com",
  "password": "password123"
}
```

**Response:**

```json
{
  "meta": {
    "status": "success",
    "message": "success"
  },
  "results": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "token_type": "Bearer",
    "expires_in": 900
  }
}
```

**Token Info:**

- Access Token: Expired dalam 15 menit
- Refresh Token: Expired dalam 7 hari

---

#### 3. Refresh Token

**Endpoint:** `POST /auth/refresh`

**Request Body:**

```json
{
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Response:**

```json
{
  "meta": {
    "status": "success",
    "message": "success"
  },
  "results": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "token_type": "Bearer",
    "expires_in": 900
  }
}
```

---

### User Management Endpoints

> **⚠️ Semua endpoint ini memerlukan Authorization header dengan Bearer token**

#### 4. Get All Users

**Endpoint:** `GET /users`

**Headers:**

```
Authorization: Bearer {access_token}
```

**Access:** Admin, SuperAdmin

**Response:**

```json
{
  "meta": {
    "status": "success",
    "message": "success"
  },
  "results": [
    {
      "name": "Daffa",
      "phone": "08123456789",
      "email": "daffa@email.com",
      "role": "User"
    },
    {
      "name": "Admin User",
      "phone": "08111111111",
      "email": "admin@email.com",
      "role": "Admin"
    }
  ]
}
```

---

#### 5. Create User

**Endpoint:** `POST /users`

**Headers:**

```
Authorization: Bearer {access_token}
```

**Access:** Admin, SuperAdmin

**Request Body:**

```json
{
  "name": "John Doe",
  "phone": "08123456789",
  "email": "john@email.com",
  "password": "password123",
  "role": "User"
}
```

**Response:**

```json
{
  "meta": {
    "status": "success",
    "message": "User created successfully"
  },
  "results": {
    "name": "John Doe",
    "phone": "08123456789",
    "email": "john@email.com",
    "role": "User"
  }
}
```

---

#### 6. Update User

**Endpoint:** `PUT /users/:id`

**Headers:**

```
Authorization: Bearer {access_token}
```

**Access:** SuperAdmin only

**Request Body:**

```json
{
  "name": "Jane Doe",
  "email": "jane@email.com",
  "role": "Admin"
}
```

**Note:** Semua field optional. Hanya field yang dikirim yang akan di-update.

**Response:**

```json
{
  "meta": {
    "status": "success",
    "message": "User updated successfully"
  },
  "results": {
    "name": "Jane Doe",
    "phone": "08123456789",
    "email": "jane@email.com",
    "role": "Admin"
  }
}
```

---

#### 7. Delete User

**Endpoint:** `DELETE /users/:id`

**Headers:**

```
Authorization: Bearer {access_token}
```

**Access:** SuperAdmin only

**Restrictions:**

- SuperAdmin tidak bisa menghapus akun mereka sendiri
- Untuk menghapus akun sendiri, minta SuperAdmin lain

**Response:**

```json
{
  "meta": {
    "status": "success",
    "message": "User deleted successfully"
  },
  "results": null
}
```

---

#### 8. Suspend/Activate User

**Endpoint:** `PATCH /users/:id/status`

**Headers:**

```
Authorization: Bearer {access_token}
```

**Access:** Admin, SuperAdmin

**Request Body:**

```json
{
  "status": "Suspended"
}
```

**Valid Status:**

- `Active` - User dapat login
- `Suspended` - User di-banned, tidak bisa login

**Response:**

```json
{
  "meta": {
    "status": "success",
    "message": "User status updated successfully"
  },
  "results": {
    "name": "John Doe",
    "phone": "08123456789",
    "email": "john@email.com",
    "role": "User"
  }
}
```

---

## 📊 Access Control Matrix

| Action         | User | Mentor | Admin | SuperAdmin |
| -------------- | ---- | ------ | ----- | ---------- |
| Register       | ✅   | ✅     | ✅    | ✅         |
| Login          | ✅   | ✅     | ✅    | ✅         |
| View All Users | ❌   | ❌     | ✅    | ✅         |
| Create User    | ❌   | ❌     | ✅    | ✅         |
| Edit User      | ❌   | ❌     | ❌    | ✅         |
| Delete User    | ❌   | ❌     | ❌    | ✅\*       |
| Suspend User   | ❌   | ❌     | ✅    | ✅         |

\*SuperAdmin tidak dapat menghapus akun mereka sendiri

## 🏗️ Architecture

Project ini menggunakan **Clean Architecture** dengan pemisahan layer:

```
src/
├── domain/           # Business logic & entities
│   ├── entities/     # User, Role, UserStatus
│   ├── repositories/ # Repository traits
│   └── dtos/         # Data Transfer Objects
├── usecases/         # Application logic
│   ├── auth.rs       # Register, Login, Refresh
│   ├── users.rs      # Get Users
│   └── user_management.rs  # CRUD Users
├── handlers/         # HTTP handlers
│   ├── auth.rs
│   ├── users.rs
│   └── user_management.rs
├── infrastructure/   # External dependencies
│   ├── database/     # DB connection
│   ├── repositories/ # Repository implementations
│   ├── auth/         # JWT, Password hashing
│   └── errors/       # Error handling
├── routes/           # Route configuration
├── utils/            # Helpers (validation, response)
└── config/           # Configuration
```

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

### Get All Users (dengan token)

```bash
curl -X GET http://localhost:8000/api/v1/users \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
```

### Suspend User

```bash
curl -X PATCH http://localhost:8000/api/v1/users/USER_ID/status \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "status": "Suspended"
  }'
```

## 🔒 Security Features

1. ✅ **Password Hashing** - Argon2 algorithm
2. ✅ **JWT Authentication** - HS256 signing
3. ✅ **Email Uniqueness** - Database constraint
4. ✅ **Role-Based Access Control** - Endpoint-level authorization
5. ✅ **Input Validation** - Request validation dengan validator crate
6. ✅ **Self-Deletion Prevention** - SuperAdmin tidak bisa hapus akun sendiri
7. ✅ **Centralized Error Handling** - Konsisten error responses

## 📝 Error Responses

Semua error menggunakan format yang konsisten:

```json
{
  "meta": {
    "status": "error",
    "message": "Error message here"
  }
}
```

**Common Error Status Codes:**

- `400` - Bad Request (validation error)
- `401` - Unauthorized (invalid token)
- `403` - Forbidden (insufficient permissions)
- `404` - Not Found
- `409` - Conflict (email already exists)
- `500` - Internal Server Error

## 📄 License

MIT License
