<script lang="ts">
	import type { ComponentType } from 'svelte';
	import type { Icon as LucideIcon } from 'lucide-svelte';
	import { ArrowRight } from 'lucide-svelte';

	interface Props {
		title: string;
		description: string;
		icon: ComponentType<LucideIcon>;
		onclick: () => void;
		variant?: 'default' | 'primary';
	}

	let { title, description, icon: Icon, onclick, variant = 'default' }: Props = $props();
</script>

<button
	{onclick}
	class="group flex w-full items-center gap-5 rounded-3xl border p-5 transition-all duration-300 hover:shadow-xl active:scale-[0.98]
    {variant === 'primary' 
        ? 'border-blue-100 bg-blue-50/50 hover:border-blue-300 hover:bg-white' 
        : 'border-slate-100 bg-slate-50/50 hover:border-blue-200 hover:bg-white'}"
>
	<div
		class="flex h-12 w-12 shrink-0 items-center justify-center rounded-2xl bg-white shadow-sm ring-1 ring-slate-200 transition-all duration-300 group-hover:bg-blue-600 group-hover:text-white group-hover:ring-blue-600"
	>
		<Icon size={20} />
	</div>
	<div class="text-left overflow-hidden flex-1">
		<p class="truncate text-sm font-black tracking-tight text-slate-900 uppercase">{title}</p>
		<p class="truncate text-[11px] font-bold text-slate-400 mt-0.5 tracking-wide">{description}</p>
	</div>
	<div class="flex h-8 w-8 items-center justify-center rounded-full bg-slate-100 opacity-0 transition-all duration-300 group-hover:bg-blue-50 group-hover:text-blue-600 group-hover:opacity-100 group-hover:translate-x-1">
		<ArrowRight size={16} />
	</div>
</button>
