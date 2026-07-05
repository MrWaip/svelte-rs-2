<script lang="ts">
	declare function g(a: number): void;
	let count = $state(0);
</script>
<button onclick={() => count++}>{count}</button>
