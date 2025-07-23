🚀 **CRDT Store v0.1.8 Release**

New aggregation functionality is here! 🎯

**✨ What's New:**
• **Generic SQL Constructor** - Now works for both aggregation filter and get by filter
• **ARRAY_AGG Support** - New aggregation type for collecting values into arrays
• **Aggregation API** - New `POST /api/store/aggregation` endpoint
• **Table-Qualified Columns** - Better SQL generation with `entity.field` format
• **Fixed Request Parsing** - Uses `entity` field from request body

**📊 Supported Aggregations:**
`Sum` | `Avg` | `Count` | `Min` | `Max` | `StdDev` | `Variance` | `ARRAY_AGG`

**Example:**
```json
{
  "entity": "products",
  "aggregations": [{
    "aggregation": "ARRAY_AGG",
    "aggregate_on": "name",
    "bucket_name": "all_names"
  }]
}
```

Great for analytics and data analysis! 📈