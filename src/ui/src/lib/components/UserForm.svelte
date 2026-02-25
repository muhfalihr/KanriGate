<script lang="ts">
	import { onMount } from 'svelte';
	import Select from 'svelte-select';
	import { api } from '$lib/api';
	import { toast } from '$lib/toast.svelte';
	import { fade, fly, slide } from 'svelte/transition';

	interface RoleAssignment {
		template: string;
		namespaces: string[];
		isNew?: boolean;
	}

	interface Props {
		onComplete: () => void;
	}

	let { onComplete }: Props = $props();

	let username = $state('');
	let templates = $state<string[]>([]);
	let namespaces = $state<string[]>([]);
	let selectedTemplate = $state<{ value: string; label: string } | null>(null);
	let selectedNamespaces = $state<{ value: string; label: string }[]>([]);
	let roleAssignments = $state<RoleAssignment[]>([]);
	let clusterAccess = $state('none');
	let isLoading = $state(false);

	onMount(async () => {
		try {
			const [tRes, nsRes] = await Promise.all([api.getTemplates(), api.getNamespaces()]);
			templates = tRes.data || [];
			namespaces = nsRes?.data?.namespaces || [];
			if (templates.length > 0) selectedTemplate = { value: templates[0], label: templates[0] };
		} catch (e) {
			toast.error('Data load failed');
		}
	});

	function addRole() {
		if (!selectedTemplate || selectedNamespaces.length === 0) return;
		roleAssignments = [
			...roleAssignments,
			{ template: selectedTemplate.value, namespaces: selectedNamespaces.map((n) => n.value) }
		];
		selectedNamespaces = [];
	}

	async function saveUser() {
		if (!username || roleAssignments.length === 0) return alert('Username/Roles required');
		isLoading = true;
		try {
			await api.createServiceAccount(username);
			await api.createSecret(username);
			for (const r of roleAssignments) {
				for (const ns of r.namespaces) await api.createRoleBinding(username, ns, r.template);
				if (clusterAccess !== 'none') await api.createClusterRoleBinding(username, r.template);
			}
			toast.success('User Provisioned Successfully');
			onComplete();
		} catch (e: any) {
			toast.error(e.message);
		} finally {
			isLoading = false;
		}
	}
</script>

<div class="form-root" in:fade={{ duration: 300 }}>
	<div class="stiff-card" in:fly={{ y: 20, duration: 400 }}>
		<div class="card-header">
			<div class="header-info">
				<h2>Provision New Identity</h2>
				<p>Assign roles and cluster scopes</p>
			</div>
			<button class="text-btn" onclick={onComplete}>Cancel</button>
		</div>

		<div class="card-body">
			<div class="field">
				<label for="sa-name">Service Account Name</label>
				<input id="sa-name" type="text" bind:value={username} placeholder="e.g. storage-manager" />
			</div>

			<div class="divider"></div>

			<div class="field">
				<h4 class="field-label">Add Role Mapping</h4>
				<div class="assignment-box">
					<div class="select-grid">
						<select
							aria-label="Select Template"
							onchange={(e) => {
								const v = (e.target as HTMLSelectElement).value;
								selectedTemplate = { value: v, label: v };
							}}
						>
							{#each templates as t}<option value={t}>{t}</option>{/each}
						</select>
						<div class="stiff-select">
							<Select
								items={namespaces.map((n) => ({ value: n, label: n }))}
								value={selectedNamespaces}
								on:select={(e) => (selectedNamespaces = e.detail || [])}
								multiple
								placeholder="Target Namespaces"
							/>
						</div>
					</div>
					<button class="ghost-btn" onclick={addRole}>+ Add Assignment</button>
				</div>
			</div>

			<div class="role-preview">
				{#each roleAssignments as r, i (r.template + r.namespaces.join())}
					<div class="preview-item" transition:slide={{ duration: 200 }}>
						<span><strong>{r.template}</strong> ➜ {r.namespaces.join(', ')}</span>
						<button class="remove-btn" onclick={() => (roleAssignments = roleAssignments.filter((_, idx) => idx !== i))}
							>Remove</button
						>
					</div>
				{/each}
			</div>

			<div class="divider"></div>

			<div class="field">
				<h4 class="field-label">Global Access Override</h4>
				<div class="radio-group">
					{#each ['none', 'read-only', 'read-write'] as level}
						<label class="radio-label {clusterAccess === level ? 'active' : ''}">
							<input type="radio" bind:group={clusterAccess} value={level} />
							{level.replace('-', ' ')}
						</label>
					{/each}
				</div>
			</div>
		</div>

		<div class="card-footer">
			<button class="primary-btn" onclick={saveUser} disabled={isLoading || !username || roleAssignments.length === 0}>
				{isLoading ? 'Executing...' : 'Commit Provisioning'}
			</button>
		</div>
	</div>
</div>

<style>
	.form-root {
		padding: 3rem 2rem;
		height: 100%;
		overflow-y: auto;
		background: var(--background);
	}

	.stiff-card {
		max-width: 720px;
		margin: 0 auto;
		background: #ffffff;
		border: 1px solid var(--border);
		border-radius: 12px;
		display: flex;
		flex-direction: column;
		box-shadow: var(--shadow-md);
		overflow: hidden;
	}

	.card-header {
		padding: 2rem;
		border-bottom: 1px solid var(--border);
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		background: #ffffff;
	}

	.header-info h2 {
		margin: 0;
		font-size: 1.25rem;
		font-weight: 800;
		color: var(--primary-dark);
	}

	.header-info p {
		margin: 4px 0 0 0;
		font-size: 0.85rem;
		color: var(--text-muted);
	}

	.card-body {
		padding: 2rem;
		display: flex;
		flex-direction: column;
		gap: 2rem;
	}

	.field label, .field-label {
		display: block;
		font-size: 0.75rem;
		font-weight: 800;
		text-transform: uppercase;
		color: var(--text-muted);
		margin-bottom: 0.75rem;
		letter-spacing: 0.5px;
		margin-top: 0;
	}

	.divider {
		height: 1px;
		background: var(--border);
	}

	.assignment-box {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		padding: 1.5rem;
		background: #f8fafb;
		border: 1px solid var(--border);
		border-radius: 10px;
	}

	.select-grid {
		display: grid;
		grid-template-columns: 1fr 2fr;
		gap: 0.75rem;
	}

	.role-preview {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.preview-item {
		padding: 0.8rem 1.25rem;
		background: #ffffff;
		border: 1px solid var(--border);
		border-radius: 8px;
		display: flex;
		justify-content: space-between;
		align-items: center;
		font-size: 0.9rem;
		box-shadow: var(--shadow-sm);
	}

	.remove-btn {
		background: transparent;
		color: var(--danger);
		font-size: 0.75rem;
		font-weight: 700;
		border: none;
	}

	.radio-group {
		display: flex;
		gap: 1rem;
	}

	.radio-label {
		flex: 1;
		padding: 0.8rem;
		border: 1px solid var(--border);
		border-radius: 8px;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		font-weight: 600;
		cursor: pointer;
		transition: var(--transition);
		background: #ffffff;
	}

	.radio-label.active {
		border-color: var(--primary);
		background: rgba(52, 152, 219, 0.05);
		color: var(--primary);
	}

	.card-footer {
		padding: 1.5rem 2rem;
		background: #f8fafb;
		border-top: 1px solid var(--border);
	}

	.primary-btn {
		background: var(--primary);
		color: white;
		width: 100%;
		padding: 1rem;
		border-radius: 8px;
		font-size: 1rem;
		font-weight: 700;
	}

	.ghost-btn {
		background: #ffffff;
		border: 1px solid var(--border);
		color: var(--primary);
		font-weight: 700;
	}

	.text-btn {
		background: transparent;
		color: var(--text-muted);
		font-weight: 600;
	}
</style>
