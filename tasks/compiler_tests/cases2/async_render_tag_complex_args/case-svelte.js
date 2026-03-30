import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
const content = ($$anchor, value = $.noop, extra = $.noop) => {
	var p = root_1();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${value() ?? ""}${extra() ?? ""}`));
	$.append($$anchor, p);
};
var root_1 = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	var response;
	var $$promises = $.run([async () => response = await fetch("/api")]);
	$.async($$anchor, [$$promises[0]], [() => response.text()], ($$anchor, $0) => {
		content($$anchor, () => response, () => $.get($0));
	});
	$.next();
}
