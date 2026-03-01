<script lang="ts">
	import { enhance } from '$app/forms';
	import { navigationState } from '$lib/nav.svelte';
	import { onMount } from 'svelte';

	let { form } = $props();
	let loading = $state(false);

	onMount(() => {
		navigationState.activePage = 'login';
	});
</script>

<div class="login-container">
	<div class="login-card">
		<div class="login-header">
			<div class="logo">
				<svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
					<path d="M3 7h18" />
					<path d="M5 11h14" />
					<path d="M8 7v14" />
					<path d="M16 7v14" />
				</svg>
			</div>
			<h1>KanriGate</h1>
			<p>Kubernetes Access Manager</p>
		</div>

		<form method="POST" use:enhance={() => {
			loading = true;
			return async ({ result }) => {
				if (result.type === 'redirect') {
					// Force a full page reload to the target to ensure clean state
					window.location.href = result.location;
				} else {
					loading = false;
					// Standard handling for errors/etc
					if (result.type === 'failure' || result.type === 'error') {
						// SvelteKit will handle mapping result to form prop automatically
						// but we need to let it run its default behavior
						import('$app/forms').then(f => f.applyAction(result));
					}
				}
			};
		}}>
			<div class="form-group">
				<label for="username">Username</label>
				<input 
					type="text" 
					id="username" 
					name="username" 
					placeholder="Enter your username"
					required 
				/>
			</div>

			<div class="form-group">
				<label for="password">Password</label>
				<input 
					type="password" 
					id="password" 
					name="password" 
					placeholder="Enter your password"
					required 
				/>
			</div>

			{#if form?.message}
				<div class="error-message">
					{form.message}
				</div>
			{/if}

			<button type="submit" disabled={loading}>
				{#if loading}
					<span class="spinner"></span>
					Logging in...
				{:else}
					Login
				{/if}
			</button>
		</form>

		<div class="login-footer">
			<p>&copy; 2026 KanriGate. Secure Access Control.</p>
		</div>
	</div>
</div>

<style>
	.login-container {
		height: 100vh;
		width: 100vw;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--background);
		background-image: 
			radial-gradient(circle at 20% 20%, rgba(52, 152, 219, 0.05) 0%, transparent 40%),
			radial-gradient(circle at 80% 80%, rgba(46, 204, 113, 0.05) 0%, transparent 40%);
	}

	.login-card {
		width: 100%;
		max-width: 400px;
		padding: 2.5rem;
		background: var(--card-bg);
		border: 1px solid var(--border-color);
		border-radius: 1rem;
		box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.2), 0 10px 10px -5px rgba(0, 0, 0, 0.1);
		backdrop-filter: blur(10px);
	}

	.login-header {
		text-align: center;
		margin-bottom: 2rem;
	}

	.logo {
		width: 48px;
		height: 48px;
		margin: 0 auto 1rem;
		background: linear-gradient(135deg, var(--primary), var(--primary-dark));
		border-radius: 12px;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
		box-shadow: 0 4px 12px rgba(52, 152, 219, 0.3);
	}

	.logo svg {
		width: 28px;
		height: 28px;
	}

	h1 {
		font-size: 1.75rem;
		font-weight: 700;
		color: var(--text-primary);
		margin-bottom: 0.25rem;
		letter-spacing: -0.025em;
	}

	.login-header p {
		color: var(--text-secondary);
		font-size: 0.875rem;
	}

	.form-group {
		margin-bottom: 1.25rem;
	}

	label {
		display: block;
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--text-secondary);
		margin-bottom: 0.5rem;
	}

	input {
		width: 100%;
		padding: 0.75rem 1rem;
		background: var(--input-bg);
		border: 1px solid var(--border-color);
		border-radius: 0.5rem;
		color: var(--text-primary);
		font-size: 0.9375rem;
		transition: all 0.2s ease;
	}

	input:focus {
		outline: none;
		border-color: var(--primary);
		box-shadow: 0 0 0 3px rgba(52, 152, 219, 0.1);
	}

	.error-message {
		padding: 0.75rem;
		background: rgba(231, 76, 60, 0.1);
		border: 1px solid rgba(231, 76, 60, 0.2);
		border-radius: 0.5rem;
		color: #e74c3c;
		font-size: 0.875rem;
		margin-bottom: 1.25rem;
		text-align: center;
	}

	button {
		width: 100%;
		padding: 0.75rem;
		background: var(--primary);
		color: white;
		border: none;
		border-radius: 0.5rem;
		font-size: 0.9375rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.2s ease;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
	}

	button:hover:not(:disabled) {
		background: var(--primary-dark);
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(52, 152, 219, 0.2);
	}

	button:active:not(:disabled) {
		transform: translateY(0);
	}

	button:disabled {
		opacity: 0.7;
		cursor: not-allowed;
	}

	.spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-radius: 50%;
		border-top-color: white;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	.login-footer {
		margin-top: 2rem;
		text-align: center;
		font-size: 0.75rem;
		color: var(--text-secondary);
	}
</style>
