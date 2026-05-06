<script>
	import { count } from './stores';
</script>
<button onclick={() => $count = 1}>set</button>
<button onclick={() => $count += 1}>+=</button>
<button onclick={() => $count ??= 5}>??=</button>
<button onclick={() => $count &&= 5}>&&=</button>
<button onclick={() => $count ||= 5}>||=</button>
<button onclick={() => $count++}>++</button>
<button onclick={() => --$count}>--pre</button>
