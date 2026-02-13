table name: classroom_course_episodes
fields: ( wrap the fields like this nullable(text()) )
required:
* if data type is integer, default value is 0 as a number not "0"
----
classroom_id: text('classroom_id').references(
    () => (classrooms as Record<string, any>).id,
  ),
  course_id: text('course_id').references(
    () => (courses as Record<string, any>).id,
  ),
  story_id: text('story_id').references(
    () => (stories as Record<string, any>).id,
  ),
  episode_id: text('episode_id').references(
    () => (episodes as Record<string, any>).id,
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