export default `id: text().primaryKey(),
  categories: text('categories')
    .array()
    .default(sqlARRAY[]::TEXT[]),
  code: text(),
  tombstone: integer().default(0),
  status: text().default('Active'),
  previous_status: text('previous_status'),
  version: integer().default(1),
  created_date: text('created_date'),
  created_time: text('created_time'),
  updated_date: text('updated_date'),
  updated_time: text('updated_time'),
  organization_id: text('organization_id')
    .references(() => schema['organizations'].id)
    .default(sql(null)),
  created_by: text('created_by')
    .references(() => schema['account_organizations'].id)
    .default(sql(null)),
  updated_by: text('updated_by')
    .references(() => schema['account_organizations'].id)
    .default(sql(null)),
  deleted_by: text('deleted_by')
    .references(() => schema['account_organizations'].id)
    .default(sql(null)),
  requested_by: text('requested_by')
    .references(() => schema['account_organizations'].id)
    .default(sql(null)),
  timestamp: text('timestamp'),
  tags: text('tags')
    .array()
    .default(sqlARRAY[]::TEXT[]),
  image_url: varchar('image_url', { length: 300 }),
    fieldname: text(),
    originalname: text(),
    encoding: text(),
    mimetype: text(),
    destination: text(),
    filename: text(),
    path: text(),
    size: integer(),
    uploaded_by: text(),
    downloaded_by: text(),
    etag: text(),
    versionId: text(),
    download_path: text(),
    presignedURL: text(),
    presignedURLExpires: integer(),



convert into this format each: 
the <label> is Pascal Case and both <name> and <type> is snake case

example:  ROW('<label>_<name>_<type>', '<label>', '<name>', '<type>', record_email)::field_type,`;
