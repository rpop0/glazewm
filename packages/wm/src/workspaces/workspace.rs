use std::{
  cell::{Ref, RefCell, RefMut},
  rc::Rc,
};

use uuid::Uuid;

use crate::{
  common::RectDelta,
  containers::{
    traits::{CommonBehavior, PositionBehavior, TilingBehavior},
    Container, ContainerType, TilingContainer,
  },
  impl_common_behavior, impl_tiling_behavior,
  user_config::WorkspaceConfig,
};

#[derive(Clone, Debug)]
pub struct Workspace(Rc<RefCell<WorkspaceInner>>);

#[derive(Debug)]
struct WorkspaceInner {
  id: Uuid,
  parent: Option<TilingContainer>,
  children: Vec<Container>,
  // TODO: Consider changing `non_tiling_children` to several fields for
  // each window type (ie. `floating_windows`, `maximized_windows`, etc.)
  non_tiling_children: Vec<Container>,
  size_percent: f32,
  config: WorkspaceConfig,
  outer_gaps: RectDelta,
}

impl Workspace {
  pub fn new(config: WorkspaceConfig, outer_gaps: RectDelta) -> Self {
    let workspace = WorkspaceInner {
      id: Uuid::new_v4(),
      parent: None,
      children: Vec::new(),
      non_tiling_children: Vec::new(),
      size_percent: 1.0,
      config,
      outer_gaps,
    };

    Self(Rc::new(RefCell::new(workspace)))
  }

  fn outer_gaps(&self) -> Ref<'_, RectDelta> {
    Ref::map(self.0.borrow(), |c| &c.outer_gaps)
  }
}

impl_common_behavior!(Workspace, ContainerType::Workspace);
impl_tiling_behavior!(Workspace);

impl PositionBehavior for Workspace {
  fn width(&self) -> i32 {
    let monitor_width = self.parent().unwrap().width();
    monitor_width
      - self.outer_gaps().left.to_pixels(monitor_width)
      - self.outer_gaps().right.to_pixels(monitor_width)
  }

  fn height(&self) -> i32 {
    let monitor_height = self.parent().unwrap().height();
    monitor_height
      - self.outer_gaps().top.to_pixels(monitor_height)
      - self.outer_gaps().bottom.to_pixels(monitor_height)
  }

  fn x(&self) -> i32 {
    let monitor_width = self.parent().unwrap().width();
    self.parent().unwrap().x()
      + self.outer_gaps().left.to_pixels(monitor_width)
  }

  fn y(&self) -> i32 {
    let monitor_height = self.parent().unwrap().height();
    self.parent().unwrap().y()
      + self.outer_gaps().top.to_pixels(monitor_height)
  }
}
