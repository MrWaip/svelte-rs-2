<script lang="ts">
	let count = $state(0);
	const label = $derived.by((): string => {
		if (!count) return 'a';
		return `a ${count}`;
	});
</script>

<button>{label}</button>
