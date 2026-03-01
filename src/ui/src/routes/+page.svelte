<script lang="ts">
	import UserList from '$lib/components/UserList.svelte';
	import UserForm from '$lib/components/UserForm.svelte';
	import { navigationState } from '$lib/nav.svelte';

	function handleComplete() {
		navigationState.activePage = 'user-list';
	}
</script>

{#if navigationState.activePage === 'user-list'}
	<UserList />
{:else if navigationState.activePage === 'new-user'}
	<UserForm onComplete={handleComplete} />
{/if}

<style>
</style>
