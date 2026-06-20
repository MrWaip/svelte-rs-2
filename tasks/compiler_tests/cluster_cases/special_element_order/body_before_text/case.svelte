<script>
	export let foo;
</script>

<svelte:body on:click={() => foo = !foo}/>

{foo}
