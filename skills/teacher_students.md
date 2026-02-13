table name: teacher_students
fields: ( wrap the fields like this nullable(text()) )  
required:
* if data type is integer, default value is 0 as a number not "0"
----
name: text('name'),
  teacher_id: text('teacher_id').references(() => (teachers as any).id),
  student_id: text('student_id').references(() => (students as any).id),
----

indexes:
----
all of the fields that are not primary keys
----

foreign keys:
----
all of the fields that ends with "_id" and provide correct naming convention for foreign keys
----