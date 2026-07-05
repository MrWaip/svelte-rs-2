App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, `${$$props["0"] ?? ""} ${$$props["ysc%%gibberish"] ?? ""}`));
	$.append($$anchor, text);
	return $.pop($$exports);
}
