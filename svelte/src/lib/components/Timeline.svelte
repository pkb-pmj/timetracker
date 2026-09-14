<script lang="ts">
	import { formatDuration, formatTime } from '$lib/util';
	import {
		createTimeline,
		type ActivityIn,
		type ActivityOut,
		type EventIn,
		type EventOut,
		type IntervalIn,
		type IntervalOut,
	} from './timeline';

	let { events, activities, intervals }: Props = $props();

	interface Props {
		events: EventIn<unknown>[];
		activities: ActivityIn<unknown>[];
		intervals: IntervalIn<unknown>[];
	}

	let { timeline, numLanes, numRows } = $derived(createTimeline(events, activities, intervals));
</script>

{#snippet timelineEvent(o: EventOut<unknown>)}
	<li class="event" style:grid-row={o.row}>
		<span class="time">{formatTime(o.time)}</span>
		<div class="marker circle"></div>
		<span class="label">{o.label}</span>
	</li>
{/snippet}

{#snippet timelineTick(row: number, time: number)}
	<li class="event" style:grid-row={row}>
		<span class="time">{formatTime(time)}</span>
		<div class="marker tick"></div>
	</li>
{/snippet}

{#snippet timelineActivity(o: ActivityOut<unknown>)}
	<li class="activity">
		<a
			href="/timeline/activities/{o.ref}"
			data-sveltekit-noscroll
			class="label-container"
			style:grid-row={o.start}
		>
			<div class="label-background left">
				<span class="label">{o.label}</span>
			</div>
			<div class="label-background right" style:grid-column-end="lane-end {-o.lane}">
				<span class="duration">{formatDuration(o.duration())}</span>
			</div>
		</a>
		<div
			class="line-container"
			style:grid-row-start={o.start}
			style:grid-row-end={o.end + 1}
			style:grid-column-start="lane-start {-o.lane}"
			style:grid-column-end="lane-end {-o.lane}"
		>
			{#if o.end - o.start > 1}
				<div class="line middle"></div>
			{/if}
			<div class="line end"></div>
		</div>
	</li>
{/snippet}

{#snippet timelineInterval(o: IntervalOut<unknown>)}
	<li class="interval">
		<span class="duration" style:grid-row={o.row}>{formatDuration(o.duration())}</span>
		<div class="line-container" style:grid-row-start={o.start} style:grid-row-end={o.end + 1}>
			<div class="line start"></div>
			{#if o.end - o.start > 1}
				<div class="line middle"></div>
			{/if}
			<div class="line end"></div>
		</div>
	</li>
{/snippet}

<ul style:grid-template-rows="repeat({numRows}, auto)" style:--num-lanes={Math.max(numLanes, 1)}>
	{#each timeline as { time, row, events, activities, intervals }}
		{#each events as event}
			{@render timelineEvent(event)}
		{:else}
			{@render timelineTick(row, time)}
		{/each}
		{#each activities as activity}
			{@render timelineActivity(activity)}
		{/each}
		{#each intervals as interval}
			{@render timelineInterval(interval)}
		{/each}
	{/each}
</ul>

<style>
	ul {
		display: grid;
		grid-template-columns:
			[activity-labels-start] 1fr
			[activity-labels-end] 0.4rem
			[activity-durations-start] min-content
			[activity-durations-end] 0.4rem
			repeat(var(--num-lanes), [lane-start] min-content [lane-end] 0.2rem)
			[timeline-start] 0.6rem
			[timeline-end] 0.2rem
			[timestamps-start] min-content
			[timestamps-end] 0.4rem
			[event-labels-start] 1fr
			[event-labels-end];
	}
	li {
		&.event {
			grid-column: timeline-start / event-labels-end;
			display: grid;
			grid-template-columns: subgrid;
			align-items: center;
		}
		&.activity,
		&.interval {
			display: contents;
		}
	}
	.event {
		.label {
			grid-row: 1;
			grid-column: event-labels;
		}
	}
	.activity {
		.label-container {
			grid-column: activity-labels-start / lane-end -1;
			display: grid;
			grid-template-columns: subgrid;
			margin-top: 0.5rem;
		}
		.label-background {
			grid-row: 1;
			align-self: stretch;
			display: grid;
			align-items: center;
			background-color: rgba(0, 128, 0, 0.1);
			border: 1.5px solid green;
			&.left {
				grid-column-start: activity-labels-start;
				grid-column-end: activity-labels-end;
				justify-self: right;
				padding-left: 0.5rem;
				border-right: none;
				border-top-left-radius: 0.3rem;
				border-bottom-left-radius: 0.3rem;
			}
			&.right {
				grid-column-start: activity-labels-end;
				grid-template-columns: subgrid;
				border-right: 1.5px solid green;
				border-left: none;
				border-top-right-radius: 0.75rem;
				border-bottom-right-radius: 0;
				border-right-width: 4px;
			}
		}
	}
	.time,
	.duration {
		font-size: 0.75rem;
		text-align: left;
		/* equal width digits */
		font-variant-numeric: tabular-nums;
		/* more compact? */
		/* line-height: 1; */
	}
	.time {
		grid-column: timestamps-start / timestamps-end;
	}
	.duration {
		.activity & {
			grid-column: activity-durations-start / activity-durations-start;
			width: 5.5ch;
		}
		.interval & {
			grid-column: timeline-end / event-labels-end;
		}
	}
	.line-container {
		display: grid;
		grid-template-rows: subgrid;
		justify-self: center;
		align-self: stretch;
		.interval & {
			grid-column: timeline;
		}
	}
	.line {
		z-index: -1;
		.activity & {
			border-left: 4px solid green;
		}
		.interval & {
			border-left: 1.5px solid black;
		}
		&.start {
			grid-row: 1;
			height: 50%;
			align-self: end;
		}
		&.middle {
			grid-row: 2 / -2;
		}
		&.end {
			grid-row: -1;
			height: 50%;
			align-self: start;
			.activity & {
				border-bottom-left-radius: 2px;
				border-bottom-right-radius: 2px;
			}
		}
	}
	.marker {
		grid-column: timeline;
		grid-row: 1;
		place-self: center;
		&.circle {
			border-radius: 50%;
			border: 1.5px solid black;
			background-color: white;
			width: 0.6rem;
			height: 0.6rem;
		}
		&.tick {
			border-top: 1.5px solid black;
			width: 0.3rem;
			justify-self: right;
			height: 0;
		}
	}
</style>
