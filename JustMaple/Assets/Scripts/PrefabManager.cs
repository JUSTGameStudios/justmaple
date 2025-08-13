using SpacetimeDB.Types;

using System.Collections;
using System.Collections.Generic;

using UnityEngine;

public class PrefabManager : MonoBehaviour {
  public static PrefabManager Instance;

  public GameObject PlayerPrefab; // Reference to visual player prefab (for all players)

  private void Awake() {
    Instance = this;
  }

}
