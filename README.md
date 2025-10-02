
Tyranny is a real-time strategy game inspired by Civilation, Warhammer, and Catan.

## Game Design

A player wins when they control all of their opponents' `capitals`.

### Setup Phase

At the start of the game, each player selects a `tile` to make their `capital`.

### Tile Groups

A `tile group` is a group of adjacent tiles that the player controls.
Each `tile` shares its resources with its `tile group`.
A single player may control more than one `tile group`.

### Capturing Tiles

The player can begin an attack on any `tile group` that is adjacent to a `tile group` they currently control.
During an attack, the `troop` resources from the attacking and defending `tile group` destroy each other.
For each expended `troop` resource, the attacker has a chance to claim one of the `tiles` in the defender's `tile group`.
If the attacker captures an opponent's last remaining `tile`, then they claim any resources their defeated opponent has.

### Resources

A `tile` will produce `troop` resources for the `tile group` that it is part of.

### Tile Improvements

A `tile improvement` changes the yield that a tile produces.
The `capital tile` is an example of a `tile improvement`, but it is a special case.
The player is only allowed one `capital tile` which is chosen at the start of the game.

#### Tile Improvements

<tabel>
    <thead>
        <tr>
            <th> Name </th>
            <th> Description </th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td> Capital </td>
            <td> The heart of the empire. Provides buffs to adjacent tiles. </td>
        </tr>
    </tbody>
</table>

### Fog of War

A player can only see `tile groups` that are immediately adjacent to a `tile group` they control.

### Diplomatic Relations


## Subsystems

### Multiplayer Networking

- Authoritative Server

