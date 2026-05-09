import * as $ from "svelte/internal/client";
import { make } from "./ctx.svelte";
export const factory = make("store", (props) => {
	let items = $.state($.proxy([]));
	function add(id) {
		$.get(items).push(id);
	}
	function remove(id) {
		$.set(items, $.get(items).filter((v) => v !== id), true);
	}
	const count = $.derived(() => $.get(items).length);
	const summary = $.derived(() => {
		if ($.get(count) === 0) return [0, 0];
		return [props.x, $.get(count)];
	});
	return {
		add,
		remove,
		get count() {
			return $.get(count);
		},
		get summary() {
			return $.get(summary);
		}
	};
});
