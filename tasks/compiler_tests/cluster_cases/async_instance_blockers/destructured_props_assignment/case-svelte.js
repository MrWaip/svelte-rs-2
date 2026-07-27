import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	var loaded, name;
	var $$promises = $.run([async () => loaded = await Promise.resolve(1), () => void 0]);
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${loaded ?? ""}${$$props.name ?? ""}`), void 0, void 0, [$$promises[0], $$promises[1]]);
	$.append($$anchor, p);
}
