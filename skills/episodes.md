table name: episodes
fields:
----
name: text('name'),
  story_id: text('story_id').references(() => (stories as any).id),
  order: integer('order').default(0),
  course_id: text('course_id').references(() => (courses as any).id),
----

indexes:
----
all of the fields that are not primary keys
----

foreign keys:
----
all of the fields that ends with "_id" and provide correct naming convention for foreign keys
----