create table follow
(
    followed_user_id  uuid        not null references "user" (user_id) on delete cascade,
    following_user_id uuid        not null references "user" (user_id) on delete cascade,
    created_at        timestamptz not null default now(),
    updated_at timestamptz,

    constraint user_cannot_follow_self check (followed_user_id != following_user_id),
    primary key (following_user_id, followed_user_id)
);

SELECT trigger_updated_at('follow');
