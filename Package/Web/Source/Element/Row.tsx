import { Show, type JSX } from "solid-js";

/** One row: a label, that row's controls, and the reset that undoes them.
 *
 *  `mark` is the row's "own value" bullet, drawn inside the label's own column
 *  after the name so it survives a name too long for the column and every row's
 *  controls still line up. `reset` is absent on a row with nothing to reset.
 */
export default (properties:{
	label:string;
	hint:string;
	mark?:string | undefined;
	reset?:string | undefined;
	Reset?:(() => void) | undefined;
	children:JSX.Element;
}) => (
	<label class="Row">
		<span title={properties.hint}>
			<span class="Name">{properties.label}</span>
			<Show when={properties.mark}>
				{(mark) => (
					<span class="Mark" title={mark()}>
						•
					</span>
				)}
			</Show>
		</span>
		{properties.children}
		<Show when={properties.reset}>
			{(reset) => (
				<button class="Reset" title={reset()} onClick={() => properties.Reset?.()}>
					↺
				</button>
			)}
		</Show>
	</label>
);
