<script>
	let obj = $state({ items: null, data: null, list: null, map: null });

	// Non-statement assignment — should use $.assign_nullish in dev
	(obj.items ??= []).push(1);

	// Non-statement assignment — should use $.assign in dev
	(obj.data = []).push(2);

	// Non-statement — $.assign_and
	(obj.list &&= []).length;

	// Non-statement — $.assign_or
	(obj.map ||= []).length;
</script>

<p>{obj.items}</p>
