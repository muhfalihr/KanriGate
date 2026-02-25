<script lang="ts">
	import { onDestroy } from 'svelte';

	interface Props {
		message: string;
		type?: 'success' | 'error' | 'info';
		duration?: number;
		onClose?: () => void;
	}

	let { message = '', type = 'info', duration = 3000, onClose = () => {} }: Props = $props();

	let visible = $state(true);
	let timeout: ReturnType<typeof setTimeout>;

	function close() {
		visible = false;
		onClose();
	}

	$effect(() => {
		if (visible && duration > 0) {
			clearTimeout(timeout);
			timeout = setTimeout(close, duration);
		}
	});

	onDestroy(() => clearTimeout(timeout));
</script>

{#if visible}
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<div class="toast {type}" onclick={close} tabindex="0" role="alert">
		<span class="toast-message">{message}</span>
		<button class="toast-close" aria-label="Close" onclick={(e) => { e.stopPropagation(); close(); }}>&times;</button>
	</div>
{/if}

<style>
	.toast {
		display: flex;
		align-items: center;
		background: var(--surface);
		color: var(--text);
		border-radius: 12px;
		box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
		padding: 1rem 1.5rem;
		margin: 0.5rem 0;
		min-width: 280px;
		max-width: 400px;
		font-size: 0.95rem;
		font-weight: 500;
		position: relative;
		border-left: 4px solid var(--primary);
		cursor: pointer;
		transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
		animation: slideIn 0.3s cubic-bezier(0.4, 0, 0.2, 1);
	}

	.toast:hover {
		transform: translateY(-2px);
		box-shadow: 0 12px 28px rgba(0, 0, 0, 0.25);
	}

	.toast.success {
		border-left-color: var(--success);
		background: color-mix(in srgb, var(--success) 8%, var(--surface));
	}

	.toast.error {
		border-left-color: var(--danger);
		background: color-mix(in srgb, var(--danger) 8%, var(--surface));
	}

	.toast.info {
		border-left-color: var(--primary);
		background: color-mix(in srgb, var(--primary) 8%, var(--surface));
	}

	.toast-message {
		flex: 1;
		line-height: 1.4;
	}

	.toast-close {
		background: none;
		border: none;
		color: var(--text-muted);
		font-size: 1.2rem;
		margin-left: 1rem;
		padding: 0.25rem;
		cursor: pointer;
		opacity: 0.7;
		transition: all 0.2s ease;
		border-radius: 4px;
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
	}

	.toast-close:hover {
		opacity: 1;
		background: rgba(255, 255, 255, 0.1);
	}

	@keyframes slideIn {
		from {
			transform: translateX(100%);
			opacity: 0;
		}
		to {
			transform: translateX(0);
			opacity: 1;
		}
	}
</style>
