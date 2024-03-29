# actix-api
![GitHub](https://img.shields.io/github/license/b-palaniappan/actix-api)
![GitHub release (latest SemVer including pre-releases)](https://img.shields.io/github/v/release/b-palaniappan/actix-api?include_prereleases)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/b-palaniappan/actix-api/rust.yml)
![GitHub top language](https://img.shields.io/github/languages/top/b-palaniappan/actix-api)
![GitHub last commit](https://img.shields.io/github/last-commit/b-palaniappan/actix-api)

Rust Actix REST API application with MongoDB

### Minimize Binary size
- To minimize the size of binaries, I followed - [article](https://github.com/johnthagen/min-sized-rust). It help me reduce the final binary size from 22M to 6.8M
- Also used [cargo-unused-features](https://github.com/TimonPost/cargo-unused-features) to cleanup unused features. This helped me reduce from 6.8M to 5.3M.
  - Run `unused-features analyze` followed by `unused-features prune --input report.json`. More info in the github link above.

### Todo

- [x] Basic CRUD calls using MongoDB.
- [x] Environment Config.
- [x] Logging.
- [x] Add JSON validator.
- [x] Add security may be JWT.
- [x] Add role based JWT.
- [x] Global and Local api Error Handling.
- [x] Form bean and Entity bean mapping.
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
- [ ] Redis Cache for frequent calls.

### Tools and IDE
- MscBookPro M1
- Rustup for managing `rustc` and plugins
- IDE used are `Intellij IDEA` and [Helix Editor](https://helix-editor.com/). Helix editor is developed using Rust 🦀.
- Use [Colima](https://github.com/abiosoft/colima). Its a light weight cli alternative to docker desktop.
- Use [Starship](https://github.com/starship/starship) for shell with fish shell. Another tool developed using Rust 🦀.
- Terminal tool using is `Alacritty` with `tmux`. `Alacritty` is another rust based 🦀.3

### Similar Project
- Working on similar project with `axum` and `sqlx`, the srouce code is [here](https://github.com/b-palaniappan/axum-api)

### Random Notes.

#### Api Error response structure.

```json
{
  "status": 404,
  "time": "2022-12-25T15:25:35.089z",
  "message": "User not found for id - 2893f9283uo2",
  "debugMessage": "User not found for id - 2893f9283uo2",
  "subErrors": [
    {
      "object": "users",
      "field": "email",
      "rejectedValue": "dummyEmailgmail.com",
      "message": "invalid email address"
    }
  ]
}
```

#### Paginated data response structure.
* Sample request - `[GET] https://.../users?offset=20&limit=20&sort_by=name`
* Sample response with three elements - `data`, `meta` and `_links`.
```json
{
  "data": [{
    "id": "usr_DwgQxN3gLRX1p0g7bwny1",
    "userName": "john_doe",
    "firstName": "john",
    "lastName": "Doe",
    "email": "john_doe@c12.io"
  },
    {...}
  ],
  "meta": {
    "current_page": 1,
    "page_size": 20,
    "page_count": 12,
    "total_results": 348,
    "search_id": "VE4heV3F5m2Vf0GO_dLhu",
    "search_criteria": "",
    "sort_by": "lastName"
  },
  "_link": {
    "self": {
      "href": "/v1/users?limt=20&offset=40"
    },
    "previous": {
      "href": "/v1/users?limit=20&offset=20"
    },
    "first": {
      "href": "/v1/users?limit=20&offset=0"
    },
    "next": {
      "href": "/v1/users?limit=20&offset=60"
    },
    "last": {
      "href": "/v1/users?limit=20&offset=120"
    }
  }
}
```

