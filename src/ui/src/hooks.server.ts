import type { Handle } from '@sveltejs/kit';
import { redirect } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	const token = event.cookies.get('access_token');

	if (token) {
		event.locals.user = { token };
	} else {
		event.locals.user = null;
	}

	// Protect routes
	const isLoginPage = event.url.pathname === '/login';
	
	if (!event.locals.user && !isLoginPage) {
		throw redirect(303, '/login');
	}

	if (event.locals.user && isLoginPage) {
		throw redirect(303, '/');
	}

	const response = await resolve(event);
	return response;
};
