App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const foo = $.effect_tracking();
	let bar = $.tag($.state(false), "bar");
	$.user_pre_effect(() => {
		$.set(bar, $.effect_tracking(), true);
	});
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${foo} ${$.get(bar) ?? ""}`));
	$.append($$anchor, p);
	return $.pop($$exports);
}
