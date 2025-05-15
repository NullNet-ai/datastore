-- DROP FUNCTION IF EXISTS mask_if_binary(ANYELEMENT);

CREATE OR REPLACE FUNCTION maskIfBytea(input ANYELEMENT)
RETURNS TEXT AS $$
DECLARE
  result TEXT;
BEGIN
  BEGIN
    -- Try to decode as UTF-8 text
    result := convert_from(input::BYTEA, 'UTF8');
    EXCEPTION WHEN OTHERS THEN
    -- If decoding fails, it's likely binary, so mask it
    result := '**********';
  END;
  RETURN result;
END;
$$ LANGUAGE plpgsql;
