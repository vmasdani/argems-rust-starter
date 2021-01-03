-- Your SQL goes here
create table todos (
    id integer primary key autoincrement,
    name text not null,
    completed integer not null,
    created_at datetime default current_timestamp,
    updated_at datetime
);

create trigger todos_ts after insert on todos begin
    update todos set updated_at=current_timestamp where id=new.id;
end;