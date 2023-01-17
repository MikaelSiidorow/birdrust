<script lang="ts">
	import LoadingSpinner from '$lib/components/LoadingSpinner.svelte';
	import ViolatorList from '$lib/components/ViolatorList.svelte';
	import DroneMap from '$lib/components/DroneMap.svelte';
	import { report } from '$lib/report';
</script>

<svelte:head>
	<title>{$report?.deviceInformation.deviceId ?? 'Drone Tracker'}</title>
</svelte:head>
<header>
	<h1 class="text-center text-3xl my-2 text-slate-900">
		{$report?.deviceInformation.deviceId ?? 'Drone Tracker'}
	</h1>
</header>
<main class="m-2 flex flex-col md:gap-4 xl:gap-8 md:flex-row justify-center">
	{#if !$report}
		<div class="flex gap-2 items-center text-lg">
			<LoadingSpinner />
			Fetching newest report...
		</div>
	{:else}
		<section class="w-screen max-w-lg ">
			<h2 class="text-xl text-slate-900">Live map</h2>
			<DroneMap />
		</section>
		<section class="p-2 rounded-lg w-96">
			<h2 class="text-xl text-slate-900">Recent violators</h2>
			<ViolatorList />
		</section>
	{/if}
</main>
