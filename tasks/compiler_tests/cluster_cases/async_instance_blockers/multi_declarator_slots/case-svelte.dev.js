import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>inc</button> <p> </p>`, 1), App[$.FILENAME], [[7, 0], [8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let gate = $.tag($.state(0), "gate");
	var loaded, first, second;
	var $$promises = $.run([async () => loaded = await $.async_derived(async () => (await $.track_reactivity_loss($.get(gate)))(), "loaded", "(unknown):3:14"), () => {
		first = $.get(gate) + 1;
		second = $.get(gate) + 2;
	}]);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var p = $.sibling(button, 2);
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(loaded) ?? ""}${first}${second}`), void 0, void 0, [
		$$promises[0],
		$$promises[1],
		$$promises[1]
	]);
	$.delegated("click", button, function click() {
		return $.update(gate);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
