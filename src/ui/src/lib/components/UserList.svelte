<script lang="ts">
	import { onMount } from 'svelte';
	import { Search } from 'lucide-svelte';
	import Select from 'svelte-select';
	import { api } from '$lib/api';
	import { toast } from '$lib/toast.svelte';
	import UserCard from './UserCard.svelte';
	import KubeConfigView from './KubeConfigView.svelte';
	import { fade, fly, slide } from 'svelte/transition';

	interface User {
		id: number;
		name: string;
	}

	interface RoleAssignment {
		template: string;
		namespaces: string[];
		isSaved: boolean;
	}

	let users = $state<User[]>([]);
	let searchQuery = $state('');
	let loading = $state(true);
	let activeUserEditUser = $state<User | null>(null);
	let activeUserKubeConfig = $state<User | null>(null);
	let showModalKubeConfig = $state(false);
	let kubeConfig = $state('');
	let kubeConfigLoading = $state(false);
	let templates = $state<string[]>([]);
	let namespaces = $state<string[]>([]);
	let selectedNamespaces = $state<{ value: string; label: string }[]>([]);
	let newTemplate = $state('');
	let currentPage = $state(1);
	const USERS_PER_PAGE = 50;

	let newRoleAssignments = $state<RoleAssignment[]>([]);
	let existingRoleAssignments = $state<RoleAssignment[]>([]);
	let deletedRoleAssignments = $state<RoleAssignment[]>([]);

	let filteredUsers = $derived(
		searchQuery
			? users.filter((user) => user.name.toLowerCase().includes(searchQuery.toLowerCase()))
			: users
	);

	let totalPages = $derived(Math.ceil(filteredUsers.length / USERS_PER_PAGE));
	let paginatedUsers = $derived(
		filteredUsers.slice((currentPage - 1) * USERS_PER_PAGE, currentPage * USERS_PER_PAGE)
	);

	let roleAssignments = $derived([
		...existingRoleAssignments.filter(
			(role) =>
				!deletedRoleAssignments.some(
					(deleted) =>
						deleted.template === role.template &&
						JSON.stringify(deleted.namespaces.sort()) === JSON.stringify(role.namespaces.sort())
				)
		),
		...newRoleAssignments
	]);

	let hasUnsavedChanges = $derived(
		newRoleAssignments.length > 0 || deletedRoleAssignments.length > 0
	);

	async function fetchUsers() {
		try {
			const result = await api.getUsers();
			users = result.service_accounts.map((name, i) => ({ id: i + 1, name }));
		} catch (e) {
			toast.error('Error fetching users');
		} finally {
			loading = false;
		}
	}

	async function fetchTemplates() {
		try {
			const [templatesResult, namespacesResult] = await Promise.all([
				api.getTemplates(),
				api.getNamespaces()
			]);
			templates = templatesResult.data || [];
			namespaces = namespacesResult?.data?.namespaces || [];
			if (templates.length > 0) newTemplate = templates[0];
		} catch (error) {
			toast.error('Error loading metadata');
		}
	}

	onMount(() => {
		Promise.all([fetchUsers(), fetchTemplates()]);
	});

	async function handleUserDropdown(user: User, action: 'edit' | 'kubeconfig') {
		if (action === 'edit') {
			activeUserEditUser = user;
			await fetchNamespaceUser();
		} else if (action === 'kubeconfig') {
			activeUserKubeConfig = user;
			showModalKubeConfig = true;
			kubeConfig = '';
			kubeConfigLoading = true;
			try {
				const res = await api.getRoleBindings(user.name);
				const bindings = res.data?.role_bindings_filtered || {};
				const nsList = Object.keys(bindings);
				if (nsList.length > 0) {
					const configRes = await api.generateConfig(user.name, nsList[0]);
					kubeConfig = configRes.data?.kube_config_dump || '';
				}
			} catch (error) {
				toast.error('Failed to load kubeconfig');
			} finally {
				kubeConfigLoading = false;
			}
		}
	}

	async function fetchNamespaceUser() {
		if (!activeUserEditUser) return;
		try {
			const result = await api.getRoleBindings(activeUserEditUser.name);
			const roleBindingsObj = result.data.role_bindings_filtered;
			const templateToNamespacesMap: Record<string, Set<string>> = {};
			if (roleBindingsObj) {
				for (const [namespace, perms] of Object.entries(roleBindingsObj)) {
					for (const template of Object.keys(perms)) {
						if (!templateToNamespacesMap[template]) templateToNamespacesMap[template] = new Set();
						templateToNamespacesMap[template].add(namespace);
					}
				}
			}
			existingRoleAssignments = Object.entries(templateToNamespacesMap).map(
				([template, namespacesSet]) => ({
					template,
					namespaces: [...namespacesSet],
					isSaved: true
				})
			);
			newRoleAssignments = [];
			deletedRoleAssignments = [];
		} catch (e) {
			toast.error('Failed to fetch role bindings');
		}
	}

	function addRoleAssignment() {
		if (!newTemplate || selectedNamespaces.length === 0) return;
		const namespacesArr = selectedNamespaces.map((ns) => ns.value);
		newRoleAssignments = [
			...newRoleAssignments,
			{ template: newTemplate, namespaces: namespacesArr, isSaved: false }
		];
		selectedNamespaces = [];
	}

	function removeRoleAssignment(idx: number) {
		const role = roleAssignments[idx];
		if (!role) return;
		if (!role.isSaved) {
			newRoleAssignments = newRoleAssignments.filter(
				(_, i) => i !== idx - (existingRoleAssignments.length - deletedRoleAssignments.length)
			);
		} else {
			deletedRoleAssignments = [...deletedRoleAssignments, role];
		}
	}

	async function saveChanges() {
		if (!activeUserEditUser) return;
		try {
			const username = activeUserEditUser.name;
			for (const role of deletedRoleAssignments) {
				for (const ns of role.namespaces) {
					await api.deleteRoleBinding(username, ns, role.template);
					await api.deleteClusterRoleBinding(username, role.template);
				}
			}
			for (const role of newRoleAssignments) {
				for (const ns of role.namespaces) {
					await api.createRoleBinding(username, ns, role.template);
					await api.createClusterRoleBinding(username, role.template);
				}
			}
			toast.success('Changes saved');
			await fetchNamespaceUser();
		} catch (e: any) {
			toast.error(`Save failed: ${e.message}`);
		}
	}
</script>

<div class="user-list-container" in:fade={{ duration: 300 }}>
	<div class="header">
		<div class="title-group">
			<div class="icon-circle">
				<Search size={18} />
			</div>
			<h2>User Registry</h2>
		</div>
		<div class="search-box">
			<input type="text" bind:value={searchQuery} placeholder="Search identities..." />
		</div>
	</div>

	<div class="content">
		{#if loading}
			<div class="loading-state" in:fade>
				<div class="spinner"></div>
				<p>Synchronizing Cluster Data...</p>
			</div>
		{:else if paginatedUsers.length === 0}
			<div class="empty-state" in:fade>
				<p>No identities found.</p>
			</div>
		{:else}
			<div class="grid">
				{#each paginatedUsers as user (user.id)}
					<div in:fly={{ y: 10, duration: 300, delay: 50 }}>
						<UserCard
							{user}
							active={activeUserEditUser?.id === user.id}
							onEdit={() => handleUserDropdown(user, 'edit')}
							onShowKubeConfig={() => handleUserDropdown(user, 'kubeconfig')}
						/>
					</div>
				{/each}
			</div>
		{/if}
	</div>

	{#if totalPages > 1}
		<div class="pagination">
			<button class="p-btn" onclick={() => currentPage--} disabled={currentPage === 1}>Prev</button>
			<span class="p-info">Page <strong>{currentPage}</strong> of {totalPages}</span>
			<button class="p-btn" onclick={() => currentPage++} disabled={currentPage === totalPages}>Next</button>
		</div>
	{/if}
</div>

<div class="edit-panel {activeUserEditUser ? 'active' : ''}">
	{#if activeUserEditUser}
		<div class="edit-header">
			<h3>Edit: <span class="highlight">{activeUserEditUser.name}</span></h3>
			<button class="close-btn" onclick={() => (activeUserEditUser = null)}>×</button>
		</div>

		<div class="edit-body">
			<div class="section">
				<h4 class="s-label">New Assignment</h4>
				<div class="form-card">
					<select bind:value={newTemplate} class="stiff-select">
						{#each templates as t}<option value={t}>{t}</option>{/each}
					</select>
					<div class="ns-select-box">
						<Select
							items={namespaces.map((ns) => ({ value: ns, label: ns }))}
							value={selectedNamespaces}
							on:select={(e) => (selectedNamespaces = e.detail || [])}
							multiple
							placeholder="Select namespaces..."
						/>
					</div>
					<button class="add-btn primary" onclick={addRoleAssignment}>Add Role Assignment</button>
				</div>
			</div>

			<div class="section">
				<h4 class="s-label">Current Permissions</h4>
				<div class="role-list">
					{#each roleAssignments as role, i (role.template + role.namespaces.join())}
						<div class="role-item {role.isSaved ? '' : 'is-new'}" transition:slide={{ duration: 200 }}>
							<div class="role-info">
								<strong>{role.template}</strong>
								<p>{role.namespaces.join(', ')}</p>
							</div>
							<button class="del-btn" onclick={() => removeRoleAssignment(i)}>Remove</button>
						</div>
					{/each}
				</div>
			</div>
		</div>

		<div class="panel-footer">
			<button class="save-btn" onclick={saveChanges} disabled={!hasUnsavedChanges}>
				{hasUnsavedChanges ? 'Commit Changes' : 'No Changes Detected'}
			</button>
		</div>
	{/if}
</div>

{#if showModalKubeConfig}
	<KubeConfigView
		show={showModalKubeConfig}
		user={activeUserKubeConfig}
		{kubeConfig}
		loading={kubeConfigLoading}
		onClose={() => (showModalKubeConfig = false)}
	/>
{/if}

<style>
	.user-list-container {
		display: flex;
		flex-direction: column;
		height: 100vh;
		background: #ffffff;
		border-left: 1px solid var(--border);
	}

	.header {
		padding: 1.5rem 2rem;
		border-bottom: 1px solid var(--border);
		display: flex;
		justify-content: space-between;
		align-items: center;
		background: #ffffff;
	}

	.title-group {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.icon-circle {
		width: 36px;
		height: 36px;
		background: #f0f4f8;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--primary);
	}

	.title-group h2 {
		margin: 0;
		font-size: 1.1rem;
		font-weight: 700;
		color: var(--primary-dark);
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.search-box {
		width: 320px;
	}

	.content {
		flex: 1;
		padding: 2rem;
		overflow-y: auto;
		background: var(--background);
	}

	.grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 0.75rem;
	}

	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100%;
		color: var(--text-muted);
		gap: 1rem;
	}

	.spinner {
		width: 30px;
		height: 30px;
		border: 3px solid #e1e8e8;
		border-top-color: var(--primary);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.pagination {
		padding: 1.25rem;
		border-top: 1px solid var(--border);
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 1.5rem;
		background: #ffffff;
	}

	.p-btn {
		background: #ffffff;
		border: 1px solid var(--border);
		padding: 0.5rem 1.2rem;
		border-radius: 6px;
	}

	.p-btn:hover:not(:disabled) {
		background: var(--background);
		border-color: var(--primary);
	}

	.edit-panel {
		position: fixed;
		top: 0;
		right: -420px;
		width: 420px;
		height: 100vh;
		background: #ffffff;
		border-left: 1px solid var(--border);
		box-shadow: -5px 0 30px rgba(0, 0, 0, 0.08);
		transition: right 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		display: flex;
		flex-direction: column;
		z-index: 200;
	}

	.edit-panel.active {
		right: 0;
	}

	.edit-header {
		padding: 1.5rem;
		border-bottom: 1px solid var(--border);
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.highlight {
		color: var(--primary);
		font-weight: 800;
	}

	.edit-body {
		flex: 1;
		padding: 2rem;
		overflow-y: auto;
	}

	.section {
		margin-bottom: 2.5rem;
	}

	.s-label {
		display: block;
		font-size: 0.75rem;
		font-weight: 800;
		text-transform: uppercase;
		color: var(--text-muted);
		margin-bottom: 1rem;
		letter-spacing: 1px;
	}

	.form-card {
		padding: 1.25rem;
		background: var(--surface-light);
		border: 1px solid var(--border);
		border-radius: 10px;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.role-list {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.role-item {
		padding: 1rem;
		border: 1px solid var(--border);
		border-radius: 8px;
		display: flex;
		justify-content: space-between;
		align-items: center;
		background: #ffffff;
		transition: var(--transition);
	}

	.role-item.is-new {
		border-left: 4px solid var(--warning);
		background: #fffdf5;
	}

	.role-info strong {
		display: block;
		font-size: 0.9rem;
		margin-bottom: 0.2rem;
	}

	.role-info p {
		margin: 0;
		font-size: 0.8rem;
		color: var(--text-muted);
	}

	.panel-footer {
		padding: 1.5rem;
		border-top: 1px solid var(--border);
		background: var(--surface-light);
	}

	.save-btn {
		background: var(--primary);
		color: white;
		width: 100%;
		padding: 1rem;
		font-size: 0.95rem;
		text-transform: uppercase;
		letter-spacing: 1px;
	}

	.save-btn:disabled {
		background: #d1d8d8;
		cursor: not-allowed;
	}

	.close-btn {
		background: transparent;
		border: none;
		font-size: 1.8rem;
		color: var(--text-muted);
		line-height: 1;
	}
</style>
