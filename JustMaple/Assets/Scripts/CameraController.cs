using System.Collections;
using System.Collections.Generic;

using UnityEngine;

public class CameraController : MonoBehaviour {
  [Header("Platformer Camera Settings")]
  public float followSpeed = 5f;
  public Vector3 offset = new Vector3(0, 1f, -10f); // Slightly above player
  
  private void LateUpdate() {
    if (PlayerController.Local == null || !GameManager.IsConnected()) {
      return;
    }

    var centerOfMass = PlayerController.Local.CenterOfMass();
    if (centerOfMass.HasValue) {
      Vector3 targetPosition = new Vector3(
        centerOfMass.Value.x + offset.x,
        centerOfMass.Value.y + offset.y,
        offset.z
      );
      
      transform.position = Vector3.Lerp(transform.position, targetPosition, followSpeed * Time.deltaTime);
    }
  }
}
