App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let left, right;
		let items = [{ value: 1 }, { value: 2 }];
		let source = {
			left: 3,
			right: 4
		};
		$: {
			total = 0;
			for (const item of items) {
				total += item.value;
			}
		}
		$: ({left, right} = source);
		$: if (items.length > 1) {
			conditional = total;
		} else {
			conditional = 0;
		}
		$: switch (left) {
			case 3:
				switched = right;
				break;
			default: switched = 0;
		}
		$$renderer.push(`<p>`);
		$.push_element($$renderer, "p", 28, 0);
		$$renderer.push(`${$.escape(total)}-${$.escape(left)}-${$.escape(right)}-${$.escape(conditional)}-${$.escape(switched)}</p>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
