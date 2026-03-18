<script>
	import Component from './Component.svelte';
	let obj = { ref: null };
</script>

<Component bind:this={obj.ref} />
