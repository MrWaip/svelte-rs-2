<script>
	let el = $state(null);
	let state = $state('visible');
</script>

<svelte:document bind:activeElement={el} bind:fullscreenElement={el} bind:pointerLockElement={el} bind:visibilityState={state} />
