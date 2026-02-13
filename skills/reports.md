table name: reports
fields: ( wrap the fields like this nullable(text()) )  
required:
* if data type is integer, default value is 0 as a number not "0"
----
user_id: text('user_id').default(''),
  report_status: text('report_status').default(''), // unsubmitted, pending, open
  source: text('source').default(''), // Ex: Deputy
  app_version: text('app_version').default(''),
  // CyberTipline
  cybertipline_report_id: text('cybertipline_report_id').default(''),
  cybertipline_report_case_number: text(
    'cybertipline_report_case_number',
  ).default(''),
  cybertipline_report_type: text('cybertipline_report_type').default(''),
  cybertipline_report_status: text('cybertipline_report_status').default(''),
  cybertipline_api_response_status_code: text(
    'cybertipline_api_response_status_code',
  ).default(''),
  // Report
  additional_info: text('additional_info').default(''),
  // Reporter
  reporter_first_name: text('reporter_first_name').default(''),
  reporter_last_name: text('reporter_last_name').default(''),
  reporter_email: text('reporter_email').default(''),
  reporter_type: text('reporter_type').default(''), // myself, someone_i_know, neither
  // Incident
  incident_date: text('incident_date').default(''), // Ex: 2012-10-15
  incident_time: text('incident_time').default(''), // Ex: 08:00:00
  incident_timezone: text('incident_timezone').default(''), // Ex: -07:00
  incident_type: text('incident_type').default(''),
  incident_escalate_to_high_priority: text(
    'incident_escalate_to_high_priority',
  ).default(''),
  // Incident Location
  incident_location_type: text('incident_location_type').default(''), // web_page, email, newsgroup, chat_im, online_gambling, cellphone, peer_2_peer, non_internet
  incident_location_type_value: jsonb('incident_location_type_value').default(
    [],
  ),
  // Reported Person
  reported_person_first_name: text('reported_person_first_name').default(''),
  reported_person_last_name: text('reported_person_last_name').default(''),
  reported_person_email: text('reported_person_email').default(''),
  reported_person_screen_name: text('reported_person_screen_name').default(''),
  reported_person_profile_url: text('reported_person_profile_url').default(''),
  // Reported Person - Address
  reported_person_address_street: text(
    'reported_person_address_street',
  ).default(''),
  reported_person_address_city: text('reported_person_address_city').default(
    '',
  ),
  reported_person_address_zip_code: text(
    'reported_person_address_zip_code',
  ).default(''),
  reported_person_address_state: text('reported_person_address_state').default(
    '',
  ),
  reported_person_address_non_usa_state: text(
    'reported_person_address_non_usa_state',
  ).default(''),
  reported_person_address_country: text(
    'reported_person_address_country',
  ).default(''),
  // Reported Person - Phone
  reported_person_phone_country_code: text(
    'reported_person_phone_country_code',
  ).default(''),
  reported_person_phone_extension: text(
    'reported_person_phone_extension',
  ).default(''),

----

indexes:
----
all of the fields that are not primary keys
----

foreign keys:
----
all of the fields that ends with "_id" and provide correct naming convention for foreign keys
----