<script>
	import A from "./A.svelte";
	let { value = $bindable() } = $props();
	let Comp = $state(A);
</script>

<svelte:component this={Comp} bind:value />
