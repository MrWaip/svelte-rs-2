<script>
	import { f, g } from './x';
	let { url = '', label = '' } = $props();
</script>

<a href={f(url)}>{g(label)}</a>
