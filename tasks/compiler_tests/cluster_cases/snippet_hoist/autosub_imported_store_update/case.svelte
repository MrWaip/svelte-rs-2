<script>
	import { count } from './store_mod.js';
</script>

{#snippet foo()}
	<button onclick={() => $count++}></button>
{/snippet}

{@render foo()}
