CREATE TABLE tweets (
    id        serial primary key,
    message   text        not null,
    posted_at timestamptz not null
);

INSERT INTO tweets (message, posted_at) VALUES ('Hello, world!', now());
INSERT INTO tweets (message, posted_at) VALUES ('Good Day!!', now());
