table name: locations
fields: ( wrap the fields like this nullable(text()) )
required:
* if data type is integer, default value is 0 as a number not "0"
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