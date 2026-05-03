import * as $ from "svelte/internal/client";
var root = $.with_script($.from_html(`<div><script>
		console.log('inline');
	<\/script><!></div>`));
export default function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
