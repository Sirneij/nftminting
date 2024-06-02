import { BASE_API_URI } from '$lib/utils/constants';
import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	if (event.locals.user) {
		// if there is already a user  in session load page as normal
		return await resolve(event);
	}

	// get cookies from browser
	const session = event.cookies.get('nftminting-sessionid');

	if (!session) {
		console.log('No session in hooks.server.ts, redirecting...');

		// if there is no session load page as normal
		return await resolve(event);
	}

	// find the user based on the session
	const res = await event.fetch(`${BASE_API_URI}/auth/current`, {
		credentials: 'include',
		headers: {
			Cookie: `sessionid=${session}`
		}
	});

	if (!res.ok) {
		console.log('res', await res.text());

		// if there is no session load page as normal
		return await resolve(event);
	}

	// if `user` exists set `events.local`
	const response = await res.json();

	console.log('response', response);

	event.locals.user = response;

	// load page as normal
	return await resolve(event);
};
