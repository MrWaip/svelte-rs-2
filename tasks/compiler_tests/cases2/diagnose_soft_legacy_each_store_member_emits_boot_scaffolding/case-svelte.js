import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { state } from "./store.js";
var root_1 = $.from_html(`<p>row</p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $state = () => $.store_get(state, "$state", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => ($state(), $.untrack(() => $state().items)), $.index, ($$anchor, _item) => {
		var p = root_1();
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}
