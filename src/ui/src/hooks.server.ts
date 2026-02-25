import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	// Authentication is currently disabled to resolve module errors.
	// You can re-enable this once $lib/server/auth is configured.
	const response = await resolve(event);
	return response;
};
