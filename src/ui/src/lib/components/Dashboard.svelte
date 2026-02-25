<script lang="ts">
	import {
		Users,
		Server,
		Shield,
		ArrowRight,
		Activity,
		Cpu,
		Database,
		Network,
		UserPlus,
		History,
		Layers,
		Check
	} from 'lucide-svelte';
	import { onMount } from 'svelte';
	import { api } from '$lib/api';
	import MetricCard from './MetricCard.svelte';
	import QuickAction from './QuickAction.svelte';

	interface Props {
		onAction: (page: string) => void;
	}

	let { onAction }: Props = $props();

	let stats = $state({ users: 0, namespaces: 0, templates: 0 });
	let loading = $state(true);

	async function loadStats() {
		try {
			loading = true;
			const [u, n, t] = await Promise.all([api.getUsers(), api.getNamespaces(), api.getTemplates()]);
			stats = {
				users: u.service_accounts?.length || 0,
				namespaces: n.data?.namespaces?.length || 0,
				templates: t.data?.length || 0
			};
		} finally {
			loading = false;
		}
	}

	onMount(loadStats);
</script>

<div class="flex flex-1 flex-col gap-12 overflow-auto p-10 animate-in fade-in duration-1000">
	<!-- Hero Section -->
	<header class="flex flex-col gap-4">
		<div class="flex items-center gap-3">
			<div class="h-px w-8 bg-blue-600"></div>
			<span class="text-[10px] font-black uppercase tracking-[0.3em] text-blue-600"
				>Cluster Management Protocol</span
			>
		</div>
		<h1 class="text-5xl font-black tracking-tighter text-slate-900 sm:text-6xl">
			Gateway <span class="text-blue-600 italic">Insights.</span>
		</h1>
		<p class="max-w-2xl text-lg font-medium leading-relaxed text-slate-500">
			Orchestrate enterprise-grade Kubernetes access with a high-fidelity automation engine designed
			for security and scale.
		</p>
	</header>

	<!-- Metrics Grid -->
	<div class="grid grid-cols-1 gap-6 sm:grid-cols-2 xl:grid-cols-4">
		<MetricCard
			label="Total Identities"
			value={stats.users}
			icon={Users}
			color="blue"
			{loading}
			delay={0}
		/>
		<MetricCard
			label="Isolated Scopes"
			value={stats.namespaces}
			icon={Server}
			color="emerald"
			{loading}
			delay={100}
		/>
		<MetricCard
			label="RBAC Templates"
			value={stats.templates}
			icon={Shield}
			color="violet"
			{loading}
			delay={200}
		/>
		<MetricCard label="Node Affinity" value="Optimal" icon={Cpu} color="amber" delay={300} />
	</div>

	<div class="grid grid-cols-1 gap-10 lg:grid-cols-12">
		<!-- Featured Action Banner -->
		<div
			class="group relative flex flex-col justify-between overflow-hidden rounded-[48px] bg-slate-900 p-12 text-white shadow-2xl shadow-blue-900/20 lg:col-span-8"
		>
			<div class="relative z-10 space-y-8">
				<div
					class="flex h-14 w-14 items-center justify-center rounded-2xl bg-blue-500/20 text-blue-400 ring-1 ring-blue-500/30 shadow-inner"
				>
					<Network size={28} />
				</div>
				<div class="max-w-xl space-y-4">
					<h2 class="text-4xl font-black leading-[1.1] tracking-tight sm:text-5xl">
						Automated Cluster <br /><span class="text-blue-500">Provisioning</span> Engine.
					</h2>
					<p class="text-lg font-medium leading-relaxed text-slate-400">
						Deploy standardized service accounts with predefined policy wrappers. Reduce manual
						configuration overhead by 80%.
					</p>
				</div>
			</div>

			<div class="relative z-10 mt-16 flex flex-wrap gap-5">
				<button
					onclick={() => onAction('create')}
					class="flex items-center gap-3 rounded-2xl bg-blue-600 px-10 py-5 text-xs font-black uppercase tracking-[0.2em] text-white shadow-xl shadow-blue-900/40 transition-all hover:bg-blue-500 hover:shadow-blue-500/20 active:scale-95"
				>
					Provision Now
					<ArrowRight size={18} />
				</button>
				<button
					onclick={() => onAction('users')}
					class="rounded-2xl border border-slate-700 bg-slate-800/50 px-10 py-5 text-xs font-black uppercase tracking-[0.2em] text-white backdrop-blur-md transition-all hover:bg-slate-700 active:scale-95"
				>
					Review Audit
				</button>
			</div>

			<!-- Background Elements -->
			<div
				class="absolute -right-20 -top-20 h-96 w-96 rounded-full bg-blue-600/10 blur-[120px] transition-all duration-1000 group-hover:scale-150 group-hover:bg-blue-600/20"
			></div>
			<div
				class="absolute -bottom-24 -left-24 h-96 w-96 rounded-full bg-indigo-600/5 blur-[100px]"
			></div>

			<div
				class="absolute bottom-0 right-0 p-12 opacity-[0.03] transition-all duration-1000 group-hover:opacity-10 group-hover:scale-110"
			>
				<Layers size={400} strokeWidth={0.5} />
			</div>
		</div>

		<!-- Actions and Status -->
		<div class="flex flex-col gap-6 lg:col-span-4">
			<div
				class="flex flex-col gap-8 rounded-[40px] border border-slate-200 bg-white p-10 shadow-sm transition-all hover:border-blue-200"
			>
				<div class="flex items-center justify-between border-b border-slate-50 pb-6">
					<div class="space-y-1">
						<h3 class="text-xs font-black uppercase tracking-[0.2em] text-slate-900">
							Quick Operations
						</h3>
						<div class="h-1 w-10 rounded-full bg-blue-600"></div>
					</div>
					<div
						class="flex h-10 w-10 items-center justify-center rounded-xl bg-slate-50 text-blue-600"
					>
						<Activity size={20} class="animate-pulse" />
					</div>
				</div>

				<div class="space-y-4">
					<QuickAction
						title="Generate Identity"
						description="Create new cluster entity"
						icon={UserPlus}
						onclick={() => onAction('create')}
						variant="primary"
					/>
					<QuickAction
						title="Security Policies"
						description="Configure global RBAC"
						icon={Shield}
						onclick={() => onAction('settings')}
					/>
					<QuickAction
						title="Provisioning Logs"
						description="System history feed"
						icon={History}
						onclick={() => onAction('dashboard')}
					/>
				</div>
			</div>

			<!-- Secondary Status Card -->
			<div
				class="group relative overflow-hidden rounded-[40px] border border-emerald-100 bg-emerald-50/30 p-10 transition-all hover:border-emerald-300 hover:bg-white hover:shadow-xl hover:shadow-emerald-500/5"
			>
				<div class="relative z-10 flex items-center gap-6">
					<div
						class="flex h-14 w-14 items-center justify-center rounded-2xl bg-white text-emerald-600 shadow-sm ring-1 ring-emerald-100 transition-transform group-hover:scale-110"
					>
						<Database size={24} />
					</div>
					<div>
						<p class="text-[10px] font-black uppercase tracking-[0.2em] text-emerald-600/60 leading-none">
							Persistence Layer
						</p>
						<p class="mt-2 text-base font-black text-emerald-900 leading-tight">Manifest Integrity Verified</p>
					</div>
				</div>
				<div class="absolute -right-4 -top-4 text-emerald-500/10 transition-transform group-hover:scale-125">
					<Check size={80} strokeWidth={3} />
				</div>
			</div>
		</div>
	</div>
</div>
