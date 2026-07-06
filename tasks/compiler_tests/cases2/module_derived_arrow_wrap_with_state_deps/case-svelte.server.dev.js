import * as $ from "svelte/internal/server";
import { make } from "./ctx.svelte";
export const factory = make("store", (props) => {
	let items = [];
	function add(id) {
		items.push(id);
	}
	function remove(id) {
		items = items.filter((v) => v !== id);
	}
	const count = $.derived(() => items.length);
	const summary = $.derived(() => {
		if (count() === 0) return [0, 0];
		return [props.x, count()];
	});
	return {
		add,
		remove,
		get count() {
			return count();
		},
		get summary() {
			return summary();
		}
	};
});
