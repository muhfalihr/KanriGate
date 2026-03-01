<script lang="ts">
	import { ChevronLeft, ChevronRight, Users, Plus, LayoutGrid, Github, LogOut } from 'lucide-svelte';
	import { navigationItems, navigationState } from '$lib/nav.svelte';
	import type { ComponentType } from 'svelte';
	import type { Icon } from 'lucide-svelte';
	import { fade } from 'svelte/transition';

	interface Props {
		expanded: boolean;
		activePage: string;
		onToggle: (expanded: boolean) => void;
		onNavigate: (page: string) => void;
	}

	let { expanded, activePage, onToggle, onNavigate }: Props = $props();

	const iconMap: Record<string, ComponentType<Icon>> = {
		LayoutGrid,
		Users,
		Plus
	};

	function handleNavigate(page: string) {
		navigationState.activeUserEditUser = null;
		onNavigate(page);
	}
</script>

<aside class="sidebar-wrapper {expanded ? 'expanded' : ''}">
	<!-- Header Section -->
	<div class="header-section">
		<div class="brand-box" onclick={() => handleNavigate('user-list')} role="button" tabindex="0" onkeydown={(e) => e.key === 'Enter' && handleNavigate('user-list')}>
			<div class="logo-icon">
				<svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
					<path d="M3 7h18" />
					<path d="M5 11h14" />
					<path d="M8 7v14" />
					<path d="M16 7v14" />
				</svg>
			</div>
			{#if expanded}
				<span class="logo-text" in:fade={{ duration: 150 }}>KANRIGATE</span>
			{/if}
		</div>
	</div>

	<!-- Navigation Section -->
	<nav class="nav-container">
		<ul class="nav-list">
			{#each navigationItems as item (item.page)}
				{@const Icon = iconMap[item.icon]}
				<li>
					<button
						class="nav-link {activePage === item.page ? 'active' : ''}"
						onclick={() => handleNavigate(item.page)}
						title={!expanded ? item.label : ''}
					>
						<div class="icon-box">
							{#if Icon}<Icon size={20} strokeWidth={activePage === item.page ? 2.5 : 2} />{/if}
						</div>
						
						{#if expanded}
							<span class="link-text" in:fade={{ duration: 150 }}>{item.label}</span>
						{/if}

						{#if activePage === item.page && expanded}
							<div class="active-pill"></div>
						{/if}
					</button>
				</li>
			{/each}
		</ul>
	</nav>

	<!-- Footer Section -->
	<div class="footer-section">
		<a 
			href="https://github.com/muhfalihr/KanriGate" 
			target="_blank" 
			rel="noopener noreferrer"
			class="github-link"
			title={!expanded ? "View on GitHub" : ""}
		>
			<div class="icon-box">
				<Github size={18} />
			</div>
			{#if expanded}
				<span class="link-text" in:fade={{ duration: 150 }}>GitHub Source</span>
			{/if}
		</a>
		<a 
			href="/logout" 
			class="logout-link"
			title={!expanded ? "Logout" : ""}
		>
			<div class="icon-box">
				<LogOut size={18} />
			</div>
			{#if expanded}
				<span class="link-text" in:fade={{ duration: 150 }}>Logout</span>
			{/if}
		</a>
		<button 
			class="collapse-btn" 
			onclick={() => onToggle(!expanded)}
		>
			<div class="icon-box">
				{#if expanded}<ChevronLeft size={18} />{:else}<ChevronRight size={18} />{/if}
			</div>
			{#if expanded}
				<span class="link-text" in:fade={{ duration: 150 }}>Collapse View</span>
			{/if}
		</button>
	</div>
</aside>

<style>
	.sidebar-wrapper {
		width: 76px;
		height: 100vh;
		background: #2c3e50;
		color: #ffffff;
		display: flex;
		flex-direction: column;
		transition: width 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		border-right: 1px solid rgba(0, 0, 0, 0.1);
		z-index: 100;
	}

	.sidebar-wrapper.expanded {
		width: 260px;
	}

	/* Alignment Base: px-4 (1rem) */
	.header-section {
		height: 72px;
		display: flex;
		align-items: center;
		padding: 0 1rem; /* px-4 */
		border-bottom: 1px solid rgba(255, 255, 255, 0.05);
	}

	.brand-box {
		display: flex;
		align-items: center;
		gap: 0.75rem; /* gap-x-3 */
		cursor: pointer;
	}

	.logo-icon {
		width: 40px;
		height: 40px;
		background: #3498db;
		border-radius: 8px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.logo-text {
		font-weight: 800;
		font-size: 1rem;
		letter-spacing: 1px;
		white-space: nowrap;
	}

	/* Navigation - Uniform Left Alignment */
	.nav-container {
		flex: 1;
		padding: 1.5rem 0.75rem; /* Menjaga kenyamanan padding luar */
	}

	.nav-list {
		list-style: none;
		padding: 0;
		margin: 0;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.nav-link {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: flex-start; /* FIX: Pastikan selalu rata kiri */
		padding: 0.5rem;
		background: transparent;
		border: none;
		border-radius: 8px;
		color: #aeb9b9;
		cursor: pointer;
		transition: all 0.2s;
		position: relative;
		gap: 0.75rem; /* gap-x-3: Jarak ikon dan teks */
	}

	.nav-link:hover {
		background: rgba(255, 255, 255, 0.05);
		color: #ffffff;
	}

	.nav-link.active {
		background: #3498db; /* Blue box for active state */
		color: #ffffff;
		box-shadow: 0 4px 12px rgba(52, 152, 219, 0.2);
	}

	/* Icon Box Fix: Menjamin ikon tetap sejajar secara vertikal */
	.icon-box {
		width: 40px;
		height: 40px;
		display: flex;
		align-items: center;
		justify-content: center; /* Ikon tetap di tengah container kecilnya */
		flex-shrink: 0;
	}

	.link-text {
		font-size: 0.9rem;
		font-weight: 600;
		white-space: nowrap;
		text-align: left; /* FIX: Teks rata kiri */
	}

	.active-pill {
		position: absolute;
		right: 12px;
		width: 6px;
		height: 6px;
		background: #ffffff;
		border-radius: 50%;
		opacity: 0.8;
	}

	/* Footer Section */
	.footer-section {
		padding: 1rem 0.75rem;
		border-top: 1px solid rgba(255, 255, 255, 0.05);
		display: flex;
		flex-direction: column;
		gap: 0.4rem;
	}

	.github-link {
		display: flex;
		align-items: center;
		justify-content: flex-start;
		padding: 0.5rem;
		text-decoration: none;
		color: #7f8c8d;
		border-radius: 8px;
		transition: all 0.2s;
		gap: 0.75rem;
	}

	.github-link:hover {
		color: #ffffff;
		background: rgba(255, 255, 255, 0.05);
	}

	.logout-link {
		display: flex;
		align-items: center;
		justify-content: flex-start;
		padding: 0.5rem;
		text-decoration: none;
		color: #e74c3c;
		border-radius: 8px;
		transition: all 0.2s;
		gap: 0.75rem;
	}

	.logout-link:hover {
		color: #ff7675;
		background: rgba(231, 76, 60, 0.1);
	}

	.collapse-btn {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: flex-start;
		padding: 0.5rem;
		background: transparent;
		border: none;
		border-radius: 8px;
		color: #7f8c8d;
		cursor: pointer;
		gap: 0.75rem;
		transition: all 0.2s;
	}

	.collapse-btn:hover {
		color: #ffffff;
		background: rgba(255, 255, 255, 0.05);
	}
</style>
