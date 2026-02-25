import { PUBLIC_API_BASE_URL } from '$env/static/public';

const BASE_URL = PUBLIC_API_BASE_URL || 'http://localhost:3232/apps';

async function request<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
	const url = `${BASE_URL}/${endpoint}`;
	const headers = {
		'Content-Type': 'application/json',
		...options.headers
	};

	const response = await fetch(url, { ...options, headers });

	if (!response.ok) {
		const text = await response.text();
		throw new Error(text || `Error ${response.status}: ${response.statusText}`);
	}

	const contentType = response.headers.get('content-type');
	if (contentType && contentType.includes('application/json')) {
		return (await response.json()) as T;
	}
	return (await response.text()) as unknown as T;
}

export interface ApiResponse<T> {
	data: T;
}

export interface ServiceAccountsResponse {
	service_accounts: string[];
}

export interface RoleBindingsResponse {
	role_bindings_filtered: Record<string, Record<string, boolean>>;
}

export interface KubeConfigResponse {
	kube_config_dump: string;
}

export const api = {
	getTemplates: () => request<ApiResponse<string[]>>('getTemplates'),
	getNamespaces: () => request<ApiResponse<{ namespaces: string[] }>>('getNamespaces'),
	getUsers: () => request<ServiceAccountsResponse>('getServiceAccounts'),

	getRoleBindings: (username: string) =>
		request<ApiResponse<RoleBindingsResponse>>('getFilteredRoleBindings', {
			method: 'POST',
			body: JSON.stringify({ username })
		}),

	getClusterRoleBindings: (username: string) =>
		request<ApiResponse<RoleBindingsResponse>>('getFilteredClusterRoleBindings', {
			method: 'POST',
			body: JSON.stringify({ username })
		}),

	createServiceAccount: (username: string) =>
		request(`createServiceAccount?username=${username}`, {
			method: 'POST'
		}),

	createSecret: (username: string) =>
		request(`createSecret?username=${username}`, {
			method: 'POST'
		}),

	createRoleBinding: (username: string, namespace: string, permission: string) =>
		request('createRoleBinding', {
			method: 'POST',
			body: JSON.stringify({ username, namespace, permission })
		}),

	createClusterRoleBinding: (username: string, permission: string) =>
		request('createClusterRoleBinding', {
			method: 'POST',
			body: JSON.stringify({ username, permission })
		}),

	generateConfig: (username: string, namespace: string) =>
		request<ApiResponse<KubeConfigResponse>>(`generateK8sConfig?username=${username}&namespace=${namespace}`, {
			method: 'POST'
		}),

	deleteServiceAccount: (username: string) =>
		request(`deleteServiceAccount?username=${username}`, { method: 'DELETE' }),

	deleteSecret: (username: string) =>
		request(`deleteSecret?username=${username}`, { method: 'DELETE' }),

	deleteRoleBinding: (username: string, namespace: string, permission: string) =>
		request(`deleteRoleBinding?username=${username}&namespace=${namespace}&permission=${permission}`, {
			method: 'DELETE'
		}),

	deleteClusterRoleBinding: (username: string, permission: string) =>
		request(`deleteClusterRoleBinding?username=${username}&permission=${permission}`, {
			method: 'DELETE'
		})
};
