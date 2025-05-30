
CREATE TABLE IF NOT EXISTS users (
	pk INTEGER NOT NULL  PRIMARY KEY AUTOINCREMENT,
	id TEXT NOT NULL UNIQUE,
	fullname TEXT NOT NULL,
	username TEXT NOT NULL UNIQUE,
	password_hash TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS refresh_tokens (
	pk INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	id TEXT NOT NULL UNIQUE,
	user_id TEXT NOT NULL,
	created_at INTEGER NOT NULL,
	FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

INSERT INTO users (id, fullname, username, password_hash) VALUES
	('c3f36444-b095-45fb-9fef-5c5cdffc1070', 'Test User', 'admin', '$argon2id$v=19$m=19456,t=2,p=1$efvpqQFxBnAPtAiuASeqdw$9e19BTMyZwmpb7cbBFZQwp43VMMoCuFkZS6rqKJE1OE');
