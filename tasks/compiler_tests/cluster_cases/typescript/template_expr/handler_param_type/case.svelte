<script lang="ts">
	let count = $state(0);
</script>
<button onclick={(e: MouseEvent) => count++}>{count}</button>
