import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root_1 = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const $state = () => $.store_get(state, "$state", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	function makeAdapter() {
		return { state: writable({ items: [
			1,
			2,
			3
		] }) };
	}
	const { state } = makeAdapter();
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => $state().items, $.index, ($$anchor, item) => {
		var p = root_1();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(item)));
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}
