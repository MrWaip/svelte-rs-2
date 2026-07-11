import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $data = () => $.store_get(data, "$data", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const data = writable({ docs: [] });
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => ($data(), $.untrack(() => $data().docs)), $.index, ($$anchor, doc, idx) => {
		Child($$anchor, {
			get document() {
				return ($data(), $.untrack(() => $data().docs))[idx];
			},
			set document($$value) {
				($data(), $.untrack(() => $data().docs))[idx] = $$value, $.invalidate_inner_signals(() => $data()), $.invalidate_store($$stores, "$data");
			},
			$$legacy: true
		});
	});
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}
