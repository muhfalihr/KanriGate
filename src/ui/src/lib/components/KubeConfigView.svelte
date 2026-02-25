<script lang="ts">
	import { X, Clipboard, Download, Loader2 } from 'lucide-svelte';

	interface User {
		id: number;
		name: string;
	}

	interface Props {
		show: boolean;
		user: User | null;
		kubeConfig: string;
		loading?: boolean;
		onClose?: () => void;
		onCopy?: () => void;
		onDownload?: () => void;
	}

	let {
		show,
		user,
		kubeConfig,
		loading = false,
		onClose = () => {},
		onCopy = () => {},
		onDownload = () => {}
	}: Props = $props();

	let userName = $derived(user && user.name ? user.name : 'Unknown User');
</script>

{#if show}
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="modal-backdrop" onclick={onClose}>
		<div class="modal-content" onclick={(e) => e.stopPropagation()}>
			<div class="modal-header">
				<h2>Kube Config - {userName}</h2>
				<button class="close-button" onclick={onClose} aria-label="Close modal">
					<X size={20} />
				</button>
			</div>
			<div class="modal-body">
				{#if loading}
					<div class="loading-message">
						<Loader2 size={24} class="spin" />
						<span>Loading configuration...</span>
					</div>
				{:else if kubeConfig}
					<pre class="config-content">{kubeConfig}</pre>
				{:else}
					<div class="loading-message">No configuration available</div>
				{/if}
			</div>
			<div class="modal-footer">
				<button class="action-button copy-button" onclick={onCopy} disabled={!kubeConfig || loading}>
					<Clipboard size={16} />
					<span>Copy to Clipboard</span>
				</button>
				<button
					class="action-button download-button"
					onclick={onDownload}
					disabled={!kubeConfig || loading}
				>
					<Download size={16} />
					<span>Download YAML</span>
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.modal-backdrop {
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background: rgba(0, 0, 0, 0.5);
		z-index: 1000;
		display: flex;
		align-items: center;
		justify-content: center;
		backdrop-filter: blur(4px);
		animation: fadeIn 0.2s ease;
	}

	.modal-content {
		background: var(--surface);
		border-radius: 12px;
		width: 90%;
		max-width: 800px;
		max-height: 90vh;
		display: flex;
		flex-direction: column;
		border: 1px solid var(--border);
		box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
		animation: slideUp 0.3s cubic-bezier(0.4, 0, 0.2, 1);
	}

	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1.25rem 1.5rem;
		border-bottom: 1px solid var(--border);
	}

	.modal-header h2 {
		margin: 0;
		font-size: 1.25rem;
		color: var(--text);
		font-weight: 600;
		letter-spacing: -0.025em;
	}

	.close-button {
		background: transparent;
		border: none;
		color: var(--text-muted);
		padding: 0.5rem;
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 8px;
		transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
	}

	.close-button:hover {
		color: var(--text);
		background: rgba(255, 255, 255, 0.1);
		transform: rotate(90deg);
	}

	.modal-body {
		padding: 1.5rem;
		overflow-y: auto;
		flex: 1;
		min-height: 200px;
		max-height: calc(90vh - 140px);
	}

	.config-content {
		margin: 0;
		padding: 1.25rem;
		background: var(--background);
		border-radius: 8px;
		border: 1px solid var(--border);
		color: var(--text);
		font-family: 'JetBrains Mono', 'Fira Code', 'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas',
			monospace;
		font-size: 0.9rem;
		line-height: 1.6;
		white-space: pre-wrap;
		word-break: break-all;
		transition: all 0.2s ease;
	}

	.config-content:hover {
		border-color: var(--primary);
		box-shadow: 0 0 0 1px var(--primary);
	}

	.loading-message {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		height: 200px;
		color: var(--text-muted);
		font-size: 1rem;
	}

	.modal-footer {
		display: flex;
		align-items: center;
		justify-content: flex-end;
		gap: 1rem;
		padding: 1.25rem 1.5rem;
		border-top: 1px solid var(--border);
	}

	.action-button {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.6rem 1.2rem;
		border-radius: 8px;
		font-size: 0.9rem;
		font-weight: 500;
		transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
		border: none;
	}

	.action-button:disabled {
		opacity: 0.7;
		cursor: not-allowed;
		transform: none !important;
	}

	.copy-button {
		background: var(--primary);
		color: white;
	}

	.copy-button:hover:not(:disabled) {
		background: var(--primary-dark);
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(52, 152, 219, 0.2);
	}

	.download-button {
		background: var(--success);
		color: white;
	}

	.download-button:hover:not(:disabled) {
		background: color-mix(in srgb, var(--success) 90%, black);
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(72, 187, 120, 0.2);
	}

	.spin {
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	@keyframes slideUp {
		from {
			opacity: 0;
			transform: translateY(20px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}
</style>
