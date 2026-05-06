<script>
	import { obj } from './stores';
</script>
<button onclick={() => $obj.x = 1}>set</button>
<button onclick={() => $obj.x += 1}>+=</button>
<button onclick={() => $obj.x ??= 1}>??=</button>
<button onclick={() => $obj.x++}>++</button>
<button onclick={() => --$obj.x}>--pre</button>
