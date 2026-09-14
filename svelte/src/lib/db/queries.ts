import type { ItemsIn } from '$lib/components/timeline';
import type { QueryCreator } from 'kysely';
import { db, type DB } from '.';

async function getEvents(q: QueryCreator<DB>) {
	return await q
		.selectFrom('events')
		.innerJoin('nodes', 'nodes.id', 'events.node_id')
		.select(['events.id', 'events.time', 'nodes.name'])
		.orderBy('events.time', 'asc')
		.execute();
}

async function getActivities(q: QueryCreator<DB>) {
	return await q
		.selectFrom('activities')
		.innerJoin('nodes', 'nodes.id', 'activities.node_id')
		.select([
			'activities.id',
			'activities.start_time as startTime',
			'activities.end_time as endTime',
			'nodes.name as name',
		])
		.orderBy('activities.start_time', 'asc')
		.execute();
}

async function getIntervals(q: QueryCreator<DB>) {
	return await q
		.selectFrom('events')
		.select(({ fn, ref }) => [
			'id',
			'time as startTime',
			fn
				.agg<number | null>('lead', [ref('time')])
				.over((ob) => ob.orderBy('time', 'asc'))
				.as('endTime'),
		])
		.orderBy('startTime', 'asc')
		.execute();
}

export async function getItems(): Promise<ItemsIn<number, number, number>> {
	const { events, activities, intervals } = await db.transaction().execute(async (tx) => ({
		events: await getEvents(tx),
		activities: await getActivities(tx),
		intervals: await getIntervals(tx),
	}));

	return {
		events: events.map((o) => ({
			time: o.time,
			label: o.name,
			ref: o.id,
		})),
		intervals: intervals.map((o) => ({
			start: o.startTime,
			end: o.endTime,
			ref: o.id,
		})),
		activities: activities.map((o) => ({
			start: o.startTime,
			end: o.endTime,
			label: o.name,
			ref: o.id,
		})),
	};
}

export async function createNode(name: string) {
	return (await db.insertInto('nodes').values({ name }).returning('id').executeTakeFirstOrThrow())
		.id;
}

export async function createActivity(node_id: number) {
	await db
		.insertInto('activities')
		.values({
			node_id,
			start_time: Date.now(),
			end_time: null,
		})
		.executeTakeFirstOrThrow();
}

export async function createEvent(node_id: number) {
	await db
		.insertInto('events')
		.values({
			time: Date.now(),
			node_id,
		})
		.returning('id')
		.executeTakeFirstOrThrow();
}
