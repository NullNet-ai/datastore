# Counter Service — HTTP endpoints for Postman

Base URL: `http://localhost:8080` (or your `CODE_SERVICE_HTTP_LISTEN`). No authentication.

---

## 1. List all counters (full details)

**GET** `/counters`

Returns all counters with full details: database, entity, prefix, default_code, digits_number, and current **counter** value.

| Type   | Key   | Value |
|--------|--------|--------|
| Params | (none) | —     |

**cURL**

```bash
curl http://localhost:8080/counters
```

**Sample response (200)**

```json
{
  "counters": [
    {
      "database": "connectivo",
      "entity": "organizations",
      "prefix": "OR",
      "default_code": 1000,
      "digits_number": 6,
      "counter": 42
    },
    {
      "database": "connectivo",
      "entity": "invoices",
      "prefix": "INV",
      "default_code": 0,
      "digits_number": 6,
      "counter": 105
    },
    {
      "database": "skyll",
      "entity": "projects",
      "prefix": "PRJ",
      "default_code": 10000,
      "digits_number": 6,
      "counter": 3
    }
  ]
}
```

---

## 2. Get one counter (config + value)

**GET** `/counters/:database/:entity`

Returns the full counter record from Redis: config (prefix, default_code, digits_number) and current counter value.

| Type   | Key      | Value        |
|--------|----------|--------------|
| Path   | database | e.g. `connectivo` |
| Path   | entity   | e.g. `organizations` |

**cURL**

```bash
curl http://localhost:8080/counters/connectivo/organizations
```

**Sample response (200)**

```json
{
  "database": "connectivo",
  "entity": "organizations",
  "prefix": "OR",
  "default_code": 1000,
  "digits_number": 6,
  "counter": 42
}
```

**Sample response (404)** — counter not found

```json
{
  "error": "Counter not found",
  "database": "connectivo",
  "entity": "unknown_entity"
}
```

---

## 3. Migrate / overwrite one counter (existing)

**POST** `/migrate`

Overwrites the counter record in Redis. Body (JSON):

```json
{
  "database": "connectivo",
  "entity": "organizations",
  "prefix": "OR",
  "default_code": 1000,
  "digits_number": 6,
  "counter": 42
}
```

**cURL**

```bash
curl -X POST http://localhost:8080/migrate \
  -H "Content-Type: application/json" \
  -d '{"database":"connectivo","entity":"organizations","prefix":"OR","default_code":1000,"digits_number":6,"counter":42}'
```

---

## Postman collection outline

1. **List counters** — GET `{{baseUrl}}/counters`
2. **Get counter** — GET `{{baseUrl}}/counters/{{database}}/{{entity}}`
3. **Migrate counter** — POST `{{baseUrl}}/migrate`, body: raw JSON as above

Use a collection variable `baseUrl` = `http://localhost:8080` and optional `database` / `entity` for the get-one request.
