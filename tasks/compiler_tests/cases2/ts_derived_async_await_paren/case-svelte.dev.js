import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	async function fetchValue() {
		return 5;
	}
	var x;
	var $$promises = $.run([async () => x = await $.async_derived(async () => (await $.track_reactivity_loss(fetchValue()))(), "x", "(unknown):3:12")]);
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, $.get(x)), void 0, void 0, [$$promises[0]]);
	$.append($$anchor, p);
	return $.pop($$exports);
}
