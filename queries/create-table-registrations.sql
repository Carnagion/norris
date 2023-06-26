create table if not exists registrations (
    user_id bigint unsigned not null primary key,
    status enum(
        "UNREGISTERED",
        "STARTED",
        "NAME_ENTERED",
        "KIND_FOUND",
        "VERIFIED",
        "PRONOUNS_PICKED",
        "REGISTERED",
        "FAILED"
    ) not null default "UNREGISTERED",
    name varchar(1024) null,
    kind enum(
        "UNDERGRAD",
        "POSTGRAD",
        "MENTOR",
        "SENIOR_MENTOR",
        "HONORARY_MENTOR",
        "FACULTY"
    ) null
);