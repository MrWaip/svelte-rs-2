<script lang="ts">
	import { type Snippet } from 'svelte';

	let { wrapWith = defaultWrapWith } = $props();
</script>

{#snippet defaultWrapWith(mf: Snippet)}
	{@render mf()}
{/snippet}

<div>x</div>
