# actix-api

Rust Actix REST API application with MongoDB

### Todo

- [x] Basic CRUD calls using MongoDB.
- [x] Environment Config.
- [x] Logging.
- [x] Add JSON validator.
- [ ] Add security may be JWT.
- [ ] Add role based JWT.
- [x] Global and Local api Error Handling.
- [ ] Form bean and Entity bean mapping.
- [ ] Add Unit testing.
- [ ] Add integration testing.
- [ ] Docker build... with minimal base image.
- [ ] Redis cache.
- [ ] Pagination & Sorting of get list.

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
