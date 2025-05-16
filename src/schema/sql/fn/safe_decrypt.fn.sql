CREATE OR REPLACE FUNCTION safe_decrypt(input JSONB, pass TEXT)
RETURNS TEXT AS $$
DECLARE
  result TEXT;
BEGIN
  BEGIN
    result := pgp_sym_decrypt(input, pass);
  EXCEPTION WHEN OTHERS THEN
    result := orig_input;
  END;
  RETURN result;
END;
$$ LANGUAGE plpgsql;