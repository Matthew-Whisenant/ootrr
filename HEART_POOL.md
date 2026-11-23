# Starting hearts and the heart item pool

## Why this changed

Link normally starts with three hearts. A Heart Container adds one heart, and
four Pieces of Heart add one heart. Changing the starting count without changing
the items in the world would leave more or fewer hearts available than before.

The generator now adjusts the shuffled item pool to compensate for starting
with a different number of hearts. It adjusts relative to the existing pool;
it does not independently establish that the resulting world contains exactly
20 hearts. See [Current limitations](#current-limitations).

## The adjustment

The **Starting hearts** setting allows 1 through 20. The new **Heart pool
adjustment** setting appears when the starting count differs from three:

- **Pieces of Heart:** prefer groups of four pieces.
- **Heart Containers:** prefer whole containers.
- **Random:** choose a feasible mix using the seed text, so the same seed and
  settings give the same adjustment.

If the preferred type cannot cover the full adjustment, the generator uses
the other type for the remainder. Pieces are always added or removed in
groups of four; a container counts as one heart.

| Starting hearts | Adjustment to the shuffled pool |
| --- | --- |
| 3 | No heart adjustment. |
| 1 or 2 | Add heart items worth `3 - starting hearts`. |
| 4 through 20 | Remove heart items worth `starting hearts - 3`. |

For example, starting with five hearts requires removing two hearts from the
pool: eight pieces, two containers, or four pieces and one container. Starting
with one heart requires adding two hearts by the same alternatives.

Adding heart items replaces shuffled rupee items; removing heart items replaces
them with green rupees. This keeps the number of items and shuffled locations
unchanged. The resulting items are then placed by the normal fill process, so
the locations originally holding the replaced items do not necessarily receive
their replacements.

If there are not enough suitable shuffled hearts or rupees to make the chosen
adjustment, generation stops with an error explaining the available counts.
The user can shuffle more applicable checks or choose a starting count closer
to three.

## Which checks count?

The adjustment looks at items from **shuffled** locations. Heart items at
unshuffled locations remain at their vanilla checks and are not removed from
the pool. Other settings determine which locations are shuffled:

| Check | How it joins the shuffled pool |
| --- | --- |
| Treasure Chest Game reward | Enable **Treasure Chest Game reward** and shuffle overworld chests. |
| Frog bug game Heart Piece | Enable **Zora's River frog bug game Heart Piece** and **Story and NPC rewards**. |
| Song of Storms frog reward | Enable **Story and NPC rewards**. |
| Hyrule Field heart scrub | Enable **Story and NPC rewards**. |
| Death Mountain Crater heart scrub | Enable **One time scrub sales** and **Story and NPC rewards**. |
| Supported freestanding heart pieces | These are shuffled even when **Freestanding items** is Off. |
| Boss Heart Containers | Enable **Boss Heart Containers**. |

The separate **Zora's River frog songs** option adds five other frog-song
checks, whose original rewards are rupees. It does not control the Song of
Storms heart reward or the bug-game heart reward.

The generator recognizes the normal Piece of Heart and the distinct logic
variants for the Treasure Chest Game and out-of-logic pieces as heart pieces.
Adding a piece uses the normal Piece of Heart item.

## How it is implemented

1. The generator selects locations using the current shuffle settings and
   builds their item pool.
2. Before filling locations, it calculates the difference between the selected
   starting hearts and the default of three.
3. It replaces the necessary shuffled heart or rupee items, choosing the mix
   according to **Heart pool adjustment**.
4. It records the number of containers and pieces added or removed in the
   generation report, then runs the existing placement rules.
5. During the mod build, added hearts receive the appropriate heart item actor,
   rather than the green-rupee actor used for removed items.

The main adjustment is in
[`randomizer/core/src/start_items/pool.rs`](randomizer/core/src/start_items/pool.rs).
It is called during generation from
[`randomizer/core/src/slice/generate.rs`](randomizer/core/src/slice/generate.rs).
The setting is defined in
[`randomizer/core/src/settings/list.rs`](randomizer/core/src/settings/list.rs),
and build-time actor assignment is handled by
[`randomizer/core/src/slice/chest_targets.rs`](randomizer/core/src/slice/chest_targets.rs).

## Current limitations

- The adjustment preserves the baseline total represented by the existing
  world and item pool. It does **not** count every actual in-game heart reward
  and enforce an exact 20-heart total.
- Some heart locations are not mapped to editable remake checks. Logic entries
  and actual in-game rewards are not necessarily a one-to-one match, so their
  counts should not be treated as a verified in-game inventory.
- Starting with 20 hearts fails under the default shuffle settings because
  there are not enough shuffled heart items to remove the required 17 hearts.
  Other high starting counts can also fail depending on the selected checks.
- Tests cover the adjustment, setting combinations, generated placements, and
  build-target actors. They do not replace an in-game playtest.
