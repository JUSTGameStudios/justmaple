using System.Collections;
using System.Collections.Generic;

using UnityEngine;

public class CameraController : MonoBehaviour {
  public static float WorldSize = 0.0f;

  private void LateUpdate() {
    var arenaCenterTransform = new Vector3(WorldSize / 2, WorldSize / 2, -10.0f);
    if (PlayerController.Local == null || !GameManager.IsConnected()) {
      // Set the camera to be in middle of the arena if we are not connected or
      // there is no local player
      transform.position = arenaCenterTransform;
      return;
    }

    var centerOfMass = PlayerController.Local.CenterOfMass();
    if (centerOfMass.HasValue) {
      // Set the camera to be the center of mass of the local player
      // if the local player has one
      transform.position = new Vector3 {
        x = centerOfMass.Value.x,
        y = centerOfMass.Value.y,
        z = transform.position.z
      };
    }
    else {
      transform.position = arenaCenterTransform;
    }

    float targetCameraDistance = CalculateCameraDistance(PlayerController.Local);
    var camPos = Camera.main.transform.position;
    camPos.z = Mathf.Lerp(camPos.z, targetCameraDistance, Time.deltaTime * 2);
    Camera.main.transform.position = camPos;
  }

  private float CalculateCameraDistance(PlayerController player) {
    return -50f - //Base distance (starts at -50)
            (player.TotalMass() / 10) - //Move further away based on total mass
            Mathf.Max(0, player.NumberOfOwnedCircles - 1) * 15; //Move further away when split
  }
}
