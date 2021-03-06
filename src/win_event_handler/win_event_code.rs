use strum_macros::EnumString;
#[derive(Clone, Copy, FromPrimitive, ToPrimitive, PartialEq, EnumString, Display, Debug)]
#[repr(u32)]
#[allow(dead_code)]
pub enum WinEventCode {
    ObjectAcceleratorChange = 0x8012,
    ObjectCloaked = 0x8017,
    ObjectContentScrolled = 0x8015,
    ObjectCreate = 0x8000,
    ObjectDefactionChange = 0x8011,
    ObjectDescriptionChange = 0x800D,
    ObjectDestroy = 0x8001,
    ObjectDragStart = 0x8021,
    ObjectDragCancel = 0x8022,
    ObjectDragComplete = 0x8023,
    ObjectDragEnter = 0x8024,
    ObjectDragLeave = 0x8025,
    ObjectDragDropped = 0x8026,
    ObjectEnd = 0x80FF,
    ObjectFocus = 0x8005,
    ObjectHelpChange = 0x8010,
    ObjectHide = 0x8003,
    ObjectHostedObjectsInvalidated = 0x8020,
    ObjectImeHide = 0x8028,
    ObjectImeShow = 0x8027,
    ObjectImeChange = 0x8029,
    ObjectInvoked = 0x8013,
    ObjectLiveRegionChanged = 0x8019,
    ObjectLocationChange = 0x800B,
    ObjectNameChange = 0x800C,
    ObjectParentChange = 0x800F,
    ObjectReorder = 0x8004,
    ObjectSelection = 0x8006,
    ObjectSelectionAdd = 0x8007,
    ObjectSelectionRemove = 0x8008,
    ObjectSelectionWithin = 0x8009,
    ObjectShow = 0x8002,
    ObjectStateChange = 0x800A,
    ObjectTextEditConversionTargetChanged = 0x8030,
    ObjectTextSelectionChanged = 0x8014,
    ObjectUncloaked = 0x8018,
    ObjectValueChange = 0x800E,
    SystemAlert = 0x0002,
    SystemArrangmentPreview = 0x8016,
    SystemCaptureEnd = 0x0009,
    SystemCaptureStart = 0x0008,
    SystemContextHelpEnd = 0x000D,
    SystemContextHelpStart = 0x000C,
    SystemDesktopSwitch = 0x0020,
    SystemDialogEnd = 0x0011,
    SystemDialogStart = 0x0010,
    SystemDragDropEnd = 0x000F,
    SystemDragDropStart = 0x000E,
    SystemEnd = 0x00FF,
    SystemForeground = 0x0003,
    SystemMenuPopupEnd = 0x0007,
    SystemMenuPopupStart = 0x0006,
    SystemMenuEnd = 0x0005,
    SystemMenuStart = 0x0004,
    SystemMinimizeEnd = 0x0017,
    SystemMinimizeStart = 0x0016,
    SystemMoveSizeEnd = 0x000B,
    SystemMoveSizeStart = 0x000A,
    SystemSrollingEnd = 0x0013,
    SystemScrollingStart = 0x0012,
    SystemSound = 0x0001,
    SystemSwitchEnd = 0x0015,
    SystemSwitchStart = 0x0014,
}
