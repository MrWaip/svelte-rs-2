import * as $ from "svelte/internal/client";
var root = $.template(`<section><span><span></span></span> <div><div><div>text</div></div></div> <p><b><i></i></b></p></section>`);
export default function App($$anchor) {
	var section = root();
	$.append($$anchor, section);
}
