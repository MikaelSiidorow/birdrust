<script lang="ts">
	import { fade } from 'svelte/transition';
	import { flip } from 'svelte/animate';
	import { quintOut } from 'svelte/easing';
	import { report } from '$lib/report';
	import { formatDate, formatDistance, formatTimes } from '$lib/format';
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
			<svg class="animate-spin text-purple-700" style="width:24px;height:24px" viewBox="0 0 24 24">
				<path fill="currentColor" d="M12,4V2A10,10 0 0,0 2,12H4A8,8 0 0,1 12,4Z" />
			</svg>
			Fetching newest report...
		</div>
	{:else}
		<section class="w-screen max-w-lg ">
			<h2 class="text-xl text-slate-900">Live map</h2>
			<div class="my-2 aspect-square w-full relative md:sticky md:top-4 bg-slate-100 overflow-clip">
				<!-- Map gridlines -->
				<div aria-hidden="true" class="absolute top-[20%] w-full border-b border-slate-500" />
				<div aria-hidden="true" class="absolute top-[40%] w-full border-b border-slate-500" />
				<div aria-hidden="true" class="absolute top-[60%] w-full border-b border-slate-500" />
				<div aria-hidden="true" class="absolute top-[80%] w-full border-b border-slate-500" />
				<div aria-hidden="true" class="absolute left-[20%] h-full border-r border-slate-500" />
				<div aria-hidden="true" class="absolute left-[40%] h-full border-r border-slate-500" />
				<div aria-hidden="true" class="absolute left-[60%] h-full border-r border-slate-500" />
				<div aria-hidden="true" class="absolute left-[80%] h-full border-r border-slate-500" />
				<!-- Danger circle -->
				<div
					aria-hidden="true"
					class="absolute top-1/2 left-1/2 h-[40%] w-[40%] -translate-x-1/2 -translate-y-1/2 rounded-full bg-red-600 bg-opacity-30"
				/>
				<!-- Nest position -->
				<div
					aria-hidden="true"
					class="absolute top-[50%] left-[50%] h-[1%] w-[1%] -translate-x-1/2 -translate-y-1/2 rounded-full bg-slate-700"
				/>
				<!-- Map scale -->
				<span class="absolute font-mono left-[10%] bottom-0 -translate-x-1/2 text-slate-700">
					100 m
				</span>
				<!-- Drones -->
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
		</section>
		<section class="p-2 rounded-lg w-96">
			<h2 class="text-xl text-slate-900">Recent violators</h2>
			<ul>
				{#each Object.entries($report.recentViolations).sort(([, a], [, b]) => Date.parse(b.latestDate) - Date.parse(a.latestDate)) as [serialNumber, violation] (serialNumber)}
					{@const { pilot, latestDate, closestDistance, timesSeen } = violation}
					<li class="my-4 first:mt-2 last:mb-2 px-4 flex justify-between relative">
						<div aria-hidden class="h-full w-1 bg-purple-500 absolute left-0 top-0" />
						<div class="inline-flex flex-col">
							<span class="text-slate-800 text-lg">
								{#if !pilot}
									unknown pilot
								{:else}
									{@const { firstName, lastName } = pilot}
									{firstName}
									{lastName}
								{/if}
							</span>
							<span class="text-slate-600">
								{formatDistance(closestDistance)} meters closest distance
							</span>
							<span class="text-slate-600">
								{formatTimes(timesSeen)} seconds in NDZ
							</span>
							<span class="text-slate-500 text-sm">
								{formatDate(latestDate)}
							</span>
						</div>
						{#if pilot}
							{@const { email, phoneNumber } = pilot}
							<div class="self-end inline-flex gap-4 font-semibold tracking-wide text-purple-500">
								<a href="tel:{phoneNumber}" class="">Call </a>
								<a href="mailto:{email}" class="">Email </a>
							</div>
						{/if}
					</li>
				{/each}
			</ul>
		</section>
	{/if}
</main>
