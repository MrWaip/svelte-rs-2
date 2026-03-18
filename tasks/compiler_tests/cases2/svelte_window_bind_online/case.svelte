<script>
	let isOnline = $state(true);
</script>

<svelte:window bind:online={isOnline} />
