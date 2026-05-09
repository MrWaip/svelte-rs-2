<script lang="ts">
	import Outer from './Outer.svelte';
	let { data } = $props();
</script>

<Outer>
	{#snippet body()}
		{#if data?.flag}
			<div>
				{#snippet inner()}
					{data?.flag?.text}
				{/snippet}
			</div>
		{/if}
	{/snippet}
</Outer>
