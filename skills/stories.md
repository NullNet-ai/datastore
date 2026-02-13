table name: stories
fields:
----
name: text('name'),
  name: text('name'),
  course_id: text('course_id').references(() => (courses as any).id),
  order: integer('order').default(0),
----

indexes:
----
all of the fields that are not primary keys
----

foreign keys:
----
all of the fields that ends with "_id" and provide correct naming convention for foreign keys
----