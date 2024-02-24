use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
  common::{platform::WindowHandle, TilingDirection},
  containers::Container,
};

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WmEvent {
  BindingModeChanged {
    new_binding_mode: String,
  },
  FocusChanged {
    focused_container: Container,
  },
  FocusedContainerMoved {
    focused_container: Container,
  },
  MonitorAdded {
    added_monitor: Container,
  },
  MonitorRemoved {
    removed_id: Uuid,
    removed_device_name: String,
  },
  NativeFocusSynced {
    focused_container: Container,
  },
  TilingDirectionChanged {
    new_tiling_direction: TilingDirection,
  },
  UserConfigReloaded,
  WindowManaged {
    managed_window: Container,
  },
  WindowUnmanaged {
    unmanaged_id: Uuid,
    unmanaged_handle: WindowHandle,
  },
  WorkspaceActivated {
    activated_workspace: Container,
  },
  WorkspaceDeactivated {
    deactivated_id: Uuid,
    deactivated_name: String,
  },
  WorkingAreaResized {
    affected_monitor: Container,
  },
}