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

