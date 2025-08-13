using System.Collections.Generic;
using System.Linq;

using SpacetimeDB;
using SpacetimeDB.Types;

using UnityEngine;
using UnityEngine.InputSystem;

public class PlayerController : MonoBehaviour {
  const int SEND_UPDATES_PER_SEC = 20;
  const float SEND_UPDATES_FREQUENCY = 1f / SEND_UPDATES_PER_SEC;

  public static PlayerController Local {
    get; private set;
  }

  private uint PlayerId;
  private float LastMovementSendTimestamp;
  private List<MovementControllerEntity> OwnedEntities = new List<MovementControllerEntity>(); // NEW: Track movement entities instead of circles

  // NEW: Input System integration
  private PlatformerInputActions inputActions;
  private float horizontalInput;
  private bool jumpInput;
  private bool jumpPressed;

  public string Username => GameManager.Conn.Db.Player.PlayerId.Find(PlayerId).Name;
  public int NumberOfOwnedEntities => OwnedEntities.Count; // NEW: Renamed from NumberOfOwnedCircles
  public bool IsLocalPlayer => this == Local;

  public void Initialize(Player player) {
    PlayerId = player.PlayerId;
    if (player.Identity == GameManager.LocalIdentity) {
      Local = this;
      SetupInputSystem(); // NEW: Initialize input system for local player
    }
  }

  // NEW: Setup Unity Input System
  private void SetupInputSystem() {
    if (inputActions == null) {
      inputActions = new PlatformerInputActions();
      inputActions.Gameplay.Move.performed += OnMoveInput;
      inputActions.Gameplay.Move.canceled += OnMoveInput;
      inputActions.Gameplay.Jump.performed += OnJumpInput;
      inputActions.Gameplay.Jump.canceled += OnJumpInput;
      inputActions.Enable();
    }
  }

  // NEW: Handle horizontal movement input
  private void OnMoveInput(InputAction.CallbackContext context) {
    horizontalInput = context.ReadValue<float>();
  }

  // NEW: Handle jump input
  private void OnJumpInput(InputAction.CallbackContext context) {
    bool wasPressed = jumpInput;
    jumpInput = context.ReadValueAsButton();
    jumpPressed = jumpInput && !wasPressed; // Edge detection for jump press
  }

  private void OnDestroy() {
    // Clean up input system
    if (inputActions != null) {
      inputActions.Disable();
      inputActions.Dispose();
    }

    // If we have any entities, destroy them
    foreach (var entity in OwnedEntities) {
      if (entity != null) {
        Destroy(entity.gameObject);
      }
    }
    OwnedEntities.Clear();
  }

  public void OnEntitySpawned(MovementControllerEntity entity) { // NEW: Updated method name
    OwnedEntities.Add(entity);
  }

  public void OnEntityDeleted(MovementControllerEntity deletedEntity) { // NEW: Updated method name
    // This means we got eaten
    if (OwnedEntities.Remove(deletedEntity) && IsLocalPlayer && OwnedEntities.Count == 0) {
      // DeathScreen.Instance.SetVisible(true);
    }
  }

  public uint TotalMass() {
    return (uint)OwnedEntities
        .Select(entity => GameManager.Conn.Db.Entity.EntityId.Find(entity.EntityId))
        .Sum(e => e?.Mass ?? 0); //If this entity is being deleted on the same frame that we're moving, we can have a null entity here.
  }

  public Vector2? CenterOfMass() {
    if (OwnedEntities.Count == 0) {
      return null;
    }

    Vector2 totalPos = Vector2.zero;
    float totalMass = 0;
    foreach (var entity in OwnedEntities) {
      var entityData = GameManager.Conn.Db.Entity.EntityId.Find(entity.EntityId);
      var position = entity.transform.position;
      totalPos += (Vector2)position * entityData.Mass;
      totalMass += entityData.Mass;
    }

    return totalPos / totalMass;
  }

  private void OnGUI() {
    if (!IsLocalPlayer || !GameManager.IsConnected()) {
      return;
    }

    GUI.Label(new Rect(0, 0, 100, 50), $"Total Mass: {TotalMass()}");
  }

  public void Update() {
    if (!IsLocalPlayer || NumberOfOwnedEntities == 0) {
      return;
    }

    // NEW: Send platformer input at 20Hz to server
    if (Time.time - LastMovementSendTimestamp >= SEND_UPDATES_FREQUENCY) {
      LastMovementSendTimestamp = Time.time;

      // Send current input state to server - no mouse conversion needed!
      float horizontal = testInputEnabled ? testInput.x : horizontalInput;
      bool jump = testInputEnabled ? testInput.y > 0.5f : (jumpInput || jumpPressed);

      GameManager.Conn.Reducers.UpdatePlayerInput(horizontal, jump);

      // Reset jump press after sending
      jumpPressed = false;
    }
  }

  //Automated testing members
  private bool testInputEnabled;
  private Vector2 testInput;

  public void SetTestInput(Vector2 input) => testInput = input;
  public void EnableTestInput() => testInputEnabled = true;
}
