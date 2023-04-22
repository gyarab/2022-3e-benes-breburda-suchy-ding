ALTER TABLE users
    DROP COLUMN email_verified,
    ADD COLUMN profile_picture TEXT,
    ADD COLUMN bio TEXT NOT NULL DEFAULT ''; -- file name

CREATE TYPE email_token_type AS ENUM ('password_restore');

CREATE TABLE email_tokens (
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    token TEXT PRIMARY KEY DEFAULT md5(random()::text),  -- NOTIME could collide, but it surely wont
    token_type email_token_type NOT NULL
);

CREATE TABLE listeners (
    listener UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    listenee UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,

    UNIQUE (listener, listenee)
);

CREATE TABLE posts (
    post_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    author_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    created TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE post_likes (
    post_id UUID NOT NULL REFERENCES posts(post_id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,

    UNIQUE (post_id, user_id)
);

-- post ranking cache
CREATE TABLE post_rank (
    post_id UUID NOT NULL REFERENCES posts(post_id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    rank INTEGER NOT NULL DEFAULT 0,

    UNIQUE (post_id, user_id)
);

CREATE TABLE comments (
    comment_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    post_id UUID NOT NULL REFERENCES posts(post_id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    created TIMESTAMPTZ NOT NULL DEFAULT now()
);
