import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<p> </p> <button>replace</button>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const $held = () => $.store_get($.get(held), "$held", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const source = writable(0);
	const held = $.derived(() => source);
	function replace() {
		$.store_set($.get(held), writable(1));
	}
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var button = $.sibling(p, 2);
	$.template_effect(() => $.set_text(text, $held()));
	$.delegated("click", button, replace);
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}
$.delegate(["click"]);
