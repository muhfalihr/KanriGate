import { browser } from '$app/environment';
import { goto } from '$app/navigation';
import { env } from '$env/dynamic/public';

// Default ke '/apps' (relatif) agar diproses oleh SvelteKit proxy di server-side.
// Ini mencegah localhost terbawa (hardcoded) saat build Docker.
const BASE_URL = env.PUBLIC_API_BASE_URL || '/apps';

// Add token management for client-side use
let clientToken: string | null = null;

export function setClientToken(token: string | null) {
	if (browser) {
		clientToken = token;
	}
}

async function request<T>(endpoint: string, options: RequestInit & { token?: string } = {}): Promise<T> {
	const url = `${BASE_URL}/${endpoint}`;
	const { token, ...fetchOptions } = options;

	const headers: Record<string, string> = {
		'Content-Type': 'application/json',
		...(fetchOptions.headers as Record<string, string>)
	};

	// Use provided token (explicitly passed, e.g. from SSR load function)
	// or fall back to client-side stored token if in browser
	const authToken = token || (browser ? clientToken : null);
	if (authToken) {
		headers['Authorization'] = `Bearer ${authToken}`;
	}

	// Create a stable abort controller for this specific request
	const internalController = new AbortController();
	const timeoutId = setTimeout(() => {
		console.warn(`[API] Request timeout for ${endpoint}`);
		internalController.abort();
	}, 15000);

	// Link external signal if provided
	const onExternalAbort = () => internalController.abort();
	if (fetchOptions.signal) {
		fetchOptions.signal.addEventListener('abort', onExternalAbort);
	}

	try {
		const response = await fetch(url, { 
			...fetchOptions, 
			headers,
			signal: internalController.signal 
		});

		if (response.status === 401 && browser && endpoint !== 'login') {
			console.error(`[API] 401 Unauthorized for ${endpoint}. Redirecting to logout.`);
			goto('/logout');
			throw new Error('Unauthorized');
		}

		if (!response.ok) {
			const text = await response.text();
			const error: any = new Error(text || `Error ${response.status}: ${response.statusText}`);
			error.status = response.status;
			throw error;
		}

		const contentType = response.headers.get('content-type');
		if (contentType && contentType.includes('application/json')) {
			return (await response.json()) as T;
		}
		return (await response.text()) as unknown as T;
	} finally {
		clearTimeout(timeoutId);
		if (fetchOptions.signal) {
			fetchOptions.signal.removeEventListener('abort', onExternalAbort);
		}
	}
}

export interface ApiResponse<T> {
	meta_data: {
		status: number;
		message: string;
		exec_time: number;
	};
	data: T;
}

export interface RoleBindingsResponse {
	[namespace: string]: Record<string, boolean>;
}

export interface ClusterRoleBindingsResponse {
	[permission: string]: boolean;
}

export interface LoginRequest {
	username: string;
	password: string;
}

export interface AuthBody {
	access_token: string;
	token_type: string;
}

export const api = {
	login: (credentials: LoginRequest, options: RequestInit = {}) =>
		request<ApiResponse<AuthBody>>('login', {
			method: 'POST',
			body: JSON.stringify(credentials),
			...options
		}),

	getTemplates: (options: RequestInit & { token?: string } = {}) => request<ApiResponse<string[]>>('getTemplates', options),
	getNamespaces: (options: RequestInit & { token?: string } = {}) => request<ApiResponse<string[]>>('getNamespaces', options),
	getUsers: (options: RequestInit & { token?: string } = {}) => request<ApiResponse<string[]>>('getServiceAccounts', options),

	getRoleBindings: (username: string, options: RequestInit & { token?: string } = {}) =>
		request<ApiResponse<RoleBindingsResponse>>(`getFilteredRoleBindings?username=${encodeURIComponent(username)}`, {
			method: 'POST',
			...options
		}),

	getClusterRoleBindings: (username: string, options: RequestInit & { token?: string } = {}) =>
		request<ApiResponse<ClusterRoleBindingsResponse>>(`getFilteredClusterRoleBindings?username=${encodeURIComponent(username)}`, {
			method: 'POST',
			...options
		}),

	createServiceAccount: (username: string, options: RequestInit & { token?: string } = {}) =>
		request<ApiResponse<string>>(`createServiceAccount?username=${encodeURIComponent(username)}`, {
			method: 'POST',
			...options
		}),

	createSecret: (username: string, options: RequestInit & { token?: string } = {}) =>
		request<ApiResponse<string>>(`createSecret?username=${encodeURIComponent(username)}`, {
			method: 'POST',
			...options
		}),

	createRoleBinding: (username: string, namespace: string, permission: string, options: RequestInit & { token?: string } = {}) =>
		request<ApiResponse<string>>(`createRoleBinding?username=${encodeURIComponent(username)}&namespace=${encodeURIComponent(namespace)}&permission=${encodeURIComponent(permission)}`, {
			method: 'POST',
			...options
		}),

	createClusterRoleBinding: (username: string, permission: string, options: RequestInit & { token?: string } = {}) =>
		request<ApiResponse<string>>(`createClusterRoleBinding?username=${encodeURIComponent(username)}&permission=${encodeURIComponent(permission)}`, {
			method: 'POST',
			...options
		}),

	generateConfig: (username: string, namespace: string, options: RequestInit & { token?: string } = {}) =>
		request<ApiResponse<string>>(`generateK8sConfig?username=${encodeURIComponent(username)}&namespace=${encodeURIComponent(namespace)}`, {
			method: 'POST',
			...options
		}),

	getGenerateConfigDownloadUrl: (username: string, namespace: string) =>
		`${BASE_URL}/generateK8sConfigDownloadFile?username=${encodeURIComponent(username)}&namespace=${encodeURIComponent(namespace)}`,

	deleteServiceAccount: (username: string, options: RequestInit & { token?: string } = {}) =>
		request<ApiResponse<string>>(`deleteServiceAccount?username=${encodeURIComponent(username)}`, { method: 'DELETE', ...options }),

	deleteSecret: (username: string, options: RequestInit & { token?: string } = {}) =>
		request<ApiResponse<string>>(`deleteSecret?username=${encodeURIComponent(username)}`, { method: 'DELETE', ...options }),

	deleteRoleBinding: (username: string, namespace: string, permission: string, options: RequestInit & { token?: string } = {}) =>
		request<ApiResponse<string>>(`deleteRoleBinding?username=${encodeURIComponent(username)}&namespace=${encodeURIComponent(namespace)}&permission=${encodeURIComponent(permission)}`, {
			method: 'DELETE',
			...options
		}),

	deleteClusterRoleBinding: (username: string, permission: string, options: RequestInit & { token?: string } = {}) =>
		request<ApiResponse<string>>(`deleteClusterRoleBinding?username=${encodeURIComponent(username)}&permission=${encodeURIComponent(permission)}`, {
			method: 'DELETE',
			...options
		})
};
