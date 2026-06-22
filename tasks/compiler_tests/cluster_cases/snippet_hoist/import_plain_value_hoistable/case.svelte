<script>
	import { noop } from './helpers.js';
</script>

{#snippet foo()}
	{noop}
{/snippet}

{@render foo()}
