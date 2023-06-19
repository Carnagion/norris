create table if not exists registrations (
    user_id bigint unsigned not null auto_increment primary key,
    status_tag enum(
        "unregistered",
        "started",
        "name_entered",
        "name_confirmed",
        "registered",
        "pronouns_picked",
        "housing_picked",
        "failed"
    ) not null default "unregistered",
    status_name varchar(1024) null,
    status_failure enum(
        "name_not_found",
        "wrong_name_entered",
        "wrong_kind_detected"
    ) null
);