<script>
	export let items = [{ value: 1 }];
	export let extra = 2;

	$: prop_total = items[0].value + extra;
	$: props_items = $$props.items[0].value;
	$: rest_class = $$restProps.class ?? 'none';
</script>

<p class="x">{prop_total}-{props_items}-{rest_class}</p>
