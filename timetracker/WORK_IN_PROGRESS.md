## Components

### File backend (can be optional)

- file <-> database sync in the background, based on last edit timestamps? if timestamps from Syncthing are reliable, otherwise it needs notifications from the UI that user edited something

- conflict resolution escalating to the UI if needed, in the form of a list of conflicts (not a popup), maybe with a more visible notification

- option to recompute database from files or files from database, but normally files are the source of truth (or maybe it can be configurable? or does it even matter if last modification wins?)

- convenience functions for file manipulation?

- allows multiple data directories, synced independently

### Database (doesn't care about files, only needs notifications when something is updated in the database)

- there's only one database for all directories, allowing displaying them together on one timeline, maybe linking between them

- UI is reactive and derived from database

- edits from the UI go to the database, which drives UI updates (+ probably some loading states on big/batch edits or loads, maybe driven by `{#await}`) (+ maybe, maybe some optimistic updates if something feels slow)

### UI

- timeline loaded in chunks, only ~3 chunks visible at once, to limit data which is reactively updated

## Reactivity approach

If the amount of stuff displayed at once is sufficiently limited, everything can be recomputed on every database change without the need for a complicated granular reactivity system.

## UX guidelines

- handle all errors gracefully, explain to user and offer alternative solutions if possible, otherwise direct to GitHub issues

- limit the number of possible actions at any single moment, so the user has an easier time deciding and less possibilities to screw up/do something that wasn't considered in the app

- suggest relevant actions, keep everything discoverable without reading the docs (e.g. keyboard shortcuts)

## User flows/stories

### First time opening the app

1. User opens the app for the first time
2. The app creates its database and config
3. There's no Timeline yet, so the app only displays:
    - an input to name the new Timeline
    - a button to (optionally?) select its directory
    - a short explanation how it works (multiple Timelines, single database, sync backend)
4. On input, the app creates the new Timeline in database and on disk
5. When Timeline is created, app enters the main Timeline view and displays options to create Events/Activities/etc.
