create table if not exists users (
    id bigint unsigned not null auto_increment primary key,
    name varchar(1024) not null,
    kind enum(
        "UNDERGRAD",
        "POSTGRAD",
        "MENTOR",
        "SENIOR_MENTOR",
        "HONORARY_MENTOR",
        "FACULTY"
    ) not null,
    registered_user_id bigint unsigned null
);