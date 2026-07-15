import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const $rows = () => $.store_get(rows, "$rows", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const rows = writable([]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, $rows, $.index, ($$anchor, row, $$index) => {
		Child($$anchor, {
			get value() {
				return $.get(row).name;
			},
			set value($$value) {
				$.get(row).name = $$value, $.invalidate_store($$stores, "$rows");
			}
		});
	});
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}
