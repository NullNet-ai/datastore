-- default permissions
update permissions SET 
-- for read
read = true,
decrypt = true,
sensitive = true,
-- for write ( TODO )
encrypt = false,
write = true,
-- others ( TODO ) - use for frontend display as actions
required = true,
archive = true,
delete = true
where id = '0b023cd7-1471-4980-902e-b67f28e2c370';

update permissions SET 
-- for read
sensitive = true,
read = true,
decrypt = true,
-- for write
encrypt = true,
write = true,
-- others
required = true,
archive = true,
delete = true
where id = '26958631-a9a0-46de-ab71-442f9c970e26';


INSERT INTO 
samples (id,categories,code,tombstone,status,created_date,created_time,updated_date,updated_time,organization_id,created_by,timestamp) 
VALUES ('01JVBDCE2H1974YPADCSTJMVZK',enc deym,'CTR16',0,'Active','2025/05/15','19:04','2025/05/15','19:04','01JBHKXHYSKPP247HZZWHA3JCT','01JV9C6X14M7X6X5JYVBYD849A','2025-05-16T02:04:45.521Z') ON CONFLICT (id) DO UPDATE SET id = '01JVBDCE2H1974YPADCSTJMVZK',categories = enc deym,code = 'CTR16',tombstone = 0,status = 'Active',created_date = '2025/05/15',created_time = '19:04',updated_date = '2025/05/15',updated_time = '19:04',organization_id = '01JBHKXHYSKPP247HZZWHA3JCT',created_by = '01JV9C6X14M7X6X5JYVBYD849A',timestamp = '2025-05-16T02:04:45.521Z'