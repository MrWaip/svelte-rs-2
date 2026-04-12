<script>
	let items = [{ value: 1 }, { value: 2 }];
	let source = { left: 3, right: 4 };

	$: {
		total = 0;
		for (const item of items) {
			total += item.value;
		}
	}

	$: ({ left, right } = source);
	$: if (items.length > 1) {
		conditional = total;
	} else {
		conditional = 0;
	}

	$: switch (left) {
		case 3:
			switched = right;
			break;
		default:
			switched = 0;
	}
</script>

<p>{total}-{left}-{right}-{conditional}-{switched}</p>
