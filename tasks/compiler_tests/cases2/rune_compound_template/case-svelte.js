import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	let title = $.state(10);
	let other = 20;
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, `${$.set(title, $.get(title) + 5) ?? ""}
${$.set(title, $.get(title) && other, true) ?? ""}`));
	$.append($$anchor, text);
}
