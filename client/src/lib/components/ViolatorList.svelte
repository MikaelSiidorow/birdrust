<script lang="ts">
	import { report } from '$lib/report';
	import { formatDate, formatDistance, formatTimes } from '$lib/format';
</script>

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
