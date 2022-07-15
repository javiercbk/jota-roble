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

CREATE INDEX IF NOT EXISTS users_email_idx ON users (email);

-- sitemberdoggoplus
INSERT INTO users VALUES (NULL, 'Jota', 'Roble', 'jota@roble.com', 'a5fd2fbd1a26b279a822fe204626705c2c24eece254755e9982c316b6944aff35a80499c32b0e94aa40280bfb877506c74624fc72a453f3f348d735f9e9ec084', '9g3UfOQZ5xCQL0FxTK0dLf0b1aEwQ4VkMB7CJ89V8pQQ4s29ATrM9giPoQxz6w06iE7o1OYsSE9yfKMgu2SxenJGiT6rbUwomQvm1c3JT3IEtsc6jTfZRijCCCXL3YxtkAiWKWKUHhvYfJ51H09Y7rvo4dXAUFk1cJJ6HEUMBlWNkEV2Rvlbc3i16PtkhrLfGt7T7PhU77VdVh5gnXW3ItlJnfT4YmM8InXMSLZ5LrKGdnnkVTLbC8YU3JOdaSUzktfSfnS9aSgtbC6Cdxm2pqOdmbOM7xkwWJ1xVaTzuAFhdpOPqf3QkiwycAHvbgW3gQVHUBHFnpE3KcqzOaj3gr7u05zAHfPBzoQIcNjkO0pdXh8sWuQZPgbuFCM5VbR9GRn39WJWK7I0zWypbhZBXkFK3mSiHyLWyxeWQJj38kCvcjv7YqXC7fpt80P2fAJ0pYW2FcENh0AFkppMuTWIF1SSgs4Cp4tErEeslkNJZQeNM47Na7lz16V16JzmZtSC', 1);