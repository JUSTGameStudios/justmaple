using SpacetimeDB.Types;
using UnityEngine;

// NEW: Entity controller for platformer players (replaces CircleController)
public class MovementControllerEntity : EntityController
{
    private PlayerController playerController;

    public static MovementControllerEntity Spawn(MovementController controller, PlayerController player)
    {
        var entity = GameManager.Conn.Db.Entity.EntityId.Find(controller.EntityId);
        if (entity == null)
        {
            Debug.LogError($"Entity {controller.EntityId} not found for MovementController");
            return null;
        }

        var prefab = PrefabManager.Instance.PlayerPrefab;
        var obj = Instantiate(prefab);
        var component = obj.GetComponent<MovementControllerEntity>();
        
        component.playerController = player; // Can be null for remote players
        component.Spawn(controller.EntityId);
        
        // Set player-specific color (could be based on player ID)
        component.SetColor(GetPlayerColor(controller.PlayerId));
        
        // Only notify if this is a local player
        if (player != null)
        {
            player.OnEntitySpawned(component);
        }
        
        return component;
    }

    public override void OnEntityUpdated(Entity newVal)
    {
        // Enhanced interpolation using velocity for smoother movement prediction
        base.OnEntityUpdated(newVal);
        
        // TODO: Add velocity-based prediction for more responsive movement
        // This could use newVal.Velocity to predict position between server updates
    }

    public override void OnDelete(EventContext context)
    {
        if (playerController != null)
        {
            playerController.OnEntityDeleted(this);
        }
        
        base.OnDelete(context);
    }

    // Generate consistent colors for players based on their ID
    private static Color GetPlayerColor(uint playerId)
    {
        // Simple color generation based on player ID
        float hue = (playerId * 0.618033988749f) % 1.0f; // Golden ratio for good color distribution
        return Color.HSVToRGB(hue, 0.8f, 0.9f);
    }
}