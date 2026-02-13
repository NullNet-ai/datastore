table name: classroom_students
fields:
----
classroom_id: text('classroom_id').references(
    () => (classrooms as Record<string, any>).id,
  ),
  student_id: text('student_id').references(
    () => (contacts as Record<string, any>).id,
  ),

----

indexes:
----
all of the fields that are not primary keys
----

foreign keys:
----
all of the fields that ends with "_id" and provide correct naming convention for foreign keys
----