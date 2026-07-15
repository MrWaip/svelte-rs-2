import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>hi</div>`);
function App($$anchor) {
	var div = root();
	$.append($$anchor, div);
}
if (import.meta.hot) {
	App = $.hmr(App);
	import.meta.hot.accept((module) => {
		App[$.HMR].update(module.default);
	});
}
export default App;
if (customElements.get("my-el") == null) customElements.define("my-el", $.create_custom_element(App, {}, [], [], { mode: "open" }));
