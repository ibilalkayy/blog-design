create table if not exists comment (
	id SERIAL PRIMARY KEY,
	names varchar not null,
	emails varchar not null,
	comments varchar not null
);
