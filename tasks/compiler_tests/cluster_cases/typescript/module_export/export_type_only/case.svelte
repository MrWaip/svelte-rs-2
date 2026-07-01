<script module lang="ts">
	type Hello = { m: number };
	export type { Hello };
	export const K = 1;
</script>
<script lang="ts">
	let count = $state(0);
</script>
<button onclick={() => count++}>{count}{K}</button>
