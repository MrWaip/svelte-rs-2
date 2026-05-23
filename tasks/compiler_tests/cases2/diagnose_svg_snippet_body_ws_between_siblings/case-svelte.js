import * as $ from "svelte/internal/client";
const shape = ($$anchor) => {
	var fragment = root_1();
	$.next();
	$.append($$anchor, fragment);
};
var root_1 = $.from_svg(`<g><path d="M1"></path></g><g><path d="M2"></path></g>`, 1);
export default function App($$anchor) {
	shape($$anchor);
}
