App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let title = $.tag($.state(10), "title");
	let other = 20;
	var $$exports = { ...$.legacy_api() };
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, `${$.set(title, $.get(title) + 5) ?? ""}
${$.set(title, $.get(title) && other, true) ?? ""}`));
	$.append($$anchor, text);
	return $.pop($$exports);
}
