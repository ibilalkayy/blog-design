create table if not exists contact (
	id SERIAL PRIMARY KEY,
	names varchar not null,
	emails varchar not null,
	messages varchar not null
);
