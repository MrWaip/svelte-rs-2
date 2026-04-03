<script>
	import { count } from './stores';
	import Component from './Component.svelte';
</script>

<Component bind:value={$count} />
