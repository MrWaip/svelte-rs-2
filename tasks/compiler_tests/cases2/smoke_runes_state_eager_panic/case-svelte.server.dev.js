App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let eager = 0;
		let eagerObj = { x: 0 };
		function script_ops() {
			eager = 1;
			eager++;
			eagerObj.x = 2;
			eagerObj.x++;
		}
		$$renderer.push(`<!---->${$.escape(eager)}
${$.escape(eagerObj.x)}
${$.escape(eager = 3)}
${$.escape(eager++)}
${$.escape(eagerObj.x = 4)}
${$.escape(eagerObj.x++)} <button>`);
		$.push_element($$renderer, "button", 20, 0);
		$$renderer.push(`run</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
