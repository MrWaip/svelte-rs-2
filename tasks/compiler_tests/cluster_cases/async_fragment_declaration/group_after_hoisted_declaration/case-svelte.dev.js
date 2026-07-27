import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div><span> </span></div>`), App[$.FILENAME], [[
	2,
	0,
	[[5, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const id = "name";
	var $$exports = { ...$.legacy_api() };
	var div = root();
	{
		const nested = "nested";
		let greeting2;
		var promises = $.run([async () => greeting2 = await $.async_derived(async () => (await $.track_reactivity_loss(`Hi ${id}`))(), "greeting2", "(unknown):4:20")]);
		var span = $.child(div);
		var text = $.child(span);
		$.reset(span);
		$.reset(div);
		$.template_effect(() => $.set_text(text, `nested ${$.get(greeting2) ?? ""}`), void 0, void 0, [promises[0]]);
	}
	$.append($$anchor, div);
	return $.pop($$exports);
}
