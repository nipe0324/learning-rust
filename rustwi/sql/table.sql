CREATE TABLE tweets (
    id        serial primary key,
    message   text        not null,
    posted_at timestamptz not null
);

INSERT INTO tweets (message, posted_at) VALUES ('Hello, world!', now());
INSERT INTO tweets (message, posted_at) VALUES ('Good Day!!', now());

CREATE TABLE accounts (
    id           serial primary key,
    email        varchar(256) not null unique,
    password     varchar(64)  not null,
    display_name varchar(16)  not null
);
