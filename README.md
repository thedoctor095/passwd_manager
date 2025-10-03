## Password Manager Backend Application

**Tech Stack:** Rust, Axum, MySQL

**Core Features:**
  - RESTful API for user CRUD, credentials storage, and retrieval
  - Secure password hashing with Argon2
  - JWT-based session management

**User API**
  - /login
  - /user/register - only already registered users can create other users, enhancing security
  - /user/get - returns a list with all users stored in the DB
  - /user/check - returns 200 if JWT session did not expire; otherwise, re-login is required
  - /user/delete - deletes the currently logged user based on JWT session

**Vault API**
  - /vault/create - create vault entry tied to the User ID inferred from JWT
  - /vault/get - returns a list with all credentials bound to the current User ID
  - /vault/delete/{entry_id} - deletes the vault record that matches {entry_id}

The `docker-compose.yml` file together with `.env` file serve as a ready-to-use example. However, in order to use the API with a logged user (JWT-based session), an initial user has to be created manually in MySQL. This situation is by design to allow only already registered users to register other users.