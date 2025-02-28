//! Types for all DOM events.

use std::{borrow::Cow, marker::PhantomData};
use wasm_bindgen::convert::FromWasmAbi;

/// A trait for converting types into [web_sys events](web_sys).
pub trait EventDescriptor: Clone {
    /// The [`web_sys`] event type, such as [`web_sys::MouseEvent`].
    type EventType: FromWasmAbi;

    /// The name of the event, such as `click` or `mouseover`.
    fn name(&self) -> Cow<'static, str>;

    /// The key used for event delegation.
    fn event_delegation_key(&self) -> Cow<'static, str>;

    /// Indicates if this event bubbles. For example, `click` bubbles,
    /// but `focus` does not.
    ///
    /// If this method returns true, then the event will be delegated globally,
    /// otherwise, event listeners will be directly attached to the element.
    fn bubbles(&self) -> bool {
        true
    }
}

/// Overrides the [`EventDescriptor::bubbles`] method to always return
/// `false`, which forces the event to not be globally delegated.
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct undelegated<Ev: EventDescriptor>(pub Ev);

impl<Ev: EventDescriptor> EventDescriptor for undelegated<Ev> {
    type EventType = Ev::EventType;

    fn name(&self) -> Cow<'static, str> {
        self.0.name()
    }

    fn event_delegation_key(&self) -> Cow<'static, str> {
        self.0.event_delegation_key()
    }

    fn bubbles(&self) -> bool {
        false
    }
}

/// A custom event.
pub struct Custom<E: FromWasmAbi = web_sys::Event> {
    name: Cow<'static, str>,
    _event_type: PhantomData<E>,
}

impl<E: FromWasmAbi> Clone for Custom<E> {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            _event_type: PhantomData,
        }
    }
}

impl<E: FromWasmAbi> EventDescriptor for Custom<E> {
    type EventType = E;

    fn name(&self) -> Cow<'static, str> {
        self.name.clone()
    }

    fn event_delegation_key(&self) -> Cow<'static, str> {
        format!("$$${}", self.name).into()
    }

    fn bubbles(&self) -> bool {
        false
    }
}

impl<E: FromWasmAbi> Custom<E> {
    /// Creates a custom event type that can be used within
    /// [`HtmlElement::on`](crate::HtmlElement::on), for events
    /// which are not covered in the [`ev`](crate::ev) module.
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self {
            name: name.into(),
            _event_type: PhantomData,
        }
    }
}

macro_rules! generate_event_types {
  {$(
    $( #[$does_not_bubble:ident] )?
    $event:ident : $web_sys_event:ident
  ),* $(,)?} => {

    $(
        #[doc = concat!("The `", stringify!($event), "` event, which receives [", stringify!($web_sys_event), "](web_sys::", stringify!($web_sys_event), ") as its argument.")]
        #[derive(Copy, Clone)]
        #[allow(non_camel_case_types)]
        pub struct $event;

        impl EventDescriptor for $event {
          type EventType = web_sys::$web_sys_event;

          fn name(&self) -> Cow<'static, str> {
            stringify!($event).into()
          }

          fn event_delegation_key(&self) -> Cow<'static, str> {
            concat!("$$$", stringify!($event)).into()
          }

          $(
            generate_event_types!($does_not_bubble);
          )?
        }
    )*
  };

  (does_not_bubble) => {
    fn bubbles(&self) -> bool { false }
  }
}

generate_event_types! {
  // =========================================================
  // WindowEventHandlersEventMap
  // =========================================================
  #[does_not_bubble]
  afterprint: Event,
  #[does_not_bubble]
  beforeprint: Event,
  #[does_not_bubble]
  beforeunload: BeforeUnloadEvent,
  #[does_not_bubble]
  gamepadconnected: GamepadEvent,
  #[does_not_bubble]
  gamepaddisconnected: GamepadEvent,
  hashchange: HashChangeEvent,
  #[does_not_bubble]
  languagechange: Event,
  #[does_not_bubble]
  message: MessageEvent,
  #[does_not_bubble]
  messageerror: MessageEvent,
  #[does_not_bubble]
  offline: Event,
  #[does_not_bubble]
  online: Event,
  #[does_not_bubble]
  pagehide: PageTransitionEvent,
  #[does_not_bubble]
  pageshow: PageTransitionEvent,
  popstate: PopStateEvent,
  rejectionhandled: PromiseRejectionEvent,
  #[does_not_bubble]
  storage: StorageEvent,
  #[does_not_bubble]
  unhandledrejection: PromiseRejectionEvent,
  #[does_not_bubble]
  unload: Event,

  // =========================================================
  // GlobalEventHandlersEventMap
  // =========================================================
  #[does_not_bubble]
  abort: UiEvent,
  animationcancel: AnimationEvent,
  animationend: AnimationEvent,
  animationiteration: AnimationEvent,
  animationstart: AnimationEvent,
  auxclick: MouseEvent,
  beforeinput: InputEvent,
  #[does_not_bubble]
  blur: FocusEvent,
  #[does_not_bubble]
  canplay: Event,
  #[does_not_bubble]
  canplaythrough: Event,
  change: Event,
  click: MouseEvent,
  #[does_not_bubble]
  close: Event,
  compositionend: CompositionEvent,
  compositionstart: CompositionEvent,
  compositionupdate: CompositionEvent,
  contextmenu: MouseEvent,
  #[does_not_bubble]
  cuechange: Event,
  dblclick: MouseEvent,
  drag: DragEvent,
  dragend: DragEvent,
  dragenter: DragEvent,
  dragleave: DragEvent,
  dragover: DragEvent,
  dragstart: DragEvent,
  drop: DragEvent,
  #[does_not_bubble]
  durationchange: Event,
  #[does_not_bubble]
  emptied: Event,
  #[does_not_bubble]
  ended: Event,
  #[does_not_bubble]
  error: ErrorEvent,
  #[does_not_bubble]
  focus: FocusEvent,
  #[does_not_bubble]
  focusin: FocusEvent,
  #[does_not_bubble]
  focusout: FocusEvent,
  formdata: Event, // web_sys does not include `FormDataEvent`
  #[does_not_bubble]
  gotpointercapture: PointerEvent,
  input: Event,
  #[does_not_bubble]
  invalid: Event,
  keydown: KeyboardEvent,
  keypress: KeyboardEvent,
  keyup: KeyboardEvent,
  #[does_not_bubble]
  load: Event,
  #[does_not_bubble]
  loadeddata: Event,
  #[does_not_bubble]
  loadedmetadata: Event,
  #[does_not_bubble]
  loadstart: Event,
  lostpointercapture: PointerEvent,
  mousedown: MouseEvent,
  #[does_not_bubble]
  mouseenter: MouseEvent,
  #[does_not_bubble]
  mouseleave: MouseEvent,
  mousemove: MouseEvent,
  mouseout: MouseEvent,
  mouseover: MouseEvent,
  mouseup: MouseEvent,
  #[does_not_bubble]
  pause: Event,
  #[does_not_bubble]
  play: Event,
  #[does_not_bubble]
  playing: Event,
  pointercancel: PointerEvent,
  pointerdown: PointerEvent,
  #[does_not_bubble]
  pointerenter: PointerEvent,
  #[does_not_bubble]
  pointerleave: PointerEvent,
  pointermove: PointerEvent,
  pointerout: PointerEvent,
  pointerover: PointerEvent,
  pointerup: PointerEvent,
  #[does_not_bubble]
  progress: ProgressEvent,
  #[does_not_bubble]
  ratechange: Event,
  reset: Event,
  #[does_not_bubble]
  resize: UiEvent,
  #[does_not_bubble]
  scroll: Event,
  securitypolicyviolation: SecurityPolicyViolationEvent,
  #[does_not_bubble]
  seeked: Event,
  #[does_not_bubble]
  seeking: Event,
  select: Event,
  #[does_not_bubble]
  selectionchange: Event,
  selectstart: Event,
  slotchange: Event,
  #[does_not_bubble]
  stalled: Event,
  submit: SubmitEvent,
  #[does_not_bubble]
  suspend: Event,
  #[does_not_bubble]
  timeupdate: Event,
  #[does_not_bubble]
  toggle: Event,
  touchcancel: TouchEvent,
  touchend: TouchEvent,
  touchmove: TouchEvent,
  touchstart: TouchEvent,
  transitioncancel: TransitionEvent,
  transitionend: TransitionEvent,
  transitionrun: TransitionEvent,
  transitionstart: TransitionEvent,
  #[does_not_bubble]
  volumechange: Event,
  #[does_not_bubble]
  waiting: Event,
  webkitanimationend: Event,
  webkitanimationiteration: Event,
  webkitanimationstart: Event,
  webkittransitionend: Event,
  wheel: WheelEvent,

  // =========================================================
  // WindowEventMap
  // =========================================================
  DOMContentLoaded: Event,
  #[does_not_bubble]
  devicemotion: DeviceMotionEvent,
  #[does_not_bubble]
  deviceorientation: DeviceOrientationEvent,
  #[does_not_bubble]
  orientationchange: Event,

  // =========================================================
  // DocumentAndElementEventHandlersEventMap
  // =========================================================
  copy: Event, // ClipboardEvent is unstable
  cut: Event, // ClipboardEvent is unstable
  paste: Event, // ClipboardEvent is unstable

  // =========================================================
  // DocumentEventMap
  // =========================================================
  fullscreenchange: Event,
  fullscreenerror: Event,
  pointerlockchange: Event,
  pointerlockerror: Event,
  #[does_not_bubble]
  readystatechange: Event,
  visibilitychange: Event,
}

// Export `web_sys` event types
pub use web_sys::{
    AnimationEvent, BeforeUnloadEvent, CompositionEvent, DeviceMotionEvent,
    DeviceOrientationEvent, DragEvent, ErrorEvent, FocusEvent, GamepadEvent,
    HashChangeEvent, InputEvent, KeyboardEvent, MouseEvent,
    PageTransitionEvent, PointerEvent, PopStateEvent, ProgressEvent,
    PromiseRejectionEvent, SecurityPolicyViolationEvent, StorageEvent,
    SubmitEvent, TouchEvent, TransitionEvent, UiEvent, WheelEvent,
};
