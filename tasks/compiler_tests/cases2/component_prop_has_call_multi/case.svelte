<script>
	import Comp from './Comp.svelte';
	function fn1() { return 'a'; }
	function fn2() { return 'b'; }
</script>

<Comp a={fn1()} b={fn2()} />