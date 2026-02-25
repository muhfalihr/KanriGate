<script lang="ts">
	import type { ComponentType } from 'svelte';
	import type { Icon as LucideIcon } from 'lucide-svelte';
	import { scale } from 'svelte/transition';

	interface Props {
		label: string;
		value: string | number;
		icon: ComponentType<LucideIcon>;
		color?: 'blue' | 'emerald' | 'violet' | 'amber';
		loading?: boolean;
		delay?: number;
	}

	let { label, value, icon: Icon, color = 'blue', loading = false, delay = 0 }: Props = $props();

	const colorMap = {
		blue: 'bg-blue-50 text-blue-600 group-hover:bg-blue-600',
		emerald: 'bg-emerald-50 text-emerald-600 group-hover:bg-emerald-600',
		violet: 'bg-violet-50 text-violet-600 group-hover:bg-violet-600',
		amber: 'bg-amber-50 text-amber-600 group-hover:bg-amber-600'
	};

	const barMap = {
		blue: 'group-hover:bg-blue-100',
		emerald: 'group-hover:bg-emerald-100',
		violet: 'group-hover:bg-violet-100',
		amber: 'group-hover:bg-amber-100'
	};
</script>

<div
	class="group relative flex flex-col gap-5 rounded-[32px] border border-slate-200 bg-white p-7 shadow-sm transition-all duration-300 hover:-translate-y-1.5 hover:border-blue-500/30 hover:shadow-2xl hover:shadow-blue-500/10"
	in:scale={{ duration: 400, delay, start: 0.95 }}
>
	<div class="flex items-center justify-between">
		<div
			class="flex h-14 w-14 items-center justify-center rounded-2xl transition-all duration-300 group-hover:text-white group-hover:shadow-lg group-hover:shadow-current/20 {colorMap[color]}"
		>
			<Icon size={28} strokeWidth={2} />
		</div>
		<div class="h-1.5 w-10 rounded-full bg-slate-100 transition-colors {barMap[color]}"></div>
	</div>

	<div class="space-y-1">
		<span class="text-3xl font-black tracking-tighter text-slate-900">
			{#if loading}
				<div class="h-8 w-16 animate-pulse rounded-lg bg-slate-100"></div>
			{:else}
				{value}
			{/if}
		</span>
		<p class="text-[10px] font-black uppercase tracking-[0.2em] text-slate-400">
			{label}
		</p>
	</div>

	<!-- Decorative background element -->
	<div class="absolute -bottom-2 -right-2 h-16 w-16 opacity-[0.02] transition-opacity group-hover:opacity-[0.05]">
		<Icon size={64} />
	</div>
</div>
