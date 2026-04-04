<script module>
	export const API_URL = '/api';
	export function formatDate(d) {
		return d.toISOString();
	}
</script>

<script>
	let value = $state('hello');
</script>

<p>{value}</p>
