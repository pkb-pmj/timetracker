import type { Kysely } from 'kysely';

export async function up(db: Kysely<any>): Promise<void> {
	await db.schema
		.createTable('nodes')
		.addColumn('id', 'integer', (col) => col.primaryKey().autoIncrement())
		.addColumn('name', 'text', (col) => col.notNull())
		.execute();

	await db.schema
		.createTable('events')
		.addColumn('id', 'integer', (col) => col.primaryKey().autoIncrement())
		.addColumn('time', 'integer', (col) => col.notNull())
		.addColumn('node_id', 'integer', (col) => col.references('nodes.id').notNull())
		.execute();

	await db.schema
		.createTable('activities')
		.addColumn('id', 'integer', (col) => col.primaryKey().autoIncrement())
		.addColumn('start_time', 'integer', (col) => col.notNull())
		.addColumn('end_time', 'integer')
		.addColumn('node_id', 'integer', (col) => col.references('nodes.id').notNull())
		.execute();
}

export async function down(db: Kysely<any>): Promise<void> {
	await db.schema.dropTable('activities').execute();
	await db.schema.dropTable('events').execute();
	await db.schema.dropTable('nodes').execute();
}
