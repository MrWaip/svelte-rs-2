<script lang="ts">
	let count = $state(0);
</script>
<button onclick={() => { count = (count + 1) as number; }}>{count}</button>
