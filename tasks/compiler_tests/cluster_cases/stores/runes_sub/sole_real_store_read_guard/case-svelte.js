import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const $source = () => $.store_get(source, "$source", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const source = writable(0);
	const doubled = $.derived($source);
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(doubled)));
	$.append($$anchor, p);
	$.pop();
	$$cleanup();
}
