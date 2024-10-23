## Insert
Insert values in the database
### Syntax Rules
```json
{
  "table": "<table_name>",
  "action": "Insert",
  "values": {
    "<key1>": <value1>,
    "<key2>": <value2>,
    ...
  }
}
```
| Key | Value-Type | description |
|-----|------------|-------------|
| table | string | the name of the table |
| action | string | the action you want to perform |
| values | object | the values you want to insert into the table (key-value pairs)

---
### Example usage
```json
{
  "table": "users",
  "action": "Insert",
  "values": {
    "email":"sdsd@ad.cd",
    "email_verified":false,
    "owner":true,
    "uid":3243294239,
    "username": "skibidi4343"
  }
}
```
