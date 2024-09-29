### Entities

*   **Player**: The main character controlled by the player.
*   **Enemies**: Various types of enemies with different behaviors.
*   **Bullets/Projectiles**: Fired by both the player and enemies.
*   **Power-ups/Items**: Collectible items that provide bonuses or abilities.
*   **Obstacles**: Static or dynamic objects that can block movement or cause damage.
*   **Rooms/Levels**: Different areas or levels within the game.

### Components

*   **Sprite**: Visual representation of the entity.
*   **Velocity**: Speed and direction of movement. (Vec2)
*   **Health**: Current and maximum health values.
*   **Damage**: Amount of damage the entity can inflict.
*   **AI**: Behavior patterns for enemies.
*   **Collider**: Collision detection properties.
*   **Inventory**: Items collected by the player.
*   **Weapon**: Information about the weapon, such as fire rate and bullet type.

### Systems

*   **Movement System**: Updates the position of entities based on their velocity.
*   **Rendering System**: Draws entities on the screen.
*   **Collision System**: Detects and handles collisions between entities.
*   **AI System**: Controls enemy behavior and decision-making.
*   **Health System**: Manages health and damage interactions.
*   **Input System**: Processes player input and updates the player entity accordingly.
*   **Weapon System**: Handles firing bullets and managing weapon cooldowns.
*   **Loot System**: Manages item drops and pickups.