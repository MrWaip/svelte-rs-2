import * as $ from "svelte/internal/client";
import Foo from "./Foo.svelte";
function App($$anchor) {
	let x = true;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var fragment_1 = $.comment();
			var node_1 = $.first_child(fragment_1);
			Foo(node_1, {});
			$.append($$anchor, fragment_1);
		};
		$.if(node, ($$render) => {
			if (x) $$render(consequent);
		});
	}
	$.append($$anchor, fragment);
}
if (import.meta.hot) {
	App = $.hmr(App);
	import.meta.hot.accept((module) => {
		App[$.HMR].update(module.default);
	});
}
export default App;
