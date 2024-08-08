CREATE TABLE achievements
(
    id          SMALLINT     NOT NULL,
    username    VARCHAR(255) NOT NULL,
    unlocked_at DATE         NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE users
(
    username   VARCHAR(255) NOT NULL,
    email      varchar(255) NOT NULL,
    password   VARCHAR(255) NOT NULL,
    top_score  INT          NOT NULL,
    created_at DATE         NOT NULL,
    PRIMARY KEY (username),
    UNIQUE (email)
);

ALTER TABLE achievements
    ADD CONSTRAINT achievements_username_foreign FOREIGN KEY (username) REFERENCES users (username);