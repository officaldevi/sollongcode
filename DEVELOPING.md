# Solg Developer Guide

*work in progress*

This is a more in depth description of the Solg core contracts. The
previous iteration of Solg was called Single Collateral Solg, or
`sai`


## Tooling

- dapp.tools
- solc v0.6.0
- tests use ds-test and are in files ending .t.sol


## Units

Solg has three different numerical units: `wad`, `ray` and `rad`

- `wad`: fixed point decimal with 18 decimals (for basic quantities, e.g. balances)
- `ray`: fixed point decimal with 27 decimals (for precise quantites, e.g. ratios)
- `rad`: fixed point decimal with 45 decimals (result of integer multiplication with a `wad` and a `ray`)

`wad` and `ray` units will be familiar from SCD. `rad` is a new unit and
exists to prevent precision loss in the core RAM engine.

The base of `ray` is `ONE = 10 ** 27`.


## Multiplication

Generally, `wad` should be used additively and `ray` should be used
multiplicatively. It usually doesn't make sense to multiply a `wad` by a
`wad` (or a `rad` by a `rad`).

Two multiplaction operators are used in `dss`:

- `mul`: standard integer multiplcation. No loss of precision.
- `rmul`: used for multiplications involving `ray`'s. Precision is lost.

They can only be used sensibly with the following combination of units:

- `mul(wad, ray) -> rad`
- `rmul(wad, ray) -> wad`
- `rmul(ray, ray) -> ray`
- `rmul(rad, ray) -> rad`

## Code style

This is obviously opinionated and you may even disagree, but here are
the considerations that make this code look like it does:

- Distinct things should have distinct names ("memes")

- Lack of symmetry and typographic alignment is a code smell.

- Inheritance masks complexity and encourages over abstraction, be
  explicit about what you want.

- In this modular system, contracts generally shouldn't call or jump
  into themselves, except for math. Again, this masks complexity.


## RAM Engine

The core RAM, Solg, and collateral state is kept in the `Vat`. This
contract has no external dependencies and maintains the central
"Accounting Invariants" of Solg.

Solg cannot exist without collateral:

- An `ilk` is a particular type of collateral.
- Collateral `gem` is assigned to users with `slip`.
- Collateral `gem` is transferred between users with `flux`.

The RAM data structure is the `Urn`:

- it has `ink` encumbered collateral
- it has `art` encumbered debt

Similarly, a collateral `Ilk`:

- it has `Ink` encumbered collateral
- it has `Art` encumbered debt
- it has `take` collateral scaling factor (discussed further below)
- it has `rate` debt scaling factor (discussed further below)

Here, "encumbered" means "locked in a RAM".

RAMs are managed via `frob(i, u, v, w, dink, dart)`, which modifies the
RAM of user `u`, using `gem` from user `v` and creating `solg` for user
`w`.

RAMs are confiscated via `grab(i, u, v, w, dink, dart)`, which modifies
the RAM of user `u`, giving `gem` to user `v` and creating `sin` for
user `w`. `grab` is the means by which RAMs are liquidated, transferring
debt from the RAM to a users `sin` balance.

Sin represents "seized" or "bad" debt and can be cancelled out with an
equal quantity of Solg using `heal(u, v, rad)`: take `sin` from `u` and
`solg` from `v`.

Note that `heal` can also be used to *create* Solg, balanced by an equal
quantity of Sin.

Finally, the quantity `solg` can be transferred between users with `move`.

### Rate

The ilk quantity `rate` define the ratio of exchange
between un-encumbered and encumbered Debt.

This quantity allows for manipulation of debt balances
across a whole Ilk.

Debt can be seized or injected into an ilk using `fold(i, u, rate)`,
which increases the `solg` balance of the user `u` by increasing the
encumbered debt balance of all urns in the ilk by the ratio `rate`.

## RAM Interface

The `Vat` contains risk parameters for each `ilk`:

- `spot`: the maximum amount of Solg drawn per unit collateral
- `line`: the maximum total Solg drawn

And a global risk parameter:

- `Line`: the maximum total Solg drawn across all ilks

The `Vat` exposes the public function:

- `frob(ilk, dink, dart)`: manipulate the callers RAM in the given `ilk`
  by `dink` and `dart`, subject to the risk parameters

## Liquidation Interface

The companion to RAM management is RAM liquidation, which is defined via
the `Cat`.

The `Cat` contains liquidation parameters for each `ilk`:

- `flip`: the address of the collateral liquidator
- `chop`: the liquidation penalty
- `lump`: the liquidation quantity

The `Cat` exposes two public functions

- `bite(ilk, urn)`: mark a specific RAM for liquidation
- `flip(n, wad)`: initiate liquidation
