import { BASE_API_URI } from '$lib/utils/constants';

import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ cookies, url }) => {
	const user_id = url.searchParams.get('user_id');

	const requestInitOptions: RequestInit = {
		method: 'POST',
		credentials: 'include',
		headers: {
			'Content-Type': 'application/json'
		}
	};

	const res = await fetch(`${BASE_API_URI}/auth/set-cookie?user_id=${user_id}`, requestInitOptions);

	if (!res.ok) {
		const error = await res.text();
		console.error('Error setting cookie:', error);

		return {
			status: res.status,
			status_text: 'error',
			error
		};
	}

	if (res.headers.has('Set-Cookie')) {
		const responseHeaders = Object.fromEntries(res.headers);
		const cookieString = responseHeaders['set-cookie'];

		const regexSessionId = /sessionid=([^;]*)/;
		const regexSameSite = /SameSite=([^;]*)/;
		const regexPath = /Path=([^;]*)/;
		const regexExpires = /Expires=([^;]*)/;

		const sessionID = (cookieString.match(regexSessionId) || [])[1];
		const sameSite = (cookieString.match(regexSameSite) || [])[1];
		const path = (cookieString.match(regexPath) || [])[1];
		const expires = (cookieString.match(regexExpires) || [])[1];

		let sameSiteValue = sameSite
			? (sameSite.toLocaleLowerCase() as 'lax' | 'strict' | 'none')
			: undefined;

		cookies.set('nftminting-sessionid', sessionID, {
			httpOnly: true,
			sameSite: sameSiteValue,
			path: path,
			secure: true,
			expires: new Date(expires)
		});
	}

	return {
		status: res.status,
		status_text: 'success',
		error: null
	};
};
