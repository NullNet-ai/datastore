DO $$
DECLARE
-- old permission 01JTMGV92R9S4ESY1QKHHFGZ0V = read is true
-- new permission 01JTQ5EAWX6VXRY4SDBF38SJTE = read is false
-- for field id
    with_read_false TEXT := '26958631-a9a0-46de-ab71-442f9c970e26';
    with_read_true TEXT := '0b023cd7-1471-4980-902e-b67f28e2c370';
    -- field "id"
    record_id TEXT := 'db8e73a0-5aa4-403f-9d0d-2abb3ba39407';
    switch BOOLEAN := FALSE;
    -- if switch is true then update the data permission with Read false
    -- if switch is false then update the data permission with Read true
BEGIN

UPDATE data_permissions 
SET inherited_permission_id = CASE WHEN switch THEN with_read_false ELSE with_read_true END
WHERE id = record_id;

END $$;