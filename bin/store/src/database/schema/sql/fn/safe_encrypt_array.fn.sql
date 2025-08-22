--DROP FUNCTION safe_encrypt_array(input JSONB, pass TEXT);
CREATE OR REPLACE FUNCTION safe_encrypt_array(input JSONB, pass TEXT)
RETURNS JSONB AS $$
DECLARE
  encrypted BYTEA;
  result TEXT;
BEGIN
  BEGIN
    -- Encrypt JSONB input converted to TEXT
    encrypted := pgp_sym_encrypt(input::TEXT, pass);
    -- Encode to base64
    result := encode(encrypted, 'base64');
    RETURN to_jsonb(result);
  EXCEPTION WHEN OTHERS THEN
    RETURN input::TEXT;  -- Fallback (returns JSON as text)
  END;
END;
$$ LANGUAGE plpgsql;
