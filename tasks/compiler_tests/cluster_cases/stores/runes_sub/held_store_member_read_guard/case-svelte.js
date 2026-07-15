import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const $held = () => $.store_get($.get(held), "$held", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const source = writable({ foo: 0 });
	const held = $.derived(() => source);
	const value = $.derived(() => $held().foo);
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(value)));
	$.append($$anchor, p);
	$.pop();
	$$cleanup();
}
