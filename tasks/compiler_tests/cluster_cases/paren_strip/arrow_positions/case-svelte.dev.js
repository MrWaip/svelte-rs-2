App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let a = $.tag($.state(0), "a");
	const handler = $.tag($.derived(() => $.get(a) ? () => {
		$.update(a);
	} : undefined), "handler");
	const list = $.tag($.derived(() => [() => $.get(a), () => $.get(a) + 1]), "list");
	const cfg = $.tag($.derived(() => ({ run: () => $.get(a) })), "cfg");
	const call = $.tag($.derived(() => [1].map(() => $.get(a))), "call");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(($0) => $.set_text(text, `${$.get(a) ?? ""} ${$.get(list).length ?? ""} ${$0 ?? ""} ${$.get(call).length ?? ""}`), [() => $.get(cfg).run()]);
	$.delegated("click", button, function(...$$args) {
		$.apply(() => $.get(handler), this, $$args, App, [9, 17]);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
