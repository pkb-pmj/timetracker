import type { ParamMatcher } from '@sveltejs/kit';

export const match = ((param: string): param is `${number}` => {
	return !isNaN(parseInt(param, 10));
}) satisfies ParamMatcher;
