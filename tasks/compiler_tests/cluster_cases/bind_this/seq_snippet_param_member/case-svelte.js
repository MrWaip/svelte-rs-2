import * as $ from "svelte/internal/client";
const funBind = ($$anchor, context = $.noop) => {
	var input = root_1();
	$.bind_this(input, (e) => context().element = e, () => {});
	$.append($$anchor, input);
};
var root_1 = $.from_html(`<input/>`);
export default function App($$anchor) {
	funBind($$anchor, () => ({ set element(e) {} }));
}
