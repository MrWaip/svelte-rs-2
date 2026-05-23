<script>
	import { A } from './a';
	import { B } from './b';

	export let x;

	$: y = (function () {
		switch (x) {
			case A.ONE: return B;
			default: return '';
		}
	})();
</script>

<p>{y}</p>
