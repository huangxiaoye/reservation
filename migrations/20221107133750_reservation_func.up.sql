CREATE OR REPLACE FUNCTION rsvp.query(
  uid text,
  rid text,
  --during TSTZRANGE DEFAULT '(-infinity, infinity)'::TSTZRANGE,
  _start timestamptz,
  _end timestamptz,
  status rsvp.reservation_status DEFAULT 'pending',
  --page integer DEFAULT 1,
  is_desc bool DEFAULT FALSE
  --page_size bigint DEFAULT 10
) RETURNS TABLE (LIKE rsvp.reservations) AS $$
DECLARE
  _during TSTZRANGE;
  _sql text;
BEGIN
  -- if start or end is null, use infinity
  _during := TSTZRANGE(
    COALESCE(_start, '-infinity'),
    COALESCE(_end, 'infinity'),
    '[)'
  );
  -- format the query based on parameters
  _sql := format(
    'SELECT * FROM rsvp.reservations WHERE %L @> timespan AND status = %L AND %s ORDER BY lower(timespan) %s',
    _during,
    status,
    CASE
        WHEN uid IS NULL AND rid IS NULL THEN 'TRUE'
        WHEN uid IS NULL THEN 'resource_id =' || quote_literal(rid)
        WHEN rid IS NULL THEN 'user_id =' || quote_literal(uid)
        ELSE 'user_id =' || quote_literal(uid) || ' AND resource_id =' || quote_literal(rid)
    END,
    CASE
      WHEN is_desc THEN 'DESC'
      ELSE 'ASC'
    END
  );
  -- log the sql
  --RAISE NOTICE '%s', _sql;
  -- execute the query
  RETURN QUERY EXECUTE _sql;
END;
$$ LANGUAGE plpgsql;
--insert into rsvp.reservations(user_id, resource_id, timespan) values('tyr', 'room-442', '("2022-11-16","2022-11-17")');

-- we filter 2 more items one for starting one for ending.
-- If starting existing, then we have previous page.
-- If ending existing, then we have next page.
CREATE OR REPLACE FUNCTION rsvp.filter(
  uid text,
  rid text,
  status rsvp.reservation_status,
  cursor bigint DEFAULT NULL,
  is_desc bool DEFAULT FALSE,
  page_size bigint DEFAULT 10
) RETURNS TABLE (LIKE rsvp.reservations) AS $$
DECLARE
  _sql text;
  _offset bigint;
BEGIN
  -- if page_size is not between 10 and 100, set it to 10
  IF page_size < 10 OR page_size > 100 THEN
    page_size := 10;
  END IF;
  -- if cursor is null or less than 0, set it to 0 if is_desc if false, or to 2^63-1 if is_des is true
  IF cursor IS NULL OR cursor < 0 THEN
    IF is_desc THEN
      cursor := 9223372036854775807;
    ELSE
      cursor := 0;
    END IF;
  END IF;
  -- format the query based on parameters
  _sql := format(
    'SELECT * FROM rsvp.reservations WHERE %s AND status = %L AND %s ORDER BY id %s LIMIT %L::integer',
    CASE
      WHEN is_desc THEN 'id <= ' || cursor
      ELSE 'id >= ' || cursor
    END,
    status,
    CASE
        WHEN uid IS NULL AND rid IS NULL THEN 'TRUE'
        WHEN uid IS NULL THEN 'resource_id =' || quote_literal(rid)
        WHEN rid IS NULL THEN 'user_id =' || quote_literal(uid)
        ELSE 'user_id =' || quote_literal(uid) || ' AND resource_id =' || quote_literal(rid)
    END,
    CASE
      WHEN is_desc THEN 'DESC'
      ELSE 'ASC'
    END,
    page_size + 1
  );
  -- log the sql
  --RAISE NOTICE '%s', _sql;
  -- execute the query
  RETURN QUERY EXECUTE _sql;
END;
$$ LANGUAGE plpgsql;
