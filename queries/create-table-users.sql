create table if not exists users (
    id bigint unsigned not null auto_increment primary key,
    name varchar(1024) not null,
    kind enum(
        "undergrad",
        "postgrad",
        "mentor",
        "senior_mentor",
        "honorary_mentor",
        "faculty"
    ) not null default "undergrad",
    registered_user_id bigint unsigned null
);