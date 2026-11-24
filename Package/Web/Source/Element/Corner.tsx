import "../Asset/CSS/Corner.css";

type CornerProperty = {
	id?: string;
};

/**
 * One corner of the overlay.
 *
 * The radius is inherited from `.Window`, which sets `--Corner` from this
 * window's own size event. A corner must not carry a `--Corner` of its own: an
 * inline value on the corner wins over the window's, which pins the radius and
 * makes a per-monitor setting appear to do nothing.
 */
export default (Property: CornerProperty) => (
	<div class="Corner" data-corner={Property.id ?? "Default"} />
);
