table name: teacher_schools
fields: ( wrap the fields like this nullable(text()) )  
required:
* if data type is integer, default value is 0 as a number not "0"
----
 school_id: text('school_id').references(
    () => (organizations as Record<string, any>).id,
  ),
  teacher_id: text('teacher_id').references(
    () => (contacts as Record<string, any>).id,
  ),
  district_id: text('district_id').references(() => (organizations as any).id),
  department_id: text('department_id').references(
    () => (organizations as any).id,
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