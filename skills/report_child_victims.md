table name: report_child_victims
fields:
----
report_id: text('report_id').references(
    () => (reports as Record<string, any>).id,
  ),
  first_name: text('first_name').default(''),
  last_name: text('last_name').default(''),
  birth_date: text('birth_date').default(''), // Ex: 2012-10-15
  email: text('email').default(''),
  // Address
  address_street: text('address_street').default(''),
  address_city: text('address_city').default(''),
  address_zip_code: text('address_zip_code').default(''),
  address_state: text('address_state').default(''),
  address_non_usa_state: text('address_non_usa_state').default(''),
  address_country: text('address_country').default(''),
  // Phone
  phone_country_code: text('phone_country_code').default(''), // Ex: 1
  phone_extension: text('phone_extension').default(''), // Ex: 1234567
  screen_name: text('screen_name').default(''),

----

indexes:
----
all of the fields that are not primary keys
----

foreign keys:
----
all of the fields that ends with "_id" and provide correct naming convention for foreign keys
----