ALTER TABLE users
    DROP COLUMN email_verified,
    ADD COLUMN profile_picture TEXT,
    ADD COLUMN bio TEXT NOT NULL DEFAULT ''; -- file name

CREATE TYPE email_token_type AS ENUM ('password_restore');

CREATE TABLE email_tokens (
    user_id UUID NOT NULL REFERENCES users(user_id),
    token TEXT PRIMARY KEY DEFAULT md5(random()::text),  -- NOTIME could collide, but it surely wont
    token_type email_token_type NOT NULL
);

CREATE TABLE listeners (
    listener UUID NOT NULL REFERENCES users(user_id),
    listenee UUID NOT NULL REFERENCES users(user_id),

    UNIQUE (listener, listenee)
);
