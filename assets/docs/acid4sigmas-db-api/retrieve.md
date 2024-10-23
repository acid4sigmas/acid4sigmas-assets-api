## Retrieve
retrieve values from the database in json format
### Syntax Rules
```json
{
    "table": "<table_name>",
    "action": "Retrieve",
    "filters": {
        "where": {
            "<column_name>": <value>
            ...
        },
        "order_by": {
            "column": "<column_name>",
            "direction": "Asc" | "Desc"
        },
        "limit": <number>,
        "offset": <number>
    }
}
```

| Key | Value-Type | description |
|-----|------------|-------------|
| table | string | the name of the table |
| action | string | the action you want to perform |
| filters (Optional) | object | the filters you may want to apply |

**Filters (Optional)**
| Key | Value-Type | Description |
| ----| ---------- | ----------- |
| where | object | conditions to filter the data (key-value pairs) |
| order_by | object | the column to order by and the direction (asc/desc) |
| limit | number | the maximum number of records to retrieve |
| offset | number | how many records to skip |


**order_by**

| Key       | Value-Type | Description |
|-----------|------------|-------------|
| column    | string     | the name of the column to sort by  |
| direction | string     | sorting direction (asc or desc)    |

---
### Example usage
```json
{
    "table": "users",
    "action": "Retrieve",
    "filter": {
        "where": {
            "email_verified": false
        },
        "order_by": {
            "column": "username",
            "direction": "Asc"
        },
        "limit": 100,
        "offset": 50
    }
}
```
