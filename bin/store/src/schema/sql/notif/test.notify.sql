DO $$
BEGIN
    --RAISE NOTICE 'Notifiying to test_channel';
    NOTIFY test_channel, 'test_message';
    
END $$;