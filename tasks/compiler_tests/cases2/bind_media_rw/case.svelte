<script>
	let currentTime = $state(0);
	let paused = $state(true);
	let volume = $state(1);
	let muted = $state(false);
	let playbackRate = $state(1);
</script>

<audio
	bind:currentTime
	bind:paused
	bind:volume
	bind:muted
	bind:playbackRate
></audio>
