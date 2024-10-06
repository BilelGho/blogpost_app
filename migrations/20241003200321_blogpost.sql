-- Add migration script here
CREATE TABLE BLOGPOST (
    id  SERIAL PRIMARY KEY,
    content varchar(255) NOT NULL,
    username varchar(255) NOT NULL,
    created_at varchar(255) NOT NULL,
    post_image_uuid varchar(255),
    user_image_uuid varchar(255)
);