table name: communication_templates
fields: ( wrap the fields like this nullable(text()) )
required:
* if data type is integer, default value is 0 as a number not "0"
----
name: text('name'),
  communication_template_status: text('communication_template_status'),
  event: text('event'),
  content: text('content'),
  subject: text('subject'),
----

indexes:
----
'name',
'communication_template_status',
'event',
'content',
'subject',
----

foreign keys:
----
all of the fields that ends with "_id" and provide correct naming convention for foreign keys
----