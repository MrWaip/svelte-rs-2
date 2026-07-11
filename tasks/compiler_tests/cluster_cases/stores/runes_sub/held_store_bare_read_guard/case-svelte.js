import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const $held = () => $.store_get($.get(held), "$held", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	const source = writable(0);
	const held = $.derived(() => source);
	function read() {
		const current = $held();
		return current;
	}
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(($0) => $.set_text(text, $0), [() => read()]);
	$.append($$anchor, p);
	$.pop();
	$$cleanup();
}
