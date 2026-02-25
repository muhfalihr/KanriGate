<script lang="ts">
	import UserList from '$lib/components/UserList.svelte';
	import UserForm from '$lib/components/UserForm.svelte';
	import { navigationState } from '$lib/nav.svelte';

	function handleComplete() {
		navigationState.activePage = 'user-list';
	}
</script>

{#if navigationState.activePage === 'user-list'}
	<UserList onEdit={() => {}} onShowConfig={() => {}} />
{:else if navigationState.activePage === 'new-user'}
	<UserForm onComplete={handleComplete} />
{:else if navigationState.activePage === 'settings'}
	<div class="settings-placeholder">
		<h2>Settings Page</h2>
		<p>Coming Soon</p>
	</div>
{/if}

<style>
	.settings-placeholder {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		text-align: center;
		padding: 2rem;
		background: var(--surface);
		border-radius: 12px;
		box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
		border: 1px solid var(--border);
		margin: 1rem;
		transition: transform 0.2s ease, box-shadow 0.2s ease;
	}

	.settings-placeholder:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 30px rgba(0, 0, 0, 0.15);
	}

	.settings-placeholder h2 {
		font-size: 2rem;
		margin-bottom: 1rem;
		color: var(--primary);
		font-weight: 600;
		letter-spacing: -0.025em;
	}

	.settings-placeholder p {
		font-size: 1.2rem;
		color: var(--text-muted);
	}
</style>
