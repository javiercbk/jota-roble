CREATE TABLE IF NOT EXISTS users
(
    id          INTEGER PRIMARY KEY NOT NULL,
    first_name  TEXT                NOT NULL,
    last_name   TEXT                NOT NULL,
    email       TEXT                NOT NULL,
    password    TEXT                NOT NULL,
    salt        TEXT                NOT NULL,
    is_admin    INTEGER             NOT NULL DEFAULT 0 CHECK (is_admin IN (0, 1))
);

CREATE INDEX users_email_idx ON users (email);

INSERT INTO users VALUES (NULL, 'Jota', 'Roble', 'jota@roble.com', '6$7aPa4A2C$7BuuoVfMDC0ncDUXZTbrDeS8DyoXBaukm3F8WtBX88T1YRviNspwi0H/DAhz/FhuvPiPcwLhR/WM0jhgiu3Js.', '7aPa4A2C', 1);