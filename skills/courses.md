table name: courses
fields:
----
title: text('title'),
order: integer('order').default(0),
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