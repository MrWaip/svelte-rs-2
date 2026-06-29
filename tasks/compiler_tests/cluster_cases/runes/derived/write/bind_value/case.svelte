<script>
	let s = $state(0);
	let d = $derived(s * 2);
</script>

<input bind:value={d} />
<button onclick={() => s++}>s</button>
{d}
