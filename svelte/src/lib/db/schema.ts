import type { Generated } from 'kysely';

export interface Node {
	id: Generated<number>;
	name: string;
}

export interface Event {
	id: Generated<number>;
	time: number;
	node_id: number;
}

export interface Activity {
	id: Generated<number>;
	start_time: number;
	end_time: number | null;
	node_id: number;
}

export interface DB {
	nodes: Node;
	events: Event;
	activities: Activity;
}
