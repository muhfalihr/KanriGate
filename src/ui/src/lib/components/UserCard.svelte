<script lang="ts">
	import { MoreVertical, Edit3, FileCode, Shield, Trash2 } from 'lucide-svelte';
	import { scale } from 'svelte/transition';

	interface UserData {
		id: number;
		name: string;
	}

	interface Props {
		user: UserData;
		active?: boolean;
		onEdit?: () => void;
		onShowKubeConfig?: () => void;
		onDelete?: () => void;
	}

	let { user, active = false, onEdit = () => {}, onShowKubeConfig = () => {}, onDelete = () => {} }: Props = $props();

	let showDropdown = $state(false);

	function toggleDropdown(e: MouseEvent) {
		e.stopPropagation();
		showDropdown = !showDropdown;
	}

	function handleEdit(e: MouseEvent) {
		e.stopPropagation();
		showDropdown = false;
		onEdit();
	}

	function handleShowKubeConfig(e: MouseEvent) {
		e.stopPropagation();
		showDropdown = false;
		onShowKubeConfig();
	}

	function handleDelete(e: MouseEvent) {
		e.stopPropagation();
		showDropdown = false;
		onDelete();
	}

	const getInitials = (name: string) => {
		return name.split('-').map(n => n[0]).join('').toUpperCase().slice(0, 2);
	};

	// Close dropdown when clicking outside
	$effect(() => {
		if (!showDropdown) return;
		
		const handleClickOutside = () => {
			showDropdown = false;
		};
		
		window.addEventListener('click', handleClickOutside);
		return () => window.removeEventListener('click', handleClickOutside);
	});
</script>

<div
	class="user-card {active ? 'active' : ''}"
	role="button"
	tabindex="0"
	onclick={onEdit}
	onkeydown={(e) => e.key === 'Enter' && onEdit()}
	style="z-index: {showDropdown ? '50' : '1'};"
>
	<div class="card-content">
		<div class="avatar">
			<span class="initials">{getInitials(user.name)}</span>
		</div>
		<div class="user-info">
			<span class="user-name">{user.name}</span>
			<div class="user-meta">
				<Shield size={12} />
				<span>Service Account</span>
			</div>
		</div>
	</div>

	<button 
		class="more-btn" 
		aria-label="More options"
		onclick={toggleDropdown}
	>
		<MoreVertical size={18} />
	</button>

	{#if showDropdown}
		<div 
			class="dropdown-menu" 
			transition:scale={{ duration: 150, start: 0.95 }}
		>
			<button class="dropdown-item" onclick={handleEdit}>
				<Edit3 size={14} />
				<span>Edit Permissions</span>
			</button>
			<button class="dropdown-item" onclick={handleShowKubeConfig}>
				<FileCode size={14} />
				<span>View Kubeconfig</span>
			</button>
			<div class="dropdown-divider"></div>
			<button class="dropdown-item delete" onclick={handleDelete}>
				<Trash2 size={14} />
				<span>Delete User</span>
			</button>
		</div>
	{/if}
</div>

<style>
	.user-card {
		padding: 1.25rem;
		background: #ffffff;
		border-radius: 16px;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: space-between;
		position: relative;
		border: 1px solid var(--border);
		outline: none;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
	}

	.user-card:hover {
		border-color: var(--primary);
		box-shadow: 0 10px 20px rgba(0, 0, 0, 0.04);
		transform: translateY(-2px);
	}

	.user-card.active {
		border-color: var(--primary);
		background: rgba(52, 152, 219, 0.02);
		box-shadow: 0 0 0 2px rgba(52, 152, 219, 0.1);
	}

	.card-content {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.avatar {
		width: 44px;
		height: 44px;
		background: linear-gradient(135deg, #f0f4f8 0%, #d9e2ec 100%);
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--primary-dark);
		transition: transform 0.3s ease;
	}

	.user-card:hover .avatar {
		transform: scale(1.05);
	}

	.initials {
		font-weight: 700;
		font-size: 0.9rem;
		letter-spacing: -0.5px;
	}

	.user-info {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.user-name {
		font-weight: 700;
		color: var(--slate-900);
		font-size: 0.95rem;
		letter-spacing: -0.2px;
	}

	.user-meta {
		display: flex;
		align-items: center;
		gap: 0.4rem;
		color: var(--text-muted);
		font-size: 0.75rem;
		font-weight: 600;
	}

	.more-btn {
		background: transparent;
		border: none;
		padding: 8px;
		color: var(--text-muted);
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.2s;
		z-index: 10;
	}

	.more-btn:hover {
		background: #f1f5f9;
		color: var(--slate-900);
	}

	.dropdown-menu {
		position: absolute;
		top: calc(100% - 10px);
		right: 1.25rem;
		background: #ffffff;
		min-width: 190px;
		z-index: 100;
		display: flex;
		flex-direction: column;
		border: 1px solid var(--border);
		box-shadow: 0 10px 30px rgba(0, 0, 0, 0.1);
		border-radius: 12px;
		padding: 6px;
		overflow: hidden;
		transform-origin: top right;
	}

	.dropdown-item {
		background: transparent;
		border: none;
		color: var(--slate-700);
		width: 100%;
		padding: 0.75rem 1rem;
		font-size: 0.85rem;
		font-weight: 600;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: flex-start;
		text-align: left;
		gap: 0.75rem;
		border-radius: 8px;
		transition: all 0.2s;
	}

	.dropdown-item:hover {
		background: #f8fafc;
		color: var(--primary);
	}

	.dropdown-item.delete {
		color: var(--danger);
	}

	.dropdown-item.delete:hover {
		background: #fff5f5;
	}

	.dropdown-divider {
		height: 1px;
		background: var(--border);
		margin: 4px 8px;
	}
</style>
