<script lang="ts">
	interface User {
		id: number;
		name: string;
	}

	interface Props {
		user: User;
		active?: boolean;
		onEdit?: () => void;
		onShowKubeConfig?: () => void;
	}

	let { user, active = false, onEdit = () => {}, onShowKubeConfig = () => {} }: Props = $props();

	let showDropdown = $state(false);

	function handleCardClick() {
		showDropdown = !showDropdown;
	}

	function handleEdit() {
		showDropdown = false;
		onEdit();
	}

	function handleShowKubeConfig() {
		showDropdown = false;
		onShowKubeConfig();
	}

	function handleBlur(e: FocusEvent) {
		const target = e.relatedTarget as HTMLElement;
		if (!e.currentTarget || !(e.currentTarget as HTMLElement).contains(target)) {
			showDropdown = false;
		}
	}
</script>

<div
	class="user-card {active ? 'active' : ''}"
	tabindex="0"
	onclick={handleCardClick}
	onkeydown={(e) => e.key === 'Enter' && handleCardClick()}
	onblur={handleBlur}
>
	<div class="user-info">
		<span class="user-name">{user.name}</span>
	</div>
	{#if showDropdown}
		<div class="dropdown-menu">
			<button class="dropdown-item" onclick={(e) => { e.stopPropagation(); handleEdit(); }}>
				Edit User
			</button>
			<button class="dropdown-item" onclick={(e) => { e.stopPropagation(); handleShowKubeConfig(); }}>
				Show Kube Config
			</button>
		</div>
	{/if}
</div>

<style>
	.user-card {
		padding: 1rem 1.25rem;
		background: #ffffff;
		margin-bottom: 0.5rem;
		border-radius: 10px;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: space-between;
		position: relative;
		border: 1px solid var(--border);
		outline: none;
		transition: var(--transition);
		box-shadow: var(--shadow-sm);
	}

	.user-card:hover {
		border-color: var(--primary-light);
		box-shadow: var(--shadow-md);
		transform: translateY(-2px);
	}

	.user-card:active {
		transform: scale(0.98);
	}

	.user-card.active {
		background: var(--primary);
		border-color: var(--primary);
	}

	.user-card.active .user-name {
		color: #ffffff;
	}

	.user-name {
		font-weight: 600;
		color: var(--text);
		font-size: 0.95rem;
	}

	.dropdown-menu {
		position: absolute;
		top: 100%;
		right: 0;
		background: #ffffff;
		min-width: 180px;
		z-index: 50;
		display: flex;
		flex-direction: column;
		border: 1px solid var(--border);
		box-shadow: 0 10px 25px rgba(0, 0, 0, 0.1);
		border-radius: 8px;
		margin-top: 8px;
		overflow: hidden;
		animation: dropdownIn 0.2s cubic-bezier(0.4, 0, 0.2, 1);
	}

	@keyframes dropdownIn {
		from { opacity: 0; transform: translateY(-10px) scale(0.95); }
		to { opacity: 1; transform: translateY(0) scale(1); }
	}

	.dropdown-item {
		background: transparent;
		border: none;
		color: var(--text);
		text-align: left;
		padding: 0.8rem 1.2rem;
		font-size: 0.875rem;
		cursor: pointer;
		transition: background 0.2s;
	}

	.dropdown-item:hover {
		background: #f8f9fa;
		color: var(--primary);
	}
</style>
