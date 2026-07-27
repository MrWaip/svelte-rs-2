App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let f = $.tag($.state(0), "f");
	async function go() {
		// svelte-ignore await_reactivity_loss
		await (async ($$value) => {
			var $$array = $.to_array($$value, 1);
			$.set(f, (await $.track_reactivity_loss($.fallback($$array[0], async () => false || await Promise.resolve(6), true)))(), true);
		})([]);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(f)));
	$.delegated("click", button, go);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
