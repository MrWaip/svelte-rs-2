<script>
	import Test from './Test.svelte';
	let entries = $state([]);
</script>

<Test bind:this={entries[0]} />
<Test bind:this={() => entries[1], (v) => (entries[1] = v)} />
