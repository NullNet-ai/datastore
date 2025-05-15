CREATE OR REPLACE FUNCTION safe_decrypt(input JSONB, pass TEXT, orig_input JSONB)
RETURNS TEXT AS $$
DECLARE
  result TEXT;
  results JSONB[];
BEGIN
  BEGIN
    result := pgp_sym_decrypt(input, pass);
  EXCEPTION WHEN OTHERS THEN
    IF orig_inputs IS NOT NULL AND array_length(orig_inputs, 1) > 0 THEN
    results := orig_inputs;
    RETURN results;
    ELSE
    result := orig_input;
    END IF;
  END;
  RETURN result;
END;
$$ LANGUAGE plpgsql;