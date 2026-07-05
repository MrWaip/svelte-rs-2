import * as $ from "svelte/internal/client";
import { make } from "./ctx.svelte";
export const factory = make("store", (props) => {
	let items = $.tag($.state($.proxy([])), "items");
	function add(id) {
		$.get(items).push(id);
	}
	function remove(id) {
		$.set(items, $.get(items).filter((v) => $.strict_equals(v, id, false)), true);
	}
	const count = $.tag($.derived(() => $.get(items).length), "count");
	const summary = $.tag($.derived(() => {
		if ($.strict_equals($.get(count), 0)) return [0, 0];
		return [props.x, $.get(count)];
	}), "summary");
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
