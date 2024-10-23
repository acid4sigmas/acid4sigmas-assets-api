### Update
update values in the database

### Syntax Rules
```json
{
  "table": "<table_name>",
  "action": "Insert",
  "values": {
    "<key1>": <value1>,
    "<key2>": <value2>,
    ...
  },
  "filters": {
    "where": {
      "<column_name>": <value>
      ...
    },
  }
}
```

| Key | Value-Type | description |
|-----|------------|-------------|
| table | string | the name of the table |
| action | string | the action you want to perform |
| values | object | the values you want to insert into the table (key-value pairs)
| filters (Optional) | object | the filters you may want to apply |

**Filters (Optional)**
| Key | Value-Type | Description |
| ----| ---------- | ----------- |
| where | object | conditions to filter the data (key-value pairs) |

### Example json
```json
{
  "table": "users",
  "action": "Update",
  "values": {
    "owner": true
  },
  "filters": {
    "where": {
      "uid": 34344543
    }
  }
}
```
