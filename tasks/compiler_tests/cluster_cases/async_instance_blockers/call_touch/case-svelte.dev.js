import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>toggle</button> <!> <p> </p>`, 1), App[$.FILENAME], [[6, 0], [8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let gate = $.tag($.state(true), "gate");
	var loaded;
	var $$promises = $.run([async () => loaded = await $.async_derived(async () => (await $.track_reactivity_loss($.get(gate)))(), "loaded", "(unknown):3:14")]);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	$.async(node, [$$promises[0]], void 0, (node) => {
		var consequent = ($$anchor) => {
			var text = $.text("yes");
			$.append($$anchor, text);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.get(gate)) $$render(consequent);
		}), "if", App, 7, 0);
	});
	var p = $.sibling(node, 2);
	var text_1 = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text_1, $.get(loaded)), void 0, void 0, [$$promises[0]]);
	$.delegated("click", button, function click() {
		return $.set(gate, !$.get(gate));
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
