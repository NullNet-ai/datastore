--DROP FUNCTION IF EXISTS maskIfBytea(ANYELEMENT);

CREATE OR REPLACE FUNCTION maskIfBytea(input ANYELEMENT)
RETURNS JSONB AS $$
DECLARE
  result TEXT;
  has_object BOOLEAN := FALSE;
BEGIN
  RAISE NOTICE 'Input type: %', pg_typeof(input);
  -- Check if input contains brackets or curly braces
  IF position('[' in input::TEXT) > 0 OR position(']' in input::TEXT) > 0
     OR position('{' in input::TEXT) > 0 OR position('}' in input::TEXT) > 0 THEN
    RAISE NOTICE 'Input contains brackets or curly braces.';
    has_object := TRUE;
  END IF;
  BEGIN
    -- Check if input is of type jsonb and has_object is true
    IF pg_typeof(input) = 'jsonb'::regtype AND has_object THEN
      RETURN to_jsonb(input);
    ELSE
      BEGIN
        -- Try to decode as UTF-8 text
        result := convert_from(input::BYTEA, 'UTF8');
      EXCEPTION WHEN OTHERS THEN
        -- If decoding fails, it's likely binary, so mask it
        result := '**********';
      END;
    END IF;
  END;
  RETURN to_jsonb(result);
END;
$$ LANGUAGE plpgsql;
