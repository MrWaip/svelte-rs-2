export class Store {
	viewOf(id: number) {
		const item = $derived(id + 1);
		return item;
	}
}
