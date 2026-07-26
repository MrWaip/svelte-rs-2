import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>inc</button> <p> </p> <p> </p>`, 1), App[$.FILENAME], [
	[8, 0],
	[9, 0],
	[10, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let gate = $.tag($.state(0), "gate");
	let plain = 1;
	var loaded;
	var $$promises = $.run([async () => loaded = await $.async_derived(async () => (await $.track_reactivity_loss($.get(gate)))(), "loaded", "(unknown):4:14"), () => void (plain = 2)]);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var p = $.sibling(button, 2);
	var text = $.child(p, true);
	$.reset(p);
	var p_1 = $.sibling(p, 2);
	var text_1 = $.child(p_1, true);
	$.reset(p_1);
	$.template_effect(() => {
		$.set_text(text, plain);
		$.set_text(text_1, $.get(loaded));
	}, void 0, void 0, [$$promises[1], $$promises[0]]);
	$.delegated("click", button, function click() {
		return $.update(gate);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
