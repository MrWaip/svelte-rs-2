<script>
	let innerWidth = $state(0);
	let innerHeight = $state(0);
	let outerWidth = $state(0);
	let outerHeight = $state(0);
	let devicePixelRatio = $state(1);
</script>

<svelte:window bind:innerWidth bind:innerHeight bind:outerWidth bind:outerHeight bind:devicePixelRatio />
