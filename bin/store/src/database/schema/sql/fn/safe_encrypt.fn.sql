CREATE OR REPLACE FUNCTION safe_encrypt(input TEXT, pass TEXT)
RETURNS TEXT AS $$
DECLARE
  result TEXT;
BEGIN
  BEGIN
    result := pgp_sym_encrypt(input, pass);
  EXCEPTION WHEN OTHERS THEN
    result := input;
  END;
  RETURN result;
END;
$$ LANGUAGE plpgsql;

