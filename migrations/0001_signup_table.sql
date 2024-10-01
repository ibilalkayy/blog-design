create table if not exists signup (
	id SERIAL PRIMARY KEY,
	usernames varchar not null,
	emails varchar not null,
	passwords varchar not null
);
