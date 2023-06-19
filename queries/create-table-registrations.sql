create table if not exists registrations (
    user_id bigint unsigned not null primary key,
    status enum(
        "unregistered",
        "started",
        "name_entered",
        "name_confirmed",
        "registered",
        "pronouns_picked",
        "housing_picked",
        "failed"
    ) not null default "unregistered",
    name varchar(1024) null
);