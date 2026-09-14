import { Kysely, Migrator } from 'kysely';
import { SQLocalKysely } from 'sqlocal/kysely';
import type { DB } from './schema';
import { browser } from '$app/environment';

const { dialect, reactiveQuery } = new SQLocalKysely({
	databasePath: 'database.sqlite3',
	reactive: true,
	verbose: true,
});

const db = new Kysely<DB>({ dialect });

const migrator = new Migrator({
	db,
	provider: {
		async getMigrations() {
			return import.meta.glob('./migrations/*.ts', { eager: true });
		},
	},
});

if (browser) {
	await migrator.migrateToLatest();
}

export { db, reactiveQuery, migrator };
