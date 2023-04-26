CREATE TABLE saved_posts (
    post_id UUID NOT NULL REFERENCES posts(post_id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    save_time TIMESTAMPTZ NOT NULL DEFAULT now(),

    UNIQUE (post_id, user_id)
);


CREATE VIEW post_ext_info AS
    SELECT
        p.post_id,
        COALESCE(COUNT(l.user_id), 0) as likes,
        COALESCE(COUNT(c.author_id), 0) as comments
    FROM posts p
    LEFT JOIN post_likes l ON p.post_id = l.post_id
    LEFT JOIN comments c ON p.post_id = c.post_id
    GROUP BY p.post_id;

CREATE OR REPLACE FUNCTION is_liked(uuid, uuid) RETURNS boolean AS $$
    SELECT COALESCE((SELECT true FROM post_likes WHERE post_id = $1 AND user_id = $2), false)
$$ LANGUAGE SQL;

CREATE OR REPLACE FUNCTION is_saved(uuid, uuid) RETURNS boolean AS $$
    SELECT COALESCE((SELECT true FROM saved_posts WHERE post_id = $1 AND user_id = $2), false)
$$ LANGUAGE SQL;

CREATE OR REPLACE FUNCTION get_liked(uuid) RETURNS TABLE (post_id uuid, liked boolean) AS $$
    SELECT p.post_id, COUNT(l.user_id) > 0
    FROM posts p
    LEFT JOIN post_likes l ON p.post_id = l.post_id AND l.user_id = $1 -- kinda like WHERE but applying only to the JOIN
    GROUP BY p.post_id
$$ LANGUAGE SQL;

CREATE OR REPLACE FUNCTION get_saved(uuid) RETURNS TABLE (post_id uuid, saved boolean) AS $$
    SELECT p.post_id, COUNT(l.user_id) > 0
    FROM posts p
    LEFT JOIN saved_posts l ON p.post_id = l.post_id AND l.user_id = $1 -- kinda like WHERE but applying only to the JOIN
    GROUP BY p.post_id
$$ LANGUAGE SQL;

CREATE INDEX posts_created_idx ON posts(created);
CREATE INDEX saved_posts_save_time_idx ON saved_posts(save_time);


-- full text user search

CREATE INDEX user_search_bio_idx ON users USING GIN (to_tsvector('simple', bio));
