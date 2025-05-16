-- DROP FUNCTION IF EXISTS mask_if_binary(ANYELEMENT);

CREATE OR REPLACE FUNCTION maskIfBytea(input ANYELEMENT)
RETURNS TEXT AS $$
DECLARE
  result TEXT;
BEGIN
  -- Check if input is of type jsonb
  IF pg_typeof(input) = 'jsonb'::regtype THEN
    result := input::TEXT;
  ELSE
    BEGIN
      -- Try to decode as UTF-8 text
      result := convert_from(input::BYTEA, 'UTF8');
    EXCEPTION WHEN OTHERS THEN
      -- If decoding fails, it's likely binary, so mask it
      result := '**********';
    END;
  END IF;
  RETURN result;
END;
$$ LANGUAGE plpgsql;
