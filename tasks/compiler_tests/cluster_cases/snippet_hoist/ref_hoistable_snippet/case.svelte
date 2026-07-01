<script>
	import { noop } from './helpers.js';
</script>

{@render b()}

{#snippet a()}
	{noop}
{/snippet}

{#snippet b()}
	{@render a()}
{/snippet}
