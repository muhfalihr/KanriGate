<script lang="ts">
	import './layout.css';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import Toast from '$lib/components/Toast.svelte';
	import { navigationState } from '$lib/nav.svelte';
	import { setClientToken } from '$lib/api';
	import { page } from '$app/state';

	let { data, children } = $props();

	// Sync token to API client immediately during initialization
	setClientToken(data.user?.token ?? null);

	// Also sync whenever data updates
	$effect(() => {
		setClientToken(data.user?.token ?? null);
	});

	function handleSidebarToggle(expanded: boolean) {
		navigationState.isSidebarExpanded = expanded;
	}

	function handleNavigate(page: string) {
		navigationState.activePage = page;
	}

	const isLoginPage = $derived(page.url.pathname === '/login');
</script>

{#if isLoginPage}
	{@render children()}
{:else}
	<div class="app-shell">
		<Sidebar
			expanded={navigationState.isSidebarExpanded}
			activePage={navigationState.activePage}
			onToggle={handleSidebarToggle}
			onNavigate={handleNavigate}
		/>
		<main class="app-main">
			{@render children()}
		</main>
	</div>
{/if}

<Toast message="" type="info" />

<style>
	.app-shell {
		display: flex;
		height: 100vh;
		width: 100vw;
		background: var(--background);
		overflow: hidden;
		gap: 0;
	}

	.app-main {
		flex: 1;
		min-width: 0;
		display: flex;
		flex-direction: column;
		background: var(--background);
		overflow: hidden;
		padding: 1.5rem;
		position: relative;
	}

	.app-main::before {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		height: 2px;
		background: linear-gradient(90deg, rgba(52, 152, 219, 0.2), rgba(52, 152, 219, 0.05));
		pointer-events: none;
	}
</style>
