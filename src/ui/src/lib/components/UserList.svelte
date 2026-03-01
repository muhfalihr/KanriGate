<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { Search, Users, X, Plus, ChevronLeft, ChevronRight, Edit3, Globe, Loader2, Trash2, AlertTriangle } from 'lucide-svelte';
	import Select from 'svelte-select';
	import { api } from '$lib/api';
	import { toast } from '$lib/toast.svelte';
	import { navigationState } from '$lib/nav.svelte';
	import type { User } from '$lib/nav.svelte';
	import UserCard from './UserCard.svelte';
	import KubeConfigView from './KubeConfigView.svelte';
	import { fade, fly, slide } from 'svelte/transition';

	interface RoleAssignment {
		template: string;
		namespaces: string[];
	}

	let users = $state<User[]>([]);
	let searchQuery = $state('');
	let loading = $state(true);
	let panelLoading = $state(false);
	let activeUserKubeConfig = $state<User | null>(null);
	let activeNamespace = $state('default');
	let showModalKubeConfig = $state(false);
	let kubeConfig = $state('');
	let kubeConfigLoading = $state(false);
	let templates = $state<string[]>([]);
	let namespaces = $state<string[]>([]);
	let selectedNamespaces = $state<{ value: string; label: string }[]>([]);
	let newTemplate = $state('');
	let currentPage = $state(1);
	const USERS_PER_PAGE = 24;

	// Request tracking
	let currentAbortController: AbortController | null = null;
	let lastRequestId = 0;

	// New state for editing
	let currentNamespacedRoles = $state<RoleAssignment[]>([]);
	let currentClusterRoles = $state<string[]>([]);
	let initialNamespacedRoles = $state<RoleAssignment[]>([]);
	let initialClusterRoles = $state<string[]>([]);

	// Delete confirmation state
	let userToDelete = $state<User | null>(null);
	let isDeleting = $state(false);

	let filteredUsers = $derived(
		searchQuery
			? users.filter((user) => user.name.toLowerCase().includes(searchQuery.toLowerCase()))
			: users
	);

	let totalPages = $derived(Math.ceil(filteredUsers.length / USERS_PER_PAGE));
	let paginatedUsers = $derived(
		filteredUsers.slice((currentPage - 1) * USERS_PER_PAGE, currentPage * USERS_PER_PAGE)
	);

	let hasUnsavedChanges = $derived.by(() => {
		const currentNS = JSON.stringify($state.snapshot(currentNamespacedRoles));
		const initialNS = JSON.stringify($state.snapshot(initialNamespacedRoles));
		const currentCluster = JSON.stringify($state.snapshot(currentClusterRoles));
		const initialCluster = JSON.stringify($state.snapshot(initialClusterRoles));
		return currentNS !== initialNS || currentCluster !== initialCluster;
	});

	async function fetchUsers() {
		try {
			const result = await api.getUsers();
			users = result.data.map((name, i) => ({ id: i + 1, name }));
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
			namespaces = namespacesResult.data || [];
			if (templates.length > 0) newTemplate = templates[0];
		} catch (error) {
			toast.error('Error loading metadata');
		} finally {
			loading = false;
		}
	}

	onMount(() => {
		Promise.all([fetchUsers(), fetchTemplates()]);
	});

	onDestroy(() => {
		if (currentAbortController) currentAbortController.abort();
	});

	async function handleUserDropdown(user: User, action: 'edit' | 'kubeconfig' | 'delete') {
		if (action === 'edit') {
			// Clear UI state
			panelLoading = false;
			currentNamespacedRoles = [];
			initialNamespacedRoles = [];
			currentClusterRoles = [];
			initialClusterRoles = [];
			
			navigationState.activeUserEditUser = user;
			await fetchNamespaceUser();
		} else if (action === 'kubeconfig') {
			activeUserKubeConfig = user;
			showModalKubeConfig = true;
			kubeConfig = '';
			kubeConfigLoading = true;
			try {
				const res = await api.getRoleBindings(user.name);
				const bindings = res.data || {};
				const nsList = Object.keys(bindings);
				const targetNamespace = nsList.length > 0 ? nsList[0] : 'default';
				
				const configRes = await api.generateConfig(user.name, targetNamespace);
				kubeConfig = configRes.data || '';
			} catch (error) {
				toast.error('Failed to load kubeconfig');
			} finally {
				kubeConfigLoading = false;
			}
		} else if (action === 'delete') {
			userToDelete = user;
		}
	}

	async function confirmDelete() {
		if (!userToDelete) return;
		isDeleting = true;
		try {
			await api.deleteServiceAccount(userToDelete.name);
			toast.success(`User ${userToDelete.name} deleted successfully`);
			users = users.filter(u => u.name !== userToDelete?.name);
			userToDelete = null;
		} catch (error: any) {
			toast.error(`Failed to delete user: ${error.message}`);
		} finally {
			isDeleting = false;
		}
	}

	async function fetchNamespaceUser() {
		if (!navigationState.activeUserEditUser) return;
		
		const requestId = ++lastRequestId;
		const targetUser = navigationState.activeUserEditUser.name;
		
		if (currentAbortController) currentAbortController.abort();
		currentAbortController = new AbortController();
		
		panelLoading = true;
		console.log(`[Permission] FETCH START: ${targetUser} (Req #${requestId})`);
		
		try {
			const [rbRes, crbRes] = await Promise.all([
				api.getRoleBindings(targetUser, { signal: currentAbortController.signal }),
				api.getClusterRoleBindings(targetUser, { signal: currentAbortController.signal })
			]);

			if (requestId !== lastRequestId) return;

			// Defensive data extraction
			const rbData = (rbRes && rbRes.data) ? rbRes.data : {};
			const crbData = (crbRes && crbRes.data) ? crbRes.data : {};
			
			console.log(`[Permission] DATA RECEIVED for ${targetUser}`, { rbData, crbData });

			// UNIFIED PROCESSING: Merge Cluster and Namespaced roles
			const unifiedRoles: Record<string, { namespaces: Set<string>, isCluster: boolean }> = {};

			// 1. Process Cluster Roles (Key is template name)
			if (crbData && typeof crbData === 'object') {
				Object.keys(crbData).forEach(template => {
					unifiedRoles[template] = { namespaces: new Set(), isCluster: true };
				});
			}

			// 2. Process Namespaced Roles
			if (rbData && typeof rbData === 'object') {
				Object.entries(rbData).forEach(([namespace, perms]) => {
					if (perms && typeof perms === 'object') {
						Object.keys(perms).forEach(template => {
							if (!unifiedRoles[template]) {
								unifiedRoles[template] = { namespaces: new Set(), isCluster: false };
							}
							unifiedRoles[template].namespaces.add(namespace);
						});
					}
				});
			}

			// Convert Map to display format
			const processedRoles = Object.entries(unifiedRoles)
				.map(([template, info]) => ({
					template,
					namespaces: Array.from(info.namespaces).sort()
				}))
				.sort((a, b) => a.template.localeCompare(b.template));

			const clusterRoleNames = Object.entries(unifiedRoles)
				.filter(([_, info]) => info.isCluster)
				.map(([template, _]) => template)
				.sort();

			if (requestId === lastRequestId) {
				currentNamespacedRoles = processedRoles;
				initialNamespacedRoles = JSON.parse(JSON.stringify(processedRoles));
				currentClusterRoles = clusterRoleNames;
				initialClusterRoles = [...clusterRoleNames];
				console.log(`[Permission] FETCH SUCCESS: ${targetUser}`);
			}
		} catch (error: any) {
			if (error.name === 'AbortError') return;
			console.error(`[Permission] ERROR for ${targetUser}:`, error);
			if (requestId === lastRequestId) {
				toast.error(`Error: ${error.message || 'Failed to load data'}`);
			}
		} finally {
			if (requestId === lastRequestId) {
				panelLoading = false;
				currentAbortController = null;
				console.log(`[Permission] LOADING STOPPED: ${targetUser}`);
			}
		}
	}

	function addRoleAssignment() {
		if (!newTemplate || !selectedNamespaces || selectedNamespaces.length === 0) return;
		const namespacesToAdd = selectedNamespaces.map((ns) => ns.value);
		
		let existingIndex = currentNamespacedRoles.findIndex(r => r.template === newTemplate);
		if (existingIndex !== -1) {
			// OVERWRITE instead of MERGE to allow removing namespaces via Edit -> Select -> Add
			currentNamespacedRoles = currentNamespacedRoles.map((r, i) => 
				i === existingIndex ? { ...r, namespaces: namespacesToAdd.sort() } : r
			);
		} else {
			currentNamespacedRoles = [
				...currentNamespacedRoles,
				{ template: newTemplate, namespaces: namespacesToAdd.sort() }
			].sort((a, b) => a.template.localeCompare(b.template));
		}
		
		selectedNamespaces = [];
	}

	function removeNamespace(templateIndex: number, ns: string) {
		const role = currentNamespacedRoles[templateIndex];
		const updatedNamespaces = role.namespaces.filter(n => n !== ns);
		
		if (updatedNamespaces.length === 0) {
			const templateToRemove = role.template;
			currentNamespacedRoles = currentNamespacedRoles.filter((_, i) => i !== templateIndex);
			currentClusterRoles = currentClusterRoles.filter(t => t !== templateToRemove);
		} else {
			// Create a new array with the updated role object
			currentNamespacedRoles = currentNamespacedRoles.map((r, i) => 
				i === templateIndex ? { ...r, namespaces: updatedNamespaces } : r
			);
		}
	}

	function removeRoleTemplate(index: number) {
		const template = currentNamespacedRoles[index].template;
		currentNamespacedRoles = currentNamespacedRoles.filter((_, i) => i !== index);
		currentClusterRoles = currentClusterRoles.filter(t => t !== template);
	}

	function toggleClusterRole(template: string) {
		if (currentClusterRoles.includes(template)) {
			currentClusterRoles = currentClusterRoles.filter(t => t !== template);
		} else {
			currentClusterRoles = [...currentClusterRoles, template].sort();
		}
	}

	function prepareEditRole(index: number) {
		const role = currentNamespacedRoles[index];
		newTemplate = role.template;
		selectedNamespaces = role.namespaces.map(ns => ({ value: ns, label: ns }));
		const editBody = document.querySelector('.edit-body');
		if (editBody) editBody.scrollTo({ top: 0, behavior: 'smooth' });
	}

	async function saveChanges() {
		if (!navigationState.activeUserEditUser) return;
		const username = navigationState.activeUserEditUser.name;
		panelLoading = true;
		try {
			console.log(`[Permission] SAVE START: ${username}`);
			// 1. Diffs for Namespaced Roles
			const initialMap: Record<string, Set<string>> = {};
			initialNamespacedRoles.forEach(r => initialMap[r.template] = new Set(r.namespaces));
			
			const currentMap: Record<string, Set<string>> = {};
			currentNamespacedRoles.forEach(r => currentMap[r.template] = new Set(r.namespaces));

			// Delete old bindings
			for (const [template, initialNs] of Object.entries(initialMap)) {
				const currentNs = currentMap[template] || new Set();
				for (const ns of initialNs) {
					if (!currentNs.has(ns)) {
						console.log(`[Permission] DELETE binding: ${template} in ${ns}`);
						await api.deleteRoleBinding(username, ns, template);
					}
				}
			}

			// Create new bindings
			for (const [template, currentNs] of Object.entries(currentMap)) {
				const initialNs = initialMap[template] || new Set();
				for (const ns of currentNs) {
					if (!initialNs.has(ns)) {
						console.log(`[Permission] CREATE binding: ${template} in ${ns}`);
						await api.createRoleBinding(username, ns, template);
					}
				}
			}

			// 2. Diffs for Cluster Roles
			const initialClusterSet = new Set(initialClusterRoles);
			const currentClusterSet = new Set(currentClusterRoles);

			for (const template of initialClusterSet) {
				if (!currentClusterSet.has(template)) {
					console.log(`[Permission] DELETE cluster binding: ${template}`);
					await api.deleteClusterRoleBinding(username, template);
				}
			}

			for (const template of currentClusterSet) {
				if (!initialClusterSet.has(template)) {
					console.log(`[Permission] CREATE cluster binding: ${template}`);
					await api.createClusterRoleBinding(username, template);
				}
			}

			toast.success('Permissions updated successfully');
			await fetchNamespaceUser();
		} catch (e: any) {
			console.error(`[Permission] SAVE ERROR:`, e);
			toast.error(`Save failed: ${e.message}`);
		} finally {
			panelLoading = false;
		}
	}

	function handleCopyKubeConfig() {
		if (kubeConfig) {
			navigator.clipboard.writeText(kubeConfig);
			toast.success('Kubeconfig copied to clipboard');
		}
	}

	function handleDownloadKubeConfig() {
		if (!activeUserKubeConfig || !kubeConfig) return;
		const blob = new Blob([kubeConfig], { type: 'text/yaml' });
		const url = URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = `kubeconfig-${activeUserKubeConfig.name}.yaml`;
		a.click();
		URL.revokeObjectURL(url);
	}
</script>

<div class="user-list-container" in:fade={{ duration: 300 }}>
	<div class="header">
		<div class="header-left">
			<div class="title-group">
				<div class="icon-circle">
					<Users size={20} />
				</div>
				<div class="title-text">
					<h2>User Registry</h2>
					<span class="count">{users.length} Total Identities</span>
				</div>
			</div>
		</div>

		<div class="header-center">
			<div class="search-box">
				<span class="search-icon-wrapper">
					<Search size={18} />
				</span>
				<input type="text" bind:value={searchQuery} placeholder="Search identities..." />
				{#if searchQuery}
					<button type="button" class="clear-search" onclick={() => searchQuery = ''}>
						<X size={14} />
					</button>
				{/if}
			</div>
		</div>

		<div class="header-right">
			<!-- Future global actions can be placed here -->
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
				<div class="empty-icon">
					<Search size={48} />
				</div>
				<h3>No identities found</h3>
				<p>Try adjusting your search query or add a new service account.</p>
			</div>
		{:else}
			<div class="grid">
				{#each paginatedUsers as user (user.id)}
					<div class="grid-item" in:fly={{ y: 10, duration: 300, delay: 50 }}>
						<UserCard
							{user}
							active={navigationState.activeUserEditUser?.id === user.id}
							onEdit={() => handleUserDropdown(user, 'edit')}
							onShowKubeConfig={() => handleUserDropdown(user, 'kubeconfig')}
							onDelete={() => handleUserDropdown(user, 'delete')}
						/>
					</div>
				{/each}
			</div>
		{/if}
	</div>

	{#if totalPages > 1}
		<div class="pagination">
			<div class="pagination-info">
				Showing <strong>{(currentPage - 1) * USERS_PER_PAGE + 1}</strong> - <strong>{Math.min(currentPage * USERS_PER_PAGE, filteredUsers.length)}</strong> of <strong>{filteredUsers.length}</strong>
			</div>
			<div class="pagination-controls">
				<button type="button" class="p-btn" onclick={() => currentPage--} disabled={currentPage === 1}>
					<ChevronLeft size={18} />
				</button>
				{#each Array.from({length: Math.min(5, totalPages)}, (_, i) => {
					if (totalPages <= 5) return i + 1;
					if (currentPage <= 3) return i + 1;
					if (currentPage >= totalPages - 2) return totalPages - 4 + i;
					return currentPage - 2 + i;
				}) as page}
					<button 
						type="button" 
						class="p-btn page-num {currentPage === page ? 'active' : ''}" 
						onclick={() => currentPage = page}
					>
						{page}
					</button>
				{/each}
				<button type="button" class="p-btn" onclick={() => currentPage++} disabled={currentPage === totalPages}>
					<ChevronRight size={18} />
				</button>
			</div>
		</div>
	{/if}
</div>

<div class="edit-panel {navigationState.activeUserEditUser ? 'active' : ''}">
	{#if navigationState.activeUserEditUser}
		<div class="edit-header">
			<div class="edit-header-info">
				<h2>Manage Permissions</h2>
				<p>Configure RBAC roles and scopes</p>
			</div>
			<button type="button" class="text-btn" onclick={() => (navigationState.activeUserEditUser = null)}>Cancel</button>
		</div>

		<div class="user-strip">
			<div class="strip-avatar">
				{navigationState.activeUserEditUser.name[0].toUpperCase()}
			</div>
			<div class="strip-info">
				<strong>{navigationState.activeUserEditUser.name}</strong>
				<span>Service Account</span>
			</div>
		</div>

		<div class="edit-body">
			{#if panelLoading}
				<div class="panel-loader" in:fade>
					<Loader2 size={32} class="animate-spin text-primary" />
					<p>Loading user permissions...</p>
				</div>
			{:else}
				<div class="section">
					<h4 class="field-label">Add Role Mapping</h4>
					<div class="assignment-box">
						<div class="select-grid">
							<select
								aria-label="Select Template"
								bind:value={newTemplate}
							>
								{#each templates as t}<option value={t}>{t}</option>{/each}
							</select>
							<div class="stiff-select-wrapper">
								<Select
									items={namespaces.map((ns) => ({ value: ns, label: ns }))}
									bind:value={selectedNamespaces}
									multiple
									placeholder="Target Namespaces"
								/>
							</div>
						</div>
						<button type="button" class="ghost-btn" onclick={addRoleAssignment}>
							{currentNamespacedRoles.some(r => r.template === newTemplate) ? 'Update Assignment' : '+ Add Assignment'}
						</button>
					</div>
				</div>

				<div class="section">
					<h4 class="field-label">Active Permissions</h4>
					<div class="role-preview">
						{#each currentNamespacedRoles as role, i (role.template)}
							{@const isSaved = initialNamespacedRoles.some(ir => ir.template === role.template && JSON.stringify(ir.namespaces) === JSON.stringify(role.namespaces))}
							{@const isCluster = currentClusterRoles.includes(role.template)}
							<div class="preview-item {isSaved ? '' : 'is-new'}" transition:slide={{ duration: 200 }}>
								<div class="role-info">
									<div class="role-header">
										<strong>{role.template}</strong>
										{#if !isSaved}
											<span class="new-tag">New</span>
										{/if}
										{#if isCluster}
											<span class="new-tag cluster">Cluster</span>
										{/if}
									</div>
									<div class="ns-tags">
										{#each role.namespaces as ns}
											<span class="ns-tag">
												{ns}
												<button type="button" class="tag-del" onclick={() => removeNamespace(i, ns)}>
													<X size={10} />
												</button>
											</span>
										{/each}
										{#if role.namespaces.length === 0}
											<span class="ns-tag empty">No namespaces (Cluster Only)</span>
										{/if}
									</div>
								</div>
								<div class="role-actions">
									<button 
										type="button" 
										class="cluster-toggle {isCluster ? 'active' : ''}"
										onclick={() => toggleClusterRole(role.template)}
										title="Toggle Cluster-wide Access"
									>
										<Globe size={14} />
										<span>Cluster</span>
									</button>
									<div class="action-btns">
										<button type="button" class="icon-action-btn" onclick={() => prepareEditRole(i)} title="Edit Assignment">
											<Edit3 size={14} />
										</button>
										<button type="button" class="remove-btn" onclick={() => removeRoleTemplate(i)}>Remove</button>
									</div>
								</div>
							</div>
						{/each}
						{#if currentNamespacedRoles.length === 0}
							<div class="empty-roles">
								<p>No roles assigned to this identity.</p>
							</div>
						{/if}
					</div>
				</div>
			{/if}
		</div>

		<div class="panel-footer">
			<button type="button" class="save-btn" onclick={saveChanges} disabled={!hasUnsavedChanges || panelLoading}>
				{panelLoading ? 'Processing...' : hasUnsavedChanges ? 'Commit Changes' : 'No Changes Detected'}
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
		onCopy={handleCopyKubeConfig}
		onDownload={handleDownloadKubeConfig}
	/>
{/if}

{#if userToDelete}
	<div class="modal-backdrop" in:fade={{ duration: 200 }} onclick={() => !isDeleting && (userToDelete = null)}>
		<div class="confirm-modal" in:fly={{ y: 20, duration: 300 }} onclick={(e) => e.stopPropagation()}>
			<div class="confirm-header">
				<div class="warning-icon">
					<AlertTriangle size={24} />
				</div>
				<h3>Confirm Deletion</h3>
			</div>
			<div class="confirm-body">
				<p>Are you sure you want to delete the user <strong>{userToDelete.name}</strong>?</p>
				<p class="warning-text">This will permanently remove the Service Account and all associated RBAC role bindings. This action cannot be undone.</p>
			</div>
			<div class="confirm-footer">
				<button type="button" class="cancel-btn" onclick={() => (userToDelete = null)} disabled={isDeleting}>Cancel</button>
				<button type="button" class="delete-confirm-btn" onclick={confirmDelete} disabled={isDeleting}>
					{#if isDeleting}
						<Loader2 size={16} class="animate-spin" />
						<span>Deleting...</span>
					{:else}
						<Trash2 size={16} />
						<span>Delete User</span>
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.user-list-container {
		display: flex;
		flex-direction: column;
		height: 100vh;
		background: #f8fafc;
	}

	.header {
		padding: 1rem 2.5rem;
		background: #ffffff;
		border-bottom: 1px solid var(--border);
		display: grid;
		grid-template-columns: 1fr auto 1fr;
		align-items: center;
		z-index: 10;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.02);
	}

	.header-left {
		display: flex;
		align-items: center;
	}

	.header-center {
		display: flex;
		justify-content: center;
		padding: 0 2rem;
	}

	.header-right {
		display: flex;
		justify-content: flex-end;
	}

	.title-group {
		display: flex;
		align-items: center;
		gap: 1.25rem;
	}

	.icon-circle {
		width: 44px;
		height: 44px;
		background: linear-gradient(135deg, var(--primary) 0%, #2980b9 100%);
		border-radius: 14px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #ffffff;
		box-shadow: 0 4px 12px rgba(52, 152, 219, 0.2);
	}

	.title-text h2 {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 800;
		color: var(--slate-900);
		letter-spacing: -0.02em;
	}

	.count {
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-muted);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.search-box {
		position: relative;
		width: 420px;
		display: flex;
		align-items: center;
	}

	.search-icon-wrapper {
		position: absolute;
		left: 14px;
		top: 50%;
		transform: translateY(-50%);
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--text-muted);
		pointer-events: none;
		z-index: 5;
	}

	.search-box input {
		width: 100%;
		padding: 0.75rem 2.5rem 0.75rem 3rem;
		background: #f1f5f9;
		border: 1px solid transparent;
		border-radius: 12px;
		font-size: 0.95rem;
		font-weight: 500;
		transition: all 0.2s;
		line-height: 1.5;
	}

	.search-box input:focus {
		background: #ffffff;
		border-color: var(--primary);
		box-shadow: 0 0 0 4px rgba(52, 152, 219, 0.1);
		outline: none;
	}

	.clear-search {
		position: absolute;
		right: 12px;
		top: 50%;
		transform: translateY(-50%);
		background: #e2e8f0;
		border: none;
		border-radius: 50%;
		width: 20px;
		height: 20px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--slate-600);
		cursor: pointer;
		padding: 0;
		transition: all 0.2s;
		z-index: 10;
	}

	.clear-search:hover {
		background: #cbd5e1;
		color: var(--slate-900);
	}

	.content {
		flex: 1;
		padding: 2.5rem;
		overflow-y: auto;
	}

	.grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		gap: 1.25rem;
	}

	.loading-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 60vh;
		color: var(--text-muted);
		gap: 1.25rem;
	}

	.spinner {
		width: 36px;
		height: 36px;
		border: 3.5px solid #e2e8f0;
		border-top-color: var(--primary);
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 60vh;
		text-align: center;
		max-width: 400px;
		margin: 0 auto;
	}

	.empty-icon {
		width: 80px;
		height: 80px;
		background: #f1f5f9;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--slate-300);
		margin-bottom: 1.5rem;
	}

	.empty-state h3 {
		font-size: 1.25rem;
		font-weight: 700;
		color: var(--slate-900);
		margin-bottom: 0.5rem;
	}

	.empty-state p {
		color: var(--text-muted);
		font-size: 0.95rem;
		line-height: 1.5;
	}

	.pagination {
		padding: 1.25rem 2.5rem;
		background: #ffffff;
		border-top: 1px solid var(--border);
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.pagination-info {
		font-size: 0.85rem;
		color: var(--text-muted);
	}

	.pagination-controls {
		display: flex;
		gap: 0.4rem;
	}

	.p-btn {
		min-width: 36px;
		height: 36px;
		padding: 0 8px;
		background: #ffffff;
		border: 1px solid var(--border);
		border-radius: 8px;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 0.85rem;
		font-weight: 600;
		color: var(--slate-700);
		cursor: pointer;
		transition: all 0.2s;
	}

	.p-btn:hover:not(:disabled) {
		background: #f8fafc;
		border-color: var(--slate-300);
	}

	.p-btn.active {
		background: var(--primary);
		color: #ffffff;
		border-color: var(--primary);
	}

	.p-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.edit-panel {
		position: fixed;
		top: 0;
		right: -540px;
		width: 540px;
		height: 100vh;
		background: #ffffff;
		border-left: 1px solid var(--border);
		box-shadow: -10px 0 40px rgba(0, 0, 0, 0.05);
		transition: right 0.4s cubic-bezier(0.4, 0, 0.2, 1);
		display: flex;
		flex-direction: column;
		z-index: 200;
	}

	.edit-panel.active {
		right: 0;
	}

	.edit-header {
		padding: 1.5rem 2.5rem;
		border-bottom: 1px solid var(--border);
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.user-strip {
		padding: 1.5rem 2.5rem;
		background: #f8fafc;
		border-bottom: 1px solid var(--border);
		display: flex;
		align-items: center;
		gap: 1.25rem;
	}

	.strip-avatar {
		width: 42px;
		height: 42px;
		background: var(--primary);
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #ffffff;
		font-weight: 800;
		font-size: 1.1rem;
	}

	.strip-info {
		display: flex;
		flex-direction: column;
	}

	.strip-info strong {
		font-size: 1rem;
		color: var(--slate-900);
	}

	.strip-info span {
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--text-muted);
		text-transform: uppercase;
		letter-spacing: 0.02em;
	}

	.edit-body {
		flex: 1;
		padding: 2.5rem;
		overflow-y: auto;
		display: flex;
		flex-direction: column;
	}

	.panel-loader {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		color: var(--text-muted);
		font-size: 0.9rem;
	}

	.section {
		margin-bottom: 3rem;
	}

	.edit-header-info h2 {
		margin: 0;
		font-size: 1.35rem;
		font-weight: 800;
		color: var(--primary-dark);
		letter-spacing: -0.02em;
	}

	.edit-header-info p {
		margin: 4px 0 0 0;
		font-size: 0.9rem;
		color: var(--text-muted);
	}

	.field-label {
		display: block;
		font-size: 0.75rem;
		font-weight: 800;
		text-transform: uppercase;
		color: var(--slate-500);
		margin-bottom: 1rem;
		letter-spacing: 0.8px;
		margin-top: 0;
	}

	.assignment-box {
		display: flex;
		flex-direction: column;
		gap: 1.25rem;
		padding: 1.75rem;
		background: #f8fafb;
		border: 1px solid var(--border);
		border-radius: 12px;
	}

	.select-grid {
		display: grid;
		grid-template-columns: 1fr 1.5fr;
		gap: 0.75rem;
		align-items: start;
	}

	.stiff-select-wrapper {
		background: white;
		border-radius: 8px;
		min-height: 42px;
	}

	.stiff-select-wrapper :global(.svelte-select) {
		border-color: var(--border) !important;
		border-radius: 8px !important;
		min-height: 42px !important;
	}

	.assignment-box select {
		height: 42px;
		padding: 0 1rem;
		background-color: white;
	}

	.role-preview {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.preview-item {
		padding: 1.5rem;
		background: #ffffff;
		border: 1px solid var(--border);
		border-radius: 16px;
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		transition: all 0.2s;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.02);
	}

	.preview-item.is-new {
		border-color: #fbd38d;
		background: #fffaf0;
		box-shadow: 0 4px 12px rgba(246, 173, 85, 0.1);
	}

	.role-info {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		flex: 1;
		min-width: 0; /* Important for text truncation/wrapping in flex */
	}

	.role-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.role-header strong {
		font-size: 1.05rem;
		color: var(--slate-900);
	}

	.new-tag {
		background: #feebc8;
		color: #9c4221;
		padding: 2px 8px;
		border-radius: 6px;
		font-size: 0.65rem;
		font-weight: 800;
		text-transform: uppercase;
	}

	.new-tag.cluster {
		background: #bee3f8;
		color: #2c5282;
	}

	.ns-tags {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.ns-tag {
		background: #f1f5f9;
		color: var(--slate-700);
		padding: 4px 10px;
		border-radius: 8px;
		font-size: 0.8rem;
		font-weight: 600;
		display: inline-flex;
		align-items: center;
		gap: 6px;
		border: 1px solid #e2e8f0;
	}

	.ns-tag.empty {
		background: #fff5f5;
		color: #c53030;
		border: 1px dashed #feb2b2;
		font-style: italic;
		font-size: 0.75rem;
	}

	.tag-del {
		background: transparent;
		border: none;
		color: var(--slate-400);
		cursor: pointer;
		display: flex;
		align-items: center;
		padding: 2px;
		border-radius: 4px;
		transition: all 0.2s;
	}

	.tag-del:hover {
		color: var(--danger);
		background: rgba(231, 76, 60, 0.1);
	}

	.role-actions {
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		gap: 1rem;
		margin-left: 1.5rem;
	}

	.cluster-toggle {
		background: #ffffff;
		border: 1px solid var(--border);
		color: var(--slate-500);
		padding: 6px 12px;
		border-radius: 8px;
		font-size: 0.75rem;
		font-weight: 700;
		display: flex;
		align-items: center;
		gap: 6px;
		transition: all 0.2s;
		cursor: pointer;
		white-space: nowrap;
	}

	.cluster-toggle.active {
		background: #ebf8ff;
		border-color: var(--primary);
		color: var(--primary);
	}

	.cluster-toggle:hover {
		border-color: var(--primary);
		background: #f0f9ff;
	}

	.action-btns {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.icon-action-btn {
		background: #ffffff;
		border: 1px solid var(--border);
		color: var(--slate-600);
		padding: 6px;
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.icon-action-btn:hover {
		color: var(--primary);
		border-color: var(--primary);
		background: #f0f9ff;
	}

	.remove-btn {
		background: transparent;
		color: var(--danger);
		font-size: 0.8rem;
		font-weight: 700;
		border: none;
		cursor: pointer;
		padding: 6px 10px;
		border-radius: 8px;
		transition: all 0.2s;
	}

	.remove-btn:hover {
		background: #fff5f5;
	}

	.ghost-btn {
		background: #ffffff;
		border: 1px solid var(--border);
		color: var(--primary);
		font-weight: 700;
		padding: 0.85rem;
		border-radius: 12px;
		cursor: pointer;
		font-size: 0.95rem;
		transition: all 0.2s;
	}

	.ghost-btn:hover {
		border-color: var(--primary);
		background: #f0f9ff;
		transform: translateY(-1px);
	}

	.text-btn {
		background: transparent;
		color: var(--slate-500);
		font-weight: 700;
		border: none;
		cursor: pointer;
		padding: 8px 16px;
		border-radius: 10px;
		transition: all 0.2s;
		font-size: 0.9rem;
	}

	.text-btn:hover {
		background: #f1f5f9;
		color: var(--slate-900);
	}

	.empty-roles {
		padding: 3rem 2rem;
		text-align: center;
		border: 2px dashed #e2e8f0;
		border-radius: 20px;
		color: var(--slate-400);
		font-size: 0.95rem;
		background: #fcfcfd;
	}

	.panel-footer {
		padding: 1.75rem 2.5rem;
		border-top: 1px solid var(--border);
		background: #ffffff;
	}

	.save-btn {
		width: 100%;
		padding: 1.1rem;
		background: var(--primary);
		color: white;
		border: none;
		border-radius: 14px;
		font-weight: 700;
		font-size: 1rem;
		cursor: pointer;
		transition: all 0.2s;
		box-shadow: 0 4px 12px rgba(52, 152, 219, 0.2);
	}

	.save-btn:disabled {
		background: #e2e8f0;
		color: var(--slate-400);
		cursor: not-allowed;
		box-shadow: none;
	}

	.save-btn:not(:disabled):hover {
		background: #2980b9;
		transform: translateY(-2px);
		box-shadow: 0 8px 20px rgba(52, 152, 219, 0.3);
	}

	/* Modal Styles */
	.modal-backdrop {
		position: fixed;
		top: 0;
		left: 0;
		width: 100vw;
		height: 100vh;
		background: rgba(15, 23, 42, 0.6);
		backdrop-filter: blur(4px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}

	.confirm-modal {
		background: #ffffff;
		width: 100%;
		max-width: 440px;
		border-radius: 20px;
		box-shadow: 0 20px 50px rgba(0, 0, 0, 0.2);
		overflow: hidden;
		border: 1px solid var(--border);
	}

	.confirm-header {
		padding: 2rem 2rem 1rem;
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
		gap: 1rem;
	}

	.warning-icon {
		width: 56px;
		height: 56px;
		background: #fff5f5;
		color: var(--danger);
		border-radius: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.confirm-header h3 {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 800;
		color: var(--slate-900);
	}

	.confirm-body {
		padding: 0 2rem 2rem;
		text-align: center;
	}

	.confirm-body p {
		margin: 0.5rem 0;
		color: var(--slate-600);
		font-size: 0.95rem;
		line-height: 1.5;
	}

	.warning-text {
		padding: 1rem;
		background: #fffaf0;
		border: 1px solid #fbd38d;
		border-radius: 12px;
		color: #9c4221 !important;
		font-size: 0.85rem !important;
		font-weight: 500;
		margin-top: 1.5rem !important;
	}

	.confirm-footer {
		padding: 1.5rem 2rem 2rem;
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1rem;
		background: #f8fafc;
		border-top: 1px solid var(--border);
	}

	.cancel-btn {
		padding: 0.8rem;
		background: white;
		border: 1px solid var(--border);
		border-radius: 12px;
		font-weight: 700;
		color: var(--slate-600);
		cursor: pointer;
		transition: all 0.2s;
	}

	.cancel-btn:hover:not(:disabled) {
		background: #f1f5f9;
		color: var(--slate-900);
	}

	.delete-confirm-btn {
		padding: 0.8rem;
		background: #ef4444;
		border: none;
		border-radius: 12px;
		font-weight: 700;
		color: #ffffff;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		transition: all 0.2s;
		box-shadow: 0 4px 12px rgba(239, 68, 68, 0.25);
	}

	.delete-confirm-btn:hover:not(:disabled) {
		background: #dc2626;
		transform: translateY(-1px);
		box-shadow: 0 6px 15px rgba(239, 68, 68, 0.35);
	}

	.delete-confirm-btn:active:not(:disabled) {
		transform: translateY(0);
	}

	.delete-confirm-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.animate-spin {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from { transform: rotate(0deg); }
		to { transform: rotate(360deg); }
	}
</style>
