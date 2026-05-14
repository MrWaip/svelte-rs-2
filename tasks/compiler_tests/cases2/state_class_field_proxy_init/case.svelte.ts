const DEFAULTS = { mode: 'idle' };

export class Store {
	current = $state(DEFAULTS);
}
