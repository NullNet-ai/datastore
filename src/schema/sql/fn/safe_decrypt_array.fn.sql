CREATE OR REPLACE FUNCTION safe_decrypt_array(input JSONB, pass TEXT)
RETURNS JSONB AS $$
DECLARE
  decoded BYTEA;
  decrypted TEXT;
  result JSONB;
BEGIN
  BEGIN
    IF jsonb_typeof(input) = 'string' THEN
      -- Get raw string from JSONB without quotes or escapes
      decoded := decode(input #>> '{}', 'base64'); 
    ELSE
      RETURN input;
    END IF;
    -- Decrypt the bytea value
    decrypted := pgp_sym_decrypt(decoded, pass);
    -- Convert decrypted text back to JSONB
    result := decrypted::JSONB;
    RETURN result;
  EXCEPTION WHEN OTHERS THEN
    RETURN input; -- Return original value on error
  END;
END;
$$ LANGUAGE plpgsql;
