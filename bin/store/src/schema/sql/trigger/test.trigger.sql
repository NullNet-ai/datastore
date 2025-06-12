create or replace function notify_trigger() returns trigger as $$
declare
  payload json;
begin
  payload := json_build_object('table', TG_TABLE_NAME, 'id', NEW.id, 'action', TG_OP);
  perform pg_notify('table_changes', payload::text);
  return new;
end;
$$ language plpgsql;

create trigger my_trigger
after insert or update or delete on permissions
for each row execute function notify_trigger();