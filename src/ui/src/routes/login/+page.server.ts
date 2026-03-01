import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { api } from '$lib/api';

export const actions: Actions = {
	default: async ({ request, cookies }) => {
		const formData = await request.formData();
		const username = formData.get('username') as string;
		const password = formData.get('password') as string;

		if (!username || !password) {
			return fail(400, { message: 'Username and password are required' });
		}

		try {
			const response = await api.login({ username, password });
			
			if (response?.data?.access_token) {
				cookies.set('access_token', response.data.access_token, {
					path: '/',
					httpOnly: true,
					sameSite: 'strict',
					secure: process.env.NODE_ENV === 'production',
					maxAge: 60 * 60 * 24 // 24 hours
				});

				throw redirect(303, '/');
			}
			
			return fail(401, { message: 'Invalid credentials' });
		} catch (error: any) {
			if (error.status === 303) throw error;
			
			// Try to parse the error message if it's a JSON string from our API
			let message = error.message || 'An unexpected error occurred';
			try {
				const errorData = JSON.parse(message);
				if (errorData?.meta_data?.message) {
					message = errorData.meta_data.message;
				}
			} catch (e) {
				// Not JSON, keep original message
			}

			return fail(error.status === 401 ? 401 : 500, { message });
		}
	}
};
