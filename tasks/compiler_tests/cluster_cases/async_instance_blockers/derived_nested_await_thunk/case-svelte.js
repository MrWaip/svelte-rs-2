import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	async function g() {
		return 1;
	}
	async function f(v) {
		return v;
	}
	var x;
	var $$promises = $.run([async () => x = await $.async_derived(async () => await f((await $.save(g()))()))]);
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(x)), void 0, void 0, [$$promises[0]]);
	$.append($$anchor, p);
}
