using System;
using System.Collections;
using System.Collections.Generic;

using SpacetimeDB;
using SpacetimeDB.Types;

using UnityEngine;

public class GameManager : MonoBehaviour {
  const string SERVER_URL = "http://127.0.0.1:3000";
  const string MODULE_NAME = "justmaple";

  public static event Action OnConnected;
  public static event Action OnSubscriptionApplied;

  [Header("Player Management")]
  public PlayerController LocalPlayerController; // Reference to PlayerController GameObject

  public static GameManager Instance {
    get; private set;
  }
  public static Identity LocalIdentity {
    get; private set;
  }
  public static DbConnection Conn {
    get; private set;
  }

  public static Dictionary<uint, EntityController> Entities = new Dictionary<uint, EntityController>();
  public static Dictionary<uint, PlayerController> Players = new Dictionary<uint, PlayerController>();

  private void Start() {
    Instance = this;
    Application.targetFrameRate = 60;

    // In order to build a connection to SpacetimeDB we need to register
    // our callbacks and specify a SpacetimeDB server URI and module name.
    var builder = DbConnection
      .Builder()
      .OnConnect(HandleConnect)
      .OnConnectError(HandleConnectError)
      .OnDisconnect(HandleDisconnect)
      .WithUri(SERVER_URL)
      .WithModuleName(MODULE_NAME);

    // If the user has a SpacetimeDB auth token stored in the Unity PlayerPrefs,
    // we can use it to authenticate the connection.
    if (AuthToken.Token != "") {
      builder = builder.WithToken(AuthToken.Token);
    }

    // Building the connection will establish a connection to the SpacetimeDB
    // server.
    Conn = builder.Build();
  }

  // Called when we connect to SpacetimeDB and receive our client identity
  // Called when we connect to SpacetimeDB and receive our client identity
  void HandleConnect(DbConnection conn, Identity identity, string token) {
    Debug.Log("Connected.");
    AuthToken.SaveToken(token);
    LocalIdentity = identity;

    conn.Db.MovementController.OnInsert += MovementControllerOnInsert;
    conn.Db.Entity.OnUpdate += EntityOnUpdate;
    conn.Db.Entity.OnDelete += EntityOnDelete;
    conn.Db.Player.OnInsert += PlayerOnInsert;
    conn.Db.Player.OnDelete += PlayerOnDelete;

    OnConnected?.Invoke();

    // Request all tables
    Conn.SubscriptionBuilder()
        .OnApplied(HandleSubscriptionApplied)
        .SubscribeToAllTables();
  }

  private static void MovementControllerOnInsert(EventContext context, MovementController insertedValue) {
    var player = GetOrCreatePlayer(insertedValue.PlayerId);
    // player will be null for remote players, which is fine - they still get visual entities
    var entityController = MovementControllerEntity.Spawn(insertedValue, player);
    if (entityController != null) {
      Entities.Add(insertedValue.EntityId, entityController);
    }
  }

  private static void EntityOnUpdate(EventContext context, Entity oldEntity, Entity newEntity) {
    if (!Entities.TryGetValue(newEntity.EntityId, out var entityController)) {
      return;
    }
    entityController.OnEntityUpdated(newEntity);
  }

  private static void EntityOnDelete(EventContext context, Entity oldEntity) {
    if (Entities.Remove(oldEntity.EntityId, out var entityController)) {
      entityController.OnDelete(context);
    }
  }


  private static void PlayerOnInsert(EventContext context, Player insertedPlayer) {
    GetOrCreatePlayer(insertedPlayer.PlayerId);
  }

  private static void PlayerOnDelete(EventContext context, Player deletedvalue) {
    if (Players.Remove(deletedvalue.PlayerId, out var playerController)) {
      GameObject.Destroy(playerController.gameObject);
    }
  }

  private static PlayerController GetOrCreatePlayer(uint playerId) {
    if (!Players.TryGetValue(playerId, out var playerController)) {
      var player = Conn.Db.Player.PlayerId.Find(playerId);

      // Check if this is the local player
      if (player.Identity == LocalIdentity) {
        // Use the referenced local PlayerController
        playerController = Instance.LocalPlayerController;
        if (playerController != null) {
          playerController.Initialize(player);
        }
      }
      else {
        // Remote players don't need PlayerController instances in this client
        // They're just visual entities managed by MovementControllerEntity
        return null;
      }

      if (playerController != null) {
        Players.Add(playerId, playerController);
      }
    }

    return playerController;
  }

  void HandleConnectError(Exception ex) {
    Debug.LogError($"Connection error: {ex}");
  }

  void HandleDisconnect(DbConnection _conn, Exception ex) {
    Debug.Log("Disconnected.");
    if (ex != null) {
      Debug.LogException(ex);
    }
  }

  private void HandleSubscriptionApplied(SubscriptionEventContext ctx) {
    Debug.Log("Subscription applied!");
    OnSubscriptionApplied?.Invoke();

    // Initialize camera controller without borders

    // Call enter game with the player name 3Blave
    ctx.Reducers.EnterGame("3Blave");
  }

  public static bool IsConnected() {
    return Conn != null && Conn.IsActive;
  }

  public void Disconnect() {
    Conn.Disconnect();
    Conn = null;
  }

}
