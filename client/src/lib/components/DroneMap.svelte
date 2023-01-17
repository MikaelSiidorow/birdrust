<script lang="ts">
	import { fade } from 'svelte/transition';
	import { flip } from 'svelte/animate';
	import { quintOut } from 'svelte/easing';
	import { report } from '$lib/report';
	import MapStaticElements from '$lib/components/MapStaticElements.svelte';
</script>

<div class="my-2 aspect-square w-full relative md:sticky md:top-4 bg-slate-100 overflow-clip">
	<MapStaticElements />
	{#each $report.latestCapture.drones as { serialNumber, positionY, positionX } (serialNumber)}
		<div
			transition:fade
			animate:flip={{ duration: 1000, easing: quintOut }}
			aria-label="Drone {serialNumber}"
			class="absolute h-6 w-6 -translate-x-1/2 -translate-y-1/2 rounded-full bg-purple-500 drop-shadow-lg"
			style="top: {positionY / 5000}%; left: {positionX / 5000}%;"
		>
			<span
				class="text-xs text-slate-900 font-mono whitespace-nowrap absolute top-0 -translate-y-full left-0 -translate-x-1/4"
			>
				{#if Object.keys($report.recentViolations).includes(serialNumber)}
					{@const pilot = $report.recentViolations[serialNumber].pilot}
					{#if pilot}
						{@const { firstName, lastName } = pilot}
						{firstName}
						{lastName}
					{:else}
						unknown
					{/if}
				{:else}
					hidden
				{/if}
			</span>
		</div>
	{/each}
</div>
