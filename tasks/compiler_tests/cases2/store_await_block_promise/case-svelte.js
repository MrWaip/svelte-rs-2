import * as $ from "svelte/internal/client";
import { promise } from "./stores";
var root_1 = $.from_html(`<p> </p>`);
var root_2 = $.from_html(`<p>p</p>`);
export default function App($$anchor) {
	const $promise = () => $.store_get(promise, "$promise", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, $promise, ($$anchor) => {
		var p_1 = root_2();
		$.append($$anchor, p_1);
	}, ($$anchor, value) => {
		var p = root_1();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(value)));
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
	$$cleanup();
}
