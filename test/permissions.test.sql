-- default permissions
update permissions SET 
-- for read
read = true,
decrypt = false,
sensitive = true,
-- for write ( TODO )
encrypt = true,
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
