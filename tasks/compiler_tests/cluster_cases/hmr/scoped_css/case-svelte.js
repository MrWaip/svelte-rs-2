import * as $ from "svelte/internal/client";
var root = $.from_html(`<h1 class="svelte-1erwonp">hello</h1>`);
function App($$anchor) {
	var h1 = root();
	$.append($$anchor, h1);
}
if (import.meta.hot) {
	App = $.hmr(App);
	import.meta.hot.accept((module) => {
		$.cleanup_styles("svelte-1erwonp");
		App[$.HMR].update(module.default);
	});
}
export default App;
