<script>
	let s = $state(0);
	let d = $derived(s * 2);
</script>

<button onclick={() => s++}>s</button>
{d}
