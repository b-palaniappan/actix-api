# actix-api
![GitHub](https://img.shields.io/github/license/b-palaniappan/actix-api)
![GitHub release (latest SemVer including pre-releases)](https://img.shields.io/github/v/release/b-palaniappan/actix-api?include_prereleases)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/b-palaniappan/actix-api/rust.yml)
![GitHub top language](https://img.shields.io/github/languages/top/b-palaniappan/actix-api)
![GitHub last commit](https://img.shields.io/github/last-commit/b-palaniappan/actix-api)

Rust Actix REST API application with MongoDB

### Todo

- [x] Basic CRUD calls using MongoDB.
- [x] Environment Config.
- [x] Logging.
- [x] Add JSON validator.
- [x] Add security may be JWT.
- [x] Add role based JWT.
- [x] Global and Local api Error Handling.
- [ ] Form bean and Entity bean mapping.
- [ ] Add Unit testing.
- [ ] Add integration testing.
- [ ] Docker build... with minimal base image.
- [x] Pagination & Sorting of get list.
- [x] CORS Support for API.
- [x] Custom validation error. 

### Feature Todo
- [ ] Add events (like, auth event, user event etc...) and persist in a MongoDB table.
- [ ] Implement Kafka or Pulsar event stream. Preferably Apache pulsar.
- [ ] Implement REST API client call using `reqwest` lib.
- [ ] Redis Cache for frequent requests.

### Local Setup and running
- Need to start MongoDB in local or cloud environment and update `.env` file with connection information.
- Start server using Cargo Run `cargo run` or `cargo watch -x run`
  - To install cargo watch package - `cargo install cargo-watch`
- Use register endpoint to register an user and then login with that credentials, this will generate auth token.
- Use the auth token from above call to make other calls.