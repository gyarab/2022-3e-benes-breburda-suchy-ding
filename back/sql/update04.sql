
DROP VIEW post_ext_info;
CREATE VIEW post_ext_info AS
    WITH like_count AS (
        SELECT
            p.post_id,
            COALESCE(COUNT(l.user_id), 0) as likes
        FROM posts p
        LEFT JOIN post_likes l ON p.post_id = l.post_id
        GROUP BY p.post_id
    ),
    comment_count AS (
        SELECT
            p.post_id,
            COALESCE(COUNT(c.comment_id), 0) as comments 
        FROM posts p
        LEFT JOIN comments c ON p.post_id = c.post_id
        GROUP BY p.post_id
    )
    SELECT
        l.post_id,
        l.likes,
        c.comments
    FROM like_count l
    JOIN comment_count c ON l.post_id = c.post_id;

