table name: classroom_courses
fields:
----
classroom_id: text('classroom_id').references(
    () => (classrooms as Record<string, any>).id,
  ),
  course_id: text('course_id').references(
    () => (courses as Record<string, any>).id,
  ),
  start_date: text('start_date'),
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