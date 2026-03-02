import type { RequestHandler } from './$types';
import { env } from '$env/dynamic/private';

// Use APP_PORT from environment if available, fallback to 3232
const BACKEND_PORT = env.APP_PORT || process.env.APP_PORT || '3232';
const BACKEND_HOST = 'localhost';

export const fallback: RequestHandler = async ({ request, url }) => {
	const backendUrl = new URL(url.pathname + url.search, `http://${BACKEND_HOST}:${BACKEND_PORT}`);

	const headers = new Headers(request.headers);
	// We might want to remove headers that could conflict
	headers.delete('host');
	headers.delete('connection');

	try {
		const response = await fetch(backendUrl.toString(), {
			method: request.method,
			headers,
			// Pass the body stream directly for efficiency
			body: request.method !== 'GET' && request.method !== 'HEAD' ? request.body : undefined,
			// @ts-ignore - duplex is needed for streaming bodies in some fetch implementations (like undici)
			duplex: 'half'
		});

		// Build a new response from the backend response
		const responseHeaders = new Headers(response.headers);
		// Remove potentially conflicting headers
		responseHeaders.delete('content-encoding');

		return new Response(response.body, {
			status: response.status,
			statusText: response.statusText,
			headers: responseHeaders
		});
	} catch (error) {
		console.error(`[Proxy Error] Failed to fetch from backend at ${backendUrl.toString()}: ${error}`);
		return new Response(JSON.stringify({
			meta_data: {
				status: 502,
				message: `Bad Gateway: Could not reach backend at ${backendUrl.toString()}`,
				exec_time: 0
			},
			data: null
		}), {
			status: 502,
			headers: { 'Content-Type': 'application/json' }
		});
	}
};
