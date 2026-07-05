<script lang="ts">
	let count = $state(0);
</script>
<button onclick={() => count++}>x</button>
{count as number}
