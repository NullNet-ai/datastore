table name: locations
fields:
----
location_name : text(),
  address_id : text().references(
    () => (addresses as Record<string, any>).id,
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