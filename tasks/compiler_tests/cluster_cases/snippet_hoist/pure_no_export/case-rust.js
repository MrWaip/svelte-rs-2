import * as $ from "svelte/internal/client";
const foo = ($$anchor, a = $.noop, b = $.noop) => {
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, `Hello world ${a() + b()}`));
	$.append($$anchor, text);
};
export default function App($$anchor) {
	foo($$anchor, () => 1, () => 2);
}
