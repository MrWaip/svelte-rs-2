<script module>
	export const K = { m: 1 } satisfies Record<string, number>;
</script>
<script lang="ts">
	let count = $state(0);
</script>
<button onclick={() => count++}>{count}{K.m}</button>
