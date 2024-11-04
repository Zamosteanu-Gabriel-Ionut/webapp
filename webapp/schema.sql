CREATE TABLE blog_posts (
    id SERIAL PRIMARY KEY,
    user_name TEXT NOT NULL,
    text TEXT NOT NULL,
    date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    image_path TEXT,
    avatar_path TEXT
);
