<script>
	import { count } from './stores';
</script>

<button onclick={() => $count++}>inc</button>
<button onclick={() => ++$count}>pre inc</button>
<button onclick={() => $count--}>dec</button>
