# actix-api

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
- [ ] Redis Cache for frequest calls.

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
* Sample request - `[GET] https://.../users?offset=20&limit=20`
```json
{
  "href": "/api/users?offset=50&limit=20",
  "next": "/api/users?offset=70&limit=20",
  "previous": "/api/users?offset=30&limit=20",
  "limit": 20,
  "offset": 50,
  "total": 74,
  "size": 20,
  "items": [..]
}
```

