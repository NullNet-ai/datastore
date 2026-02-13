table name: report_files
fields: ( wrap the fields like this nullable(text()) )  
required:
* if data type is integer, default value is 0 as a number not "0"
----
report_id: text('report_id').references(
    () => (reports as Record<string, any>).id,
  ),
  cybertipline_report_file_id: text('cybertipline_report_file_id').default(''),
  evidence_url: text('evidence_url').default(''),

----

indexes:
----
all of the fields that are not primary keys
----

foreign keys:
----
all of the fields that ends with "_id" and provide correct naming convention for foreign keys
----