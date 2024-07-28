import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
	if (typeof(params.slug) === 'string') {
		return {
			app: params.slug,
		};
	}

	error(404, 'Not found');
};