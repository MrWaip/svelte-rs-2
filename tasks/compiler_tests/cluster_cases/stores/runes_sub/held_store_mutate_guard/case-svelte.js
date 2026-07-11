import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<p> </p> <button>bump</button>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const $held = () => $.store_get($.get(held), "$held", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const source = writable({ bar: 0 });
	const held = $.derived(() => source);
	function bump() {
		$.store_mutate($.get(held), $.untrack($held).bar = 6, $.untrack($held));
	}
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var button = $.sibling(p, 2);
	$.template_effect(() => $.set_text(text, $held().bar));
	$.delegated("click", button, bump);
	$.append($$anchor, fragment);
	$.pop();
	$$cleanup();
}
$.delegate(["click"]);
