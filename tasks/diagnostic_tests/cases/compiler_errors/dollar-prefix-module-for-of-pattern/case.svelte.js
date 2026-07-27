export function walk(list) {
	for (const { $$a } of list) {
		list.push($$a);
	}
}
