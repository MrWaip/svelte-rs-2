<script>
	let buffered = $state();
	let seekable = $state();
	let seeking = $state(false);
	let ended = $state(false);
	let readyState = $state(0);
	let played = $state();
</script>

<audio
	bind:buffered
	bind:seekable
	bind:seeking
	bind:ended
	bind:readyState
	bind:played
></audio>
