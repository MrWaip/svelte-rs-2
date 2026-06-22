<svelte:options runes={true} />
<script>
	import { foo } from './utils';
</script>
{#each foo.bar as bar}
	<span>{bar}</span>
{/each}
