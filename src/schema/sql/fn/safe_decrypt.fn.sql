CREATE OR REPLACE FUNCTION safe_decrypt(input BYTEA, pass TEXT)
RETURNS TEXT AS $$
DECLARE
  result TEXT;
BEGIN
  RAISE NOTICE 'Input type: %', pg_typeof(input);
  
  BEGIN
    -- Try to decrypt the input
    result := pgp_sym_decrypt(input, pass);
    RAISE NOTICE 'Successfully decrypted bytea input';
  EXCEPTION WHEN OTHERS THEN
    -- If decryption fails, try to convert to text
    BEGIN
      result := convert_from(input, 'UTF8');
      RAISE NOTICE 'Decryption failed, converted bytea to UTF8 text';
    EXCEPTION WHEN OTHERS THEN
      -- If conversion fails too, return a placeholder
      result := input::TEXT;
      RAISE NOTICE 'Both decryption and conversion failed, returning placeholder';
    END;
  END;
  RETURN result;
END;
$$ LANGUAGE plpgsql;