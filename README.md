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

**Project structure & stats**
```
========================================================================================================================================
 Language                                                                     Files        Lines         Code     Comments       Blanks
========================================================================================================================================
 Rust                                                                            22          952          855           11           86
----------------------------------------------------------------------------------------------------------------------------------------
 ./api/src/auth/argon.rs                                                                      35           33            0            2
 ./api/src/auth/jwt.rs                                                                        74           64            5            5
 ./api/src/auth/middleware.rs                                                                 56           51            0            5
 ./api/src/auth/mod.rs                                                                         3            3            0            0
 ./api/src/common.rs                                                                          94           80            5            9
 ./api/src/config.rs                                                                          52           43            0            9
 ./api/src/db.rs                                                                              15           14            0            1
 ./api/src/lib.rs                                                                             48           40            0            8
 ./api/src/router.rs                                                                          53           50            0            3
 ./api/src/users/handlers.rs                                                                 162          147            1           14
 ./api/src/users/mod.rs                                                                        5            5            0            0
 ./api/src/users/models.rs                                                                    21           19            0            2
 ./api/src/users/routes.rs                                                                    36           33            0            3
 ./api/src/users/schemas.rs                                                                   22           19            0            3
 ./api/src/users/services.rs                                                                  78           69            0            9
 ./api/src/vault/handlers.rs                                                                  87           83            0            4
 ./api/src/vault/mod.rs                                                                        5            5            0            0
 ./api/src/vault/models.rs                                                                    12           11            0            1
 ./api/src/vault/routes.rs                                                                    22           21            0            1
 ./api/src/vault/schemas.rs                                                                   11           10            0            1
 ./api/src/vault/services.rs                                                                  56           51            0            5
 ./app/src/main.rs                                                                             5            4            0            1
----------------------------------------------------------------------------------------------------------------------------------------
 JSON                                                                             7          220          220            0            0
----------------------------------------------------------------------------------------------------------------------------------------
 |.sqlx/query-24ab6af098211128589de67400737a2e53f0195754cfdbbae242574c64e2028c.json           12           12            0            0
 |.sqlx/query-2c3066bba6727ec14e489a68482c18e77001933a88774205498f85efdcedc831.json           34           34            0            0
 |.sqlx/query-838b9bfa3bad2e80d9b23ef982c632c7b1699d1f6dbd797858ac109428302727.json           12           12            0            0
 |.sqlx/query-a0f0f586208983a08cbb97eca5e8df4f61c14c6f0a11f53369533891fc83215d.json           74           74            0            0
 |.sqlx/query-d03ced71e21c1769a337c5cf76e1ebc0aa004c4e41c0b5887c615c62b0f0ea1b.json           12           12            0            0
 |.sqlx/query-f5d531d61c9ddaa8d4714f0d4b09466eb84ffa6adf52ca9ee69bc4f671eff82e.json           64           64            0            0
 |.sqlx/query-fd86fbfa34edd945f7c8059792ff188d8fa9c432be2503b43cc525d5225b62e7.json           12           12            0            0
----------------------------------------------------------------------------------------------------------------------------------------
 TOML                                                                             3           34           32            0            2
----------------------------------------------------------------------------------------------------------------------------------------
 ./Cargo.toml                                                                                  3            3            0            0
 ./api/Cargo.toml                                                                             23           22            0            1
 ./app/Cargo.toml                                                                              8            7            0            1
----------------------------------------------------------------------------------------------------------------------------------------
 SQL                                                                              2           22           21            0            1
----------------------------------------------------------------------------------------------------------------------------------------
 ./migrations/20251002163556_init.down.sql                                                     2            2            0            0
 ./migrations/20251002163556_init.up.sql                                                      20           19            0            1
----------------------------------------------------------------------------------------------------------------------------------------
 Dockerfile                                                                       1           17           12            0            5
----------------------------------------------------------------------------------------------------------------------------------------
 ./Dockerfile                                                                                 17           12            0            5
----------------------------------------------------------------------------------------------------------------------------------------
 Markdown                                                                         1           89            0           83            6
----------------------------------------------------------------------------------------------------------------------------------------
 ./README.md                                                                                  89            0           83            6
----------------------------------------------------------------------------------------------------------------------------------------
 YAML                                                                             1           39           37            2            0
----------------------------------------------------------------------------------------------------------------------------------------
 ./docker-compose.yml                                                                         39           37            2            0
========================================================================================================================================
 Total                                                                           37         1373         1177           96          100
========================================================================================================================================
```
