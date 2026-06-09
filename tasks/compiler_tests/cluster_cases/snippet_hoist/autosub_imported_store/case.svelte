<script>
	import { count } from './store_mod.js';
</script>

{#snippet foo()}
	{$count}
{/snippet}

{@render foo()}
