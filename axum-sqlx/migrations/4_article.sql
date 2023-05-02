create table article
(
    article_id  uuid primary key     default uuid_generate_v1mc(),
    user_id     uuid        not null references "user" (user_id) on delete cascade,
    slug        text unique not null,
    title       text        not null,
    description text        not null,
    body        text        not null,
    tag_list    text[]      not null,
    created_at  timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

select trigger_updated_at('article');

create index article_tags_gin on article using gin (tag_list);

create table article_favorite
(
    article_id uuid        not null references article (article_id) on delete cascade,
    user_id    uuid        not null references "user" (user_id) on delete cascade,
    created_at timestamptz not null default now(),
    updated_at timestamptz,
    primary key (article_id, user_id)
);

select trigger_updated_at('article_favorite');

create table article_comment
(
    comment_id bigserial primary key,
    article_id uuid        not null references article (article_id) on delete cascade,
    user_id    uuid        not null references "user" (user_id) on delete cascade,
    body       text        not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

select trigger_updated_at('article_comment');

create index on article_comment (article_id, created_at);
