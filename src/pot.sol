// SPDX-License-Identifier: AGPL-3.0-or-later

/// pot.sol -- Solg Savings Rate

// Copyright (C) 2018 Rain <rainbreak@riseup.net>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

pragma solidity ^0.6.12;

// FIXME: This contract was altered compared to the production version.
// It doesn't use LibNote anymore.
// New deployments of this contract will need to include custom events (TO DO).

/*
   "Savings Solg" is obtained when Solg is deposited into
   this contract. Each "Savings Solg" accrues Solg interest
   at the "Solg Savings Rate".

   This contract does not implement a user tradeable token
   and is intended to be used with adapters.

         --- `save` your `solg` in the `pot` ---

   - `dsr`: the Solg Savings Rate
   - `pie`: user balance of Savings Solg

   - `join`: start saving some solg
   - `exit`: remove some solg
   - `drip`: perform rate collection

*/

interface VatLike {
    function move(address,address,uint256) external;
    function suck(address,address,uint256) external;
}

contract Pot {
    // --- Auth ---
    mapping (address => uint) public wards;
    function rely(address guy) external auth { wards[guy] = 1; }
    function deny(address guy) external auth { wards[guy] = 0; }
    modifier auth {
        require(wards[msg.sender] == 1, "Pot/not-authorized");
        _;
    }

    // --- Data ---
    mapping (address => uint256) public pie;  // Normalised Savings Solg [wad]

    uint256 public Pie;   // Total Normalised Savings Solg  [wad]
    uint256 public dsr;   // The Solg Savings Rate          [ray]
    uint256 public chi;   // The Rate Accumulator          [ray]

    VatLike public vat;   // RAM Engine
    address public vow;   // Debt Engine
    uint256 public rho;   // Time of last drip     [unix epoch time]

    uint256 public live;  // Active Flag

    // --- Init ---
    constructor(address vat_) public {
        wards[msg.sender] = 1;
        vat = VatLike(vat_);
        dsr = ONE;
        chi = ONE;
        rho = now;
        live = 1;
    }

    // --- Math ---
    uint256 constant ONE = 10 ** 27;
    function _rpow(uint x, uint n, uint base) internal pure returns (uint z) {
        assembly {
            switch x case 0 {switch n case 0 {z := base} default {z := 0}}
            default {
                switch mod(n, 2) case 0 { z := base } default { z := x }
                let half := div(base, 2)  // for rounding.
                for { n := div(n, 2) } n { n := div(n,2) } {
                    let xx := mul(x, x)
                    if iszero(eq(div(xx, x), x)) { revert(0,0) }
                    let xxRound := add(xx, half)
                    if lt(xxRound, xx) { revert(0,0) }
                    x := div(xxRound, base)
                    if mod(n,2) {
                        let zx := mul(z, x)
                        if and(iszero(iszero(x)), iszero(eq(div(zx, x), z))) { revert(0,0) }
                        let zxRound := add(zx, half)
                        if lt(zxRound, zx) { revert(0,0) }
                        z := div(zxRound, base)
                    }
                }
            }
        }
    }

    function _rmul(uint x, uint y) internal pure returns (uint z) {
        z = _mul(x, y) / ONE;
    }

    function _add(uint x, uint y) internal pure returns (uint z) {
        require((z = x + y) >= x);
    }

    function _sub(uint x, uint y) internal pure returns (uint z) {
        require((z = x - y) <= x);
    }

    function _mul(uint x, uint y) internal pure returns (uint z) {
        require(y == 0 || (z = x * y) / y == x);
    }

    // --- Administration ---
    function file(bytes32 what, uint256 data) external auth {
        require(live == 1, "Pot/not-live");
        require(now == rho, "Pot/rho-not-updated");
        if (what == "dsr") dsr = data;
        else revert("Pot/file-unrecognized-param");
    }

    function file(bytes32 what, address addr) external auth {
        if (what == "vow") vow = addr;
        else revert("Pot/file-unrecognized-param");
    }

    function cage() external auth {
        live = 0;
        dsr = ONE;
    }

    // --- Savings Rate Accumulation ---
    function drip() external returns (uint tmp) {
        require(now >= rho, "Pot/invalid-now");
        tmp = _rmul(_rpow(dsr, now - rho, ONE), chi);
        uint chi_ = _sub(tmp, chi);
        chi = tmp;
        rho = now;
        vat.suck(address(vow), address(this), _mul(Pie, chi_));
    }

    // --- Savings Solg Management ---
    function join(uint wad) external {
        require(now == rho, "Pot/rho-not-updated");
        pie[msg.sender] = _add(pie[msg.sender], wad);
        Pie             = _add(Pie,             wad);
        vat.move(msg.sender, address(this), _mul(chi, wad));
    }

    function exit(uint wad) external {
        pie[msg.sender] = _sub(pie[msg.sender], wad);
        Pie             = _sub(Pie,             wad);
        vat.move(address(this), msg.sender, _mul(chi, wad));
    }
}
