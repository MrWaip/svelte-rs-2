import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>inc</button> <p> </p>`, 1), App[$.FILENAME], [[8, 0], [9, 0]]);
export default function App($$anchor, $$props) {
	const uid = $.props_id();
	$.check_target(new.target);
	$.push($$props, true, App);
	let gate = $.tag($.state(0), "gate");
	var loaded, after;
	var $$promises = $.run([async () => loaded = await $.async_derived(async () => (await $.track_reactivity_loss($.get(gate)))(), "loaded", "(unknown):3:14"), () => after = $.get(gate) + 1]);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var p = $.sibling(button, 2);
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${uid}${after}`), void 0, void 0, [$$promises[1]]);
	$.delegated("click", button, function click() {
		return $.update(gate);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
