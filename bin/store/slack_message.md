🚀 **New Feature Release: Aggregation Filter System v0.1.8**

Hey team! We've just shipped a comprehensive aggregation functionality that significantly enhances our data analysis capabilities:

**✨ What's New:**
• **Aggregation API Endpoint** - New `POST /api/store/aggregation` for complex data aggregation
• **ARRAY_AGG Support** - Added array aggregation for collecting values into arrays
• **Enhanced SQL Generation** - Table-qualified column names for clearer, more robust queries
• **Request Body Parsing** - Fixed to use `entity` field from request body (not URL params)

**🔧 Technical Highlights:**
• Extended `AggregationType` enum with proper JSON serialization
• Enhanced `construct_aggregation` method with table prefixing
• Supports: `Sum`, `Avg`, `Count`, `Min`, `Max`, `StdDev`, `Variance`, `ArrayAgg`

**📝 Example Usage:**
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

**🎯 Generated SQL:**
```sql
SELECT ARRAY_AGG(products.name) AS all_names FROM products
```

This opens up powerful analytics capabilities for our platform! 📊

*Full changelog available in CHANGELOG.md*