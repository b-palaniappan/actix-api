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
- [ ] CORS Support for API.
- [x] Custom validation error. 

### Feature Todo
- [ ] Add events (like, auth event, user event etc...) and persist in a MongoDB table.
- [ ] Implement Kafka or Pulsar event stream. Preferably Apache pulsar.

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
```json
{
  "message": "some message",
  "data": [
    {
      "id": "rRCOeyvrlZC0o0NZmVs7L",
      "name": "John Doe",
      "created_at": "2023-01-20T23:43:51.459Z"
    },
    {
      "id": "a1zFe085r2bh3ji5JjeYf",
      "name": "Jane Doe",
      "created_at": "2023-01-20T23:45:34.764Z"
    }
  ],
  "offset": 1250,
  "no_of_elements": 2,
  "page_num": 5,
  "page_size": 20,
  "total_elements": 123,
  "total_pages": 7
}
```
