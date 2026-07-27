export function render(items, push) {
	push(() => {
		for (let $$index = 0; $$index < items.length; $$index++) {
			items[$$index];
		}
	});
}
