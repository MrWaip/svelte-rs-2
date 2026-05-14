<script lang="ts">
function log(_target: unknown, _key: string) {}

class Store {
	@log
	value = 0;
}
</script>
