
CREATE TABLE IF NOT EXISTS users (
	id TEXT PRIMARY KEY NOT NULL,
	name TEXT NOT NULL,
	username TEXT NOT NULL UNIQUE,
	password_hash TEXT NOT NULL
);

INSERT INTO users VALUES
	('c3f36444-b095-45fb-9fef-5c5cdffc1070', 'Test User', 'admin', '$argon2id$v=19$m=19456,t=2,p=1$efvpqQFxBnAPtAiuASeqdw$9e19BTMyZwmpb7cbBFZQwp43VMMoCuFkZS6rqKJE1OE');
